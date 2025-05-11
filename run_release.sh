RISC0_DEV_MODE=0 cargo run --release -- -p ./data/generated/ed25519-preprocessed/ --mode prove --output-file ./sparql_result.json
RISC0_DEV_MODE=0 cargo run --release -- --mode verify --output-file ./sparql_result.json
