use core::verify::VerifyInput;
use core::load::load_dataset;
use core::query::{query, parse_query};
use core::output::Output;
use risc0_zkvm::guest::env;

fn main() {
    let verify_inputs: Vec<VerifyInput> = env::read();
    let (pub_keys, dataset) = load_dataset(verify_inputs);

    // TODO: Make this query configurable
    let query_string: String = env::read();
    let query_object = parse_query(&query_string);

    // write public output to the journal
    env::commit(&Output {
        result_string: query(dataset, query_object),
        query_string,
        pub_keys,
    });
}
