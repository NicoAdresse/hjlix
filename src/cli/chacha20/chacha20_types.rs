// src/cli/chacha20/chacha20_types.rs

use crate::cli::traits::generate_chacha20_key::GenerateChaCha20Key;
use chacha20poly1305::{
    ChaCha12Poly1305, ChaCha20Poly1305, ChaCha8Poly1305, Error,
    aead::{Aead, Generate, Key, KeyInit},
};
use clap::ValueEnum;

#[derive(Debug, Clone, PartialEq, ValueEnum)]
pub enum ChaCha20Types {
    ChaCha8Poly1305,
    ChaCha12Poly1305,
    ChaCha20Poly1305,
}

pub enum ChaChaCipher {
    ChaCha8Poly1305(ChaCha8Poly1305),
    ChaCha12Poly1305(ChaCha12Poly1305),
    ChaCha20Poly1305(ChaCha20Poly1305),
}

impl ChaCha20Types {
    pub fn create_cipher(&self, key: &[u8]) -> Result<ChaChaCipher, Error> {
        let key = Key::<ChaCha20Poly1305>::try_from(key).map_err(|_| Error)?;

        Ok(match self {
            Self::ChaCha8Poly1305 => ChaChaCipher::ChaCha8Poly1305(ChaCha8Poly1305::new(&key)),
            Self::ChaCha12Poly1305 => ChaChaCipher::ChaCha12Poly1305(ChaCha12Poly1305::new(&key)),
            Self::ChaCha20Poly1305 => ChaChaCipher::ChaCha20Poly1305(ChaCha20Poly1305::new(&key)),
        })
    }

    pub fn encrypt(&self, key: &[u8], nonce: &[u8], plaintext: &[u8]) -> Result<Vec<u8>, Error> {
        let nonce = nonce.try_into().map_err(|_| Error)?;

        match self.create_cipher(key)? {
            ChaChaCipher::ChaCha8Poly1305(c) => c.encrypt(&nonce, plaintext),
            ChaChaCipher::ChaCha12Poly1305(c) => c.encrypt(&nonce, plaintext),
            ChaChaCipher::ChaCha20Poly1305(c) => c.encrypt(&nonce, plaintext),
        }
    }

    #[allow(unused)]
    pub fn decrypt(&self, key: &[u8], nonce: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, Error> {
        let nonce = nonce.try_into().map_err(|_| Error)?;

        match self.create_cipher(key)? {
            ChaChaCipher::ChaCha8Poly1305(c) => c.decrypt(&nonce, ciphertext),
            ChaChaCipher::ChaCha12Poly1305(c) => c.decrypt(&nonce, ciphertext),
            ChaChaCipher::ChaCha20Poly1305(c) => c.decrypt(&nonce, ciphertext),
        }
    }
}

impl GenerateChaCha20Key for ChaCha20Types {
    fn generate_chacha20_key(&self) -> Vec<u8> {
        match self {
            ChaCha20Types::ChaCha8Poly1305 => Key::<ChaCha8Poly1305>::generate().to_vec(),
            ChaCha20Types::ChaCha12Poly1305 => Key::<ChaCha12Poly1305>::generate().to_vec(),
            ChaCha20Types::ChaCha20Poly1305 => Key::<ChaCha20Poly1305>::generate().to_vec(),
        }
    }
}