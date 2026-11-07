// src/cli/base64/base64_types.rs

use std::default::Default;
use clap::ValueEnum;
use base64::{Engine as _, engine::general_purpose};

#[derive(Debug, Clone, PartialEq, ValueEnum, Default)]
pub enum Base64Types {
    #[default]
    Standard,
    StandaradNoPad,
    StandardNoPadIndifferent,
    StandardPadIndifferent,
    UrlSafe,
    UrlSafeNoPad,
    UrlSafeNoPadIndifferent,
    UrlSafePadIndifferent,
}

impl Base64Types {
    pub fn encode(&self, data: &str) -> String {
        match self {
            Base64Types::Standard                   =>  general_purpose::STANDARD.encode(data),
            Base64Types::StandaradNoPad             =>  general_purpose::STANDARD_NO_PAD.encode(data),
            Base64Types::StandardNoPadIndifferent   =>  general_purpose::STANDARD_NO_PAD_INDIFFERENT.encode(data),
            Base64Types::StandardPadIndifferent     =>  general_purpose::STANDARD_PAD_INDIFFERENT.encode(data),
            Base64Types::UrlSafe                    =>  general_purpose::URL_SAFE.encode(data),
            Base64Types::UrlSafeNoPad               =>  general_purpose::URL_SAFE_NO_PAD.encode(data),
            Base64Types::UrlSafeNoPadIndifferent    =>  general_purpose::URL_SAFE_NO_PAD_INDIFFERENT.encode(data),
            Base64Types::UrlSafePadIndifferent      =>  general_purpose::URL_SAFE_PAD_INDIFFERENT.encode(data),
        }
    }
}