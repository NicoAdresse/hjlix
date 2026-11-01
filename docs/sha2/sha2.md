# SHA-2 Hashing

The SHA-2 (Secure Hash Algorithm 2) family is a set of cryptographic hash functions designed by the NSA. `hjlix` utilizes the `sha2` crate to provide high-performance implementations of these standards.

## Supported Variants
- **SHA-224**
- **SHA-256**
- **SHA-384**
- **SHA-512**
- **SHA-512/224**
- **SHA-512/256**

## Usage Example
```bash
hjlix sha2 "Hello, World!" --sha-type sha384
```

Result:
```txt
5485cc9b3365b4305dfb4e8337e0a598a574f8242bf17289e0dd6c20a3cd44a089de16ab4ab308f63e44b1170eb5f515
```

## Use Case
SHA-2 is widely used for data integrity verification, digital signatures, and password hashing (when combined with proper salting/key stretching).
