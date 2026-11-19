// src/cli/chacha20/chacha20_decryption_impl.rs

use std::path::{PathBuf, Path};
use chacha20poly1305::Nonce;
use crate::cli::err::io_err::IOError;
use crate::cli::err::chacha20_err::ChaCha20Error;
use crate::cli::chacha20::chacha20_types::identify_chacha20_type;

#[derive(Debug, Clone, PartialEq)]
pub struct ChaCha20DecryptionImpl {
    ciphertexts: Vec<String>,
    chacha20_key_file: PathBuf,
    no_verbose: bool,
}

impl ChaCha20DecryptionImpl {
    pub fn new(ciphertexts: Vec<String>, chacha20_key_file: PathBuf, no_verbose: bool) -> Self {
        Self { ciphertexts, chacha20_key_file, no_verbose }
    }

    fn load_key(&self, path: impl AsRef<Path>) -> anyhow::Result<Vec<u8>> {
        let key = std::fs::read(path)
            .map_err(|_| IOError::FailedToReadFile)?;

        if key.len() != 32 {
            return Err(ChaCha20Error::InvalidKeyLength.into());
        }
    
        Ok(key)
    }

    pub fn execute(&self) -> anyhow::Result<()> {
        if self.ciphertexts.is_empty() {
            return Err(IOError::NoGivenFile.into());
        }

        let key = self.load_key(&self.chacha20_key_file)?;

        for ciphertext in &self.ciphertexts {
            let raw_bytes = hex::decode(ciphertext)
                .map_err(|_| ChaCha20Error::FailedToDecrypt)?;

            if raw_bytes.len() < 28 {
                return Err(ChaCha20Error::FailedToDecrypt.into());
            }

            let (nonce_bytes, encrypted_payload) = raw_bytes.split_at(12);
            let chacha20_type = identify_chacha20_type(&key, nonce_bytes, encrypted_payload)
                .ok_or(ChaCha20Error::FailedToFigureOutChaCha20Type)?;
            
            let nonce = Nonce::try_from(nonce_bytes)
                .map_err(|_| ChaCha20Error::FailedToGenerateNonce)?;
            let decrypted_bytes = chacha20_type.decrypt(&key, &nonce, encrypted_payload)
                .map_err(|_| ChaCha20Error::FailedToDecrypt)?;
            let result = String::from_utf8(decrypted_bytes)
                .map_err(|_| ChaCha20Error::FailedToDecrypt)?;

            if !self.no_verbose {
                println!("Original: {}", ciphertext);
                println!("Result: {}", result);
            } else {
                println!("{}", result);
            }
        }

        Ok(())
    }
}
