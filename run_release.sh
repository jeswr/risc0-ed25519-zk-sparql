#!/bin/bash

# Function to run a command with warmup and multiple trials
run_with_timing() {
    local cmd="$1"
    local desc="$2"
    local trials=100
    local warmup_runs=3
    
    echo "Running warmup for: $desc"
    # Perform warmup runs
    for ((i=1; i<=warmup_runs; i++)); do
        echo "Warmup run $i/$warmup_runs"
        eval "$cmd" > /dev/null 2>&1
    done
    
    echo "Running $trials trials for: $desc"
    # Perform timed trials
    local total_time=0
    for ((i=1; i<=trials; i++)); do
        echo -ne "Trial $i/$trials\r"
        local start_time=$(date +%s.%N)
        eval "$cmd" > /dev/null 2>&1
        local end_time=$(date +%s.%N)
        local elapsed=$(echo "$end_time - $start_time" | bc)
        total_time=$(echo "$total_time + $elapsed" | bc)
    done
    echo ""  # New line after progress indicator
    
    # Calculate and print average
    local avg_time=$(echo "scale=6; $total_time / $trials" | bc)
    echo "Average time for $desc: $avg_time seconds (over $trials trials)"
}

# Install Oxigraph if not available in current environment
if ! command -v oxigraph &> /dev/null; then
    echo "Oxigraph could not be found in current environment"
    echo "Installing Oxigraph"
    brew install oxigraph
fi

for dir in ./minimal/ ./data/generated/ed25519-preprocessed/
do
    # Preprocess the data for Oxigraph by collecting, concatenating and saving the 
    # verifyData.canonicalDocument field in all files within the directory.
    echo "Processing $dir for Oxigraph preprocessing..."
    output_file="${dir%/}_canonicalDocument.nq"
    
    # Clear output file if it exists
    > "$output_file"
    
    # Process each JSON file in the directory
    for file in "$dir"/*.json; do
        if [ -f "$file" ]; then
            # Extract canonicalDocument field and append to output file
            jq -r '.verifyData.canonicalDocument // empty' "$file" >> "$output_file"
        fi
    done

    rm -rf .oxigraph

    echo "Loading data into Oxigraph"
    run_with_timing "oxigraph load --location .oxigraph --file \"$output_file\" --format nq" "Oxigraph load for $dir"

    for query in ./queries/query.rq ./queries/can-drive.rq ./queries/employment-status.rq
    do
        echo "Running $query on $dir"
        run_with_timing "oxigraph query --location .oxigraph --query-file \"$query\" --results-file oxires.json --results-format application/sparql-results+json" "Oxigraph query $query on $dir"
    done

    rm "$output_file"
    rm oxires.json
    rm -rf .oxigraph
done

# Only build the release binary if it doesn't exist
if [ ! -f ./target/release/host ]; then
    cargo build --release
fi

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
       
        # Generate profiling info
        echo "
        PROOF PROFIlE GENERATION (DATA: $dir) (QUERY: $query)"
        RUST_BACKTRACE=full RISC0_DEV_MODE=1 RISC0_INFO=1 RISC0_PPROF_OUT=./profile_${data_name}_${query_name}.pb cargo run -- --mode prove --output-file "$result_file-extra.json" --path $dir --query-file $query --profile

        
        # Create results directory if it doesn't exist
        mkdir -p ./results
        
        echo "
        PROOF GENERATION (DATA: $dir) (QUERY: $query)"
        time (./target/release/host --mode prove --output-file "$result_file" --path $dir --query-file $query)
        echo "
        VERIFICATION (DATA: $dir) (QUERY: $query)"
        time (./target/release/host --mode verify --output-file "$result_file")
    done
done
