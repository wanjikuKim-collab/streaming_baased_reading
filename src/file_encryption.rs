
use ring::aead::{Aad, Nonce, SealingKey, UnboundKey, AES_256_GCM};
use ring::rand::{SecureRandom, SystemRandom};

fn encrypt_file(input_file_path: &str, output_file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file_data = mp4_reader::read_file(input_file_path).unwrap();

    let mut rng = SystemRandom::new();
    let mut key_bytes = [0u8; 32];
    rng.fill(&mut key_bytes)?;

    let mut nonce_bytes = [0u8; 12];
    rng.fill(&mut nonce_bytes)?;

    let key = UnboundKey::new(&AES_256_GCM, &key_bytes)?;
    let nonce = Nonce::try_assume_unique_for_key(&nonce_bytes)?;
    let sealing_key = SealingKey::new(key, &nonce)?;

    let mut encrypted_data = Vec::new();
    let mut tag = vec![0u8; 16];

    for chunk in file_data.chunks(1024 * 1024) {
        let aad = Aad::empty();
        let mut buffer = chunk.to_vec();
        let encrypted_len = sealing_key.seal_in_place_append_tag(&mut buffer, &aad)?;
        encrypted_data.extend_from_slice(&buffer[..encrypted_len]);
        tag.copy_from_slice(&buffer[encrypted_len..]);
    }

    let mut output_file = std::fs::File::create(output_file_path)?;
    output_file.write_all(&encrypted_data)?;
    output_file.write_all(&tag)?;

    Ok(())
}
