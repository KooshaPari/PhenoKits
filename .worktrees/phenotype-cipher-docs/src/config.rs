//! Configuration validation for cryptographic parameters
//!
//! Ensures key lengths, algorithm selections, and parameters are secure.

use phenotype_validation::{Result, ValidationResult, Validator};
use serde::{Deserialize, Serialize};

/// Cipher configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CipherConfig {
    /// Algorithm selection
    pub algorithm: String,
    /// Key size in bits
    pub key_bits: u32,
    /// Nonce/IV size in bits
    pub nonce_bits: u32,
}

/// Validates cipher configuration for security
pub fn validate_cipher_config(config: &CipherConfig) -> Result<ValidationResult> {
    let mut validator = Validator::new()
        .required("algorithm")
        .string("algorithm")
        .one_of(vec![
            "AES-256-GCM",
            "AES-128-GCM",
            "ChaCha20-Poly1305",
            "XChaCha20-Poly1305",
        ]);

    let json = serde_json::json!(config);
    let mut result = validator.validate(&json)?;

    // Additional security validation
    match config.algorithm.as_str() {
        "AES-256-GCM" => {
            if config.key_bits != 256 {
                result.add_error(phenotype_validation::ValidationError::constraint(
                    "key_bits",
                    "AES-256-GCM requires exactly 256-bit keys",
                ));
            }
            if config.nonce_bits != 96 {
                result.add_error(phenotype_validation::ValidationError::constraint(
                    "nonce_bits",
                    "AES-GCM requires 96-bit nonce",
                ));
            }
        }
        "AES-128-GCM" => {
            if config.key_bits != 128 {
                result.add_error(phenotype_validation::ValidationError::constraint(
                    "key_bits",
                    "AES-128-GCM requires exactly 128-bit keys",
                ));
            }
        }
        "ChaCha20-Poly1305" => {
            if config.key_bits != 256 {
                result.add_error(phenotype_validation::ValidationError::constraint(
                    "key_bits",
                    "ChaCha20 requires exactly 256-bit keys",
                ));
            }
            if config.nonce_bits != 96 {
                result.add_error(phenotype_validation::ValidationError::constraint(
                    "nonce_bits",
                    "ChaCha20 uses 96-bit nonce",
                ));
            }
        }
        _ => {}
    }

    // Reject weak key sizes
    if config.key_bits < 128 {
        result.add_error(phenotype_validation::ValidationError::constraint(
            "key_bits",
            "Key size must be at least 128 bits",
        ));
    }

    Ok(result)
}

/// Validates Ed25519 signing configuration
pub fn validate_ed25519_config() -> Result<ValidationResult> {
    // Ed25519 has fixed parameters
    Ok(ValidationResult::valid())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_aes256_config() {
        let config = CipherConfig {
            algorithm: "AES-256-GCM".to_string(),
            key_bits: 256,
            nonce_bits: 96,
        };

        let result = validate_cipher_config(&config).unwrap();
        assert!(result.is_valid);
    }

    #[test]
    fn test_invalid_aes256_key_size() {
        let config = CipherConfig {
            algorithm: "AES-256-GCM".to_string(),
            key_bits: 128, // Wrong size
            nonce_bits: 96,
        };

        let result = validate_cipher_config(&config).unwrap();
        assert!(!result.is_valid);
    }

    #[test]
    fn test_weak_key_rejection() {
        let config = CipherConfig {
            algorithm: "AES-128-GCM".to_string(),
            key_bits: 64, // Too weak
            nonce_bits: 96,
        };

        let result = validate_cipher_config(&config).unwrap();
        assert!(!result.is_valid);
    }

    #[test]
    fn test_valid_chacha20_config() {
        let config = CipherConfig {
            algorithm: "ChaCha20-Poly1305".to_string(),
            key_bits: 256,
            nonce_bits: 96,
        };

        let result = validate_cipher_config(&config).unwrap();
        assert!(result.is_valid);
    }
}
