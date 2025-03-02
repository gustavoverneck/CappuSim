# graphics.py

# External Imports
import numpy as np
import matplotlib.pyplot as plt

class Graphics:
    def __init__(self):
        pass

    def plot2DImage(self, data, title="2D Plot", xlabel="X", ylabel="Y", label="Data"):
        """
        Plot 2D data using PyVista.
        
        Parameters:
        - data: 2D numpy array to plot.
        - title: Title of the plot.
        - xlabel: Label for the x-axis.
        - ylabel: Label for the y-axis.
        """
        plt.imshow(data, cmap='viridis', origin='lower', label=label)
        plt.colorbar(label='Intensity')
        plt.title(title)
        plt.xlabel(xlabel)
        plt.ylabel(ylabel)
        plt.show()
    
    def plot2DSurface(self, data, title="2D Surface Plot", xlabel="X", ylabel="Y", zlabel="Z", levels=10, label="Data"):
        """
        Plot 2D surface data using PyVista.
        
        Parameters:
        - data: 2D numpy array to plot.
        - title: Title of the plot.
        - xlabel: Label for the x-axis.
        - ylabel: Label for the y-axis.
        """
        fig = plt.figure()
        ax = fig.add_subplot(111, projection='3d')
        X, Y = np.meshgrid(np.arange(data.shape[0]), np.arange(data.shape[1]))
        ax.plot_surface(X, Y, data.T, cmap='viridis')
        ax.set_title(title)
        ax.set_xlabel(xlabel)
        ax.set_ylabel(ylabel)
        plt.show()
    
    def plot2DContour(self, data, title="Contour Plot", xlabel="X", ylabel="Y", levels=10, label="Data"):
        """
        Plot contour data using PyVista.
        
        Parameters:
        - data: 2D numpy array to plot.
        - title: Title of the plot.
        - xlabel: Label for the x-axis.
        - ylabel: Label for the y-axis.
        """
        plt.contourf(data, cmap='viridis', levels=levels, label=label)
        plt.colorbar(label='Intensity')
        plt.title(title)
        plt.xlabel(xlabel)
        plt.ylabel(ylabel)
        plt.show()




if __name__ == "__main__":
    # Example usage
    from random import random
    rho = np.array([[random() for _ in range(100)] for _ in range(100)])
    graphics = Graphics()
    graphics.plot2DSurface(rho, title="Random 2D Data", xlabel="X-axis", ylabel="Y-axis")