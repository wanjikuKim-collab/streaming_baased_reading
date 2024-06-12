use hex;
use std::fs::File;
use std::io::Write;
use ring::aead::{
    Aad, BoundKey, Nonce, NonceSequence, SealingKey, Tag, UnboundKey, AES_256_GCM, NONCE_LEN,
};
use ring::error::Unspecified;
use ring::rand::{SecureRandom, SystemRandom};

use crate::mp4_reader::read_file;
use crate::mp4_reader::MP4ReaderError;

//Create unit struct for the counter nonce sequence
pub struct CounterNonceSequence(u32);

impl NonceSequence for CounterNonceSequence {
    // This method generates a new nonce for each encryption operation.
    fn advance(&mut self) -> Result<Nonce, Unspecified> {
        let mut nonce_bytes = vec![0; NONCE_LEN];

        let bytes = self.0.to_be_bytes();
        nonce_bytes[8..].copy_from_slice(&bytes);
        println!("nonce_bytes = {}", hex::encode(&nonce_bytes));

        self.0 += 1; //advance the counter
        Nonce::try_assume_unique_for_key(&nonce_bytes)
    }
}

// Function to encrypt file data and write the encrypted data to an output file.
pub fn encrypt_file(
    data: &Vec<u8>,
    output_file_path: &str,
    associated_data: &[u8],
) -> Result<(), MP4ReaderError> {
    // Create a secure random number generator.
    let rand = SystemRandom::new();

    // Generate a 256-bit (32-byte) key for AES-256-GCM.
    let mut key_bytes = vec![0; AES_256_GCM.key_len()]; //32 byte vector
    println!("key_bytes in binary, {:?}", key_bytes);

    rand.fill(&mut key_bytes);
    println!("key_bytes in hex, {}", hex::encode(&key_bytes));

    // Create a new AEAD key using the generated key bytes
    let unbound_key = match UnboundKey::new(&AES_256_GCM, &key_bytes) {
        Ok(key) => key,
        Err(err) => panic!("Failed to create unbound key: {:?}", err),
    }; // takes in 2 arguments, the algorithim and the 32 key byte vector

    // Create a nonce sequence starting with the counter value 1.
    let nonce_sequence = CounterNonceSequence(1);

    // Create a sealing key (used for encryption) bound to the nonce sequence.
    // This sealing key can be used multiple times
    let mut sealing_key: SealingKey<CounterNonceSequence> =
        SealingKey::new(unbound_key, nonce_sequence); // takes in 2 arguments a key and a nonce

    // This data will be authenticated but not encrypted
    // let associated_data: Aad<&[u8; 22]> = Aad::from(b"additional public data");

    // Vector to hold the encrypted data.
    let mut encrypted_data = Vec::new();

    // Encrypt the data in 1 MB chunks.
    for chunk in data.chunks(1024 * 1024) {
      let mut in_out = chunk.to_vec();
      match sealing_key.seal_in_place_separate_tag(Aad::from(associated_data), &mut in_out) {
          Ok(tag) => {
              encrypted_data.extend_from_slice(&in_out);
              encrypted_data.extend_from_slice(tag.as_ref());
          }
          Err(e) => return Err(MP4ReaderError::EncryptionError(e)),
      }
  }

    let mut output_file = File::create(output_file_path).map_err(|e| MP4ReaderError::IoError(e))?;
    output_file.write_all(&encrypted_data)
        .map_err(|e| MP4ReaderError::IoError(e))?;
    Ok(())
}
