// src/cli/sha/digest.rs

use crate::cli::sha2::sha2_types::Sha2Types;
use sha2::{Sha224, Sha256, Sha512_224, Sha512_256, Sha384, Sha512, Digest};

impl Sha2Types {
    pub fn digest(&self, data: &[u8]) -> Vec<u8> {
        match self {
            Sha2Types::SHA224            => Sha224::digest(data).to_vec(),
            Sha2Types::SHA256            => Sha256::digest(data).to_vec(),
            Sha2Types::SHA512_224        => Sha512_224::digest(data).to_vec(),
            Sha2Types::SHA512_256        => Sha512_256::digest(data).to_vec(),
            Sha2Types::SHA384            => Sha384::digest(data).to_vec(),
            Sha2Types::SHA512            => Sha512::digest(data).to_vec(),
        }
    }
}