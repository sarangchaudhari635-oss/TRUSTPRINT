//! TRUSTPRINT Cryptographic Evidence & Attestation Engine
//!
//! Generates RFC 8785 canonical JSON, computes SHA-512 cryptographic digests,
//! and signs attestation records using Ed25519 into .tpe certificate bundles.

pub mod bundle;
pub mod canonical;
pub mod signer;

pub use bundle::*;
pub use canonical::*;
pub use signer::*;
