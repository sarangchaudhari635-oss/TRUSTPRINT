//! Ed25519 Cryptographic Signer and Verifier Engine
//!
//! Provides deterministic key generation, SHA-512 hashing,
//! detached signature creation, and cryptographic verification.

use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use sha2::{Digest, Sha512};
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EvidenceError {
    #[error("Signature verification failed: invalid signature")]
    VerificationFailed,

    #[error("SHA-512 payload hash mismatch: expected {expected}, actual {actual}")]
    HashMismatch { expected: String, actual: String },

    #[error("Invalid public key encoding: {0}")]
    InvalidPublicKey(String),

    #[error("Invalid private key encoding: {0}")]
    InvalidPrivateKey(String),

    #[error("Invalid signature encoding: {0}")]
    InvalidSignature(String),

    #[error("Signer public key '{0}' is not present in trusted key whitelist")]
    UntrustedSigner(String),

    #[error("OS entropy source error: {0}")]
    EntropySourceFailed(String),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

#[cfg(windows)]
#[link(name = "advapi32")]
extern "system" {
    #[link_name = "SystemFunction036"]
    fn RtlGenRandom(buffer: *mut u8, length: u32) -> u8;
}

#[cfg(windows)]
pub fn os_random_bytes(buffer: &mut [u8]) -> Result<(), EvidenceError> {
    let ret = unsafe { RtlGenRandom(buffer.as_mut_ptr(), buffer.len() as u32) };
    if ret == 0 {
        return Err(EvidenceError::EntropySourceFailed(
            "RtlGenRandom returned 0 failure".to_string(),
        ));
    }
    Ok(())
}

#[cfg(not(windows))]
pub fn os_random_bytes(buffer: &mut [u8]) -> Result<(), EvidenceError> {
    use std::io::Read;
    let mut file = std::fs::File::open("/dev/urandom")
        .map_err(|e| EvidenceError::EntropySourceFailed(e.to_string()))?;
    file.read_exact(buffer)
        .map_err(|e| EvidenceError::EntropySourceFailed(e.to_string()))?;
    Ok(())
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

    /// Generates a new cryptographically secure random Ed25519 keypair using OS kernel entropy.
    pub fn generate_random_keypair() -> Result<(SigningKey, VerifyingKey), EvidenceError> {
        let mut seed = [0u8; 32];
        os_random_bytes(&mut seed)?;
        Ok(Self::keypair_from_seed(&seed))
    }

    /// Saves an Ed25519 signing key to disk as a hex-encoded file.
    pub fn save_signing_key(signing_key: &SigningKey, path: &Path) -> Result<(), EvidenceError> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let hex_str = hex::encode(signing_key.to_bytes());
        std::fs::write(path, hex_str)?;
        Ok(())
    }

    /// Loads an Ed25519 signing key from a hex-encoded file.
    pub fn load_signing_key(path: &Path) -> Result<SigningKey, EvidenceError> {
        let content = std::fs::read_to_string(path)?;
        let bytes = hex::decode(content.trim())
            .map_err(|e| EvidenceError::InvalidPrivateKey(e.to_string()))?;
        if bytes.len() != 32 {
            return Err(EvidenceError::InvalidPrivateKey(format!(
                "Key file must contain 32 bytes (64 hex characters), got {} bytes",
                bytes.len()
            )));
        }
        let mut key_bytes = [0u8; 32];
        key_bytes.copy_from_slice(&bytes);
        Ok(SigningKey::from_bytes(&key_bytes))
    }

    /// Returns the standard path for storing the development signing key:
    /// $TRUSTPRINT_KEY_PATH or ~/.trustprint/dev_signing_key.sec
    pub fn default_key_path() -> PathBuf {
        if let Ok(path) = std::env::var("TRUSTPRINT_KEY_PATH") {
            return PathBuf::from(path);
        }
        if let Ok(home) = std::env::var("USERPROFILE").or_else(|_| std::env::var("HOME")) {
            return PathBuf::from(home)
                .join(".trustprint")
                .join("dev_signing_key.sec");
        }
        PathBuf::from(".trustprint_dev_signing_key.sec")
    }

    /// Loads an existing signing key or generates a fresh one from OS CSPRNG.
    pub fn load_or_generate_key(
        custom_path: Option<&Path>,
    ) -> Result<(SigningKey, VerifyingKey, PathBuf, bool), EvidenceError> {
        let target_path = match custom_path {
            Some(p) => p.to_path_buf(),
            None => Self::default_key_path(),
        };

        if target_path.exists() {
            let sk = Self::load_signing_key(&target_path)?;
            let vk = sk.verifying_key();
            Ok((sk, vk, target_path, false))
        } else {
            let (sk, vk) = Self::generate_random_keypair()?;
            Self::save_signing_key(&sk, &target_path)?;
            Ok((sk, vk, target_path, true))
        }
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
            return Err(EvidenceError::InvalidPublicKey(
                "Key must be 32 bytes".to_string(),
            ));
        }

        let mut key_arr = [0u8; 32];
        key_arr.copy_from_slice(&vk_bytes);
        let verifying_key = VerifyingKey::from_bytes(&key_arr)
            .map_err(|e| EvidenceError::InvalidPublicKey(e.to_string()))?;

        let sig_bytes = hex::decode(signature_hex)
            .map_err(|e| EvidenceError::InvalidSignature(e.to_string()))?;
        if sig_bytes.len() != 64 {
            return Err(EvidenceError::InvalidSignature(
                "Signature must be 64 bytes".to_string(),
            ));
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
        data.as_ref().iter().map(|b| format!("{:02x}", b)).collect()
    }

    pub fn decode(hex_str: &str) -> Result<Vec<u8>, String> {
        let clean = hex_str.trim();
        if !clean.len().is_multiple_of(2) {
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

    #[test]
    fn test_os_random_keypair_generation_and_persistence() {
        let (sk, vk) = CryptoSigner::generate_random_keypair().unwrap();
        assert_eq!(sk.to_bytes().len(), 32);
        assert_eq!(vk.to_bytes().len(), 32);

        // Sign and verify with freshly generated key
        let msg = b"test message for random keypair";
        let sig = CryptoSigner::sign(&sk, msg);
        let vk_hex = hex::encode(vk.to_bytes());
        assert!(CryptoSigner::verify(&vk_hex, msg, &sig).unwrap());

        // Test save and load
        let temp_dir = std::env::temp_dir();
        let key_path = temp_dir.join("trustprint_test_key.sec");
        CryptoSigner::save_signing_key(&sk, &key_path).unwrap();

        let loaded_sk = CryptoSigner::load_signing_key(&key_path).unwrap();
        assert_eq!(loaded_sk.to_bytes(), sk.to_bytes());

        let _ = std::fs::remove_file(key_path);
    }
}
