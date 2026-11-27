// src/cli/chacha20/chacha20_file_impl.rs

use std::fs::{OpenOptions, File};
use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};
use std::io::{BufRead, BufReader, Write};
use std::path::{PathBuf, Path};
use aes_gcm::aead::Generate;
use chacha20poly1305::Nonce;
use crate::cli::chacha20::chacha20_types::ChaCha20Types;
use crate::cli::err::{io_err::IOError, chacha20_err::ChaCha20Error};
use crate::cli::consts::constants::FILE_NAME_CHACHA20_KEY;
use crate::cli::traits::generate_chacha20_key::GenerateChaCha20Key;

#[derive(Debug, Clone, PartialEq)]
pub struct ChaCha20EncryptionFileImpl {
    files: Vec<PathBuf>,
    chacha20_type: ChaCha20Types,
    gen_key: bool,
    no_verbose: bool,
}

impl ChaCha20EncryptionFileImpl {
    pub fn new(files: Vec<PathBuf>, chacha20_type: ChaCha20Types, gen_key: bool, no_verbose: bool) -> Self {
        Self { files, chacha20_type, gen_key, no_verbose }
    }

    pub fn execute(&self) -> anyhow::Result<()> {
        let path = Path::new(FILE_NAME_CHACHA20_KEY);

        let key = if self.gen_key {
            let new_key = self.chacha20_type.generate_chacha20_key();

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
            file.write_all(&new_key)
                .map_err(|_| IOError::FailedToWrite)?;
            file.sync_all()
                .map_err(|_| IOError::FailedToWrite)?;

            new_key
        } else {
            let existing_key = std::fs::read(path)
                .map_err(|_| IOError::FailedToReadFile)?;

            if existing_key.len() != 32 {
                return Err(ChaCha20Error::InvalidKeyLength.into());
            }

            existing_key
        };

        for file in &self.files {
            let file = File::open(file)
                .map_err(|_| IOError::FailedToOpenFile)?;
            let buf_reader = BufReader::new(file);

            for line in buf_reader.lines() {
                let line = line.map_err(|_| IOError::FailedToReadLines)?;
                let nonce = Nonce::generate();
                let ciphertext = self.chacha20_type.encrypt(&key, &nonce, line.as_bytes())
                    .map_err(|_| ChaCha20Error::FailedToEncrypt)?;

                let mut combined = Vec::with_capacity(nonce.len() + ciphertext.len());
                combined.extend_from_slice(&nonce);
                combined.extend_from_slice(&ciphertext);

                let result = hex::encode(combined);

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
}