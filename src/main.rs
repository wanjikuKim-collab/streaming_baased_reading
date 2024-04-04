mod mp4_reader;

use mp4_reader::{read_file, MP4ReaderError};
use ring::{aead, cipher}; // Import necessary modules from ring

fn encrypt_chunks(chunks: &[Vec<u8>], key: &[u8]) -> Result<Vec<Vec<u8>>, MP4ReaderError> {
    let aad = aead::Aad::empty(); // Authenticated Additional Data (optional)
  
    let mut sealed_chunks = Vec::new();
    for chunk in chunks.iter() {
      let nonce = ring::rand::SystemRandom::new()?.generate::<[u8; 12]>()?; // Generate unique nonce
      let sealing_key = aead::SealingKey::new(&ring::hmac::Key::new(ring::hmac::HMAC_SHA256, key)?, &aad)?;
      let ciphertext = aead::encrypt(&sealing_key, &nonce, chunk, &aad)?;
      sealed_chunks.push(nonce.to_vec());
      sealed_chunks.extend_from_slice(&ciphertext);
    }
  
    Ok(sealed_chunks)
  }
  


  fn main() {
    match read_file("/home/wanjiku/Development/code/rust/streaming_baased_reading/src/How to use FFMPEG.mp4") {
      Ok(data) => {
        // Read the key from a file (implement logic to read the key)
        let key = // ... (Code to read the key from a file)
        
        // Encrypt the data chunks
        match encrypt_chunks(&data, &key) {
          Ok(encrypted_chunks) => {
            // Write the encrypted data to a new file (implement logic)
            // ... (Code to write encrypted data to a file)
            println!("Data encrypted successfully!");
          }
          Err(e) => println!("Error encrypting data: {:?}", e),
        }
      }
      Err(e) => println!("Error reading file: {:?}", e),
    }
  }
  
