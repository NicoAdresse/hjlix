# hjlix

`hjlix` is a lightweight, fast command-line utility written in Rust for computing cryptographic SHA-2 and SHA-3/Keccak hashes, performing AES-GCM & ChaCha20 encryption/decryption, and Base64 encoding/decryption, with support for multiple algorithm variants and optional file output.

## Features

- **Multiple SHA-2 Algorithms**: Compute SHA-224, SHA-256, SHA-512/224, SHA-512/256, SHA-384, and SHA-512 hashes.
- **Multiple SHA-3 & Keccak Algorithms**: Compute SHA3-224, SHA3-256, SHA3-384, SHA3-512, Keccak-224, Keccak-256, Keccak-384, Keccak-512, and Keccak-256-Full hashes.
- **AES-GCM & ChaCha20 Support**: Perform AES-GCM & ChaCha20 encryption and decryption.
- **Base64 Support**: Perform Base64 encoding/encryption and decoding/decryption using standard and URL-safe engines.
- **Key Management**: Automatically generate and store AES keys in a secure local file.
- **File & String Support**: Process both raw strings and entire files.
- **File Output Option**: Easily write the computed results directly into files.
- **About Command**: Quick information about the utility and authors.
- **Fast & Safe**: Built in Rust using `clap` for robust CLI parsing and secure hashing/encryption/encoding crates.

---

Check [docs](./docs) for more information.

---

## License

This project is licensed under the [MIT License.](./LICENSE)
