import pandas as pd
import matplotlib.pyplot as plt
import matplotlib.ticker as mticker
import numpy as np
import sys

# Get filename from command line arguments
if len(sys.argv) > 1:
    benchmark_file = sys.argv[1]
    print(f"Using provided file: {benchmark_file}")
else:
    benchmark_file = "benchmark_results_1754441146.csv"
    print(f"Using default file: {benchmark_file}")

# Get max bandwidth from command line (optional)
if len(sys.argv) > 2:
    max_bandwidth_GBs = float(sys.argv[2])
    print(f"Using provided max bandwidth: {max_bandwidth_GBs} GB/s")
else:
    max_bandwidth_GBs = 256
    print(f"Using default max bandwidth: {max_bandwidth_GBs} GB/s")


# ----------------------------------------------------------------
# Load CSV efficiently
df = pd.read_csv(f'benchmarks/{benchmark_file}')

# Preview data
print("Columns:", df.columns.tolist())

# Get GPU name for title
gpu_name = df['DeviceName'].iloc[0] if not df.empty else "Unknown GPU"

# Clean GPU name for filename (remove spaces and special characters)
gpu_name_safe = gpu_name.replace(' ', '_').replace('/', '_')

# --- New Calculation: Bandwidth in GB/s ---
df['Bandwidth_GBs'] = (
    df['MLUps'] * 1_000_000 * df['CellMemoryBytes'] / 1_000_000_000
)

df['Efficiency'] = df['Bandwidth_GBs'] / max_bandwidth_GBs

# Create MLUps vs Grid Size plot with lines for each model
plt.figure(figsize=(12, 8))

# Get unique models and assign colors
models = df['Model'].unique()
colors = plt.cm.tab10(range(len(models)))

for i, model in enumerate(models):
    model_data = df[df['Model'] == model].sort_values('GridSize')
    plt.plot(model_data['GridSize'], model_data['MLUps'], 
             marker='o', linewidth=2, markersize=8, 
             label=model, color=colors[i])
    
    #Add performance annotations to each point
    for _, row in model_data.iterrows():
        plt.annotate(f'{row["MLUps"]:.1f}', 
                    (row['GridSize'], row['MLUps']),
                    textcoords="offset points", 
                    xytext=(0,10), 
                    ha='center', 
                    fontsize=9, 
                    color=colors[i],
                    fontweight='bold')

plt.xlabel('Grid Size (Number of Cells, log scale)', fontsize=12)
plt.ylabel('Performance (MLUps)', fontsize=12)
plt.title(f'LBM Performance: MLUps vs Grid Size by Model\n{gpu_name}', fontsize=14, fontweight='bold')
plt.grid(True, alpha=0.3)
plt.legend(fontsize=10)
plt.xscale('log')  # Log scale for better visualization of grid sizes
#plt.yscale('log')  # Log scale for better visualization of performance



# --------------------------------------------------------------------------

plt.tight_layout()
plt.savefig(f'benchmarks/performance_vs_gridsize_{gpu_name_safe}.png')  # Save performance plot
plt.show()

# --- New Plot: Memory Usage vs Grid Size by Model ---
plt.figure(figsize=(12, 8))

for i, model in enumerate(models):
    model_data = df[df['Model'] == model].sort_values('GridSize')
    plt.plot(model_data['GridSize'], model_data['MemoryUsageMB'],
             marker='s', linewidth=2, markersize=8,
             label=model, color=colors[i])
    # Annotate memory usage at each point
    # for _, row in model_data.iterrows():
    #     plt.annotate(f'{row["MemoryUsageMB"]:.0f}MB',
    #                  (row['GridSize'], row['MemoryUsageMB']),
    #                  textcoords="offset points",
    #                  xytext=(0,10),
    #                  ha='center',
    #                  fontsize=9,
    #                  color=colors[i],
    #                  fontweight='bold')

plt.xlabel('Grid Size (Number of Cells, log scale)', fontsize=12)
plt.ylabel('Memory Usage (MB)', fontsize=12)
plt.title(f'LBM Memory Usage: MB vs Grid Size by Model\n{gpu_name}', fontsize=14, fontweight='bold')
plt.grid(True, alpha=0.3)
plt.legend(fontsize=10)
plt.xscale('log')  # Keep x-axis log for grid size
plt.gca().yaxis.set_major_formatter(mticker.FormatStrFormatter('%.0f'))  # Integer ticks on y-axis
plt.tight_layout()
plt.savefig(f'benchmarks/memory_vs_gridsize_{gpu_name_safe}.png')
plt.show()

# --- New Plot: Real vs Theoretical Bandwidth (GB/s) as Bar Plot ---
plt.figure(figsize=(14, 8))

all_grids = sorted(df['GridSize'].unique())
bar_width = 0.8 / len(models)  # Make bars fit nicely
index = np.arange(len(all_grids))

# Plot real bandwidth bars for each model
for i, model in enumerate(models):
    model_data = df[df['Model'] == model].sort_values('GridSize')
    grid_indices = [all_grids.index(gs) for gs in model_data['GridSize']]
    x = index[grid_indices] + (i - (len(models)-1)/2) * bar_width
    bars = plt.bar(x, model_data['Bandwidth_GBs'], bar_width, label=model, color=colors[i], alpha=0.85)
    # Annotate efficiency
    for rect, row in zip(bars, model_data.itertuples()):
        eff = row.Bandwidth_GBs / max_bandwidth_GBs
        plt.text(rect.get_x() + rect.get_width()/2, rect.get_height() + 2, f'{eff:.2f}x',
                ha='center', va='bottom', fontsize=8, color=colors[i])

# Plot a single horizontal line for the theoretical bandwidth
plt.axhline(max_bandwidth_GBs, color='black', linestyle='--', linewidth=2, label='Theoretical Max')

plt.xlabel('Grid Size (Number of Cells)', fontsize=12)
plt.ylabel('Bandwidth (GB/s)', fontsize=12)
plt.title(f'LBM Real vs Theoretical Bandwidth (GB/s)\n{gpu_name}', fontsize=14, fontweight='bold')
plt.grid(True, axis='y', alpha=0.3)
plt.xticks(index, [str(gs) for gs in all_grids], rotation=30)
plt.legend(fontsize=10)
plt.tight_layout()
plt.savefig(f'benchmarks/real_vs_theoretical_bandwidth_{gpu_name_safe}.png')
plt.show()
