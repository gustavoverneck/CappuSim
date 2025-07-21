import React from 'react';

const Documentation = () => (
  <div className="documentation">
    <h1>Source Folder & File Reference</h1>

    <section>
      <h2>Overview</h2>
      <p>
        The <b>src</b> folder contains all core code for CappuSim. It is organized into modules for simulation logic, GPU kernels, solver routines, and utilities. Below is a description of each directory and key files.
      </p>
    </section>

    <section>
      <h2>Top-Level Files</h2>
      <ul>
        <li>
          <b>main.rs</b><br />
          <span>Main entry point. Selects and configures the simulation, sets up initial and boundary conditions, and runs benchmarks or examples.</span>
        </li>
        <li>
          <b>mod.rs</b><br />
          <span>Declares the main modules (solver, utils) for the project.</span>
        </li>
      </ul>
    </section>

    <section>
      <h2>Folders</h2>
      <ul>
        <li>
          <b>kernels/</b><br />
          <span>OpenCL kernel source files for GPU acceleration. Includes:</span>
          <ul>
            <li><b>kernel_collision.cl</b>: Collision step kernel (BGK and equilibrium).</li>
            <li><b>kernel_equilibrium.cl</b>: Computes equilibrium distribution functions.</li>
            <li><b>kernel_streaming.cl</b>: Streaming step kernel.</li>
            <li><b>kernel_velocity_sets.cl</b>: Defines velocity sets for different lattice models.</li>
          </ul>
        </li>
        <li>
          <b>solver/</b><br />
          <span>Core LBM routines and simulation logic:</span>
          <ul>
            <li><b>benchmark.rs</b>: Benchmarking and performance tests.</li>
            <li><b>check.rs</b>: Validation and error checking utilities.</li>
            <li><b>flags.rs</b>: Defines cell flags (fluid, solid, equilibrium).</li>
            <li><b>init.rs</b>: Initialization routines for LBM objects and arrays.</li>
            <li><b>kernel.rs</b>: Loads and manages OpenCL kernels.</li>
            <li><b>lbm.rs</b>: Main LBM struct and methods (collision, streaming, etc.).</li>
            <li><b>opencl.rs</b>: OpenCL device/context management and buffer handling.</li>
            <li><b>output.rs</b>: Output routines for VTK/CSV file generation.</li>
            <li><b>run.rs</b>: Simulation loop and time stepping.</li>
            <li><b>transforms.rs</b>: Coordinate transforms and indexing utilities.</li>
            <li><b>mod.rs</b>: Module declarations for the solver.</li>
          </ul>
        </li>
        <li>
          <b>utils/</b><br />
          <span>General utilities:</span>
          <ul>
            <li><b>mod.rs</b>: Module declarations for utils.</li>
            <li><b>terminal_utils.rs</b>: Terminal output and progress bar helpers.</li>
            <li><b>velocity.rs</b>: Velocity vector utilities and types.</li>
          </ul>
        </li>
      </ul>
    </section>

    <section>
      <h2>How to Use</h2>
      <ol>
        <li>
          <b>Edit <code>main.rs</code></b> to select and configure your simulation scenario (e.g., grid size, viscosity, boundary conditions).
        </li>
        <li>
          <b>Customize kernels</b> in <code>kernels/</code> if you need to modify GPU computation steps.
        </li>
        <li>
          <b>Modify solver logic</b> in <code>solver/</code> to change LBM routines, output, or benchmarking.
        </li>
        <li>
          <b>Use utilities</b> in <code>utils/</code> for velocity handling and terminal feedback.
        </li>
        <li>
          <b>Run the simulation</b> and check output files for results and visualization.
        </li>
      </ol>
    </section>
  </div>
);

export default Documentation;