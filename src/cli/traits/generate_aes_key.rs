// src/cli/traits/generate_aes_key.rs

pub trait HjlixGenerateAESKey {
    fn generate_key_for_aes_type(&self) -> Vec<u8>;
}