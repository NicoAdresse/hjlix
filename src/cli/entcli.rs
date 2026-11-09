// src/cli/entcli.rs

use crate::cli::aes_gcm::aes_gcmdecryption_impl::AesGcmDecryptionImpl;
use crate::cli::aes_gcm::aes_gcmdecryptionfile_impl::AesGcmDecryptionFileImpl;
use crate::cli::aes_gcm::{aes_gcmstr_impl::AESGCMStrImpl, aes_gcmfile_impl::AesGcmFileImpl};
use crate::cli::aes_gcm::aes_gcm_types::AESGCMTypes;
use crate::cli::base64::base64_decryption_impl::Base64DecryptionImpl;
use crate::cli::base64::base64_types::Base64Types;
use crate::cli::base64::base64_encryption_impl::Base64EncryptionImpl;
use crate::cli::consts::constants::*;
use crate::cli::misc::about::About;
use crate::cli::sha2::sha2file_impl::Sha2FileImpl;
use crate::cli::sha2::{sha2str_impl::Sha2StrImpl, sha2_types::Sha2Types};
use crate::cli::sha3::sha3file_impl::Sha3FileImpl;
use crate::cli::sha3::{sha3str_impl::Sha3StrImpl, sha3_types::Sha3Types};
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug, Clone, PartialEq)]
#[command(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    long_about = None,
)]
pub struct CliArgs {
    #[command(subcommand)]
    command: Commands,
}

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
    /// Tells the user what hjlix is.
    About {},
}

pub fn run_hjlix() {
    let args = CliArgs::parse();

    match args.command {
        Commands::Sha2Str {
            strings,
            sha_type,
            no_verbose
        } => {
            let sha_impl = Sha2StrImpl::new(
                strings,
                sha_type,
                no_verbose
            );

            if let Err(error) = sha_impl.execute() {
                eprintln!("Error: {}", error);
            }
        }
        Commands::Sha2File { files, sha_type, no_verbose } => {
            let sha2file_impl = Sha2FileImpl::new(files, sha_type, no_verbose);

            if let Err(error) = sha2file_impl.execute() {
                eprintln!("Error: {}", error);
            }
        },
        Commands::Sha3Str {
            strings,
            sha_type,
            no_verbose
        } => {
            let sha_impl = Sha3StrImpl::new(
                strings,
                sha_type,
                no_verbose
            );

            if let Err(error) = sha_impl.execute() {
                eprintln!("Error: {}", error);
            }
        },
        Commands::Sha3File { files, sha_type, no_verbose } => {
            let sha3file_impl = Sha3FileImpl::new(files, sha_type, no_verbose);

            if let Err(error) = sha3file_impl.execute() {
                eprintln!("Error: {}", error);
            }
        },
        Commands::AesGcmStr {
            strings,
            aes_gcm_type,
            no_verbose,
            key_gen
        } => {
            let aes_impl = AESGCMStrImpl::new(
                strings,
                aes_gcm_type,
                no_verbose,
                key_gen
            );

            if let Err(error) = aes_impl.execute() {
                eprintln!("Error: {}", error);
            }
        },
        Commands::AesGcmFile { files, aes_gcm_type, no_verbose } => {
            let aesfile_impl = AesGcmFileImpl::new(files, aes_gcm_type, no_verbose);

            if let Err(error) = aesfile_impl.execute() {
                eprintln!("Error: {}", error);
            }
        },
        Commands::AesGcmDecrypt {
            ciphertexts,
            key_file,
            no_verbose
        } => {
            let aesdecrytion_impl = AesGcmDecryptionImpl::new(
                ciphertexts,
                key_file,
                no_verbose
            );

            if let Err(error) = aesdecrytion_impl.execute() {
                eprintln!("Error: {}", error);
            }
        },
        Commands::AesGcmDecryptFile { files, key_file, no_verbose } => {
            let aes_gcm_decryption_file_impl = AesGcmDecryptionFileImpl::new(files, key_file, no_verbose);

            if let Err(error) = aes_gcm_decryption_file_impl.execute() {
                eprintln!("Error: {}", error);
            }
        },
        Commands::Base64Encrypt { texts, base64_type, no_verbose } => {
            let base64_encryption_impl = Base64EncryptionImpl::new(texts, base64_type, no_verbose);
            base64_encryption_impl.execute();
        },
        Commands::Base64Decrypt { encoded_texts, base64_type, no_verbose } => {
            let base64_decryption_impl = Base64DecryptionImpl::new(encoded_texts, base64_type, no_verbose);

            if let Err(error) = base64_decryption_impl.execute() {
                eprintln!("Error: {}", error);
            }
        }
        Commands::About {} => {
            About::execute();
        }
    }
}
