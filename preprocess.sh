#!/bin/bash
echo "Starting proof generation..."
time (RISC0_DEV_MODE=1 cargo run --quiet -- --path ./data/generated/ed25519-preprocessed/ --mode preprocess)
echo "Proof generation complete."
