use risc0_zkvm::guest::env;
use vc_utils::{VerifyInput, ed25519_verify};
use oxrdf::{Dataset, GraphName, Quad};
use oxttl::NQuadsParser;

fn main() {
    let verify_inputs: Vec<VerifyInput> = env::read();
    let mut pub_keys: Vec<String> = Vec::new();
    let mut dataset: Dataset = Dataset::new();

    for input in verify_inputs {
        for triple in NQuadsParser::new().for_reader(input.canonical_document.clone().as_bytes()) {
            // TODO: Handle bnode differentiation
            let t1 = triple.unwrap();
            let subject = t1.subject;
            let predicate = t1.predicate;
            let object = t1.object;
            let quad = Quad::new(subject, predicate, object, GraphName::DefaultGraph);
            dataset.insert(&quad);
        }

        pub_keys.push(input.public_key.clone());
        ed25519_verify(input).expect("Signature verification failed");
    }

    // write public output to the journal
    env::commit(&pub_keys);
}
