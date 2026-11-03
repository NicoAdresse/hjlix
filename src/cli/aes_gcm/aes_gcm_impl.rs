// src/cli/aes_gcm/aes_gcm_impl.rs

use aes_gcm::KeyInit;
use aes_gcm::aead::Aead;
use aes_gcm::{Key, Aes128Gcm, Aes256Gcm, aead::Generate, Nonce};
use crate::cli::aes_gcm::aes_gcm_types::AESGCMTypes;
use crate::cli::err::aes_err::AESError;

#[derive(Debug, Clone)]
pub struct AESGCMImpl {
    strings: Vec<String>,
    aes_gcm_type: AESGCMTypes,
}

impl AESGCMImpl {
    pub fn new(strings: Vec<String>, aes_gcm_type: AESGCMTypes) -> Self {
        Self { strings, aes_gcm_type }
    }

    fn generate_key_for_aes_type(&self) -> Vec<u8> {
        match &self.aes_gcm_type {
            AESGCMTypes::AES128GCM(_) => {
                let key = Key::<Aes128Gcm>::generate();
                key.to_vec()
            }
            AESGCMTypes::AES256GCM(_) => {
                let key = Key::<Aes256Gcm>::generate();
                key.to_vec()
            }
        }
    }

    pub fn execute(&self) -> Result<(), AESError> {
        let key = self.generate_key_for_aes_type();
        let aes_gcm_type = self.aes_gcm_type.figure_out_type(&key)
            .map_err(|_| AESError::FailedToFigureOutType)?;

        match aes_gcm_type {
            AESGCMTypes::AES128GCM(_) => {
                let aes_key = Key::<Aes128Gcm>::try_from(key.as_slice())
                    .map_err(|_| AESError::FailedToCreateKey)?;
                let cipher = Aes128Gcm::new(&aes_key);

                for string in &self.strings {
                    let nonce = Nonce::generate();
                    let ciphertext = cipher.encrypt(&nonce, string.as_ref())
                        .map_err(|_| AESError::FailedToCreateCiphertext)?;
                    let result = hex::encode(ciphertext);

                    println!("Original: {}", string);
                    println!("Result: {}", result);
                }
            },
            AESGCMTypes::AES256GCM(_) => {
                let aes_key = Key::<Aes256Gcm>::try_from(key.as_slice())
                    .map_err(|_| AESError::FailedToCreateKey)?;
                let cipher = Aes256Gcm::new(&aes_key);

                for string in &self.strings {
                    let nonce = Nonce::generate();
                    let ciphertext = cipher.encrypt(&nonce, string.as_ref())
                        .map_err(|_| AESError::FailedToCreateCiphertext)?;
                    let result = hex::encode(ciphertext);

                    println!("Original: {}", string);
                    println!("Result: {}", result);
                }

            }
        };

        Ok(())
    }
}