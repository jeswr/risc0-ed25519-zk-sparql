use risc0_zkvm::guest::env;
use oxrdf::{Dataset, GraphName, Quad};
use oxttl::NQuadsParser;
use spareval::{QueryEvaluator, QueryResults};
use serde::{Deserialize, Serialize};
use spargebra::Query;
use core::{Output, verify::{VerifyInput, ed25519_verify}};

fn main() {
    let verify_inputs: Vec<VerifyInput> = env::read();
    let mut pub_keys: Vec<String> = Vec::new();
    let mut dataset: Dataset = Dataset::new();

    for input in verify_inputs {
        // This takes approx 95 seconds for the VCs that we have
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

    // TODO: Make this query configurable
    let query_string = "SELECT ?s ?p ?o WHERE { ?s ?p ?o }";
    let query = Query::parse(query_string, None).unwrap();
    let results = QueryEvaluator::new().execute(dataset, &query);
    let solution: QueryResults = results.unwrap();

    let mut result_string = String::new();

    // TODO: Serialize the solution

    // write public output to the journal
    env::commit(&Output {
        result_string,
        pub_keys,
    });
}
