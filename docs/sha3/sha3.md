# SHA-3 and Keccak Hashing

SHA-3 (Secure Hash Algorithm 3) is the latest member of the Secure Hash Algorithm family of standards, released by NIST in 2015. It is based on the Keccak algorithm.

## Supported Variants
`hjlix` supports both the NIST-standardized SHA-3 variants and the original Keccak variants.

### NIST SHA-3 Variants
- **SHA3-224**
- **SHA3-256**
- **SHA3-384**
- **SHA3-512**

### Keccak Variants
- **Keccak-224**
- **Keccak-256**
- **Keccak-384**
- **Keccak-512**
- **Keccak-256-Full**

## Usage Example
```bash
hjlix sha3 "Hello, World!" --sha-type sha3-512
```

Result:
```txt
38e05c33d7b067127f217d8c856e554fcff09c9320b8a5979ce2ff5d95dd27ba35d1fba50c562dfd1d6cc48bc9c5baa4390894418cc942d968f97bcb659419ed
```

## Note on Keccak vs. SHA-3
While SHA-3 is based on Keccak, there are subtle differences in the padding and input processing between the original Keccak submissions and the final NIST SHA-3 standard. `hjlix` allows you to choose exactly which implementation you need.
