// src/cli/aes_gcm/aes_gcmstr_impl.rs

use std::path::Path;
use std::os::unix::fs::{PermissionsExt, OpenOptionsExt};
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};
use aes_gcm::KeyInit;
use aes_gcm::aead::Aead;
use aes_gcm::{Key, Aes128Gcm, Aes256Gcm, aead::Generate, Nonce};
use crate::cli::aes_gcm::aes_gcm_types::AESGCMTypes;
use crate::cli::err::aes_err::AESError;
use crate::cli::consts::constants::{FILE_NAME_AES_GCM, FILE_NAME_AES_GCM_KEY};
use crate::cli::err::io_err::IOError;
use crate::cli::traits::generate_aes_key::HjlixGenerateAESKey;

#[derive(Debug, Clone, PartialEq)]
pub struct AESGCMStrImpl {
    strings: Vec<String>,
    aes_gcm_type: AESGCMTypes,
    write_into_file: bool,
    optional_filename: Option<String>,
    no_verbose: bool,
    gen_key: bool,
}

impl AESGCMStrImpl {
    pub fn new(
        strings: Vec<String>,
        aes_gcm_type: AESGCMTypes,
        write_into_file: bool,
        optional_filename: Option<String>,
        no_verbose: bool,
        gen_key: bool,
    ) -> Self {
        Self {
            strings,
            aes_gcm_type,
            write_into_file,
            optional_filename,
            no_verbose,
            gen_key
        }
    }

    pub fn execute(&self) -> anyhow::Result<()> {
        let key = self.aes_gcm_type.generate_key_for_aes_type();
        let aes_gcm_type = self.aes_gcm_type.figure_out_type(&key)
            .map_err(|_| AESError::FailedToFigureOutType)?;

        if self.gen_key {
            let mut file = OpenOptions::new()
                .create_new(true)
                .write(true)
                .mode(0o600)
                .open(FILE_NAME_AES_GCM_KEY)
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

        match aes_gcm_type {
            AESGCMTypes::AES128GCM => {
                let aes_key = Key::<Aes128Gcm>::try_from(key.as_slice())
                    .map_err(|_| AESError::FailedToCreateKey)?;
                let cipher = Aes128Gcm::new(&aes_key);

                for string in &self.strings {
                    let nonce = Nonce::generate();
                    let ciphertext = cipher.encrypt(&nonce, string.as_ref())
                        .map_err(|_| AESError::FailedToCreateCiphertext)?;
                    let mut encrypted_data = nonce.to_vec();
                    encrypted_data.extend_from_slice(&ciphertext);
                    let result = hex::encode(encrypted_data);

                    if !self.no_verbose {
                        println!("Original: {}", string);
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

                for string in &self.strings {
                    let nonce = Nonce::generate();
                    let ciphertext = cipher.encrypt(&nonce, string.as_ref())
                        .map_err(|_| AESError::FailedToCreateCiphertext)?;
                    let mut encrypted_data = nonce.to_vec();
                    encrypted_data.extend_from_slice(&ciphertext);
                    let result = hex::encode(encrypted_data);

                    if !self.no_verbose {
                        println!("Original: {}", string);
                        println!("Result: {}", result);
                    } else {
                        println!("{}", result);
                    }
                }

            }
        };

        Ok(())
    }

    pub fn execute_write_into_file(&self) -> anyhow::Result<()> {
        let key = self.aes_gcm_type.generate_key_for_aes_type();
        let aes_gcm_type = self.aes_gcm_type.figure_out_type(&key)
            .map_err(|_| AESError::FailedToFigureOutType)?;

        let filename = &self.optional_filename.clone().unwrap_or(FILE_NAME_AES_GCM.to_owned());
        let path = Path::new(filename);
        let file = File::create(path)
            .map_err(|_| IOError::FailedToCreateFile)?;
        let mut buf_writer = BufWriter::new(file);

        if self.gen_key {
            let mut file = OpenOptions::new()
                .create_new(true)
                .write(true)
                .mode(0o600)
                .open(FILE_NAME_AES_GCM_KEY)
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

        match aes_gcm_type {
            AESGCMTypes::AES128GCM => {
                let aes_key = Key::<Aes128Gcm>::try_from(key.as_slice())
                    .map_err(|_| AESError::FailedToCreateKey)?;
                let cipher = Aes128Gcm::new(aes_key.as_ref());

                for string in &self.strings {
                    let nonce = Nonce::generate();
                    let ciphertext = cipher.encrypt(&nonce, string.as_ref())
                        .map_err(|_| AESError::FailedToCreateCiphertext)?;
                    let result = hex::encode(ciphertext);

                    writeln!(buf_writer, "{}", result)
                        .map_err(|_| IOError::FailedToWrite)?;
                }

                buf_writer.flush()
                    .map_err(|_| IOError::FailedToFlushBufWriter)?;
            },
            AESGCMTypes::AES256GCM => {
                let aes_key = Key::<Aes256Gcm>::try_from(key.as_slice())
                    .map_err(|_| AESError::FailedToCreateKey)?;
                let cipher = Aes256Gcm::new(aes_key.as_ref());

                for string in &self.strings {
                    let nonce = Nonce::generate();
                    let ciphertext = cipher.encrypt(&nonce, string.as_ref())
                        .map_err(|_| AESError::FailedToCreateCiphertext)?;
                    let result = hex::encode(ciphertext);

                    writeln!(buf_writer, "{}", result)
                        .map_err(|_| IOError::FailedToWrite)?;
                }

                buf_writer.flush()
                    .map_err(|_| IOError::FailedToFlushBufWriter)?;
            }
        }

        Ok(())
    }
}