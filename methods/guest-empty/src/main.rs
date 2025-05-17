use risc0_zkvm::guest::env;

fn main() {
    // write public output to the journal
    env::commit(&true);
}
