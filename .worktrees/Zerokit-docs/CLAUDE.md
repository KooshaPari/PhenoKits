# cryptokit

Rust cryptography utilities library with support for zero-knowledge proofs and encryption. Provides high-level crypto primitives for Phenotype services.

## Stack
- Language: Rust (inferred from kit naming pattern)
- Key deps: Cargo, ring or rustcrypto, zk proof library (bellman or arkworks)

## Structure
- `src/`: Cryptography utilities
  - Symmetric encryption (AES-GCM, ChaCha20-Poly1305)
  - Asymmetric encryption and key exchange
  - Zero-knowledge proof generation and verification
  - Hashing and key derivation

## Key Patterns
- Use well-audited crates (ring, rustcrypto) — no hand-rolled crypto primitives
- Constant-time comparisons for all secret data
- Keys are zeroized on drop
- Errors are opaque (no timing leaks via error messages)

## Adding New Functionality
- New crypto primitive: wrap an existing audited crate, do not implement from scratch
- Add to appropriate module in `src/`
- Run `cargo test` to verify
