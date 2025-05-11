use risc0_zkvm::guest::env;
use oxrdf::{Dataset, GraphName, Quad};
use oxttl::NQuadsParser;
use spareval::{QueryEvaluator, QueryResults};
use serde::{Deserialize, Serialize};
use spargebra::Query;
use core::{Output, verify::{VerifyInput, ed25519_verify}, query::query, load::load_dataset};
use sparesults::QueryResultsSerializer;

fn main() {
    let verify_inputs: Vec<VerifyInput> = env::read();
    let (pub_keys, dataset) = load_dataset(verify_inputs);

    // TODO: Make this query configurable
    let query_string: String = env::read();
    let queryObject = Query::parse(&query_string, None).unwrap();

    // write public output to the journal
    env::commit(&Output {
        result_string: query(dataset, queryObject),
        query_string,
        pub_keys,
    });
}
