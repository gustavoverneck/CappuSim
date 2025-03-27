import os
import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns
from matplotlib.animation import FuncAnimation, PillowWriter
import numpy as np

# Folder containing CSV files
folder_path = "output"

def get_data(i):
    filename = folder_path + f"/data{i*50}.csv"
    print(filename)
    data = {}
    x = []
    y = []
    z = []
    rho = []
    ux = []
    uy = []
    uz = []
    v = []
    with open(filename, 'r') as f:
        for line in f.readlines()[1:]:
            line = line.split(",")
            x.append(float(line[0]))
            y.append(float(line[1]))
            z.append(float(line[2]))
            rho.append(float(line[3]))
            ux.append(float(line[4]))
            uy.append(float(line[5]))
            uz.append(float(line[6]))
            v.append(float(line[7]))
    data['x'] = np.array(x)
    data['y'] = np.array(y)
    data['z'] = np.array(z)
    data['rho'] = np.array(rho)
    data['ux'] = np.array(ux)
    data['uy'] = np.array(uy)
    data['uz'] = np.array(uz)
    data['v'] = np.array(v)
    return data

def update(i):
    global fig, ax
    data = get_data(i)
    ax.clear()
    heatmap_data = data['rho'].reshape((int(np.sqrt(len(data['rho']))), -1))
    sns.heatmap(heatmap_data, cmap="inferno", ax=ax, cbar=False)
    ax.set_title(f"2D Heatmap - Frame {i}")
    ax.set_xlabel("x")
    ax.set_ylabel("y")
    
# Main script
if __name__ == "__main__":
    fig, ax = plt.subplots(figsize=(10, 8))
    # Create animation
    ani = FuncAnimation(fig, update, frames=199,)
    ani.save(f"output/heatmap.gif",  writer = 'ffmpeg', fps = 24)
    plt.close(fig)  # Close the figure to avoid resource warnings
    # output_file = os.path.join(folder_path, "heatmap.gif")
    # try:
    #     ani.save(output_file, writer=PillowWriter(fps=10))
    #     print(f"Saved combined animation to {output_file}")
    # except (RuntimeError, Exception) as e:
    #     print(f"Error saving animation: {e}")
