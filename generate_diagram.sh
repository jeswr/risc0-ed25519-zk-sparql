#!/bin/bash

# Check if @mermaid-js/mermaid-cli is installed
if ! command -v mmdc &> /dev/null; then
    echo "Installing @mermaid-js/mermaid-cli..."
    npm install -g @mermaid-js/mermaid-cli
fi

# Generate SVG from Mermaid diagram
echo "Generating SVG from Mermaid diagram..."
mmdc -i architecture.mmd -o architecture.svg -b white

# Check if generation was successful
if [ $? -eq 0 ]; then
    echo "SVG generated successfully at architecture.svg"
else
    echo "Error generating SVG"
    exit 1
fi 