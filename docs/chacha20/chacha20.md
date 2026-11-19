# ChaCha20-Poly1305 Encryption and Decryption

`hjlix` supports the ChaCha8-Poly1305, ChaCha12-Poly1305, and ChaCha20-Poly1305 authenticated encryption variants. Each uses a 256-bit key and a 96-bit nonce. The selected variant is required when encrypting.

## Encrypting strings

```bash
hjlix cha-cha20-str "secret data" --chacha20-type cha-cha20-poly1305 --key-gen
```

`--key-gen` (or `-k`) creates a new key in `.chacha20_key` in the current directory, replacing an existing file. The key file is created with Unix mode `0600`. Keep it private and do not commit or share it.

Without `--key-gen`, encryption reads the existing `.chacha20_key` file. It must contain a valid 32-byte key. To select another variant, use `cha-cha8-poly1305` or `cha-cha12-poly1305` as the value of `--chacha20-type`.

The result is a hex string containing the 12-byte nonce followed by the authenticated ciphertext and tag. Save the complete output for decryption.

## Decrypting strings

Pass the ciphertext and the key file used for encryption:

```bash
hjlix cha-cha20-decrypt "hex-encoded-ciphertext" --chacha20-keyfile .chacha20_key
```

The decryption command identifies the ChaCha variant by trying the supported variants and verifying the authentication tag. The plaintext must be valid UTF-8.

Use `--no-verbose` (or `-n`) on either command to print only the result.
