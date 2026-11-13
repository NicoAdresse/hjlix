// src/cli/chacha20/chacha20_types.rs

use clap::ValueEnum;
use crate::cli::traits::generate_chacha20_key::GenerateChaCha20Key;
use chacha20poly1305::{
    ChaCha8Poly1305, ChaCha12Poly1305, ChaCha20Poly1305, aead::{Generate, Key}
};

#[derive(Debug, Clone, PartialEq, ValueEnum)]
pub enum ChaCha20Types {
    ChaCha8Poly1305,
    ChaCha12Poly1305,
    ChaCha20Poly1305,
}

impl GenerateChaCha20Key for ChaCha20Types {
    fn generate_chacha20_key(&self) -> Vec<u8> {
        match self {
            ChaCha20Types::ChaCha8Poly1305 => {
                let key = Key::<ChaCha8Poly1305>::generate();
                key.to_vec()
            },
            ChaCha20Types::ChaCha12Poly1305 => {
                let key = Key::<ChaCha12Poly1305>::generate();
                key.to_vec()
            },
            ChaCha20Types::ChaCha20Poly1305 => {
                let key = Key::<ChaCha20Poly1305>::generate();
                key.to_vec()
            },
        }
    }
}