// src/cli/misc/about.rs

#[derive(Debug, Clone, PartialEq)]
pub struct About {}

impl About {
    pub fn new() -> Self { Self {} }

    pub fn execute(&self) {
        let authors = env!("CARGO_PKG_AUTHORS")
            .split(':')
            .collect::<Vec<_>>()
            .join("\n   ");

        println!(r#"
This is Hjlix. Created on 30.10.2026. Current version: {}

hjlix is a lightweight, fast command-line utility written in Rust for
computing cryptographic SHA hashes with support for multiple algorithm variants and optional file output.

Authors: {}
        "#, env!("CARGO_PKG_VERSION"), authors);
    } 
}