// src/cli/aes_gcm/aes_gcmdecryption_impl.rs

use crate::cli::aes_gcm::aes_gcm_types::AESGCMTypes;
use crate::cli::err::{aes_err::AESError, io_err::IOError};
use crate::cli::traits::generate_aes_key::HjlixGenerateAESKey;
use aes_gcm::{Nonce, Key, Aes128Gcm, KeyInit, aead::{Generate, Aead}};

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
                for text in &self.ciphertexts {
                    let text_bytes = text.as_bytes();
                    let aes_key = Key::<Aes128Gcm>::try_from(key.as_slice())
                        .map_err(|_| AESError::FailedToCreateKey)?;
                    let cipher = Aes128Gcm::new(&aes_key);
                    let nonce = Nonce::generate();
                    let decrypted_text = cipher.decrypt(&nonce, text_bytes)
                        .map_err(|_| AESError::FailedToDecrypt)?;
                    let result = hex::encode(decrypted_text);

                    if !self.no_verbose {
                        println!("Original: {}", text);
                        println!("Result: {}", result);
                    } else {
                        println!("{}", result);
                    }
                }
            },
            AESGCMTypes::AES256GCM => todo!(),
        }


        Ok(())
    }
}