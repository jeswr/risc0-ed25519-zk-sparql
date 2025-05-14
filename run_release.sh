#!/bin/bash

cargo build --release

for dir in ./minimal/ ./data/generated/ed25519-preprocessed/
do
    for query in ./queries/query.rq ./queries/can-drive.rq ./queries/employment-status.rq
    do
        # Create a unique filename based on directory and query
        query_name=$(basename "$query" .rq)
        data_name=$(basename "$dir" .json)
        result_file="./results/${data_name}_${query_name}_result.json"

        # Create results directory if it doesn't exist
        mkdir -p ./results
       
        echo "
        PROOF PROFIlE GENERATION (DATA: $dir) (QUERY: $query)"
        RUST_BACKTRACE=full RISC0_DEV_MODE=1 RISC0_INFO=1 RISC0_PPROF_OUT=./profile_${data_name}_${query_name}.pb cargo run -- --mode prove --output-file "$result_file-extra.json" --path $dir --query-file $query --profile

        
        # Create results directory if it doesn't exist
        # mkdir -p ./results
        
        # echo "
        # PROOF GENERATION (DATA: $dir) (QUERY: $query)"
        # time (./target/release/host --mode prove --output-file "$result_file" --path $dir --query-file $query)
        # echo "
        # VERIFICATION (DATA: $dir) (QUERY: $query)"
        # time (./target/release/host --mode verify --output-file "$result_file")
    done
done
