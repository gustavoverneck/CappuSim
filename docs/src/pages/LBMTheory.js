import React from 'react';

const LBMTheory = () => {
  return (
    <div className="lbm-theory">
      <h1>Lattice Boltzmann Method Theory</h1>
      
      <section>
        <h2>Introduction</h2>
        <p>
          The Lattice Boltzmann Method (LBM) is a computational fluid dynamics method 
          based on the kinetic theory of gases. It simulates fluid flow by tracking 
          the distribution of particles on a discrete lattice.
        </p>
      </section>

      <section>
        <h2>Fundamental Equation</h2>
        <p>
          The lattice Boltzmann equation describes the evolution of distribution functions:
        </p>
        <code>f_i(x + c_i δt, t + δt) = f_i(x,t) - 1/τ [f_i(x,t) - f_i^eq(x,t)]</code>
      </section>

      <section>
        <h2>Supported Models</h2>
        <ul>
          <li><strong>D2Q9:</strong> 2D lattice with 9 velocities</li>
          <li><strong>D3Q7:</strong> 3D lattice with 7 velocities</li>
          <li><strong>D3Q15:</strong> 3D lattice with 15 velocities</li>
          <li><strong>D3Q19:</strong> 3D lattice with 19 velocities</li>
          <li><strong>D3Q27:</strong> 3D lattice with 27 velocities</li>
        </ul>
      </section>
    </div>
  );
};

export default LBMTheory;