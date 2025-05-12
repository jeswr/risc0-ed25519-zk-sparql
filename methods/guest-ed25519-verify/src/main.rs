use risc0_zkvm::guest::env;
use core::verify::{VerifyInput, ed25519_verify};

fn main() {
    let input: VerifyInput = env::read();
    let canonical_document = input.canonical_document.clone();
    let public_key = input.public_key.clone();
    ed25519_verify(input).expect("Signature verification failed");
    env::commit((&canonical_document, &public_key));
}
