---
layout: home
title: phenotype-cipher - Simple, Safe Cryptography for Rust
titleTemplate: false
---

# phenotype-cipher

Simple, safe cryptography for Rust.

## Overview

Encryption and cipher utilities for the Phenotype ecosystem.

## Features

- **Encryption**: AES-GCM, ChaCha20-Poly1305
- **Hashing**: SHA-256, BLAKE3, Argon2
- **Signatures**: Ed25519, ECDSA
- **Key Derivation**: HKDF, PBKDF2

## Quick Start

```rust
use cipher::{encrypt, decrypt, hash};

let ciphertext = encrypt(b"secret message", &key)?;
let plaintext = decrypt(&ciphertext, &key)?;
let digest = hash::sha256(b"data");
```

## Security

- **Security First**: All cryptographic operations use constant-time implementations
- **Secure Defaults**: Require explicit opt-in for insecure options
- **No Key Logging**: Keys and sensitive data are never logged
- **Fail Loudly**: Cryptographic failures produce clear errors

## Links

- [Repository](https://github.com/KooshaPari/phenotype-cipher)
