// src/cli/sha3/sha3_types.rs

#[derive(Debug, Clone, PartialEq)]
pub enum Sha3Types {
    Keccack224,
    Keccack256,
    Keccack384,
    Keccack512,
    Keccack256Full,
    SHA3_224,
    SHA3_256,
    SHA3_384,
    SHA3_512,
}