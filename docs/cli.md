# hjlix CLI Documentation

`hjlix` is a command-line tool designed to provide fast, reliable cryptographic hashing for strings. It supports both SHA-2 and SHA-3 (including Keccak) families.

## Overview
The tool is modularly organized. You can interact with it using specific subcommands.

### Commands
- `sha2`: Computes hashes using the SHA-2 family.
- `sha3`: Computes hashes using the SHA-3/Keccak family.
- `about`: Displays metadata about the project, including version and authorship.

### Global Options
- `--write-into-file`: When provided, the output is saved to `file_with_sha.txt` in the current directory instead of being printed to stdout.
