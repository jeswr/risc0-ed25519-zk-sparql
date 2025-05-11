RISC0_DEV_MODE=1 cargo run -- -p ./data/generated/ed25519-preprocessed/ --mode prove --output-file ./sparql_result.json
RISC0_DEV_MODE=1 cargo run -- --mode verify --output-file ./sparql_result.json
