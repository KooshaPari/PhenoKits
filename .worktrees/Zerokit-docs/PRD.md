# Product Requirements Document — cryptokit

## Overview

`cryptokit` is a Rust cryptography utility library for the Phenotype ecosystem. It provides safe, auditable abstractions for common cryptographic operations: hashing, HMAC, AES-GCM encryption, JWT signing/verification, and key derivation. It wraps established, audited Rust cryptography crates from the RustCrypto and ring ecosystems.

## Problem Statement

Cryptographic operations appear across multiple Phenotype services (auth, API signing, token validation). Hand-rolling these per service is error-prone. `cryptokit` provides a single, audited, well-tested surface that all services can depend on.

## Goals

- Safe, ergonomic wrappers for: SHA-256/512 hashing, HMAC-SHA256/512, AES-256-GCM encryption/decryption, Ed25519 and RS256 JWT signing/verification, PBKDF2 / Argon2id key derivation.
- All operations are constant-time where required (comparisons, key material handling).
- Zero `unsafe` code in library code.
- Publish to crates.io as `cryptokit`.

## Non-Goals

- Not a TLS library — does not implement TLS handshakes.
- Does not provide certificate management or PKI.
- Does not implement custom cryptographic algorithms — only wraps audited implementations.

## Epics & User Stories

### E1 — Hashing
- E1.1: `hash::sha256(data: &[u8]) -> [u8; 32]` returns the SHA-256 digest.
- E1.2: `hash::sha512(data: &[u8]) -> [u8; 64]` returns the SHA-512 digest.
- E1.3: Output can be hex-encoded via `to_hex()` helper.

### E2 — HMAC
- E2.1: `hmac::sign_sha256(key: &[u8], message: &[u8]) -> [u8; 32]` computes HMAC-SHA256.
- E2.2: `hmac::verify_sha256(key, message, expected)` performs constant-time comparison.

### E3 — Symmetric Encryption
- E3.1: `aes_gcm::encrypt(key: &[u8; 32], plaintext: &[u8]) -> Result<Vec<u8>, CryptoError>` returns nonce-prefixed ciphertext.
- E3.2: `aes_gcm::decrypt(key: &[u8; 32], ciphertext: &[u8]) -> Result<Vec<u8>, CryptoError>` decrypts and authenticates.

### E4 — JWT
- E4.1: `jwt::sign_hs256(claims: &impl Serialize, secret: &[u8]) -> Result<String, CryptoError>` produces a compact JWT.
- E4.2: `jwt::verify_hs256<T: DeserializeOwned>(token: &str, secret: &[u8]) -> Result<T, CryptoError>` verifies and decodes.
- E4.3: Expired tokens produce `CryptoError::TokenExpired`.

### E5 — Key Derivation
- E5.1: `kdf::argon2id(password: &[u8], salt: &[u8]) -> [u8; 32]` derives a key using Argon2id.
- E5.2: `kdf::pbkdf2_sha256(password, salt, iterations) -> [u8; 32]` derives a key using PBKDF2.

### E6 — Testing
- E6.1: `cargo test` passes with zero failures.
- E6.2: Known-answer tests (KATs) validate all hash and HMAC implementations against NIST vectors.

## Acceptance Criteria

- `cargo build` and `cargo test` succeed.
- `#\![forbid(unsafe_code)]` present in lib.rs.
- All public functions have doc comments with security notes.
