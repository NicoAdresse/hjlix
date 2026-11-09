// src/cli/aes_gcm/aes_gcmdecryptionfile_impl.rs

use std::fs::File;
use std::path::{Path, PathBuf};
use aes_gcm::{Nonce, Key, Aes128Gcm, Aes256Gcm, KeyInit, aead::Aead};
use crate::cli::err::{io_err::IOError, aes_err::AESError};
use crate::cli::aes_gcm::aes_gcm_types::AESGCMTypes;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone, PartialEq)]
pub struct AesGcmDecryptionFileImpl {
    files: Vec<PathBuf>,
    aes_key: PathBuf,
    no_verbose: bool,
}

impl AesGcmDecryptionFileImpl {
    pub fn new(files: Vec<PathBuf>, aes_key: PathBuf, no_verbose: bool) -> Self {
        Self { files, aes_key, no_verbose }
    }

    fn load_aes_key(&self, path: impl AsRef<Path>) -> anyhow::Result<Vec<u8>> {
        let key = std::fs::read(path)
            .map_err(|_| IOError::FailedToReadFile)?;

        if key.is_empty() {
            return Err(AESError::InvalidKeyLength.into());
        }

        Ok(key)
    }

    fn do_aes_128_gcm(&self, key: &[u8]) -> anyhow::Result<()> {
        let aes_key = Key::<Aes128Gcm>::try_from(key)
            .map_err(|_| AESError::FailedToCreateKey)?;
        let cipher = Aes128Gcm::new(&aes_key);

        for file_path in &self.files {
            let file = File::open(file_path)
                .map_err(|_| IOError::FailedToOpenFile)?;
            let buf_reader = BufReader::new(file);

            for line in buf_reader.lines() {
                let line = line.map_err(|_| IOError::FailedToReadLines)?;
                let raw_bytes = hex::decode(&line)
                    .map_err(|_| AESError::FailedToDecrypt)?;

                if raw_bytes.len() < 28 {
                    return Err(AESError::InvalidKeyLength.into());
                }

                let (nonce_bytes, ciphertext_with_tag) = raw_bytes.split_at(12);
                let nonce = Nonce::try_from(nonce_bytes)
                    .map_err(|_| AESError::FailedToGenerateNonce)?;
                let decrypted_bytes = cipher.decrypt(&nonce, ciphertext_with_tag)
                    .map_err(|_| AESError::FailedToDecrypt)?;
                
                let result = String::from_utf8(decrypted_bytes)
                    .map_err(|_| AESError::FailedToDecrypt)?;

                if !self.no_verbose {
                    println!("Original: {}", line);
                    println!("Result: {}", result);
                } else {
                    println!("{}", result);
                }
            }
        }      

        Ok(())
    }

    fn do_aes_256_gcm(&self, key: &[u8]) -> anyhow::Result<()> {
        let aes_key = Key::<Aes256Gcm>::try_from(key)
            .map_err(|_| AESError::FailedToCreateKey)?;
        let cipher = Aes256Gcm::new(&aes_key);

        for file_path in &self.files {
            let file = File::open(file_path)
                .map_err(|_| IOError::FailedToOpenFile)?;
            let buf_reader = BufReader::new(file);

            for line in buf_reader.lines() {
                let line = line.map_err(|_| IOError::FailedToReadLines)?;
                let raw_bytes = hex::decode(&line)
                    .map_err(|_| AESError::FailedToDecrypt)?;

                if raw_bytes.len() < 28 {
                    return Err(AESError::InvalidKeyLength.into());
                }

                let (nonce_bytes, ciphertext_with_tag) = raw_bytes.split_at(12);
                let nonce = Nonce::try_from(nonce_bytes)
                    .map_err(|_| AESError::FailedToGenerateNonce)?;
                let decrypted_bytes = cipher.decrypt(&nonce, ciphertext_with_tag)
                    .map_err(|_| AESError::FailedToDecrypt)?;
                
                let result = String::from_utf8(decrypted_bytes)
                    .map_err(|_| AESError::FailedToDecrypt)?;

                if !self.no_verbose {
                    println!("Original: {}", line);
                    println!("Result: {}", result);
                } else {
                    println!("{}", result);
                }
            }
        }      

        Ok(())
    }

    pub fn execute(&self) -> anyhow::Result<()> {
        if self.files.is_empty() {
            return Err(IOError::NoGivenFile.into());
        }

        let key = self.load_aes_key(&self.aes_key)?;
        let aes_type = AESGCMTypes::figure_out_type(&key)
            .map_err(|_| AESError::FailedToFigureOutType)?;

        match aes_type {
            AESGCMTypes::AES128GCM => self.do_aes_128_gcm(&key)?,
            AESGCMTypes::AES256GCM => self.do_aes_256_gcm(&key)?,
        }

        Ok(())
    }
}