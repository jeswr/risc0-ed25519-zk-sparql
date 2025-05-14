import json
import matplotlib.pyplot as plt
import numpy as np

# Read the JSON data
with open('query_sizes.json', 'r') as f:
    data = json.load(f)

# Extract data for both scenarios
queries = list(data['4-credentials']['query_sizes_with_proof'].keys())
scenarios = ['4-credentials', '1-credentials']

# Prepare data for plotting
sizes_with_proof = {
    '4-credentials': [data['4-credentials']['query_sizes_with_proof'][q]['size_bytes'] for q in queries],
    '1-credentials': [data['1-credentials']['query_sizes_with_proof'][q]['size_bytes'] for q in queries]
}

sizes_no_proof = {
    '4-credentials': [data['4-credentials']['query_sizes_no_proof'][q]['size_bytes'] for q in queries],
    '1-credentials': [data['1-credentials']['query_sizes_no_proof'][q]['size_bytes'] for q in queries]
}

# Calculate ratios
ratios_with_proof = [s4/s1 for s4, s1 in zip(sizes_with_proof['4-credentials'], sizes_with_proof['1-credentials'])]
ratios_no_proof = [s4/s1 for s4, s1 in zip(sizes_no_proof['4-credentials'], sizes_no_proof['1-credentials'])]

# Convert to MB for better readability
sizes_with_proof_mb = {k: [size / (1024 * 1024) for size in v] for k, v in sizes_with_proof.items()}
sizes_no_proof_mb = {k: [size / (1024 * 1024) for size in v] for k, v in sizes_no_proof.items()}

# Set up the plot
plt.figure(figsize=(12, 8))
x = np.arange(len(queries))
width = 0.35  # Wider bars since we're not using as many

# Colors for with proof and without proof
proof_color = '#2ecc71'  # Green
no_proof_color = '#e74c3c'  # Red

# Create the bars
plt.bar(x - width/2, sizes_with_proof_mb['4-credentials'], width, 
        label='4-credentials With Proof', color=proof_color, alpha=0.4)
plt.bar(x - width/2, sizes_with_proof_mb['1-credentials'], width,
        label='1-credentials With Proof', color=proof_color, alpha=0.8)

plt.bar(x + width/2, sizes_no_proof_mb['4-credentials'], width,
        label='4-credentials Without Proof', color=no_proof_color, alpha=0.4)
plt.bar(x + width/2, sizes_no_proof_mb['1-credentials'], width,
        label='1-credentials Without Proof', color=no_proof_color, alpha=0.8)

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
plt.legend(bbox_to_anchor=(1.05, 1), loc='upper left')

# Add value labels on top of bars
def add_value_labels(ax, rects):
    for rect in rects:
        height = rect.get_height()
        ax.text(rect.get_x() + rect.get_width()/2., height,
                f'{height:.4f} MB',
                ha='center', va='bottom', rotation=0)

add_value_labels(plt.gca(), plt.gca().patches)

# Add ratio labels above the "Without Proof" bars
for i, (ratio_with, ratio_no) in enumerate(zip(ratios_with_proof, ratios_no_proof)):
    # Get the maximum height of the bars for this query
    max_height = max(
        sizes_with_proof_mb['4-credentials'][2],
        sizes_with_proof_mb['1-credentials'][2],
        sizes_no_proof_mb['4-credentials'][2],
        sizes_no_proof_mb['1-credentials'][2]
    )
    
    # Calculate x position relative to the bar position
    x_pos = (x[i] + width/2) * 1.05  # Move 5% right relative to the bar position
    y_pos = max_height * .75  # Move to the height of the maximum bar
    
    plt.text(x_pos, y_pos,
            f'Ratios (4:1):\nWith: {ratio_with:.2f}x\nW/o: {ratio_no:.2f}x',
            ha='center', va='bottom',
            bbox=dict(facecolor='white', alpha=0.8, edgecolor='none'))

# Adjust layout and save
plt.tight_layout()
plt.savefig('query_sizes_comparison.svg', format='svg', bbox_inches='tight')
plt.close() 