use core::query::parse_query;
use risc0_zkvm::guest::env;
use spargebra::Query;

fn main() {
    let _query_object: Query = env::read();

    // write public output to the journal
    env::commit(&true);
}
