# AES-GCM Encryption

AES-GCM (Advanced Encryption Standard in Galois/Counter Mode) is an authenticated encryption algorithm that provides both confidentiality and data integrity/authenticity assurances. `hjlix` utilizes the `aes-gcm` crate to provide high-performance, secure authenticated encryption.

## Supported Variants
- **AES-128-GCM** (`aes128gcm`) - AES-GCM using a 128-bit key.
- **AES-256-GCM** (`aes256gcm`) - AES-GCM using a 256-bit key.

## Usage Example
```bash
hjlix aes-gcm "Hello, World!" --aes-gcm-type aes256gcm
```

Result:
```txt
Original: Hello, World!
Result: <hex-encoded-ciphertext>
```

With file output enabled (`-w` or `--write-into-file`):
```bash
hjlix aes-gcm "Hello, World!" --aes-gcm-type aes256gcm -w
```
This writes the hex-encoded ciphertext output directly into `file_with_aes-gcm.txt`.

## Features and Characteristics
- **Authenticated Encryption**: Ensures both privacy and authenticity of the encrypted message.
- **Random Nonce Generation**: Automatically generates a cryptographically secure random nonce for each encryption operation.
- **Hex Output**: Encrypted ciphertexts are formatted as hex-encoded strings for easy transportation and storage.