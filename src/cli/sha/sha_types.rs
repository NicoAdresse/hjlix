// src/cli/sha/hashing_types.rs

use clap::ValueEnum;

#[derive(Debug, Clone, PartialEq, ValueEnum)]
pub enum ShaTypes {
    SHA224,
    SHA256,
    SHA512_224,
    SHA512_256,
    SHA384,
    SHA512,
}