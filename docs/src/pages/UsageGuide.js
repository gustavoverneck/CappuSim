import React from 'react';

const UsageGuide = () => {
  return (
    <div className="usage-guide">
      <h1>Usage Guide</h1>
      
      <section>
        <h2>Installation</h2>
        <ol>
          <li>Clone the repository:
            <pre><code>git clone https://github.com/gustavoverneck/CappuSim.git</code></pre>
          </li>
          <li>Install Rust and Cargo</li>
          <li>Install OpenCL drivers for your GPU</li>
          <li>Build the project:
            <pre><code>cargo build --release</code></pre>
          </li>
        </ol>
      </section>

      <section>
        <h2>Running Examples</h2>
        <p>To run the lid-driven cavity example:</p>
        <pre><code>cargo run --example liddriven_cavity</code></pre>
        
        <p>To run with custom parameters:</p>
        <pre><code>cargo run --example poiseuille -- --width 256 --height 128</code></pre>
      </section>

      <section>
        <h2>Configuration</h2>
        <p>
          Simulation parameters can be configured through the configuration files 
          or command-line arguments. See the examples directory for reference configurations.
        </p>
      </section>
    </div>
  );
};

export default UsageGuide;