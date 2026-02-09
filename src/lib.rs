//! CRT-ZKVM: Hardened Circuit Registry Template Zero-Knowledge Virtual Machine
//!
//! A production-grade, security-hardened proving system with:
//! - 128-bit security guarantees (Goldilocks field)
//! - Cryptographically bound template calls
//! - Merkle tree registry commitments
//! - Fiat-Shamir aggregated proofs
//!
//! SECURITY WARNING: This implementation has been hardened for production use.
//! DO NOT use test profiles (Test, Bit80) in production deployments.

pub mod blockchain;
pub mod cli;
pub mod compiler;
pub mod crt;
pub mod db;
pub mod demo;
pub mod economics;
pub mod field;
pub mod isa;
pub mod merkle;
pub mod prover;
pub mod security;
pub mod stark;
pub mod token;
pub mod wallet;

// Re-export main types
// Production field (Goldilocks) - USE THIS FOR PRODUCTION
pub use field::secure::{FieldElement, FieldElementExt4};

// Test field (Baby Bear) - ONLY FOR TESTING
pub use field::{BabyBear, BabyBearExt4};

// Security configuration
pub use security::{assert_production_safe, ProverSecurityConfig, SecurityError, SecurityProfile};

// Merkle tree for registry
pub use merkle::{MerkleInclusionProof, MerkleTree, RegistryMerkleTracker, TemplateLeaf};

/// Current crate version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Security hardening version
pub const SECURITY_VERSION: u32 = 1;

/// Assert that the system is configured for production use
///
/// Call this at application startup to ensure secure configuration.
pub fn assert_production_ready() -> Result<(), security::SecurityError> {
    security::assert_production_safe(security::SecurityProfile::Bit128)
}
