#!/bin/bash

# Check if @mermaid-js/mermaid-cli is installed
if ! command -v mmdc &> /dev/null; then
    echo "Installing @mermaid-js/mermaid-cli..."
    npm install -g @mermaid-js/mermaid-cli
fi

# Generate SVG from Mermaid diagram
echo "Generating SVG from Mermaid diagram..."
mmdc -i architecture.mmd -o architecture.svg -b white

# Generate PDF from Mermaid diagram
echo "Generating PDF from Mermaid diagram..."
mmdc -i architecture.mmd -o architecture.pdf -b white

echo "Generating PNG from Mermaid diagram..."
mmdc -i architecture.mmd -o architecture.png -b white -w 3000

# Check if generation was successful
if [ $? -eq 0 ]; then
    echo "Diagrams generated successfully at architecture.svg and architecture.pdf"
else
    echo "Error generating diagrams"
    exit 1
fi 