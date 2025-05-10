//! Ed25519 signature validation for preprocessed documents.
//! This module provides functionality to validate Ed25519 signatures on preprocessed documents
//! that contain canonical proof and document data.

use bs58;
use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Serialize, Deserialize)]
pub struct VerifyInput {
    pub canonical_document: String,
    pub canonical_proof: String,
    pub public_key: String,
    pub proof: String,
}

pub fn ed25519_verify(verify: VerifyInput) -> Result<(), String> {
    let mut message_bytes = Vec::with_capacity(64); // 32 bytes for each hash

    // 1. Hash the canonical proof and document
    let mut hasher = Sha256::new();
    hasher.update(verify.canonical_proof.as_bytes());
    message_bytes.extend_from_slice(&hasher.finalize());

    let mut hasher = Sha256::new();
    hasher.update(verify.canonical_document.as_bytes());
    message_bytes.extend_from_slice(&hasher.finalize());

    let public_key_bytes = bs58::decode(verify.public_key)
        .into_vec()
        .map_err(|_| "Failed to decode base58 public key")?;

    let signature_bytes = bs58::decode(verify.proof)
        .into_vec()
        .map_err(|_| "Failed to decode base58 signature")?;

    // Signature verification takes approx. 200 seconds
    // TODO: See if there is a way of doing less type castings to make this
    // run faster
    VerifyingKey::from_bytes(&public_key_bytes[2..].try_into().unwrap())
        .unwrap()
        .verify(
            &message_bytes,
            &Signature::from_bytes(&signature_bytes.try_into().unwrap()),
        )
        .map_err(|_| "Signature verification failed")?;

    Ok(())
}
