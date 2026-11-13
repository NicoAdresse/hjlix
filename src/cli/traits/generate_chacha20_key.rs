// src/cli/traits/generate_chacha20_key.rs

pub trait GenerateChaCha20Key {
    fn generate_chacha20_key(&self) -> Vec<u8>;
}