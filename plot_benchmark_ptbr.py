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

# Define markers and colors
precision_markers = {'FP32': 'o', 'FP16S': 's', 'FP16C': 'x'}
model_colors = plt.cm.tab10(range(10))  # Up to 10 models

# Get unique models and precision modes
models = df['Model'].unique()
precision_modes = df['Precision'].unique()

# Create Model+Precision combination plot (Performance)
plt.figure(figsize=(14, 10))

# For custom legends
from matplotlib.lines import Line2D

plot_handles = []
model_handles = []
precision_handles = []
model_seen = set()
precision_seen = set()

for i, model in enumerate(models):
    for j, precision in enumerate(precision_modes):
        subset = df[(df['Model'] == model) & (df['Precision'] == precision)].sort_values('GridSize')
        if not subset.empty:
            marker = precision_markers.get(precision, 'o')
            color = model_colors[i % len(model_colors)]
            linestyle = ['-', ':', '--'][j % 3]
            label = f"{model} ({precision})"
            h, = plt.plot(subset['GridSize'], subset['MLUps'], 
                         marker=marker, linewidth=2, markersize=8, 
                         label=label, color=color, linestyle=linestyle)
            plot_handles.append(h)
            # For model legend (color)
            if model not in model_seen:
                model_handles.append(Line2D([0], [0], color=color, lw=3, label=model))
                model_seen.add(model)
            # For precision legend (marker)
            if precision not in precision_seen:
                precision_handles.append(Line2D([0], [0], color='gray', marker=marker, linestyle='', markersize=10, label=precision))
                precision_seen.add(precision)
            # Adiciona anotações de performance (apenas para o maior grid)
            max_grid = subset['GridSize'].max()
            for _, row in subset[subset['GridSize'] == max_grid].iterrows():
                plt.annotate(f'{row["MLUps"]:.1f}', 
                            (row['GridSize'] * 1.05, row['MLUps']),
                            textcoords="data", 
                            ha='left', va='center',
                            fontsize=9, 
                            color=color,
                            fontweight='bold')

plt.xlabel('Tamanho do Grid (número de células, escala log)', fontsize=12)
plt.ylabel('Performance (MLUps)', fontsize=12)
plt.title(f'Desempenho LBM: MLUps vs Tamanho do Grid por Modelo e Precisão\n{gpu_name}', fontsize=14, fontweight='bold')
plt.grid(True, alpha=0.3)
# Legendas customizadas empilhadas (modelo/cor em cima, precisão/marcador embaixo)
first_legend = plt.legend(handles=model_handles, title='Modelo (Cor)', fontsize=10, title_fontsize=11, loc='upper left', bbox_to_anchor=(0, 1))
plt.gca().add_artist(first_legend)
plt.legend(handles=precision_handles, title='Precisão (Marcador)', fontsize=10, title_fontsize=11, loc='upper left', bbox_to_anchor=(0, 0.78))
plt.xscale('log')
plt.tight_layout()
plt.savefig(f'benchmarks/performance_model_precision_{gpu_name_safe}.png')
plt.show()

# --- Precision comparison by model (using bar chart, using highest MLUps for each model+precision) ---
# For each model+precision, select the row with the highest MLUps
idx = df.groupby(['Model', 'Precision'])['MLUps'].idxmax()
comparison_df = df.loc[idx].copy()

# Create bar chart comparing precision modes for each model
plt.figure(figsize=(14, 8))

models = sorted(comparison_df['Model'].unique())
precision_modes = sorted(comparison_df['Precision'].unique())

# Setup x positions for grouped bars
x = np.arange(len(models))
width = 0.8 / len(precision_modes)

# Create grouped bars
for i, precision in enumerate(precision_modes):
    values = []
    for model in models:
        subset = comparison_df[(comparison_df['Model'] == model) & (comparison_df['Precision'] == precision)]
        values.append(subset['MLUps'].values[0] if not subset.empty else 0)
    
    offset = i * width - width * (len(precision_modes) - 1) / 2
    bars = plt.bar(x + offset, values, width, label=precision)
    
    # Add values on top of bars
    for j, v in enumerate(values):
        if v > 0:
            plt.text(x[j] + offset, v + 20, f'{v:.0f}', 
                     ha='center', va='bottom', fontsize=9, fontweight='bold')

