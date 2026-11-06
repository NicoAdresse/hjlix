# hjlix

`hjlix` is a lightweight, fast command-line utility written in Rust for computing cryptographic SHA-2 and SHA-3/Keccak hashes, and performing AES-GCM encryption/decryption, with support for multiple algorithm variants and optional file output.

## Features

- **Multiple SHA-2 Algorithms**: Compute SHA-224, SHA-256, SHA-512/224, SHA-512/256, SHA-384, and SHA-512 hashes.
- **Multiple SHA-3 & Keccak Algorithms**: Compute SHA3-224, SHA3-256, SHA3-384, SHA3-512, Keccak-224, Keccak-256, Keccak-384, Keccak-512, and Keccak-256-Full hashes.
- **AES-GCM Support**: Perform AES-GCM encryption and decryption.
- **Key Management**: Automatically generate and store AES keys in a secure local file.
- **File & String Support**: Process both raw strings and entire files.
- **File Output Option**: Easily write the computed results directly into files.
- **About Command**: Quick information about the utility and authors.
- **Fast & Safe**: Built in Rust using `clap` for robust CLI parsing and secure hashing/encryption crates.

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
Computes the SHA-2 cryptographic hash of one or more strings.

```bash
hjlix sha2-str <STRING>... --sha-type <SHA_TYPE> [--write-into-file]
```

- `<STRING>...`: One or more input strings to hash.
- `-s, --sha-type <SHA_TYPE>`: The SHA-2 algorithm variant to use. Supported values:
  - `sha224`
  - `sha256`
  - `sha512-224`
  - `sha512-256`
  - `sha384`
  - `sha512`
- `-w, --write-into-file`: Flag to write the resulting hashes into a file named `file_with_sha.txt`.

#### `sha3`
Computes the SHA-3 or Keccak cryptographic hash of one or more strings.

```bash
hjlix sha3-str <STRING>... --sha-type <SHA_TYPE> [--write-into-file]
```

- `<STRING>...`: One or more input strings to hash.
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
- `-w, --write-into-file`: Flag to write the resulting hashes into a file named `file_with_sha.txt`.

#### `aes-gcm`
Performs encryption and decryption using AES-GCM.

**Encryption (Strings):**
```bash
hjlix aes-gcm-str <STRING>... --aes-gcm-type <TYPE> [--write-into-file] [--key-gen]
```

- `<STRING>...`: One or more input strings to encrypt.
- `-a, --aes-gcm-type <TYPE>`: The AES-GCM variant to use (`aes128gcm` or `aes256gcm`).
- `-w, --write-into-file`: Flag to write the resulting encrypted output into a file named `file_with_aes-gcm.txt`.
- `-k, --key-gen`: Generates and saves the AES key to `.aes_key`.

**Decryption (Strings):**
```bash
hjlix aes-gcm-decrypt <CIPHERTEXT>... --key-file <KEY_FILE>
```

- `<CIPHERTEXT>...`: One or more hex-encoded ciphertexts to decrypt.
- `-k, --key-file <KEY_FILE>`: Path to the file containing the AES key (e.g., `.aes_key`).


#### `about`
Displays project information and current version.

```bash
hjlix about
```

---

Check [docs](./docs) for more information.

---

## License

This project is licensed under the [MIT License](./LICENSE).
