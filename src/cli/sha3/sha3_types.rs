// src/cli/sha3/sha3_types.rs

use clap::ValueEnum;

#[derive(Debug, Clone, PartialEq, ValueEnum)]
pub enum Sha3Types {
    Keccak224,
    Keccak256,
    Keccak384,
    Keccak512,
    Keccak256Full,
    SHA3_224,
    SHA3_256,
    SHA3_384,
    SHA3_512,
}