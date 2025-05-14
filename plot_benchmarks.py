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

rename_environments = {
    'macos-ci': 'macOS CI',
    'ubuntu-ci': 'Ubuntu CI',
    'macos-local': 'macOS Local'
}

# Set up the x-axis positions
x = np.arange(len(environments))
width = 0.2  # Increased width of the bars

# Plot prove times
for i, query in enumerate(queries):
    # Plot 4 credentials first (so it's in the background)
    config = "ed25519-preprocessed"
    config_label = "4 Credentials"
    query_label = "SELECT ALL" if query == "query" else "Can Drive" if query == "can-drive" else "Employment"
    prove_times_4 = [data[env][config][query]['prove'] for env in environments]
    bar = ax1.bar(x + (i * width * 1.2), prove_times_4, width,
            label=f'{config_label} {query_label}', alpha=0.7)
    color = bar[0].get_facecolor()
    
    # Add text labels for 4 credentials
    for j, v in enumerate(prove_times_4):
        ax1.text(x[j] + (i * width * 1.2), v/2, query_label,
                ha='center', va='center', color='white', rotation=90,
                fontweight='bold', fontsize=16)
    
    # Plot 1 credential on top with darker color
    config = "minimal"
    config_label = "1 Credential"
    prove_times_1 = [data[env][config][query]['prove'] for env in environments]
    ax1.bar(x + (i * width * 1.2), prove_times_1, width,
            label=f'{config_label} {query_label}', alpha=0.9,
            color=darken_color(color))
    
    # Add percentage labels above 4-credential bars
    for j, (time_1, time_4) in enumerate(zip(prove_times_1, prove_times_4)):
        percentage = (time_1 / time_4) * 100
        ax1.text(x[j] + (i * width * 1.2), time_4 + 50, f'{percentage:.1f}%',
                ha='center', va='bottom', fontsize=10)

# Add explanation text for prove times next to legend
ax1.text(0.82, 0.98, 'Percentages show what portion of the 4-credential time\nthe 1-credential query takes',
         transform=ax1.transAxes, fontsize=10, verticalalignment='top', horizontalalignment='right',
         bbox=dict(boxstyle='round', facecolor='white', alpha=0.8))

ax1.set_ylabel('Time (seconds)')
ax1.set_title('Proof Generation Times')
ax1.set_xticks(x + width * 1.2)  # Adjusted tick positions
ax1.set_xticklabels([rename_environments[env] for env in environments])
ax1.legend(loc='upper right')
ax1.grid(True, alpha=0.3)

# Plot verify times
for i, query in enumerate(queries):
    # Plot 4 credentials first (so it's in the background)
    config = "ed25519-preprocessed"
    config_label = "4 Credentials"
    query_label = "SELECT ALL" if query == "query" else "Can Drive" if query == "can-drive" else "Employment"
    verify_times_4 = [data[env][config][query]['verify'] for env in environments]
    bar = ax2.bar(x + (i * width * 1.2), verify_times_4, width,
            label=f'{config_label} {query_label}', alpha=0.7)
    color = bar[0].get_facecolor()
    
    # Add text labels for 4 credentials
    for j, v in enumerate(verify_times_4):
        ax2.text(x[j] + (i * width * 1.2), v/2, query_label,
                ha='center', va='center', color='white', rotation=90,
                fontweight='bold', fontsize=16)
    
    # Plot 1 credential on top with darker color
    config = "minimal"
    config_label = "1 Credential"
    verify_times_1 = [data[env][config][query]['verify'] for env in environments]
    ax2.bar(x + (i * width * 1.2), verify_times_1, width,
            label=f'{config_label} {query_label}', alpha=0.9,
            color=darken_color(color))
    
    # Add percentage labels above 4-credential bars
    for j, (time_1, time_4) in enumerate(zip(verify_times_1, verify_times_4)):
        percentage = (time_1 / time_4) * 100
        ax2.text(x[j] + (i * width * 1.2), time_4 + 0.1, f'{percentage:.1f}%',
                ha='center', va='bottom', fontsize=10)


ax2.set_ylabel('Time (seconds)')
ax2.set_title('Verification Times')
ax2.set_xticks(x + width * 1.2)  # Adjusted tick positions
ax2.set_xticklabels([rename_environments[env] for env in environments])
ax2.grid(True, alpha=0.3)

# Adjust layout and save
plt.tight_layout()
plt.savefig('bench-results/benchmark_results.svg', format='svg', bbox_inches='tight')
plt.close() 