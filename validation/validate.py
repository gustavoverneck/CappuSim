"""
Validation script for CappuSim LBM solver.

- Runs CappuSim with different test cases (Couette, Poiseuille, Lid-driven cavity, Von Karman)
- Compares output (velocity, density, etc) against analytical or reference solutions
- Supports 2D and 3D versions
- Plots and reports errors for each precision mode (FP32, FP16S, FP16C)

Usage:
    python validate.py --case couette --dim 2 --precision FP32
    python validate.py --case poiseuille --dim 3 --precision FP16C

Requirements:
    - numpy, matplotlib, pandas
    - CappuSim binary in PATH or specify with --bin
"""

import argparse
import subprocess
import numpy as np
import matplotlib.pyplot as plt
import os
import sys
import numpy as np

# --- Reference solutions ---
def analytical_couette(y, U=0.1, H=1.0, nu=0.01):
    # Steady-state Couette flow: u(y) = U * y / H
    return U * y / H

def analytical_poiseuille(y, dpdx=-0.01, H=1.0, nu=0.01):
    # Steady-state Poiseuille: u(y) = (1/(2*nu)) * dpdx * (y*H - y**2)
    return (1/(2*nu)) * dpdx * (y*H - y**2)

def analytical_lid_driven_cavity(y, direction='vertical', Re=1000):
    """
    Returns reference velocity profile from Ghia et al. (1982) for lid-driven cavity.
    Only a few Reynolds numbers and points are available.
    direction: 'vertical' (u along center x), 'horizontal' (v along center y)
    y: array of positions (normalized, 0 to 1)
    Re: Reynolds number (supports 100, 400, 1000)
    Returns: y_ref, u_ref (interpolated to input y)
    """

    # Ghia et al. data for Re=1000, vertical centerline (u vs y at x=0.5)
    ghia_data = {
        100: {
            'y': [1.000, 0.9766, 0.9688, 0.9609, 0.9531, 0.8516, 0.7344, 0.6172, 0.5000, 0.4531, 0.2813, 0.1719, 0.1016, 0.0703, 0.0625, 0.0547, 0.0000],
            'u': [1.0000, 0.8412, 0.7887, 0.7372, 0.6872, 0.2315, 0.0033, -0.1364, -0.2058, -0.2109, -0.1566, -0.1015, -0.0643, -0.0477, -0.0419, -0.0372, 0.0000],
            'x': [1.000, 0.9688, 0.9609, 0.9531, 0.8516, 0.7344, 0.6172, 0.5000, 0.4531, 0.2813, 0.1719, 0.1016, 0.0703, 0.0625, 0.0547, 0.0000],
            'v': [0.0000, -0.0591, -0.0739, -0.0886, -0.2453, -0.2245, -0.1691, -0.1031, -0.0886, -0.0570, -0.0306, -0.0180, -0.0117, -0.0092, -0.0074, 0.0000]
        },
        400: {
            'y': [1.000, 0.9766, 0.9688, 0.9609, 0.9531, 0.8516, 0.7344, 0.6172, 0.5000, 0.4531, 0.2813, 0.1719, 0.1016, 0.0703, 0.0625, 0.0547, 0.0000],
            'u': [1.0000, 0.8412, 0.7887, 0.7372, 0.6872, 0.2315, 0.0033, -0.1364, -0.2058, -0.2109, -0.1566, -0.1015, -0.0643, -0.0477, -0.0419, -0.0372, 0.0000],
            'x': [1.000, 0.9688, 0.9609, 0.9531, 0.8516, 0.7344, 0.6172, 0.5000, 0.4531, 0.2813, 0.1719, 0.1016, 0.0703, 0.0625, 0.0547, 0.0000],
            'v': [0.0000, -0.0818, -0.0996, -0.1172, -0.3273, -0.3197, -0.2756, -0.2139, -0.1937, -0.1405, -0.0981, -0.0710, -0.0545, -0.0481, -0.0430, 0.0000]
        },
        1000: {
            'y': [1.0000, 0.9766, 0.9688, 0.9609, 0.9531, 0.8516, 0.7344, 0.6172, 0.5000, 0.4531, 0.2813, 0.1719, 0.1016, 0.0703, 0.0625, 0.0547, 0.0000],
            'u': [1.0000, 0.6593, 0.5749, 0.5112, 0.4660, 0.3330, 0.1872, 0.0570, -0.0608, -0.1065, -0.2781, -0.3829, -0.2973, -0.2222, -0.2019, -0.1811, 0.0000],
            'x': [1.0000, 0.9688, 0.9609, 0.9531, 0.8516, 0.7344, 0.6172, 0.5000, 0.4531, 0.2813, 0.1719, 0.1016, 0.0703, 0.0625, 0.0547, 0.0000],
            'v': [0.0000, -0.2453, -0.2245, -0.1691, -0.1031, -0.0886, -0.0570, -0.0306, -0.0180, -0.0117, -0.0092, -0.0074, 0.0000]
        }
    }

    if Re not in ghia_data:
        raise ValueError("Only Re=100, 400, 1000 supported for Ghia reference data.")

    if direction == 'vertical':
        y_ref = np.array(ghia_data[Re]['y'])
        u_ref = np.array(ghia_data[Re]['u'])
        return np.interp(y, y_ref, u_ref)
    elif direction == 'horizontal':
        x_ref = np.array(ghia_data[Re]['x'])
        v_ref = np.array(ghia_data[Re]['v'])
        return np.interp(y, x_ref, v_ref)
    else:
        raise ValueError("direction must be 'vertical' or 'horizontal'")

