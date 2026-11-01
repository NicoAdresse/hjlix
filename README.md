# hjlix

`hjlix` is a lightweight, fast command-line utility written in Rust for computing cryptographic SHA-2 and SHA-3/Keccak hashes with support for multiple algorithm variants and optional file output.

## Features

- **Multiple SHA-2 Algorithms**: Compute SHA-224, SHA-256, SHA-512/224, SHA-512/256, SHA-384, and SHA-512 hashes.
- **Multiple SHA-3 & Keccak Algorithms**: Compute SHA3-224, SHA3-256, SHA3-384, SHA3-512, Keccak-224, Keccak-256, Keccak-384, Keccak-512, and Keccak-256-Full hashes.
- **File Output Option**: Easily write the computed digest directly into a file (`file_with_sha.txt`).
- **About Command**: Quick information about the utility and authors.
- **Fast & Safe**: Built in Rust using `clap` for robust CLI parsing and secure hashing crates.

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

- `<STRING>`: The input string to hash.
- `-s, --sha-type <SHA_TYPE>`: The SHA-2 algorithm variant to use. Supported values:
  - `sha224`
  - `sha256`
  - `sha512-224`
  - `sha512-256`
  - `sha384`
  - `sha512`
- `-w, --write-into-file`: Flag to write the resulting hash into a file named `file_with_sha.txt`.

#### `sha3`
Computes the SHA-3 or Keccak cryptographic hash of a given string.

```bash
hjlix sha3 <STRING> --sha-type <SHA_TYPE> [--write-into-file]
```

- `<STRING>`: The input string to hash.
- `-s, --sha-type <SHA_TYPE>`: The SHA-3/Keccak algorithm variant to use. Supported values:
  - `keccak224`
  - `keccak256`
  - `keccak384`
  - `keccak512`
  - `keccak256-full`
  - `sha3-224`
  - `sha3-256`
  - `sha3-384`
  - `sha3-512`
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

Compute a SHA-3 (SHA3-256) hash and write it to `file_with_sha.txt`:
```bash
cargo run -- sha3 "secure message" --sha-type sha3-256 --write-into-file
```

Display information about `hjlix`:
```bash
cargo run -- about
```

Check [docs](./docs) for more information.

---

## File Structure

```txt
hjlix
├── Cargo.lock
├── Cargo.toml
├── CODE_OF_CONDUCT.md
├── CONTRIBUTING.md
├── Dockerfile
├── docs
│   ├── cli.md
│   ├── sha2
│   │   └── sha2.md
│   └── sha3
│       └── sha3.md
├── LICENSE
├── README.md
├── SECURITY.md
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
    │   ├── sha2
    │   │   ├── mod.rs
    │   │   ├── sha2_impl.rs
    │   │   └── sha2_types.rs
    │   ├── sha3
    │   │   ├── mod.rs
    │   │   ├── sha3_impl.rs
    │   │   └── sha3_types.rs
    │   └── traits
    │       ├── digest.rs
    │       └── mod.rs
    └── main.rs
```

---

## License

This project is licensed under the MIT License.