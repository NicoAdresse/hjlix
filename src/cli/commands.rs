// src/cli/commands.rs

use crate::cli::aes_gcm::aes_gcm_types::AESGCMTypes;
use crate::cli::base64::base64_types::Base64Types;
use crate::cli::consts::constants::*;
use crate::cli::sha2::sha2_types::Sha2Types;
use crate::cli::sha3::sha3_types::Sha3Types;
use crate::cli::chacha20::chacha20_types::ChaCha20Types;
use clap::Subcommand;
use std::path::PathBuf;

#[allow(variant_size_differences)]
#[allow(clippy::large_enum_variant)]
#[derive(Subcommand, Debug, Clone, PartialEq)]
pub enum Commands {
    /// Takes a String and puts it in a SHA2 cryptographic algorithm.
    Sha2Str {
        #[arg(required = true, help = SHA_STR_HELP)]
        strings: Vec<String>,
        #[arg(short, long, required = true, help = SHA_TYPE_HELP)]
        sha_type: Sha2Types,
        #[arg(short, long, default_value_t = false, help = NO_VERBOSE_HELP)]
        no_verbose: bool,
    },
    /// Takes Strings from files and puts it in a SHA2 cryptographic algorithm.
    Sha2File {
        #[arg(required = true, help = SHA_FILE_HELP)]
        files: Vec<PathBuf>,
        #[arg(short, long, help = SHA_TYPE_HELP)]
        sha_type: Sha2Types,
        #[arg(short, long, default_value_t = false, help = NO_VERBOSE_HELP)]
        no_verbose: bool,
    },
    /// Takes a String and puts it in a SHA3 cryptographic algorithm.
    Sha3Str {
        #[arg(required = true, help = SHA_STR_HELP)]
        strings: Vec<String>,
        #[arg(short, long, required = true, help = SHA_TYPE_HELP)]
        sha_type: Sha3Types,
        #[arg(short, long, default_value_t = false, help = NO_VERBOSE_HELP)]
        no_verbose: bool,
    },
    /// Takes Strings from files and puts it in a SHA3 cryptographic algorithm.
    Sha3File {
        #[arg(required = true, help = SHA_FILE_HELP)]
        files: Vec<PathBuf>,
        #[arg(short, long, required = true, help = SHA_TYPE_HELP)]
        sha_type: Sha3Types,
        #[arg(short, long, default_value_t = false, help = NO_VERBOSE_HELP)]
        no_verbose: bool,
    },
    /// Takes a String and puts it in a AES-GCM cryptographic algorithm.
    AesGcmStr {
        #[arg(required = true, help = AESGCM_STR_HELP)]
        strings: Vec<String>,
        #[arg(short, long, required = true, help = AESGCM_TYPE_HELP)]
        aes_gcm_type: AESGCMTypes,
        #[arg(short, long, default_value_t = false, help = AESGCM_KEY_GEN_HELP)]
        key_gen: bool,
        #[arg(short, long, default_value_t = false, help = NO_VERBOSE_HELP)]
        no_verbose: bool,
    },
    /// Takes Strings from files and puts them into a AES-GCM cryptographic algorithm.
    AesGcmFile {
        #[arg(required = true, help = AESGCM_FILE_HELP)]
        files: Vec<PathBuf>,
        #[arg(short, long, required = true, help = AESGCM_TYPE_HELP)]
        aes_gcm_type: AESGCMTypes,
        #[arg(short, long, default_value_t = false, help = AESGCM_KEY_GEN_HELP)]
        key_gen: bool,
        #[arg(short, long, default_value_t = false, help = NO_VERBOSE_HELP)]
        no_verbose: bool,
    },
    /// Takes encrypted Strings and decrypts them using AES-GCM.
    AesGcmDecrypt {
        #[arg(required = true)]
        ciphertexts: Vec<String>,
        #[arg(short, long, required = true, help = AESGCM_KEYFILE_HELP)]
        key_file: PathBuf,
        #[arg(short, long, default_value_t = false, help = NO_VERBOSE_HELP)]
        no_verbose: bool,
    },
    /// Takes encrypted Strings from files and decrypts them using AES-GCM.
    AesGcmDecryptFile {
        #[arg(required = true)]
        files: Vec<PathBuf>,
        #[arg(short, long, required = true, help = AESGCM_KEYFILE_HELP)]
        key_file: PathBuf,
        #[arg(short, long, default_value_t = false, help = NO_VERBOSE_HELP)]
        no_verbose: bool,
    },
    /// Takes Strings and encrypts them using Base64.
    Base64Encrypt {
        #[arg(required = true)]
        texts: Vec<String>,
        #[arg(short, long, required = true, help = BASE64_TYPE_HELP)]
        base64_type: Base64Types,
        #[arg(short, long, default_value_t = false, help = NO_VERBOSE_HELP)]
        no_verbose: bool,
    },
    /// Takes Encoded Strings and decodes them using Base64.
    Base64Decrypt {
        #[arg(required = true)]
        encoded_texts: Vec<String>,
        #[arg(short, long, required = true, help = BASE64_TYPE_HELP)]
        base64_type: Base64Types,
        #[arg(short, long, default_value_t = false, help = NO_VERBOSE_HELP)]
        no_verbose: bool,
    },
    /// Takes strings and encodes them using ChaCha20.
    ChaCha20Str {
        #[arg(required = true)]
        strings: Vec<String>,
        #[arg(short, long, required = true, help = CHACHA20_TYPES_HELP)]
        chacha20_type: ChaCha20Types,
        #[arg(short, long, default_value_t = false, help = CHACHA20_KEY_GEN_HELP)]
        key_gen: bool,
        #[arg(short, long, default_value_t = false, help = NO_VERBOSE_HELP)]
        no_verbose: bool,
    },
    /// Takes encrypted strings and decodes them using ChaCha20.
    ChaCha20Decrypt {
        #[arg(required = true)]
        ciphertexts: Vec<String>,
        #[arg(short, long, required = true, help = CHACHA20_KEYFILE_HELP)]
        chacha20_keyfile: PathBuf,
        #[arg(short, long, default_value_t = false, help = NO_VERBOSE_HELP)]
        no_verbose: bool,
    },
    /// Takes in files and encodes them using ChaCha20.
    ChaCha20File {
        #[arg(required = true)]
        files: Vec<PathBuf>,
        #[arg(short, long, required = true, help = CHACHA20_TYPES_HELP)]
        chacha20_type: ChaCha20Types,
        #[arg(short, long, default_value_t = false, help = CHACHA20_KEY_GEN_HELP)]
        key_gen: bool,
        #[arg(short, long, default_value_t = false, help = NO_VERBOSE_HELP)]
        no_verbose: bool,
    },
    /// Tells the user what hjlix is.
    About {},
}