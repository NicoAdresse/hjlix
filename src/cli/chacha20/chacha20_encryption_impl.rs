// src/cli/chacha20/chacha20_encryption_impl.rs

use crate::cli::chacha20::chacha20_types::ChaCha20Types;
use crate::cli::traits::generate_chacha20_key::GenerateChaCha20Key;
use crate::cli::err::chacha20_err::ChaCha20Error;
use crate::cli::err::io_err::IOError;
use crate::cli::consts::constants::FILE_NAME_CHACHA20_KEY;
use chacha20poly1305::{Nonce, aead::Generate};
use std::io::Write;
use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};
use std::path::Path;
use std::fs::OpenOptions;

#[derive(Debug, Clone, PartialEq)]
pub struct ChaCha20EncryptionImpl {
    texts: Vec<String>,
    chacha20_type: ChaCha20Types,
    no_verbose: bool,
    gen_key: bool,
}

impl ChaCha20EncryptionImpl {
    pub fn new(texts: Vec<String>, chacha20_type: ChaCha20Types, no_verbose: bool, gen_key: bool) -> Self {
        Self { texts, chacha20_type, no_verbose, gen_key }
    }

    pub fn execute(&self) -> anyhow::Result<()> {
        let key = self.chacha20_type.generate_chacha20_key();

        if self.gen_key {
            let path = Path::new(FILE_NAME_CHACHA20_KEY);

            if path.exists() {
                std::fs::remove_file(path)
                    .map_err(|_| IOError::FileNotFound)?;
            }

            let mut file = OpenOptions::new()
                .create_new(true)
                .write(true)
                .mode(0o600)
                .open(FILE_NAME_CHACHA20_KEY)
                .map_err(|_| IOError::FailedToCreateFile)?;

            let mut perms = file
                .metadata()
                .map_err(|_| IOError::FailedToGetMetadata)?
                .permissions();
            perms.set_mode(0o600);

            file.set_permissions(perms)
                .map_err(|_| IOError::FailedToSetPermissions)?;
            file.write_all(&key)
                .map_err(|_| IOError::FailedToWrite)?;
            file.sync_all()
                .map_err(|_| IOError::FailedToWrite)?;

        }

        for text in &self.texts {
            let nonce = Nonce::generate();
            let ciphertext = self.chacha20_type.encrypt(&key, &nonce, text.as_bytes())
                .map_err(|_| ChaCha20Error::FailedToEncrypt)?;
            let result = hex::encode(ciphertext);

            if !self.no_verbose {
                println!("Original: {}", text);
                println!("Result: {}", result);
            } else {
                println!("{}", result);
            }
        }

        Ok(())
    }
}