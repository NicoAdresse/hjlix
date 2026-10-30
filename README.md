# hjlix

`hjlix` is a lightweight, fast command-line utility written in Rust for computing cryptographic SHA hashes with support for multiple algorithm variants and optional file output.

## Features

- **Multiple SHA Algorithms**: Compute SHA-224, SHA-256, SHA-512/224, SHA-512/256, SHA-384, and SHA-512 hashes.
- **File Output Option**: Easily write the computed digest directly into a file.
- **Fast & Safe**: Built in Rust using `clap` for robust CLI parsing and `sha2` for secure hashing.

---

## Installation

Ensure you have [Rust](https://www.rust-lang.org/) installed, then build the project from source:

```bash
cargo build --release
```

The compiled binary will be available at `target/release/hjlix`.

---

## Usage

```bash
hjlix sha <STRING> --sha-type <SHA_TYPE> [--write-into-file]
```

### Arguments

- `<STRING>`: The input text/string to hash.
- `-s, --sha-type <SHA_TYPE>`: The SHA algorithm variant to use. Supported values:
  - `sha224`
  - `sha256`
  - `sha512-224`
  - `sha512-256`
  - `sha384`
  - `sha512`
- `-w, --write-into-file`: Flag to write the resulting hash into a file.

### Examples

Compute a SHA-256 hash:
```bash
cargo run -- sha "hello world" --sha-type sha256
```

Compute a SHA-512 hash and write it to a file:
```bash
cargo run -- sha "secure message" --sha-type sha512 --write-into-file
```

---

## License

This project is licensed under the MIT License.

---

## File Structure

```txt
hjlix
├── Cargo.toml
├── Dockerfile
├── LICENSE
├── README.md
└── src
    ├── cli
    │   ├── entcli.rs
    │   ├── misc
    │   │   ├── about.rs
    │   │   └── mod.rs
    │   ├── mod.rs
    │   └── sha
    │       ├── digest.rs
    │       ├── io_err.rs
    │       ├── mod.rs
    │       ├── sha_impl.rs
    │       └── sha_types.rs
    └── main.rs
```