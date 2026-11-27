// src/cli/entcli.rs

use crate::cli::aes_gcm::aes_gcmdecryption_impl::AesGcmDecryptionImpl;
use crate::cli::aes_gcm::aes_gcmdecryptionfile_impl::AesGcmDecryptionFileImpl;
use crate::cli::aes_gcm::{aes_gcmstr_impl::AESGCMStrImpl, aes_gcmfile_impl::AesGcmFileImpl};
use crate::cli::base64::base64_decryption_impl::Base64DecryptionImpl;
use crate::cli::base64::base64_encryption_impl::Base64EncryptionImpl;
use crate::cli::misc::about::About;
use crate::cli::sha2::sha2file_impl::Sha2FileImpl;
use crate::cli::sha2::sha2str_impl::Sha2StrImpl;
use crate::cli::sha3::sha3file_impl::Sha3FileImpl;
use crate::cli::sha3::sha3str_impl::Sha3StrImpl;
use crate::cli::chacha20::chacha20_encryption_impl::ChaCha20EncryptionImpl;
use crate::cli::chacha20::chacha20_decryption_impl::ChaCha20DecryptionImpl;
use crate::cli::chacha20::chacha20_file_impl::ChaCha20EncryptionFileImpl;
use clap::Parser;
use crate::cli::commands::Commands;

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
        },
        Commands::ChaCha20Str { strings, chacha20_type, key_gen, no_verbose } => {
            let chacha20_encryption_impl = ChaCha20EncryptionImpl::new(strings, chacha20_type, no_verbose, key_gen);

            if let Err(error) = chacha20_encryption_impl.execute() {
                eprintln!("Error: {}", error);
            }
        }
        Commands::ChaCha20Decrypt { ciphertexts, chacha20_keyfile, no_verbose } => {
            let chacha20_decryption_impl = ChaCha20DecryptionImpl::new(ciphertexts, chacha20_keyfile, no_verbose);

            if let Err(error) = chacha20_decryption_impl.execute() {
                eprintln!("Error: {}", error);
            }
        }
        Commands::ChaCha20File { files, chacha20_type, key_gen, no_verbose } => {
            let chacha20_file_impl = ChaCha20EncryptionFileImpl::new(files, chacha20_type, key_gen, no_verbose);

            if let Err(error) = chacha20_file_impl.execute() {
                eprintln!("Error: {}", error);
            }
        }
        Commands::About {} => {
            About::execute();
        }
    }
}
