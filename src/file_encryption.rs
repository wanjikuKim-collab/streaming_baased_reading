
use ring::aead::{Aad, BoundKey, Nonce, SealingKey, UnboundKey, AES_256_GCM, NONCE_LEN};
use ring::error::Unspecified;
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
    
    // Create a new AEAD key without a designated role or nonce sequence
    let unbound_key: Result<UnboundKey, ring::error::Unspecified> = UnboundKey::new(&AES_256_GCM, &key_bytes);// takes in 2 arguments, the algorithim and the 32 key byte vector

    //Create unit struct for the counter nonce sequence
    struct CounterNonceSequence(u32);

    impl NonceSequence for CounterNonceSequence{
      // called once for each seal operation
      fn advance(&mut self)-> Result<Nonce, Unspecified>{
        let mut nonce_bytes = vec![0; NONCE_LEN];

        let bytes = self.0.to_be_bytes();
        nonce_bytes[8..].copy_from_slice(&bytes);
        println!("nonce_bytes = {}", hex::encode(&nonce_bytes));

        self.0 += 1; //advance the counter
        Nonce::try_assume_unique_for_key(&nonce_bytes)
      }
    }

    //Create a new NonceSequence type that generates nonces
    let nonce_sequence = CounterNonceSequence(1);

    //Create a new AEAD key for encrypting and signing("sealing"),bound to a nonce sequence
    // This sealing key can be used multiple times
    let sealing_key = SealingKey::new(unbound_key, nonce_sequence);// takes in 2 arguments a key and a nonce
  }