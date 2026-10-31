// src/cli/traits/digest.rs

pub trait HjlixDigest {
    fn new() -> Self;
    fn digest(&self, data: dyn AsRef<[u8]>) -> Vec<u8>;
}