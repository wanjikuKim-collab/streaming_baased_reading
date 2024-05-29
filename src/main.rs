mod mp4_reader;
mod file_encryption;


use std::io::Write;
// use ring::aead::BoundKey;
use std::fs;

// fn encrypt_file(input_file_path: &str, output_file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
//     let file_data = mp4_reader::read_file(input_file_path).unwrap();

//     let mut rng = SystemRandom::new();
//     let mut key_bytes = [0u8; 32];
//     rng.fill(&mut key_bytes).unwrap();

//     let mut nonce_bytes = [0u8; 12];
//     rng.fill(&mut nonce_bytes)?;

//     let key = UnboundKey::new(&AES_256_GCM, &key_bytes);
//     let nonce = Nonce::try_assume_unique_for_key(&nonce_bytes);
//     let sealing_key = SealingKey::new(key, &nonce).unwrap();

//     let mut encrypted_data = Vec::new();
//     let mut tag = vec![0u8; 16];

//     for chunk in file_data.chunks(1024 * 1024) {
//         let aad = Aad::empty();
//         let mut buffer = chunk.to_vec();
//         let encrypted_len = sealing_key.seal_in_place_append_tag(&mut buffer, &aad)?;
//         encrypted_data.extend_from_slice(&buffer[..encrypted_len]);
//         tag.copy_from_slice(&buffer[encrypted_len..]);
//     }

//     let mut output_file = std::fs::File::create(outpuuse ring::aead::{Aad, NonceSequence, SealingKey, UnboundKey, AES_256_GCM};
//       // let mut output_file =fs::File
//         use ring::rand::SystemRandom;
        
        
//         pub fn encrypt_file_data(
//           data: &[u8],
//           key: &[u8; 32],
//         ) -> Result<Vec<u8>, ring::error::Unspecified> {
//           let mut random = SystemRandom::new()?;
//           let mut nonce_bytes = [0u8; 12]; // Change to desired nonce size
//           random.fill(&mut nonce_bytes)?;
//           let nonce = NonceSequence::from_slice(&nonce_bytes)?;
        
//           let key = UnboundKey::new(&AES_256_GCM, key)?;
//           let sealing_key = SealingKey::new(key.as_ref(), &nonce)?;
        
//           let aad = Aad::empty();
//           let encrypted_data = sealing_key.seal_in_place_append_tag(data, &aad)?;
        
//           Ok(encrypted_data)
//         }
//         t_file_path)?;
//     output_file.write_all(&encrypted_data)?;
//     output_file.write_all(&tag)?;

//     Ok(())
// }




use mp4_reader::read_file;
fn main() {
  match read_file("/home/wanjiku/Development/code/rust/streaming_baased_reading/src/How to use FFMPEG.mp4") {
      Ok(data) => {
      //Process the data (e'g print the bytes)
      println!("Read the bytes: {:?}", &data);
      }
      Err(e) => {
      println!("Error reading file: {:?}", e)
      }      
 } 
}