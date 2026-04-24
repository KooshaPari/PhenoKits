# Architecture Decision Records — cryptokit

## ADR-001: RustCrypto Crates as Foundation

**Status:** Accepted

**Context:** Rust cryptography options include `ring`, `openssl` bindings, `rustls` internals, and RustCrypto crates.

**Decision:** Use RustCrypto crates (`sha2`, `hmac`, `aes-gcm`, `argon2`) as the foundation. `ring` may be used for Ed25519 where RustCrypto coverage is weaker.

**Rationale:** RustCrypto crates are pure Rust, no C bindings, `#\![forbid(unsafe_code)]`-compatible (or have audited unsafe), and are widely reviewed. They are the ecosystem standard for pure-Rust cryptography.

**Alternatives Considered:**
- `openssl` crate: requires OpenSSL system library; not portable to WASM/embedded.
- `ring`: excellent but opinionated API, C internals, less modular than RustCrypto.

**Consequences:** No system library dependencies. Binary size increases by ~200 KB for all features; acceptable for service deployments.

---

## ADR-002: Nonce-Prepended Ciphertext Format for AES-GCM

**Status:** Accepted

**Context:** AES-GCM requires a unique nonce per encryption. Callers should not manage nonces manually.

**Decision:** `encrypt` generates a random 12-byte nonce internally and prepends it to the ciphertext. `decrypt` extracts the nonce from the first 12 bytes.

**Rationale:** Eliminates nonce reuse errors. Callers only handle a single opaque blob.

**Consequences:** Ciphertext is 28 bytes longer than plaintext (12-byte nonce + 16-byte GCM tag). Format is internal to `cryptokit` — cross-service compatibility requires both sides using `cryptokit`.

---

## ADR-003: JWT via jsonwebtoken Crate

**Status:** Accepted

**Context:** JWT libraries include `jsonwebtoken`, `frank_jwt`, and manual implementation.

**Decision:** Use `jsonwebtoken` crate for JWT operations.

**Rationale:** `jsonwebtoken` is the most widely used Rust JWT library with comprehensive algorithm support and active maintenance.

**Consequences:** `jsonwebtoken` is a direct dependency; its API shapes the `jwt::` module interface.
