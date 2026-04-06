# Zerokit

Zero-config cryptography toolkit for the Phenotype ecosystem.

## Overview

`Zerokit` (also published as `cryptokit`) is a Rust cryptography utility library providing safe, auditable abstractions for common cryptographic operations: hashing, HMAC, AES-GCM encryption, JWT signing/verification, and key derivation. It wraps established, audited Rust cryptography crates from the RustCrypto and ring ecosystems.

## Features

- **Hashing**: SHA-256/512 with hex encoding helpers
- **HMAC**: SHA256/512-based message authentication
- **AES-GCM**: Authenticated encryption with associated data (AEAD)
- **JWT**: HS256, Ed25519, and RS256 signing/verification
- **Key Derivation**: Argon2id and PBKDF2-HMAC-SHA256
- **Constant-Time Operations**: Prevents timing attacks
- **Zero Unsafe Code**: `#![forbid(unsafe_code)]`

## Installation

```bash
# Add to Cargo.toml
[dependencies]
cryptokit = { git = "https://github.com/KooshaPari/Zerokit" }
```

## Quick Start

```rust
use zerokit::{hash, hmac, aes_gcm, jwt, kdf};

// SHA-256 hashing
let digest = hash::sha256(b"hello world");
let hex = digest.to_hex();

// HMAC-SHA256
let key = b"secret key";
let message = b"hello world";
let tag = hmac::sign_sha256(key, message);
assert!(hmac::verify_sha256(key, message, &tag));

// AES-256-GCM encryption
let key = [0u8; 32];
let plaintext = b"secret data";
let ciphertext = aes_gcm::encrypt(&key, plaintext)?;
let decrypted = aes_gcm::decrypt(&key, &ciphertext)?;

// JWT signing
let claims = serde_json::json!({"sub": "user123", "exp": 1234567890});
let token = jwt::sign_hs256(&claims, b"secret")?;
let verified: serde_json::Value = jwt::verify_hs256(&token, b"secret")?;

// Argon2id key derivation
let password = b"user password";
let salt = b"random salt...";
let key = kdf::argon2id(password, salt);
```

## API Overview

### Hash Module

| Function | Description |
|----------|-------------|
| `hash::sha256(data)` | Compute SHA-256 digest |
| `hash::sha512(data)` | Compute SHA-512 digest |

### HMAC Module

| Function | Description |
|----------|-------------|
| `hmac::sign_sha256(key, message)` | Compute HMAC-SHA256 |
| `hmac::verify_sha256(key, message, tag)` | Constant-time verification |

### AES-GCM Module

| Function | Description |
|----------|-------------|
| `aes_gcm::encrypt(key, plaintext)` | Encrypt with random nonce |
| `aes_gcm::decrypt(key, ciphertext)` | Decrypt and verify tag |

### JWT Module

| Function | Description |
|----------|-------------|
| `jwt::sign_hs256(claims, secret)` | Sign with HMAC-SHA256 |
| `jwt::verify_hs256(token, secret)` | Verify and decode HS256 JWT |
| `jwt::sign_ed25519(claims, keypair)` | Sign with Ed25519 |
| `jwt::verify_ed25519(token, public_key)` | Verify Ed25519 signature |

### KDF Module

| Function | Description |
|----------|-------------|
| `kdf::argon2id(password, salt)` | Argon2id key derivation |
| `kdf::pbkdf2_sha256(password, salt, iterations)` | PBKDF2 key derivation |

### Error Types

```rust
pub enum CryptoError {
    AuthFailed,      // Authentication tag mismatch
    TokenExpired,    // JWT exp claim in the past
    TokenInvalid,    // JWT signature or format invalid
    KeyInvalid,      // Invalid key length or format
    Internal,        // Internal library error
}
```

## Architecture

```
Zerokit/
├── src/
│   ├── hash.rs       # SHA-256/512 implementations
│   ├── hmac.rs       # HMAC-SHA256/512
│   ├── aes_gcm.rs    # AES-256-GCM encryption
│   ├── jwt.rs        # JWT signing and verification
│   ├── kdf.rs        # Argon2id and PBKDF2
│   ├── error.rs      # Error types
│   └── lib.rs        # Crate root with #![forbid(unsafe_code)]
├── tests/            # Known-answer tests (NIST vectors)
└── Cargo.toml
```

## Security

- **No unsafe code**: Entirely safe Rust with `#![forbid(unsafe_code)]`
- **Constant-time comparisons**: Prevents timing side-channel attacks
- **Zeroization**: Keys are zeroed from memory on drop
- **Audited dependencies**: Only uses well-reviewed crypto crates
- **NIST test vectors**: All primitives validated against known-answer tests

## Development

```bash
# Build
cargo build

# Run tests (includes KATs)
cargo test

# Check lints
cargo clippy -- -D warnings

# Format code
cargo fmt

# Security audit
cargo audit
```

## Testing

All cryptographic implementations include:

1. **Unit tests**: Basic functionality testing
2. **Known-answer tests**: Validated against NIST CAVP test vectors
3. **Fuzz tests**: For parsers and edge cases

```bash
# Run all tests
cargo test

# Run specific test
cargo test sha256_kat

# Fuzz testing
cargo fuzz run fuzz_target_name
```

## Links

- **Docs**: [https://docs.rs/cryptokit](https://docs.rs/cryptokit) (pending publication)
- **Repository**: https://github.com/KooshaPari/Zerokit
- **Security Issues**: Please report security issues privately

## Contributing

1. **Security first**: All crypto changes require security review
2. No custom algorithms - only wrap audited implementations
3. All functions must have security documentation
4. Constant-time operations for secret data
5. Full test coverage including KATs

## License

MIT OR Apache-2.0
