// src/cli/traits/digest.rs

pub trait HjlixDigest {
    #[allow(unused)]
    fn new() -> Self;
    fn digest(&self, data: &[u8]) -> Vec<u8>;
}