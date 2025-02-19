use ring::aead::{Aad, BoundKey, LessSafeKey, Nonce, UnboundKey, AES_256_GCM, NONCE_LEN};
use ring::rand::{SecureRandom, SystemRandom};
use ring::{aead, error};
use std::str;

pub fn encrypt(password: &str, key: &[u8]) -> Result<Vec<u8>, error::Unspecified> {
    let rng = SystemRandom::new();
    let mut nonce = [0u8; NONCE_LEN];
    rng.fill(&mut nonce)?;

    let unbound_key = UnboundKey::new(&AES_256_GCM, key)?;
    let mut key = LessSafeKey::new(unbound_key);

    let mut password_bytes = password.as_bytes().to_vec();
    key.seal_in_place_append_tag(aead::Aad::empty(), &mut password_bytes)?;

    let mut encrypted_data = nonce.to_vec();
    encrypted_data.extend(password_bytes);
    Ok(encrypted_data)
}

pub fn decrypt(encrypted_data: &[u8], key: &[u8]) -> Result<String, error::Unspecified> {
    let unbound_key = UnboundKey::new(&AES_256_GCM, key)?;
    let mut key = LessSafeKey::new(unbound_key);

    let nonce = Nonce::try_assume_unique_for_key(&encrypted_data[..NONCE_LEN])?;
    let mut data = encrypted_data[NONCE_LEN..].to_vec();

    key.open_in_place(aead::Aad::empty(), &mut data)?;
    Ok(String::from_utf8_lossy(&data).to_string())
}
