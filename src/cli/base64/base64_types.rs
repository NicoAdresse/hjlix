// src/cli/base64/base64_types.rs

use crate::cli::err::base64_err::Base64Error;
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

    pub fn decode(&self, data: &str) -> Result<Vec<u8>, Base64Error> {
        match self {
            Base64Types::Standard                   => Ok(general_purpose::STANDARD.decode(data).map_err(|_| Base64Error::FailedToDecode)?),
            Base64Types::StandaradNoPad             => Ok(general_purpose::STANDARD_NO_PAD.decode(data).map_err(|_| Base64Error::FailedToDecode)?),
            Base64Types::StandardNoPadIndifferent   => Ok(general_purpose::STANDARD_NO_PAD_INDIFFERENT.decode(data).map_err(|_| Base64Error::FailedToDecode)?),
            Base64Types::StandardPadIndifferent     => Ok(general_purpose::STANDARD_PAD_INDIFFERENT.decode(data).map_err(|_| Base64Error::FailedToDecode)?),
            Base64Types::UrlSafe                    => Ok(general_purpose::URL_SAFE.decode(data).map_err(|_| Base64Error::FailedToDecode)?),
            Base64Types::UrlSafeNoPad               => Ok(general_purpose::URL_SAFE_NO_PAD.decode(data).map_err(|_| Base64Error::FailedToDecode)?),
            Base64Types::UrlSafeNoPadIndifferent    => Ok(general_purpose::URL_SAFE_NO_PAD_INDIFFERENT.decode(data).map_err(|_| Base64Error::FailedToDecode)?),
            Base64Types::UrlSafePadIndifferent      => Ok(general_purpose::URL_SAFE_PAD_INDIFFERENT.decode(data).map_err(|_| Base64Error::FailedToDecode)?),
        }
    }
}