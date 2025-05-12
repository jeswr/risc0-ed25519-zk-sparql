import json
import matplotlib.pyplot as plt
import numpy as np
from matplotlib.colors import rgb2hex
import colorsys

def darken_color(color, factor=0.7):
    # Extract RGB components from RGBA tuple
    r, g, b = color[:3]
    # Convert RGB to HSV
    hsv = colorsys.rgb_to_hsv(r, g, b)
    # Darken the value
    hsv = (hsv[0], hsv[1], hsv[2] * factor)
    # Convert back to RGB
    return colorsys.hsv_to_rgb(*hsv)

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
    # Plot 4 credentials first (so it's in the background)
    config = "ed25519-preprocessed"
    config_label = "4 Credentials"
    query_label = "SELECT ALL" if query == "query" else query.replace("-", " ").title()
    prove_times = [data[env][config][query]['prove'] for env in environments]
    bar = ax1.bar(x + (i * width), prove_times, width,
            label=f'{config_label} {query_label}', alpha=0.7)
    color = bar[0].get_facecolor()
    
    # Plot 1 credential on top with darker color
    config = "minimal"
    config_label = "1 Credential"
    prove_times = [data[env][config][query]['prove'] for env in environments]
    ax1.bar(x + (i * width), prove_times, width,
            label=f'{config_label} {query_label}', alpha=0.9,
            color=darken_color(color))

ax1.set_ylabel('Time (seconds)')
ax1.set_title('Proof Generation Times')
ax1.set_xticks(x + width)
ax1.set_xticklabels([env.replace("-", " ").replace("ci", "CI").title() for env in environments])
ax1.legend(bbox_to_anchor=(1.05, 1), loc='upper left')
ax1.grid(True, alpha=0.3)

# Plot verify times
for i, query in enumerate(queries):
    # Plot 4 credentials first (so it's in the background)
    config = "ed25519-preprocessed"
    config_label = "4 Credentials"
    query_label = "SELECT ALL" if query == "query" else query.replace("-", " ").title()
    verify_times = [data[env][config][query]['verify'] for env in environments]
    bar = ax2.bar(x + (i * width), verify_times, width,
            label=f'{config_label} {query_label}', alpha=0.7)
    color = bar[0].get_facecolor()
    
    # Plot 1 credential on top with darker color
    config = "minimal"
    config_label = "1 Credential"
    verify_times = [data[env][config][query]['verify'] for env in environments]
    ax2.bar(x + (i * width), verify_times, width,
            label=f'{config_label} {query_label}', alpha=0.9,
            color=darken_color(color))

ax2.set_ylabel('Time (seconds)')
ax2.set_title('Verification Times')
ax2.set_xticks(x + width)
ax2.set_xticklabels([env.replace("-", " ").replace("ci", "CI").title() for env in environments])
ax2.legend(bbox_to_anchor=(1.05, 1), loc='upper left')
ax2.grid(True, alpha=0.3)

# Adjust layout and save
plt.tight_layout()
plt.savefig('bench-results/benchmark_results.svg', format='svg', bbox_inches='tight')
plt.close() 