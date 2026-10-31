// src/cli/sha/digest.rs

use crate::cli::sha::sha_types::ShaTypes;
use sha2::{Sha224, Sha256, Sha512_224, Sha512_256, Sha384, Sha512, Digest};

impl ShaTypes {
    pub fn digest(&self, data: &[u8]) -> Vec<u8> {
        match self {
            ShaTypes::SHA224            => Sha224::digest(data).to_vec(),
            ShaTypes::SHA256            => Sha256::digest(data).to_vec(),
            ShaTypes::SHA512_224        => Sha512_224::digest(data).to_vec(),
            ShaTypes::SHA512_256        => Sha512_256::digest(data).to_vec(),
            ShaTypes::SHA384            => Sha384::digest(data).to_vec(),
            ShaTypes::SHA512            => Sha512::digest(data).to_vec(),
        }
    }
}