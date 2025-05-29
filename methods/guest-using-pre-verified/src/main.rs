use core::verify::VerifyInput;
use core::load::load_dataset;
use core::query::query;
use core::output::HashedOutput;
use risc0_zkvm::guest::env;
use spargebra::Query;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    t.hash(&mut hasher);
    hasher.finish()
}

fn main() {
    let verify_inputs: Vec<VerifyInput> = env::read();
    let (pub_keys, dataset) = load_dataset(verify_inputs);

    let query_object: Query = env::read();
    let hash = calculate_hash(&query_object);

    // write public output to the journal
    env::commit(&HashedOutput {
        result_string: query(dataset, query_object),
        query_hash: hash,
        pub_keys,
    });
}
