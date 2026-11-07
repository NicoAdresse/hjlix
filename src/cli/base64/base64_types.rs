// src/cli/base64/base64_types.rs

use std::default::Default;
use clap::ValueEnum;

#[derive(Debug, Clone, PartialEq, ValueEnum, Default)]
pub enum Base64Types {
    #[default]
    Standard,
    StandaradNoPad,
    UrlSafe,
    UrlSafeNoPad,
}

impl Base64Types {
    pub fn encode(&self, data: &str) -> Vec<u8> {
        match self {

        }
    }
}