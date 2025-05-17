use core::query::parse_query;
use risc0_zkvm::guest::env;

fn main() {
    let query_string: String = env::read();
    let _query_object = parse_query(&query_string);

    // write public output to the journal
    env::commit(&true);
}
