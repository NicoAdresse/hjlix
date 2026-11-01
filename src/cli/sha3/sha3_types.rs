// src/cli/sha3/sha3_types.rs

use std::default::Default;
use clap::ValueEnum;
use crate::cli::traits::digest::HjlixDigest;
use sha3::{Keccak224, Keccak256, Keccak256Full, Keccak384, Keccak512, Sha3_224, Sha3_256, Sha3_384, Sha3_512, Digest};

#[derive(Debug, Clone, PartialEq, ValueEnum, Default)]
pub enum Sha3Types {
    #[default]
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

impl HjlixDigest for Sha3Types {
    fn new() -> Self {
        Self::default()
    }

    fn digest(&self, data: &[u8]) -> Vec<u8> {
        match self {
            Sha3Types::Keccak224            => Keccak224::digest(data).to_vec(),
            Sha3Types::Keccak256            => Keccak256::digest(data).to_vec(),
            Sha3Types::Keccak384            => Keccak384::digest(data).to_vec(),
            Sha3Types::Keccak512            => Keccak512::digest(data).to_vec(),
            Sha3Types::Keccak256Full        => Keccak256Full::digest(data).to_vec(),
            Sha3Types::SHA3_224             => Sha3_224::digest(data).to_vec(),
            Sha3Types::SHA3_256             => Sha3_256::digest(data).to_vec(),
            Sha3Types::SHA3_384             => Sha3_384::digest(data).to_vec(),
            Sha3Types::SHA3_512             => Sha3_512::digest(data).to_vec(),
        }
    }
}