# hjlix

`hjlix` is a lightweight, fast command-line utility written in Rust for computing cryptographic SHA-2 hashes with support for multiple algorithm variants and optional file output.

## Features

- **Multiple SHA-2 Algorithms**: Compute SHA-224, SHA-256, SHA-512/224, SHA-512/256, SHA-384, and SHA-512 hashes.
- **File Output Option**: Easily write the computed digest directly into a file (`file_with_sha.txt`).
- **About Command**: Quick information about the utility and authors.
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
hjlix <COMMAND>
```

### Commands

#### `sha2`
Computes the SHA-2 cryptographic hash of a given string.

```bash
hjlix sha2 <STRING> --sha-type <SHA_TYPE> [--write-into-file]
```

- `<STRING>`: The input string to hash (helps with: `"Your string comes here."`).
- `-s, --sha-type <SHA_TYPE>`: The SHA algorithm variant to use. Supported values:
  - `sha224`
  - `sha256`
  - `sha512-224`
  - `sha512-256`
  - `sha384`
  - `sha512`
- `-w, --write-into-file`: Flag to write the resulting hash into a file named `file_with_sha.txt`.

#### `about`
Displays project information and current version.

```bash
hjlix about
```

---

### Examples

Compute a SHA-256 hash:
```bash
cargo run -- sha2 "hello world" --sha-type sha256
```

Compute a SHA-512 hash and write it to `file_with_sha.txt`:
```bash
cargo run -- sha2 "secure message" --sha-type sha512 --write-into-file
```

Display information about `hjlix`:
```bash
cargo run -- about
```

---

## File Structure

```txt
hjlix
├── Cargo.lock
├── Cargo.toml
├── Dockerfile
├── LICENSE
├── README.md
└── src
    ├── cli
    │   ├── consts
    │   │   ├── constants.rs
    │   │   └── mod.rs
    │   ├── entcli.rs
    │   ├── err
    │   │   ├── io_err.rs
    │   │   └── mod.rs
    │   ├── misc
    │   │   ├── about.rs
    │   │   └── mod.rs
    │   ├── mod.rs
    │   └── sha2
    │       ├── digest.rs
    │       ├── mod.rs
    │       ├── sha2_impl.rs
    │       └── sha2_types.rs
    └── main.rs
```

---

## License

This project is licensed under the MIT License.