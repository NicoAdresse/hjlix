# Cargo Tree

```txt
hjlix v0.1.5-3
├── aes-gcm v0.11.1
│   ├── aead v0.6.1
│   │   ├── crypto-common v0.2.2
│   │   │   ├── getrandom v0.4.3
│   │   │   │   ├── cfg-if v1.0.5
│   │   │   │   ├── libc v0.2.191
│   │   │   │   └── rand_core v0.10.1
│   │   │   ├── hybrid-array v0.4.15
│   │   │   │   └── typenum v1.20.1
│   │   │   └── rand_core v0.10.1
│   │   └── inout v0.2.5
│   │       └── hybrid-array v0.4.15 (*)
│   ├── aes v0.9.4
│   │   ├── cipher v0.5.2
│   │   │   ├── block-buffer v0.12.1
│   │   │   │   └── hybrid-array v0.4.15 (*)
│   │   │   ├── crypto-common v0.2.2 (*)
│   │   │   └── inout v0.2.5 (*)
│   │   ├── cpubits v0.1.1
│   │   └── cpufeatures v0.3.1
│   ├── cipher v0.5.2 (*)
│   ├── ctr v0.10.1
│   │   └── cipher v0.5.2 (*)
│   ├── ctutils v0.4.8
│   │   └── cmov v0.5.4
│   └── ghash v0.6.0
│       └── polyval v0.7.3
│           ├── cpubits v0.1.1
│           ├── cpufeatures v0.3.1
│           └── universal-hash v0.6.1
│               ├── crypto-common v0.2.2 (*)
│               └── ctutils v0.4.8 (*)
├── anyhow v1.0.104
├── base64 v0.23.1
├── chacha20poly1305 v0.11.0
│   ├── aead v0.6.1 (*)
│   ├── chacha20 v0.10.2
│   │   ├── cfg-if v1.0.5
│   │   ├── cipher v0.5.2 (*)
│   │   └── cpufeatures v0.3.1
│   ├── cipher v0.5.2 (*)
│   └── poly1305 v0.9.1
│       ├── cpufeatures v0.3.1
│       └── universal-hash v0.6.1 (*)
├── clap v4.6.7
│   ├── clap_builder v4.6.7
│   │   ├── anstream v1.0.0
│   │   │   ├── anstyle v1.0.14
│   │   │   ├── anstyle-parse v1.0.0
│   │   │   │   └── utf8parse v0.2.2
│   │   │   ├── anstyle-query v1.1.5
│   │   │   ├── colorchoice v1.0.5
│   │   │   ├── is_terminal_polyfill v1.70.2
│   │   │   └── utf8parse v0.2.2
│   │   ├── anstyle v1.0.14
│   │   ├── clap_lex v1.1.1
│   │   └── strsim v0.11.1
│   └── clap_derive v4.6.7 (proc-macro)
│       ├── heck v0.5.0
│       ├── proc-macro2 v1.0.107
│       │   └── unicode-ident v1.0.27
│       ├── quote v1.0.47
│       │   └── proc-macro2 v1.0.107 (*)
│       └── syn v3.0.9
│           ├── proc-macro2 v1.0.107 (*)
│           ├── quote v1.0.47 (*)
│           └── unicode-ident v1.0.27
├── hex v0.4.3
├── sha2 v0.11.1
│   ├── cfg-if v1.0.5
│   ├── cpufeatures v0.3.1
│   └── digest v0.11.3
│       ├── block-buffer v0.12.1 (*)
│       ├── const-oid v0.10.2
│       └── crypto-common v0.2.2 (*)
└── sha3 v0.12.0
    ├── digest v0.11.3 (*)
    ├── keccak v0.2.2
    │   └── cfg-if v1.0.5
    └── sponge-cursor v0.1.0
```