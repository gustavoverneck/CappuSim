import pandas as pd
import matplotlib.pyplot as plt

# Load CSV efficiently
df = pd.read_csv('benchmarks/benchmark_results_1752848558.csv')

# Preview data
print("Columns:", df.columns.tolist())

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
    
    # Add performance annotations to each point
    for _, row in model_data.iterrows():
        plt.annotate(f'{row["MLUps"]:.1f}', 
                    (row['GridSize'], row['MLUps']),
                    textcoords="offset points", 
                    xytext=(0,10), 
                    ha='center', 
                    fontsize=9, 
                    color=colors[i],
                    fontweight='bold')

plt.xlabel('Grid Size (Number of Cells)', fontsize=12)
plt.ylabel('Performance (MLUps)', fontsize=12)
plt.title('LBM Performance: MLUps vs Grid Size by Model', fontsize=14, fontweight='bold')
plt.grid(True, alpha=0.3)
plt.legend(fontsize=10)
plt.xscale('log')  # Log scale for better visualization of grid sizes
#plt.yscale('log')  # Log scale for better visualization of performance
plt.tight_layout()
plt.show()

# Print summary statistics
print("\nPerformance Summary by Model:")
for model in models:
    model_data = df[df['Model'] == model]
    max_mlups = model_data['MLUps'].max()
    avg_mlups = model_data['MLUps'].mean()
    print(f"{model}: Max {max_mlups:.2f} MLUps, Avg {avg_mlups:.2f} MLUps")