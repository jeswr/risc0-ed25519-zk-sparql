import json
import matplotlib.pyplot as plt
import numpy as np

# Read the benchmark results
with open('bench-results/res.json', 'r') as f:
    data = json.load(f)

# Set up the plot style
plt.style.use('bmh')  # Using a built-in style instead of seaborn
plt.rcParams['figure.figsize'] = (15, 10)

# Create subplots for prove and verify times
fig, (ax1, ax2) = plt.subplots(2, 1, figsize=(15, 12))

# Prepare data for plotting
environments = ['macos-ci', 'ubuntu-ci', 'macos-local']
configurations = ['minimal', 'ed25519-preprocessed']
queries = ['query', 'can-drive', 'employment-status']

# Set up the x-axis positions
x = np.arange(len(environments))
width = 0.15  # Width of the bars

# Plot prove times
for i, query in enumerate(queries):
    for j, config in enumerate(configurations):
        prove_times = [data[env][config][query]['prove'] for env in environments]
        ax1.bar(x + (i * width) + (j * width/2), prove_times, width/2,
                label=f'{config}-{query}')

ax1.set_ylabel('Time (seconds)')
ax1.set_title('Proof Generation Times')
ax1.set_xticks(x + width)
ax1.set_xticklabels(environments)
ax1.legend(bbox_to_anchor=(1.05, 1), loc='upper left')
ax1.grid(True, alpha=0.3)

# Plot verify times
for i, query in enumerate(queries):
    for j, config in enumerate(configurations):
        verify_times = [data[env][config][query]['verify'] for env in environments]
        ax2.bar(x + (i * width) + (j * width/2), verify_times, width/2,
                label=f'{config}-{query}')

ax2.set_ylabel('Time (seconds)')
ax2.set_title('Verification Times')
ax2.set_xticks(x + width)
ax2.set_xticklabels(environments)
ax2.legend(bbox_to_anchor=(1.05, 1), loc='upper left')
ax2.grid(True, alpha=0.3)

# Adjust layout and save
plt.tight_layout()
plt.savefig('bench-results/benchmark_results.svg', format='svg', bbox_inches='tight')
plt.close() 