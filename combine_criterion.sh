#!/bin/bash

# Script to combine all criterion estimates.json files into a single JSON file
# Usage: ./combine_criterion.sh [output_file]

OUTPUT_FILE=${1:-"combined_criterion.json"}
CRITERION_DIR="target/criterion"

# Check if jq is installed
if ! command -v jq &> /dev/null; then
    echo "Error: jq is required but not installed. Please install jq."
    exit 1
fi

# Initialize the result JSON object
echo "{" > "$OUTPUT_FILE"

# Find all estimates.json files in the 'new' folder and process them
first=true
for file in $(find "$CRITERION_DIR" -path "*/new/estimates.json"); do
    # Extract benchmark name from path
    # Pattern: target/criterion/BENCHMARK_NAME/new/estimates.json
    benchmark=$(echo "$file" | sed -E "s|$CRITERION_DIR/([^/]+)/.*|\1|")
    
    if [ "$first" = true ]; then
        first=false
    else
        # Add a comma for JSON validity between entries
        echo "," >> "$OUTPUT_FILE"
    fi
    
    # Add the benchmark data to the result file
    echo "  \"$benchmark\": $(cat "$file")" | tr -d '\n' >> "$OUTPUT_FILE"
done

# Close the JSON object
echo -e "\n}" >> "$OUTPUT_FILE"

echo "Combined criterion data written to $OUTPUT_FILE" 