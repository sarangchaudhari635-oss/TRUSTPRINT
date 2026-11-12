//! Canonical .tpe (TrustPrint Evidence) Certificate Bundle

use crate::canonical::CanonicalJson;
use crate::signer::{hex, CryptoSigner, EvidenceError};
use ed25519_dalek::SigningKey;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TargetHardwareEvidence {
    pub board_type: String,
    pub silicon_stepping: String,
    pub public_serial: String,
    pub device_serializer: String,
    pub uuid: String,
    pub wafer_lot_tracking_id: String,
    pub boot0_hash: String,
    pub boot1_hash: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TierClaim {
    pub tier: String,
    pub claim_name: String,
    pub description: String,
    pub artifact_digest_sha512: String,
    pub verified_passed: bool,
    pub metadata: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EvidencePayload {
    pub schema_version: String,
    pub bundle_id: String,
    pub created_at_utc: String,
    pub target_hardware: TargetHardwareEvidence,
    pub claims: Vec<TierClaim>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EvidenceBundle {
    pub payload: EvidencePayload,
    pub canonical_payload_sha512: String,
    pub attestation_signature: String,
    pub signer_public_key_hex: String,
    pub verification_instructions: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationReport {
    pub bundle_id: String,
    pub is_valid: bool,
    pub claims_count: usize,
    pub signer_public_key: String,
    pub sha512_digest: String,
    pub details: Vec<String>,
}

pub struct EvidenceEngine;

impl EvidenceEngine {
    /// Builds and cryptographically signs a complete .tpe evidence bundle.
    pub fn create_bundle(
        hardware: TargetHardwareEvidence,
        claims: Vec<TierClaim>,
        signing_key: &SigningKey,
    ) -> Result<EvidenceBundle, EvidenceError> {
        let verifying_key = signing_key.verifying_key();
        let pubkey_hex = hex::encode(verifying_key.to_bytes());

        let payload = EvidencePayload {
            schema_version: "trustprint.v1.evidence".to_string(),
            bundle_id: format!("TPE-{}", &pubkey_hex[..8]),
            created_at_utc: chrono::Utc::now().to_rfc3339(),
            target_hardware: hardware,
            claims,
        };

        // 1. Serialize payload to canonical RFC 8785 JSON
        let payload_value = serde_json::to_value(&payload)?;
        let canonical_json = CanonicalJson::canonicalize(&payload_value);

        // 2. Compute SHA-512 digest over canonical JSON bytes
        let digest_sha512 = CryptoSigner::sha512_digest(canonical_json.as_bytes());

        // 3. Sign the SHA-512 digest with Ed25519
        let signature = CryptoSigner::sign(signing_key, digest_sha512.as_bytes());

        Ok(EvidenceBundle {
            payload,
            canonical_payload_sha512: digest_sha512,
            attestation_signature: signature,
            signer_public_key_hex: pubkey_hex,
            verification_instructions: "Verify by re-canonicalizing payload (RFC 8785), computing SHA-512, and verifying Ed25519 signature.".to_string(),
        })
    }

    /// Verifies the cryptographic integrity and digital signature of an EvidenceBundle.
    pub fn verify_bundle(bundle: &EvidenceBundle) -> Result<VerificationReport, EvidenceError> {
        let mut details = Vec::new();

        // 1. Re-canonicalize payload
        let payload_value = serde_json::to_value(&bundle.payload)?;
        let canonical_json = CanonicalJson::canonicalize(&payload_value);

        // 2. Re-compute SHA-512
        let computed_digest = CryptoSigner::sha512_digest(canonical_json.as_bytes());
        if computed_digest != bundle.canonical_payload_sha512 {
            return Err(EvidenceError::HashMismatch {
                expected: bundle.canonical_payload_sha512.clone(),
                actual: computed_digest,
            });
        }
        details.push("Payload SHA-512 canonical hash match confirmed.".to_string());

        // 3. Verify Ed25519 signature
        CryptoSigner::verify(
            &bundle.signer_public_key_hex,
            bundle.canonical_payload_sha512.as_bytes(),
            &bundle.attestation_signature,
        )?;
        details.push("Ed25519 cryptographic digital signature verified.".to_string());

        // 4. Summarize claims
        for claim in &bundle.payload.claims {
            let status_str = if claim.verified_passed { "PASS" } else { "FAIL" };
            details.push(format!("[{}] {} -> {}", claim.tier, claim.claim_name, status_str));
        }

        Ok(VerificationReport {
            bundle_id: bundle.payload.bundle_id.clone(),
            is_valid: true,
            claims_count: bundle.payload.claims.len(),
            signer_public_key: bundle.signer_public_key_hex.clone(),
            sha512_digest: computed_digest,
            details,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_and_verify_bundle() {
        let seed = [99u8; 32];
        let (sk, _) = CryptoSigner::keypair_from_seed(&seed);

        let hw = TargetHardwareEvidence {
            board_type: "Dabao".to_string(),
            silicon_stepping: "A0".to_string(),
            public_serial: "NNBT1T".to_string(),
            device_serializer: "4332af0a-3f263177-494a5f1c-68bbebf6".to_string(),
            uuid: "0b4d2b1c-198122bd-2cb59a01-3aebf575".to_string(),
            wafer_lot_tracking_id: "47521066M06T".to_string(),
            boot0_hash: "a0680a73106600c4".to_string(),
            boot1_hash: "a12eb17f37c88e83".to_string(),
        };

        let claims = vec![
            TierClaim {
                tier: "DESIGN_RTL".to_string(),
                claim_name: "bio.sv SFR Interface Verification".to_string(),
                description: "Extracted 34 SFR registers from silicon RTL".to_string(),
                artifact_digest_sha512: "11223344".to_string(),
                verified_passed: true,
                metadata: serde_json::json!({ "sfr_count": 34, "ports": 70 }),
            },
            TierClaim {
                tier: "SIMULATION_VERILATOR".to_string(),
                claim_name: "Multi-Core BIO 700MHz Waveform Conformance".to_string(),
                description: "Evaluated 131 cycles, 108.528ns mean period, 0.0ps jitter".to_string(),
                artifact_digest_sha512: "55667788".to_string(),
                verified_passed: true,
                metadata: serde_json::json!({ "cycles": 131, "period_ns": 108.528 }),
            },
        ];

        let bundle = EvidenceEngine::create_bundle(hw, claims, &sk).unwrap();
        let report = EvidenceEngine::verify_bundle(&bundle).unwrap();

        assert!(report.is_valid);
        assert_eq!(report.claims_count, 2);
    }
}
