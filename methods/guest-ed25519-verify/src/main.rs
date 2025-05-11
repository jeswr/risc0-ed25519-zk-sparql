use risc0_zkvm::guest::env;
use core::{Output, verify::{VerifyInput, ed25519_verify}, query::query, load::load_dataset};

fn main() {
    let verify_inputs: VerifyInput = env::read();
    ed25519_verify(input).expect("Signature verification failed");
    env::commit(&verify_inputs.canonical_document);
}
