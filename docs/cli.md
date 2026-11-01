# hjlix CLI Documentation

`hjlix` is a command-line tool designed to provide fast, reliable cryptographic hashing for strings. It supports both SHA-2 and SHA-3 (including Keccak) families.

## Overview
The tool is modularly organized. You can interact with it using specific subcommands.

### Commands
- `sha2`: Computes hashes using the SHA-2 family.
- `sha3`: Computes hashes using the SHA-3/Keccak family.
- `about`: Displays metadata about the project, including version and authorship.

### Examples

**SHA-2:**
```bash
hjlix sha2 "Hello, World!" --sha-type sha384
```

**SHA-3/Keccak:**
```bash
hjlix sha3 "Hello, World!" --sha-type sha3-512
```

### Global Options
- `--write-into-file`: When provided, the output is saved to `file_with_sha.txt` in the current directory instead of being printed to stdout.

## File Tree & Cargo Tree

### File Tree

```txt
├── Cargo.lock
├── Cargo.toml
├── CODE_OF_CONDUCT.md
├── CONTRIBUTING.md
├── Dockerfile
├── docs
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
    │   ├── consts
    │   │   ├── constants.rs
    │   │   └── mod.rs
    │   ├── entcli.rs
    │   ├── err
    │   │   ├── io_err.rs
    │   │   └── mod.rs
    │   ├── misc
    │   │   ├── about.rs
    │   │   └── mod.rs
    │   ├── mod.rs
    │   ├── sha2
    │   │   ├── mod.rs
    │   │   ├── sha2_impl.rs
    │   │   └── sha2_types.rs
    │   ├── sha3
    │   │   ├── mod.rs
    │   │   ├── sha3_impl.rs
    │   │   └── sha3_types.rs
    │   └── traits
    │       ├── digest.rs
    │       └── mod.rs
    └── main.rs
```

### Cargo Tree

```txt
hjlix v0.1.2
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
│       ├── block-buffer v0.12.1
│       │   └── hybrid-array v0.4.15
│       │       └── typenum v1.20.1
│       ├── const-oid v0.10.2
│       └── crypto-common v0.2.2
│           └── hybrid-array v0.4.15 (*)
└── sha3 v0.12.0
    ├── digest v0.11.3 (*)
    ├── keccak v0.2.2
    │   └── cfg-if v1.0.5
    └── sponge-cursor v0.1.0
```