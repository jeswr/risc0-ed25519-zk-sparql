import json
import matplotlib.pyplot as plt
import numpy as np

# Read the JSON data
with open('query_sizes.json', 'r') as f:
    data = json.load(f)

# Extract data
queries = list(data['query_sizes_with_proof'].keys())
sizes_with_proof = [data['query_sizes_with_proof'][q]['size_bytes'] for q in queries]
sizes_no_proof = [data['query_sizes_no_proof'][q]['size_bytes'] for q in queries]

# Convert to MB for better readability
sizes_with_proof_mb = [size / (1024 * 1024) for size in sizes_with_proof]
sizes_no_proof_mb = [size / (1024 * 1024) for size in sizes_no_proof]

# Set up the plot
plt.figure(figsize=(10, 6))
x = np.arange(len(queries))
width = 0.35

# Create the bars
plt.bar(x - width/2, sizes_with_proof_mb, width, label='With Proof', color='#2ecc71')
plt.bar(x + width/2, sizes_no_proof_mb, width, label='Without Proof', color='#e74c3c')

query_names = {
    'query': 'SELECT ALL',
    'can_drive': 'Can Drive',
    'employment_status': 'Employment'
}

# Customize the plot
plt.yscale('log')  # Set logarithmic scale
plt.ylabel('Size (MB)')
plt.title('Query Result Sizes Comparison (Logarithmic Scale)')
plt.xticks(x, [query_names[q] for q in queries])
plt.legend()


# Add value labels on top of bars
def add_value_labels(ax, rects):
    for rect in rects:
        height = rect.get_height()
        ax.text(rect.get_x() + rect.get_width()/2., height,
                f'{height:.4f} MB',
                ha='center', va='bottom', rotation=0)

add_value_labels(plt.gca(), plt.gca().patches)

# Adjust layout and save
plt.tight_layout()
plt.savefig('query_sizes_comparison.svg', format='svg', bbox_inches='tight')
plt.close() 