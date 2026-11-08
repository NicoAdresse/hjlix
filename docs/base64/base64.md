# Base64 Encoding and Decryption

Base64 is a binary-to-text encoding scheme that represents binary data in an ASCII string format by translating it into a radix-64 representation. `hjlix` supports multiple Base64 engine standards and formatting variants for both encoding and decoding strings.

## Supported Base64 Types
`hjlix` supports the following Base64 variants via the `--base64-type` (or `-b`) flag:
- **`standard`** (`standard`): Standard Base64 alphabet (RFC 4648) with padding (`=`).
- **`standard-no-pad`** (`standard-no-pad`): Standard Base64 alphabet without padding.
- **`standard-no-pad-indifferent`** (`standard-no-pad-indifferent`): Standard Base64 decoding/encoding with indifferent padding handling without padding.
- **`standard-pad-indifferent`** (`standard-pad-indifferent`): Standard Base64 encoding/decoding with indifferent padding handling with padding.
- **`url-safe`** (`url-safe`): URL and filename safe alphabet (RFC 4648 §5) with padding (`=`).
- **`url-safe-no-pad`** (`url-safe-no-pad`): URL-safe Base64 alphabet without padding.
- **`url-safe-no-pad-indifferent`** (`url-safe-no-pad-indifferent`): URL-safe decoding/encoding with indifferent padding handling without padding.
- **`url-safe-pad-indifferent`** (`url-safe-pad-indifferent`): URL-safe encoding/decoding with indifferent padding handling with padding.

---

## Usage Example (Encoding)

### Strings
```bash
hjlix base64-encrypt "Hello, World!" --base64-type standard
```

Result:
```txt
Original: Hello, World!
Result: SGVsbG8sIFdvcmxkIQ==
```

Using URL-safe encoding:
```bash
hjlix base64-encrypt "Hello, World!?" --base64-type url-safe
```

---

## Usage Example (Decryption)

### Decoded Strings
```bash
hjlix base64-decrypt "SGVsbG8sIFdvcmxkIQ==" --base64-type standard
```

Result:
```txt
Original: SGVsbG8sIFdvcmxkIQ==
Result: Hello, World!
```

---

## Features and Characteristics
- **Multiple Engine Standards**: Supports both standard and URL-safe alphabets.
- **Padding Control**: Options for strict padding, no padding, or indifferent padding handling.
- **String Support**: Encrypt or decrypt raw input strings directly from the command line.
- **Fast & Reliable**: Leverages the robust `base64` Rust crate.
