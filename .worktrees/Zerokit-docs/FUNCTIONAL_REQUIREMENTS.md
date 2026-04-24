# Functional Requirements — cryptokit

## FR-HASH-001
`hash::sha256(data: &[u8]) -> [u8; 32]` SHALL return the SHA-256 digest of `data`.

## FR-HASH-002
`hash::sha512(data: &[u8]) -> [u8; 64]` SHALL return the SHA-512 digest of `data`.

## FR-HMAC-001
`hmac::sign_sha256(key: &[u8], message: &[u8]) -> [u8; 32]` SHALL compute HMAC-SHA256.

## FR-HMAC-002
`hmac::verify_sha256(key, message, tag)` SHALL perform constant-time comparison and return `bool`.

## FR-AES-001
`aes_gcm::encrypt(key: &[u8; 32], plaintext: &[u8]) -> Result<Vec<u8>, CryptoError>` SHALL return nonce (12 bytes) prepended to ciphertext.

## FR-AES-002
`aes_gcm::decrypt(key: &[u8; 32], ciphertext: &[u8]) -> Result<Vec<u8>, CryptoError>` SHALL authenticate and decrypt, returning `CryptoError::AuthFailed` on tag mismatch.

## FR-JWT-001
`jwt::sign_hs256<T: Serialize>(claims: &T, secret: &[u8]) -> Result<String, CryptoError>` SHALL produce a valid compact JWT.

## FR-JWT-002
`jwt::verify_hs256<T: DeserializeOwned>(token: &str, secret: &[u8]) -> Result<T, CryptoError>` SHALL verify signature and expiry.

## FR-JWT-003
Tokens with `exp` in the past SHALL produce `CryptoError::TokenExpired`.

## FR-KDF-001
`kdf::argon2id(password: &[u8], salt: &[u8]) -> [u8; 32]` SHALL use Argon2id with OWASP-recommended parameters.

## FR-KDF-002
`kdf::pbkdf2_sha256(password, salt, iterations)` SHALL use PBKDF2-HMAC-SHA256.

## FR-ERR-001
`CryptoError` SHALL have variants: `AuthFailed`, `TokenExpired`, `TokenInvalid`, `KeyInvalid`, `Internal`.

## FR-SAFE-001
`#\![forbid(unsafe_code)]` SHALL be present in `src/lib.rs`.

## FR-TEST-001
`cargo test` SHALL pass including known-answer tests against NIST vectors for SHA-256 and HMAC-SHA256.
