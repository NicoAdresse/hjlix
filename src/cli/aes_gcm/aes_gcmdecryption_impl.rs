// src/cli/aes_gcm/aes_gcmdecryption_impl.rs

use crate::cli::aes_gcm::aes_gcm_types::AESGCMTypes;
use crate::cli::err::{aes_err::AESError, io_err::IOError};
use crate::cli::traits::generate_aes_key::HjlixGenerateAESKey;
use aes_gcm::{Nonce, Key, Aes128Gcm, Aes256Gcm, KeyInit, aead::Aead};

#[derive(Debug, Clone, PartialEq)]
pub struct AesGcmDecryptionImpl {
    ciphertexts: Vec<String>,
    aes_gcm_type: AESGCMTypes,
    no_verbose: bool,
}

impl AesGcmDecryptionImpl {
    pub fn new(ciphertexts: Vec<String>, aes_gcm_type: AESGCMTypes, no_verbose: bool) -> Self {
        Self { ciphertexts, aes_gcm_type, no_verbose }
    }

    pub fn execute(&self) -> anyhow::Result<()> {
        if self.ciphertexts.is_empty() {
            return Err(IOError::NoGivenFile.into());
        }

        let key = self.aes_gcm_type.generate_key_for_aes_type();
        let aes_gcm_type = self.aes_gcm_type.figure_out_type(&key)
            .map_err(|_| AESError::FailedToFigureOutType)?;

        match aes_gcm_type {
            AESGCMTypes::AES128GCM => {
                let aes_key = Key::<Aes128Gcm>::try_from(key.as_slice())
                    .map_err(|_| AESError::FailedToCreateKey)?;
                let cipher = Aes128Gcm::new(&aes_key);

                for text in &self.ciphertexts {
                    let raw_bytes = hex::decode(text)
                        .map_err(|_| AESError::FailedToDecrypt)?;

                    if raw_bytes.len() < 28 {
                        return Err(AESError::FailedToDecrypt.into());
                    }

                    let (nonce_bytes, ciphertext_with_tag) = raw_bytes.split_at(12);
                    let nonce = Nonce::try_from(nonce_bytes)
                        .map_err(|_| AESError::FailedToGenerateNonce)?;

                    let decrypted_bytes = cipher.decrypt(&nonce, ciphertext_with_tag)
                        .map_err(|_| AESError::FailedToDecrypt)?;
                    
                    let result = String::from_utf8(decrypted_bytes)
                        .map_err(|_| AESError::FailedToDecrypt)?;

                    if !self.no_verbose {
                        println!("Original (Hex): {}", text);
                        println!("Result: {}", result);
                    } else {
                        println!("{}", result);
                    }
                }
            },
            AESGCMTypes::AES256GCM => {
                let aes_key = Key::<Aes256Gcm>::try_from(key.as_slice())
                    .map_err(|_| AESError::FailedToCreateKey)?;
                let cipher = Aes256Gcm::new(&aes_key);

                for text in &self.ciphertexts {
                    let raw_bytes = hex::decode(text).map_err(|_| AESError::FailedToDecrypt)?;
                    if raw_bytes.len() < 28 {
                        return Err(AESError::FailedToDecrypt.into());
                    }

                    let (nonce_bytes, ciphertext_with_tag) = raw_bytes.split_at(12);
                    let nonce = Nonce::try_from(nonce_bytes)
                        .map_err(|_| AESError::FailedToGenerateNonce)?;

                    let decrypted_bytes = cipher.decrypt(&nonce, ciphertext_with_tag)
                        .map_err(|_| AESError::FailedToDecrypt)?;
                    
                    let result = String::from_utf8(decrypted_bytes).map_err(|_| AESError::FailedToDecrypt)?;

                    if !self.no_verbose {
                        println!("Original (Hex): {}", text);
                        println!("Result: {}", result);
                    } else {
                        println!("{}", result);
                    }
                }
            },
        }

        Ok(())
    }
}
