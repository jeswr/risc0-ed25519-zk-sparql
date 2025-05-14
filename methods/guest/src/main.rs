use risc0_zkvm::guest::env;
use spargebra::Query;
use core::{Output, VerifyInput, query::query};
use bs58;
use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use oxrdf::{BlankNode, Dataset, GraphName, Quad, Subject, Term};
use oxttl::NQuadsParser;

fn main() {
    let verify_inputs: Vec<VerifyInput> = env::read();
    let (pub_keys, dataset) = load_dataset(verify_inputs);

    // TODO: Make this query configurable
    let query_string: String = env::read();
    let query_object = Query::parse(&query_string, None).expect("Failed to parse query");

    // write public output to the journal
    env::commit(&Output {
        result_string: query(dataset, query_object),
        query_string,
        pub_keys,
    });
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
    VerifyingKey::from_bytes(&public_key_bytes[2..].try_into().expect("Failed to convert public key bytes to array"))
        .expect("Failed to convert public key bytes to array")
        .verify(
            &message_bytes,
            &Signature::from_bytes(&signature_bytes.try_into().expect("Failed to convert signature bytes to array")),
        )
        .map_err(|_| "Signature verification failed")?;

    Ok(())
}

pub fn load_dataset(verify_inputs: Vec<VerifyInput>) -> (Vec<String>, Dataset) {
    let mut pub_keys: Vec<String> = Vec::new();
    let mut dataset: Dataset = Dataset::new();

    let mut i: u128 = 0;

    for input in verify_inputs {
        let mut map = HashMap::<String, u128>::new();

        // This takes approx 95 seconds for the VCs that we have
        for triple in NQuadsParser::new().for_reader(input.canonical_document.clone().as_bytes()) {
            let t1 = triple.expect("Failed to parse triple");

            // If graph_name is not default_graph, error
            if !t1.graph_name.is_default_graph() {
              panic!("Graph name is not default_graph");
            }

            let mut subject = t1.subject;
            let predicate = t1.predicate;
            let mut object = t1.object;

            if let Subject::BlankNode(blank_node) = subject {
              let term_string = blank_node.into_string();
              let id: u128;
              if !map.contains_key(&term_string) {
                id = i;
                map.insert(term_string, id);
                i += 1;
                if i > 1000000000000000000 {
                  panic!("i is too large");
                }
              } else {
                id = map[&term_string];
              }
              subject = Subject::BlankNode(BlankNode::new_from_unique_id(id));
            }

            if let Term::BlankNode(blank_node) = object {
              let term_string = blank_node.into_string();
              let id: u128;
              if !map.contains_key(&term_string) {
                id = i;
                map.insert(term_string, id);
                i += 1;
                if i > 1000000000000000000 {
                  panic!("i is too large");
                }
              } else {
                id = map[&term_string];
              }
              object = Term::BlankNode(BlankNode::new_from_unique_id(id));
            }

            let quad = Quad::new(subject, predicate, object, GraphName::DefaultGraph);
            dataset.insert(&quad);
        }

        pub_keys.push(input.public_key.clone());
        ed25519_verify(input).expect("Signature verification failed");
    }

    return (pub_keys, dataset)
}
