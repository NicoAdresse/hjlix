// src/cli/entcli.rs

use crate::cli::aes_gcm::aes_gcm_impl::AESGCMImpl;
use crate::cli::aes_gcm::aes_gcm_types::AESGCMTypes;
use crate::cli::consts::constants::*;
use crate::cli::misc::about::About;
use crate::cli::sha2::{sha2_impl::Sha2Impl, sha2_types::Sha2Types};
use crate::cli::sha3::{sha3_impl::Sha3Impl, sha3_types::Sha3Types};
use clap::{Parser, Subcommand};

#[derive(Parser, Debug, Clone)]
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
#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    /// Takes a String and puts it in a SHA2 cryptographic algorithm.
    Sha2 {
        #[arg(required = true, help = SHA_STR_HELP)]
        strings: Vec<String>,
        #[arg(short, long, required = true, help = SHA_TYPE_HELP)]
        sha_type: Sha2Types,
        #[arg(short, long, default_value_t = false, help = SHA_WRITE_INTO_FILE_HELP)]
        write_into_file: bool,
    },
    /// Takes a String and puts it in a SHA3 cryptographic algorithm.
    Sha3 {
        #[arg(required = true, help = SHA_STR_HELP)]
        strings: Vec<String>,
        #[arg(short, long, required = true, help = SHA_TYPE_HELP)]
        sha_type: Sha3Types,
        #[arg(short, long, default_value_t = false, help = SHA_WRITE_INTO_FILE_HELP)]
        write_into_file: bool,
    },
    /// Takes a String and puts it in a AES-GCM cryptographic algorithm.
    AesGcm {
        #[arg(required = true)]
        strings: Vec<String>,
        #[arg(short, long, required = true)]
        aes_gcm_type: AESGCMTypes,
    },
    /// Tells the user what hjlix is.
    About {},
}

pub fn run_hjlix() {
    let args = CliArgs::parse();

    match args.command {
        Commands::Sha2 {
            strings,
            sha_type,
            write_into_file,
        } => {
            let sha_impl = Sha2Impl::new(strings, sha_type, write_into_file);

            if let Err(error) = sha_impl.execute() {
                eprintln!("Error: {:?}", error);
            }
        }
        Commands::Sha3 {
            strings,
            sha_type,
            write_into_file,
        } => {
            let sha_impl = Sha3Impl::new(strings, sha_type, write_into_file);

            if let Err(error) = sha_impl.execute() {
                eprintln!("Error: {:?}", error);
            }
        },
        Commands::AesGcm {
            strings,
            aes_gcm_type
        } => {
            let aes_impl = AESGCMImpl::new(strings, aes_gcm_type);

            if let Err(error) = aes_impl.execute() {
                eprintln!("Error: {:?}", error);
            }
        },
        Commands::About {} => {
            About::execute();
        }
    }
}
