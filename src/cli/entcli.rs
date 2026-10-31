// src/cli/entcli.rs

use clap::{Parser, Subcommand};
use crate::cli::sha2::{sha2_types::Sha2Types, sha2_impl::Sha2Impl};
use crate::cli::sha3::{sha3_types::Sha3Types, sha3_impl::Sha3Impl};
use crate::cli::misc::about::About;
use crate::cli::consts::constants::*;

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

#[derive(Subcommand, Debug, Clone, PartialEq)]
pub enum Commands {
    /// Takes a String and puts it in a SHA2 cryptographic algorithm.
    Sha2 {
        #[arg(required = true, help = SHA_STR_HELP)]
        str: String,
        #[arg(short, long, required = true, help = SHA_TYPE_HELP)]
        sha_type: Sha2Types,
        #[arg(short, long, default_value_t = false, help = SHA_WRITE_INTO_FILE_HELP)]
        write_into_file: bool,
    },
    /// Takes a String and puts it in a SHA3 cryptographic algorithm.
    Sha3 {
        #[arg(required = true, help = SHA_STR_HELP)]
        str: String,
        #[arg(short, long, required = true, help = SHA_TYPE_HELP)]
        sha_type: Sha3Types,
        #[arg(short, long, default_value_t = false, help = SHA_WRITE_INTO_FILE_HELP)]
        write_into_file: bool,
    },
    /// Tells the user what hjlix is.
    About {},
}

pub fn run_hjlix() {
    let args = CliArgs::parse();

    match args.command {
        Commands::Sha2 { str, sha_type, write_into_file } => {
            let sha_impl = Sha2Impl::new(str, sha_type, write_into_file);

            if let Err(error) = sha_impl.execute() {
                eprintln!("Error: {:?}", error);
            }
        },
        Commands::Sha3 { str, sha_type, write_into_file } => {
            let sha_impl = Sha3Impl::new(str, sha_type, write_into_file);

            if let Err(error) = sha_impl.execute() {
                eprintln!("Error: {:?}", error);
            }
        }
        Commands::About {} => {
            About::execute();
        },
    }
}