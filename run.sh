#!/bin/bash
echo "Starting proof generation..."
time (RISC0_DEV_MODE=1 cargo run --quiet -- --path ./data/generated/ed25519-preprocessed/ --mode prove --output-file ./sparql_result.json)
echo "Proof generation complete."

echo "Starting verification..."
time (RISC0_DEV_MODE=1 cargo run --quiet -- --mode verify --output-file ./sparql_result.json)
echo "Verification complete."
