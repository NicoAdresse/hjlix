// src/cli/chacha20/chacha20_types.rs

use clap::ValueEnum;

#[derive(Debug, Clone, PartialEq, ValueEnum)]
pub enum ChaCha20Types {
    ChaCha8Poly1305,
    ChaCha12Poly1305,
    ChaCha20Poly1305,
}

