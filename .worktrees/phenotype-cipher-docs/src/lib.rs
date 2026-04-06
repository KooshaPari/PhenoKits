//! Cryptographic primitives for the Phenotype ecosystem.
//!
//! Provides authenticated encryption (AES-256-GCM, ChaCha20-Poly1305),
//! digital signatures (Ed25519), and hashing (SHA-256).
//!
//! # Quick Start
//!
//! ```rust
//! use phenotype_cipher::{AesGcmCipher, sha256, CipherResult};
//!
//! // Generate a key and encrypt data
//! let key = AesGcmCipher::generate_key();
//! let cipher = AesGcmCipher::new(&key).unwrap();
//! let ciphertext = cipher.encrypt(b"hello world").unwrap();
//! let plaintext = cipher.decrypt(&ciphertext).unwrap();
//!
//! // Hash data
//! let hash = sha256(b"hello world");
//!
//! // Generate signing keys
//! let keypair = phenotype_cipher::generate_keypair();
//! ```
//! ```

#[doc(hidden)]
pub mod core;

pub use core::encryption::{
    decrypt_with_suite, encrypt_with_suite, AesGcmCipher, ChaChaCipher, CipherSuite, Ciphertext,
    EncryptionError,
};
pub use core::hashing::sha256;
pub use core::signatures::{generate_keypair, sign, verify, SignatureError};

use thiserror::Error;

/// Unified error type for all cipher operations
#[derive(Error, Debug, Clone, PartialEq)]
pub enum CipherError {
    #[error("invalid key: {0}")]
    InvalidKey(String),
    #[error("encryption failed: {0}")]
    EncryptionFailed(String),
    #[error("decryption failed: {0}")]
    DecryptionFailed(String),
    #[error("signature failed: {0}")]
    SignatureFailed(String),
    #[error("hashing failed: {0}")]
    HashingFailed(String),
}

/// Type alias for cipher results
pub type CipherResult<T> = Result<T, CipherError>;

impl From<EncryptionError> for CipherError {
    fn from(e: EncryptionError) -> Self {
        match e {
            EncryptionError::InvalidKeyLength { expected, got } => {
                CipherError::InvalidKey(format!("expected {} bytes, got {}", expected, got))
            }
            EncryptionError::EncryptionFailed(s) => CipherError::EncryptionFailed(s),
            EncryptionError::DecryptionFailed(s) => CipherError::DecryptionFailed(s),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aes_gcm_encryption() {
        let key = AesGcmCipher::generate_key();
        let cipher = AesGcmCipher::new(&key).unwrap();
        let PT = b"hello world";
        let ct = cipher.encrypt(PT).unwrap();
        let decrypted = cipher.decrypt(&ct).unwrap();
        assert_eq!(PT, decrypted.as_slice());
    }

    #[test]
    fn test_chacha_encryption() {
        let key = ChaChaCipher::generate_key();
        let cipher = ChaChaCipher::new(&key).unwrap();
        let PT = b"hello world";
        let ct = cipher.encrypt(PT).unwrap();
        let decrypted = cipher.decrypt(&ct).unwrap();
        assert_eq!(PT, decrypted.as_slice());
    }

    #[test]
    fn test_sha256_hashing() {
        let data = b"hello world";
        let hash1 = sha256(data);
        let hash2 = sha256(data);
        assert_eq!(hash1, hash2);
        assert_eq!(hash1.len(), 32);
    }

    #[test]
    fn test_ed25519_signing() {
        let kp = generate_keypair();
        let msg = b"hello world";
        let sig = sign(msg, &kp.secret_key).unwrap();
        verify(msg, &sig, &kp.public_key).unwrap();
    }

    #[test]
    fn test_error_display() {
        let err = CipherError::InvalidKey("test".to_string());
        assert!(err.to_string().contains("invalid key"));
    }
}
