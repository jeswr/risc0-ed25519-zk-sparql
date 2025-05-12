#!/bin/bash

# Iterate through the directories ./data/generated/ed25519-preprocessed/ and ./minimal
# Iterate through the queries in ./data/queries/ in addition to ./query.sparql

for dir in ./minimal/ ./data/generated/ed25519-preprocessed/
do
    for query in ./data/queries/can-drive.rq ./data/queries/employment-status.rq ./query.sparql
    do
        /usr/bin/time bash -c "echo 'PROOF GENERATION (DATA: $dir) (QUERY: $query)' && ./target/release/host -p $dir --mode prove --output-file ./sparql_result.json"
        /usr/bin/time bash -c "echo 'VERIFICATION (DATA: $dir) (QUERY: $query)' && ./target/release/host --mode verify --output-file ./sparql_result.json"
    done
done
