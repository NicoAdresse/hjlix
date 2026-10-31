// src/cli/sha3/digest.rs

use crate::cli::sha3::sha3_types::Sha3Types;
use sha3::{Keccak224, Keccak256, Keccak384, Keccak512, Keccak256Full, Sha3_224, Sha3_256, Sha3_384, Sha3_512, Digest};

impl Sha3Types {
    pub fn digest(&self, data: &[u8]) -> Vec<u8> {
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