//! Ed25519 Cryptographic Signer and Verifier Engine
//!
//! Provides deterministic key generation, SHA-512 hashing,
//! detached signature creation, and cryptographic verification.

use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use sha2::{Digest, Sha512};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EvidenceError {
    #[error("Signature verification failed: invalid signature")]
    VerificationFailed,

    #[error("SHA-512 payload hash mismatch: expected {expected}, actual {actual}")]
    HashMismatch { expected: String, actual: String },

    #[error("Invalid public key encoding: {0}")]
    InvalidPublicKey(String),

    #[error("Invalid signature encoding: {0}")]
    InvalidSignature(String),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

pub struct CryptoSigner;

impl CryptoSigner {
    /// Computes the hex-encoded SHA-512 cryptographic digest of raw byte data.
    pub fn sha512_digest(data: &[u8]) -> String {
        let mut hasher = Sha512::new();
        hasher.update(data);
        let hash = hasher.finalize();
        hex::encode(hash)
    }

    /// Derives an Ed25519 keypair from a 32-byte secret seed.
    pub fn keypair_from_seed(seed: &[u8; 32]) -> (SigningKey, VerifyingKey) {
        let signing_key = SigningKey::from_bytes(seed);
        let verifying_key = signing_key.verifying_key();
        (signing_key, verifying_key)
    }

    /// Signs an arbitrary message using the given SigningKey.
    pub fn sign(signing_key: &SigningKey, message: &[u8]) -> String {
        let signature: Signature = signing_key.sign(message);
        hex::encode(signature.to_bytes())
    }

    /// Verifies an Ed25519 signature against the message and public key.
    pub fn verify(
        verifying_key_hex: &str,
        message: &[u8],
        signature_hex: &str,
    ) -> Result<bool, EvidenceError> {
        let vk_bytes = hex::decode(verifying_key_hex)
            .map_err(|e| EvidenceError::InvalidPublicKey(e.to_string()))?;
        if vk_bytes.len() != 32 {
            return Err(EvidenceError::InvalidPublicKey("Key must be 32 bytes".to_string()));
        }

        let mut key_arr = [0u8; 32];
        key_arr.copy_from_slice(&vk_bytes);
        let verifying_key = VerifyingKey::from_bytes(&key_arr)
            .map_err(|e| EvidenceError::InvalidPublicKey(e.to_string()))?;

        let sig_bytes = hex::decode(signature_hex)
            .map_err(|e| EvidenceError::InvalidSignature(e.to_string()))?;
        if sig_bytes.len() != 64 {
            return Err(EvidenceError::InvalidSignature("Signature must be 64 bytes".to_string()));
        }

        let mut sig_arr = [0u8; 64];
        sig_arr.copy_from_slice(&sig_bytes);
        let signature = Signature::from_bytes(&sig_arr);

        verifying_key
            .verify(message, &signature)
            .map(|_| true)
            .map_err(|_| EvidenceError::VerificationFailed)
    }
}

pub mod hex {
    pub fn encode<T: AsRef<[u8]>>(data: T) -> String {
        data.as_ref()
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect()
    }

    pub fn decode(hex_str: &str) -> Result<Vec<u8>, String> {
        let clean = hex_str.trim();
        if clean.len() % 2 != 0 {
            return Err("Hex string has odd length".to_string());
        }
        (0..clean.len())
            .step_by(2)
            .map(|i| {
                u8::from_str_radix(&clean[i..i + 2], 16)
                    .map_err(|e| format!("Invalid hex digit: {}", e))
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha512_and_ed25519_sign_verify() {
        let seed = [42u8; 32];
        let (sk, vk) = CryptoSigner::keypair_from_seed(&seed);
        let vk_hex = hex::encode(vk.to_bytes());

        let payload = b"TRUSTPRINT Silicon Attestation Record";
        let digest_hex = CryptoSigner::sha512_digest(payload);
        assert_eq!(digest_hex.len(), 128); // 64 bytes * 2 hex chars

        let sig_hex = CryptoSigner::sign(&sk, digest_hex.as_bytes());
        assert_eq!(sig_hex.len(), 128); // 64 bytes * 2 hex chars

        // Verification should succeed
        let valid = CryptoSigner::verify(&vk_hex, digest_hex.as_bytes(), &sig_hex).unwrap();
        assert!(valid);

        // Verification must fail if digest is tampered
        let tampered_digest = format!("00{}", &digest_hex[2..]);
        let tampered_result = CryptoSigner::verify(&vk_hex, tampered_digest.as_bytes(), &sig_hex);
        assert!(tampered_result.is_err());
    }
}
