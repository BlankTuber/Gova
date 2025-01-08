use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Key, Nonce,
};
use sha2::{Sha256, Digest};
use base64::{Engine as _, engine::general_purpose::STANDARD as base64};

pub struct FileCrypto {
    cipher: Aes256Gcm,
}

impl FileCrypto {
    pub fn new(master_password: &str) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(master_password.as_bytes());
        let key = hasher.finalize();
        
        let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&key));
        Self { cipher }
    }

    pub fn encrypt_to_string(&self, data: &[u8]) -> Result<String, &'static str> {
        let nonce = [0u8; 12];
        
        let encrypted = self.cipher.encrypt(Nonce::from_slice(&nonce), data).map_err(|_| "Encryption failed")?;

        let mut result = Vec::with_capacity(12 + encrypted.len());
        result.extend_from_slice(&nonce);
        result.extend_from_slice(&encrypted);
        
        Ok(base64.encode(result))
    }

    pub fn decrypt_from_string(&self, encoded: &str) -> Result<Vec<u8>, &'static str> {
        let combined = base64.decode(encoded).map_err(|_| "Invalid base64")?;
        
        if combined.len() < 12 {
            return Err("Data too short");
        }
        
        let (nonce, encrypted) = combined.split_at(12);
        
        self.cipher.decrypt(Nonce::from_slice(nonce), encrypted).map_err(|_| "Decryption failed")
    }
}