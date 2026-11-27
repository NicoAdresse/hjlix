# File Tree

```txt
├── Cargo.lock
├── Cargo.toml
├── CODE_OF_CONDUCT.md
├── CONTRIBUTING.md
├── Dockerfile
├── docs
│   ├── aes-gcm
│   │   └── aes-gcm.md
│   ├── base64
│   │   └── base64.md
│   ├── chacha20
│   │   └── chacha20.md
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
    │   ├── aes_gcm
    │   │   ├── aes_gcmdecryptionfile_impl.rs
    │   │   ├── aes_gcmdecryption_impl.rs
    │   │   ├── aes_gcmfile_impl.rs
    │   │   ├── aes_gcmstr_impl.rs
    │   │   ├── aes_gcm_types.rs
    │   │   └── mod.rs
    │   ├── base64
    │   │   ├── base64_decryption_impl.rs
    │   │   ├── base64_encryption_impl.rs
    │   │   ├── base64_types.rs
    │   │   └── mod.rs
    │   ├── chacha20
    │   │   ├── chacha20_decryption_impl.rs
    │   │   ├── chacha20_encryption_impl.rs
    │   │   ├── chacha20_file_impl.rs
    │   │   ├── chacha20_types.rs
    │   │   └── mod.rs
    │   ├── commands.rs
    │   ├── consts
    │   │   ├── constants.rs
    │   │   └── mod.rs
    │   ├── entcli.rs
    │   ├── err
    │   │   ├── aes_err.rs
    │   │   ├── base64_err.rs
    │   │   ├── chacha20_err.rs
    │   │   ├── io_err.rs
    │   │   └── mod.rs
    │   ├── misc
    │   │   ├── about.rs
    │   │   └── mod.rs
    │   ├── mod.rs
    │   ├── sha2
    │   │   ├── mod.rs
    │   │   ├── sha2file_impl.rs
    │   │   ├── sha2str_impl.rs
    │   │   └── sha2_types.rs
    │   ├── sha3
    │   │   ├── mod.rs
    │   │   ├── sha3file_impl.rs
    │   │   ├── sha3str_impl.rs
    │   │   └── sha3_types.rs
    │   └── traits
    │       ├── digest.rs
    │       ├── generate_aes_key.rs
    │       ├── generate_chacha20_key.rs
    │       └── mod.rs
    └── main.rs
```