use risc0_zkvm::guest::env;
use core::VerifyInput;
use bs58;
use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use sha2::{Digest, Sha256};

fn main() {
    let input: VerifyInput = env::read();
    let canonical_document = input.canonical_document.clone();
    let public_key = input.public_key.clone();
    ed25519_verify(input).expect("Signature verification failed");
    env::commit(&(&canonical_document, &public_key));
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