plt.xlabel('Modelo', fontsize=12)
plt.ylabel('Performance (MLUps)', fontsize=12)
plt.title(f'Desempenho LBM por Modelo e Precisão\n{gpu_name} - Maior MLUps', 
          fontsize=14, fontweight='bold')
plt.xticks(x, models)
plt.legend(title='Precisão')
plt.grid(axis='y', alpha=0.3)
plt.tight_layout()
plt.savefig(f'benchmarks/precision_comparison_{gpu_name_safe}_highest.png')
plt.show()

# --- Improved Cell Size Comparison (Bar Chart, in Bytes) ---
plt.figure(figsize=(8, 5))

selected_precisions = ['FP32', 'FP16S', 'FP16C']
cell_sizes = []

for precision in selected_precisions:
    subset = df[df['Precision'] == precision]
    if not subset.empty:
        cell_size = subset['CellMemoryBytes'].iloc[0]
        cell_sizes.append(cell_size)
    else:
        cell_sizes.append(0)

x = np.arange(len(selected_precisions))
colors = ['#4F81BD', '#F79646', '#9BBB59']  # Custom, visually appealing colors
bars = plt.bar(x, cell_sizes, color=colors, edgecolor='black', linewidth=1.2)

# Add value labels with shadow for better visibility
for i, v in enumerate(cell_sizes):
    plt.text(
        x[i], v + max(cell_sizes) * 0.03, f'{v:,} B',
        ha='center', va='bottom', fontsize=12, fontweight='bold',
        color='#333', bbox=dict(facecolor='white', alpha=0.7, edgecolor='none', boxstyle='round,pad=0.2')
    )

plt.xlabel('Precisão', fontsize=13, fontweight='bold')
plt.ylabel('Tamanho da célula (Bytes)', fontsize=13, fontweight='bold')
plt.title('Tamanho da célula por precisão', fontsize=15, fontweight='bold', pad=15)
plt.xticks(x, selected_precisions, fontsize=12)
plt.yticks(fontsize=11)
plt.grid(axis='y', alpha=0.18, linestyle='--', zorder=0)
plt.ylim(0, max(cell_sizes) * 1.18)
plt.tight_layout(pad=1.2)
plt.savefig(f'benchmarks/cell_size_comparison_{gpu_name_safe}_bytes.png', dpi=120)
plt.show()


# --- Bandwidth Comparison Plot (Bar Chart) ---
plt.figure(figsize=(14, 8))

# Use the highest Bandwidth_GBs for each model+precision
idx = df.groupby(['Model', 'Precision'])['Bandwidth_GBs'].idxmax()
bandwidth_comparison_df = df.loc[idx].copy()

# Setup for grouped bar chart
models = sorted(bandwidth_comparison_df['Model'].unique())
precision_modes = sorted(bandwidth_comparison_df['Precision'].unique())
x = np.arange(len(models))
width = 0.8 / len(precision_modes)

for i, precision in enumerate(precision_modes):
    values = []
    effs = []
    for model in models:
        subset = bandwidth_comparison_df[(bandwidth_comparison_df['Model'] == model) & (bandwidth_comparison_df['Precision'] == precision)]
        if not subset.empty:
            values.append(subset['Bandwidth_GBs'].values[0])
            effs.append(subset['Efficiency'].values[0])
        else:
            values.append(0)
            effs.append(0)
    offset = i * width - width * (len(precision_modes) - 1) / 2
    bars = plt.bar(x + offset, values, width, label=precision)
    # Annotate efficiency on top of bars
    for j, (v, e) in enumerate(zip(values, effs)):
        if v > 0:
            plt.text(x[j] + offset, v + max(values) * 0.03, f'{e:.2f}x',
                     ha='center', va='bottom', fontsize=9, fontweight='bold')

plt.axhline(max_bandwidth_GBs, color='k', linestyle='--', linewidth=1.5, label='Máximo Teórico')
plt.xlabel('Modelo', fontsize=12)
plt.ylabel('Largura de banda (GB/s)', fontsize=12)
plt.title(f'LBM: Largura de banda real vs teórica (GB/s)\n{gpu_name}', fontsize=14, fontweight='bold')
plt.xticks(x, models)
plt.legend(framealpha=1)
plt.grid(axis='y', alpha=0.3)
plt.tight_layout()
plt.savefig(f'benchmarks/bandwidth_comparison_{gpu_name_safe}.png')
plt.show()
