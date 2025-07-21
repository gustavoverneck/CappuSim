import React from 'react';
import 'katex/dist/katex.min.css';
import { InlineMath, BlockMath } from 'react-katex';

const LBMTheory = () => (
  <div className="lbm-theory">
    <h1>Lattice Boltzmann Method (LBM) Theory</h1>

    <section>
      <h2>What is LBM?</h2>
      <p>
        The <b>Lattice Boltzmann Method (LBM)</b> is a modern computational fluid dynamics (CFD) technique. Unlike traditional CFD, which solves the macroscopic Navier-Stokes equations directly, LBM models fluid flow by simulating the microscopic movement and collisions of particle distribution functions on a discrete lattice grid.
      </p>
      <p>
        LBM is especially powerful for simulating complex flows, porous media, multiphase fluids, and flows with intricate boundaries. Its algorithm is highly parallelizable, making it well-suited for GPU acceleration.
      </p>
    </section>

    <section>
      <h2>Origins</h2>
      <p>
        LBM evolved from the <b>Lattice Gas Automaton (LGA)</b> models developed in the 1980s. LGA used discrete particles moving and colliding on a grid, but suffered from statistical noise. LBM replaced particles with distribution functions, providing smoother and more accurate results while retaining the lattice-based approach.
      </p>
    </section>

    <section>
      <h2>Fundamental Equation</h2>
      <p>
        The evolution of the particle distribution function in LBM is governed by the discrete lattice Boltzmann equation:
      </p>
      <div style={{ margin: "1em 0", fontSize: "1.1em" }}>
        <BlockMath math={
          `f_i(\\mathbf{x} + \\mathbf{c}_i \\delta t,\, t + \\delta t) = f_i(\\mathbf{x}, t) - \\frac{1}{\\tau} \\left[ f_i(\\mathbf{x}, t) - f_i^{eq}(\\mathbf{x}, t) \\right]`
        } />
      </div>
      <p>
        Here:
        <ul>
          <li><InlineMath math="f_i" /> is the particle distribution function in direction <InlineMath math="i" /></li>
          <li><InlineMath math="{c}_i" /> is the discrete velocity vector for direction <InlineMath math="i" /></li>
          <li><InlineMath math="\tau" /> is the relaxation time (related to viscosity)</li>
          <li><InlineMath math="f_i^{eq}" /> is the local equilibrium distribution</li>
        </ul>
      </p>
    </section>

    <section>
      <h2>Macroscopic Quantities</h2>
      <p>
        Macroscopic fluid properties are recovered from the distribution functions:
      </p>
      <div style={{ margin: "1em 0", fontSize: "1.1em" }}>
        <b>Density:</b> <InlineMath math="\rho = \sum_i f_i" /> <br />
        <b>Velocity:</b> <InlineMath math="\mathbf{u} = \frac{1}{\\rho} \sum_i f_i \mathbf{c}_i" />
      </div>
    </section>

    <section>
      <h2>Lattice Directions</h2>
      <p>
        The lattice in LBM is defined by its dimensionality and the number of discrete velocity directions. Common models include:
      </p>
      <ul>
        <li><b>D2Q9:</b> 2D lattice, 9 directions</li>
        <li><b>D3Q15:</b> 3D lattice, 15 directions</li>
        <li><b>D3Q19:</b> 3D lattice, 19 directions</li>
        <li><b>D3Q27:</b> 3D lattice, 27 directions</li>
      </ul>
      <p>
        For example, the D2Q9 model uses the following velocity vectors:
      </p>
      <div style={{ margin: "1em 0", fontSize: "1.1em" }}>
        <BlockMath math={
          `\\mathbf{c}_i = 
          \\begin{cases}
            (0, 0) & i = 0 \\\\
            (1, 0), (0, 1), (-1, 0), (0, -1) & i = 1,2,3,4 \\\\
            (1, 1), (-1, 1), (-1, -1), (1, -1) & i = 5,6,7,8
          \\end{cases}`
        } />
      </div>
    </section>

    <section>
      <h2>Summary</h2>
      <ul>
        <li>LBM is a CFD method based on kinetic theory and lattice grids.</li>
        <li>It is efficient, parallelizable, and handles complex boundaries well.</li>
        <li>Widely used for single-phase and multiphase flows, porous media, and microfluidics.</li>
      </ul>
    </section>
  </div>
);

export default LBMTheory;
