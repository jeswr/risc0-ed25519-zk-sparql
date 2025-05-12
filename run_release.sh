#!/bin/bash

cargo build --release

for dir in ./minimal/ ./data/generated/ed25519-preprocessed/
do
    for query in ./query.sparql ./data/queries/can-drive.rq ./data/queries/employment-status.rq
    do
        echo "
        PROOF GENERATION (DATA: $dir) (QUERY: $query)"
        time (./target/release/host -p $dir --mode prove --output-file ./sparql_result.json)
        echo "
        VERIFICATION (DATA: $dir) (QUERY: $query)"
        time (./target/release/host --mode verify --output-file ./sparql_result.json)
    done
done
