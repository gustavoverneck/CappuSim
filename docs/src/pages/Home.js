import React from 'react';

const Home = () => {
  return (
    <div className="home">
      <section className="hero">
        <h1>CappuSim</h1>
        <p className="lead">
          <strong>CappuSim</strong> is a GPU-accelerated, Rust-based framework for simulating fluid dynamics using the <strong>Lattice Boltzmann Method (LBM)</strong>. It is designed to be modular, readable, and easy to extend for research and academic applications in computational fluid dynamics (CFD).
        </p>
      </section>

      <section className="features">
        <h2>Features</h2>
        <div className="feature-grid">
          <div className="feature-card">
            <h3>Multiple Models</h3>
            <p>D2Q9, D3Q7, D3Q15, D3Q19, D3Q27</p>
          </div>
          <div className="feature-card">
            <h3>Boundary Conditions</h3>
            <p>Solid / Fluid / Equilibrium boundaries</p>
          </div>
          <div className="feature-card">
            <h3>GPU Acceleration</h3>
            <p>OpenCL for high performance</p>
          </div>
          <div className="feature-card">
            <h3>Output Formats</h3>
            <p>CSV and VTK for plotting</p>
          </div>
        </div>
      </section>

      <section className="examples">
        <h2>Examples</h2>
        <div className="examples-table">
          <div className="example-row">
            <span className="case">Lid-Driven Cavity</span>
            <code>liddriven_cavity.rs</code>
          </div>
          <div className="example-row">
            <span className="case">Poiseuille Flow</span>
            <code>poiseuille.rs</code>
          </div>
          <div className="example-row">
            <span className="case">Taylor-Green Vortex</span>
            <code>taylor_green_vortex.rs</code>
          </div>
          <div className="example-row">
            <span className="case">Von Kármán Vortex Street</span>
            <code>von_karman_vortex.rs</code>
          </div>
        </div>
      </section>
    </div>
  );
};

export default Home;