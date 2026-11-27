# hjlix CLI Documentation

`hjlix` is a command-line tool for cryptographic hashing, encryption, and encoding. It supports SHA-2, SHA-3 (including Keccak), AES-GCM, ChaCha20-Poly1305 variants, and Base64.

## Overview
The tool is modularly organized. You can interact with it using specific subcommands.

### Commands
- `sha2`: Computes hashes using the SHA-2 family.
  - `sha2-str`: Hashes input strings.
  - `sha2-file`: Hashes files.
- `sha3`: Computes hashes using the SHA-3/Keccak family.
  - `sha3-str`: Hashes input strings.
  - `sha3-file`: Hashes files.
- `aes-gcm`: Performs encryption and decryption using the AES-GCM family (details in [aes-gcm.md](./aes-gcm/aes-gcm.md)).
  - `aes-gcm-str`: Encrypts input strings.
  - `aes-gcm-file`: Encrypts files.
  - `aes-gcm-decrypt`: Decrypts encrypted strings using a key file.
  - `aes-gcm-decrypt-file`: Decrypts encrypted files using a key file.
- `chacha20`: Encrypts and decrypts strings with ChaCha8/12/20-Poly1305 (details in [chacha20.md](./chacha20/chacha20.md)).
  - `cha-cha20-str`: Encrypts input strings.
  - `cha-cha20-decrypt`: Decrypts encrypted strings using a key file.
- `base64`: Performs Base64 encoding and decryption (details in [base64.md](./base64/base64.md)).
  - `base64-encrypt`: Encrypts/encodes input strings using Base64.
  - `base64-decrypt`: Decrypts/decodes Base64 encoded strings.
- `about`: Displays metadata about the project, including version and authorship.

### Examples

**SHA-2:**
```bash
hjlix sha2-str "Hello, World!" --sha-type sha384
hjlix sha2-file "path/to/file.txt" --sha-type sha256
```

**SHA-3/Keccak:**
```bash
hjlix sha3-str "Hello, World!" --sha-type sha3-512
hjlix sha3-file "path/to/file.txt" --sha-type sha3-256
```

**AES-GCM:**
```bash
# Encrypt strings and generate a key file
hjlix aes-gcm-str "secret data" --aes-gcm-type aes256gcm --key-gen

# Encrypt files
hjlix aes-gcm-file "path/to/file.txt" --aes-gcm-type aes256gcm

# Decrypt strings using the generated key file
hjlix aes-gcm-decrypt "hex-encoded-ciphertext" --key-file .aes_key
```

**ChaCha20-Poly1305:**
```bash
# Encrypt a string and create .chacha20_key
hjlix cha-cha20-str "secret data" --chacha20-type cha-cha20-poly1305 --key-gen

# Decrypt the resulting hex ciphertext using that key
hjlix cha-cha20-decrypt "hex-encoded-ciphertext" --chacha20-keyfile .chacha20_key
```

**Base64:**
```bash
# Encrypt/encode strings using standard Base64
hjlix base64-encrypt "Hello, World!" --base64-type standard

# Decrypt/decode Base64 encoded strings
hjlix base64-decrypt "SGVsbG8sIFdvcmxkIQ==" --base64-type standard
```

### Global Options
- `--write-into-file`: When provided, the output is saved to `file_with_sha.txt` (for hashes) or other relevant output files in the current directory instead of being printed to stdout.