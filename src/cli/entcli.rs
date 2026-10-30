// src/cli/entcli.rs

use clap::{Parser, Subcommand};
use crate::cli::sha::{sha_types::ShaTypes, sha_impl::ShaImpl};
use crate::cli::misc::about::About;

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
    Sha {
        #[arg(required = true, help = "Your string")]
        str: String,
        #[arg(short, long, required = true, help = "Your SHA algorithm")]
        sha_type: ShaTypes,
        #[arg(short, long, default_value_t = false, help = "Writes into a file")]
        write_into_file: bool,
    },
    /// Tells the user what hjlix is.
    About {},
}

pub fn run_hjlix() {
    let args = CliArgs::parse();

    match args.command {
        Commands::Sha { str, sha_type, write_into_file } => {
            let sha_impl = ShaImpl::new(str, sha_type, write_into_file);

            if let Err(error) = sha_impl.execute() {
                eprintln!("Error: {:?}", error);
            }
        },
        Commands::About {} => {
            About::execute();
        },
    }
}