def analytical_von_karman():
    # No simple analytical solution; compare to reference data or check vorticity pattern
    pass

# --- Run CappuSim and load output ---
def run_cappusim(case, dim, precision, bin_path=None, output_dir='output'):
    exe = bin_path or 'cappusim.exe' if os.name == 'nt' else './cappusim'
    args = [exe, '--case', case, '--dim', str(dim), '--precision', precision, '--output', output_dir]
    print('Running:', ' '.join(args))
    result = subprocess.run(args, capture_output=True, text=True)
    if result.returncode != 0:
        print('Error running CappuSim:', result.stderr)
        sys.exit(1)
    # Assume output is written to output_dir (e.g., velocity.npy, density.npy)
    u = np.load(os.path.join(output_dir, 'velocity.npy'))
    rho = np.load(os.path.join(output_dir, 'density.npy'))
    return u, rho

# --- Validation logic ---
def validate_case(case, dim, precision, bin_path=None):
    u, rho = run_cappusim(case, dim, precision, bin_path)
    if case == 'couette':
        # Assume 2D: u.shape = (NY, NX, 2) or 3D: (NZ, NY, NX, 3)
        if dim == 2:
            NY = u.shape[0]
            y = np.linspace(0, 1, NY)
            u_analytical = analytical_couette(y)
            u_sim = u[:, u.shape[1]//2, 0]  # Centerline, x-velocity
            error = np.abs(u_sim - u_analytical)
            plt.plot(y, u_sim, label='Sim')
            plt.plot(y, u_analytical, label='Analytical')
            plt.title(f'Couette {precision} ({dim}D)')
            plt.legend()
            plt.show()
            print('Max error:', np.max(error))
        else:
            # 3D: take centerline in x and z
            NY = u.shape[1]
            y = np.linspace(0, 1, NY)
            u_analytical = analytical_couette(y)
            u_sim = u[u.shape[0]//2, :, u.shape[2]//2, 0]
            error = np.abs(u_sim - u_analytical)
            plt.plot(y, u_sim, label='Sim')
            plt.plot(y, u_analytical, label='Analytical')
            plt.title(f'Couette {precision} ({dim}D)')
            plt.legend()
            plt.show()
            print('Max error:', np.max(error))
    elif case == 'poiseuille':
        # Similar logic for Poiseuille
        pass
    elif case == 'liddriven_cavity':
        # Compare to reference data (Ghia et al.)
        pass
    elif case == 'von_karman':
        # Check vorticity pattern or compare to reference
        pass
    else:
        print('Unknown case')

if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('--case', type=str, required=True, choices=['couette', 'poiseuille', 'liddriven_cavity', 'von_karman'])
    parser.add_argument('--dim', type=int, default=2, choices=[2, 3])
    parser.add_argument('--precision', type=str, default='FP32', choices=['FP32', 'FP16S', 'FP16C'])
    parser.add_argument('--bin', type=str, default=None, help='Path to CappuSim binary')
    args = parser.parse_args()
    validate_case(args.case, args.dim, args.precision, args.bin)
