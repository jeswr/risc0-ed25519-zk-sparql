use risc0_zkvm::guest::env;
use spargebra::Query;
use core::{Output, verify::VerifyInput, query::query, load::load_dataset};

fn main() {
    let verify_inputs: Vec<VerifyInput> = env::read();
    let (pub_keys, dataset) = load_dataset(verify_inputs);

    // TODO: Make this query configurable
    let query_string: String = env::read();
    let query_object = Query::parse(&query_string, None).unwrap();

    // write public output to the journal
    env::commit(&Output {
        result_string: query(dataset, query_object),
        query_string,
        pub_keys,
    });
}
