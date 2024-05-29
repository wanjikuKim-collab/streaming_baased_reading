
use ring::aead::{Aad, Nonce, SealingKey, UnboundKey, AES_256_GCM};
use ring::rand::{SecureRandom, SystemRandom};
use hex;

pub fn encrypt_file(data: &Vec<u8>){
    //**Generate ENCRYPTION KEY
    let rand = SystemRandom::new();
    
    //Generate a random number
    let mut key_bytes = vec![0;AES_256_GCM.key_len()];//32 byte vector
    println!("key_bytes in binary, {:?}", key_bytes);
    
    rand.fill(&mut key_bytes);
    println!("key_bytes in hex, {}", hex::encode(&key_bytes));

  }