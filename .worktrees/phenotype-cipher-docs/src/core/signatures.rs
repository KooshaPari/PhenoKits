//! Digital signature utilities
//! Provides Ed25519 digital signatures for authentication and non-repudiation

use ed25519_dalek::{Signature as DalekSignature, Signer, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;
use rand::RngCore;
use thiserror::Error;

/// Signature error types
#[derive(Debug, Error)]
pub enum SignatureError {
    #[error("invalid secret key")]
    InvalidSecretKey,
    #[error("invalid public key")]
    InvalidPublicKey,
    #[error("signature verification failed")]
    VerificationFailed,
}

/// KeyPair container
#[derive(Debug, Clone, PartialEq)]
pub struct KeyPair {
    pub public_key: Vec<u8>,
    pub secret_key: Vec<u8>,
}

/// Generate a new Ed25519 signing keypair
pub fn generate_keypair() -> KeyPair {
    let mut secret_key_bytes = [0u8; 32];
    OsRng.fill_bytes(&mut secret_key_bytes);
    let signing_key = SigningKey::from_bytes(&secret_key_bytes);
    let verifying_key = signing_key.verifying_key();

    KeyPair {
        public_key: verifying_key.to_bytes().to_vec(),
        secret_key: secret_key_bytes.to_vec(),
    }
}

/// Sign data with Ed25519
pub fn sign(message: &[u8], secret_key: &[u8]) -> Result<Vec<u8>, SignatureError> {
    let key_bytes: [u8; 32] = secret_key
        .try_into()
        .map_err(|_| SignatureError::InvalidSecretKey)?;
    let signing_key = SigningKey::from_bytes(&key_bytes);
    let sig = signing_key.sign(message);
    Ok(sig.to_bytes().to_vec())
}

/// Verify Ed25519 signature
pub fn verify(message: &[u8], signature: &[u8], public_key: &[u8]) -> Result<(), SignatureError> {
    let pk_bytes: [u8; 32] = public_key
        .try_into()
        .map_err(|_| SignatureError::InvalidPublicKey)?;
    let verifying_key =
        VerifyingKey::from_bytes(&pk_bytes).map_err(|_| SignatureError::InvalidPublicKey)?;

    let sig_bytes: [u8; 64] = signature
        .try_into()
        .map_err(|_| SignatureError::InvalidPublicKey)?;
    let sig = DalekSignature::from_bytes(&sig_bytes);

    verifying_key
        .verify(message, &sig)
        .map_err(|_| SignatureError::VerificationFailed)
}
