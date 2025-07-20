import React from 'react';

const Documentation = () => {
  return (
    <div className="documentation">
      <h1>API Documentation</h1>
      
      <section>
        <h2>Core Modules</h2>
        
        <h3>Simulation</h3>
        <p>Main simulation engine handling the LBM computation loop.</p>
        
        <h3>Lattice</h3>
        <p>Defines the lattice structure and velocity models.</p>
        
        <h3>Boundaries</h3>
        <p>Implements various boundary conditions for different flow scenarios.</p>
        
        <h3>GPU</h3>
        <p>OpenCL integration for GPU acceleration.</p>
      </section>

      <section>
        <h2>Key Structs</h2>
        
        <h3><code>LBMSimulation</code></h3>
        <p>Primary struct for running LBM simulations.</p>
        
        <h3><code>Lattice&lt;T&gt;</code></h3>
        <p>Generic lattice implementation supporting different velocity models.</p>
        
        <h3><code>BoundaryCondition</code></h3>
        <p>Enum defining available boundary condition types.</p>
      </section>

      <section>
        <h2>Examples Reference</h2>
        <p>
          For detailed usage examples, see the <code>examples/</code> directory in the repository.
          Each example demonstrates different aspects of the framework.
        </p>
      </section>
    </div>
  );
};

export default Documentation;