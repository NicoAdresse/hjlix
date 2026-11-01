// src/cli/sha/hashing_types.rs

use std::default::Default;
use clap::ValueEnum;
use crate::cli::traits::digest::HjlixDigest;
use sha2::{Sha224, Sha256, Sha384, Sha512, Sha512_224, Sha512_256, Digest};

#[derive(Debug, Clone, PartialEq, ValueEnum, Default)]
pub enum Sha2Types {
    #[default]
    SHA256,
    SHA224,
    SHA512_224,
    SHA512_256,
    SHA384,
    SHA512,
}

impl HjlixDigest for Sha2Types {
    fn new() -> Self {
        Self::default()
    }
    
    fn digest(&self, data: &[u8]) -> Vec<u8> {
        match self {
            Sha2Types::SHA224       => Sha224::digest(data).to_vec(),
            Sha2Types::SHA256       => Sha256::digest(data).to_vec(),
            Sha2Types::SHA384       => Sha384::digest(data).to_vec(),
            Sha2Types::SHA512       => Sha512::digest(data).to_vec(),
            Sha2Types::SHA512_224   => Sha512_224::digest(data).to_vec(),
            Sha2Types::SHA512_256   => Sha512_256::digest(data).to_vec(),
        }
    }
}