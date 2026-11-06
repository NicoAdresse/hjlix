# AES-GCM Encryption and Decryption

AES-GCM (Advanced Encryption Standard in Galois/Counter Mode) is an authenticated encryption algorithm that provides both confidentiality and data integrity/authenticity assurances. `hjlix` utilizes the `aes-gcm` crate to provide high-performance, secure authenticated encryption.

## Supported Variants
- **AES-128-GCM** (`aes128gcm`) - AES-GCM using a 128-bit key.
- **AES-256-GCM** (`aes256gcm`) - AES-GCM using a 256-bit key.

## Usage Example (Encryption)

### Strings
```bash
hjlix aes-gcm-str "Hello, World!" --aes-gcm-type aes256gcm
```

Result:
```txt
Original: Hello, World!
Result: <hex-encoded-ciphertext>
```

### Key Generation
You can generate and save the AES key to a file using the `--key-gen` (or `-k`) flag. This is useful for later decryption.

```bash
hjlix aes-gcm-str "secret data" --aes-gcm-type aes256gcm --key-gen
```
This saves the key to a file named `.aes_key` in the current directory.

**Warning**: The `.aes_key` file contains the raw cryptographic key. **Do not commit this file to version control (e.g., Git) or share it publicly.**

### Files
```bash
hjlix aes-gcm-file "path/to/file.txt" --aes-gcm-type aes256gcm
```

## Usage Example (Decryption)

To decrypt a ciphertext, use the `aes-gcm-decrypt` subcommand and provide the key file.

```bash
hjlix aes-gcm-decrypt "hex-encoded-ciphertext" --key-file .aes_key
```

Result:
```txt
Original: <hex-encoded-ciphertext>
Result: Hello, World!
```

## Features and Characteristics
- **Authenticated Encryption**: Ensures both privacy and authenticity of the encrypted message.
- **Random Nonce Generation**: Automatically generates a cryptographically secure random nonce (12 bytes) for each encryption operation.
- **Nonce Prepending**: The 12-byte nonce is prepended to the ciphertext in the hex-encoded output, allowing for easy decryption.
- **Key Management**: Supports saving keys to a local `.aes_key` file with restricted file permissions (0600 on Unix systems).
- **Hex Output**: Encrypted ciphertexts are formatted as hex-encoded strings for easy transportation and storage.
- **Clear Old Key**: When `--key-gen` is used, any existing `.aes_key` file is automatically removed and replaced with a new one.
