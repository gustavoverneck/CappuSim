import React from 'react';
import { 
  FaBook, 
  FaRocket, 
  FaCogs, 
  FaChartBar, 
  FaWrench,
  FaClipboardList,
  FaLaptop,
  FaTools,
  FaGamepad,
  FaDownload,
  FaRust,
  FaHammer,
  FaCheckCircle,
  FaEdit,
  FaSyncAlt,
  FaPlay,
  FaFileExport,
  FaTable,
  FaShieldAlt,
  FaBan,
  FaExclamationTriangle,
  FaTachometerAlt,
  FaMemory,
  FaLifeRing,
  FaSearch,
  FaExternalLinkAlt
} from 'react-icons/fa';

const UsageGuide = () => {
  return (
    <div className="usage-guide">
      <div className="guide-header">
        <h1>
          <FaBook className="header-icon" /> Usage Guide
        </h1>
        <p className="guide-intro">
          Complete guide to installing, configuring, and running CappuSim simulations.
          Follow this step-by-step tutorial to get started with lattice Boltzmann fluid dynamics.
        </p>
      </div>

      <nav className="guide-nav">
  <div className="nav-header">
    <h3><FaClipboardList /> Quick Navigation</h3>
    <p className="nav-subtitle">Jump to any section</p>
  </div>
  <div className="nav-grid">
    <a href="#prerequisites" className="nav-item nav-item--prerequisites">
      <div className="nav-item-icon">
        <FaClipboardList />
      </div>
      <div className="nav-item-content">
        <span className="nav-text">Prerequisites</span>
        <span className="nav-description">System requirements & setup</span>
      </div>
    </a>
    
    <a href="#installation" className="nav-item nav-item--installation">
      <div className="nav-item-icon">
        <FaCogs />
      </div>
      <div className="nav-item-content">
        <span className="nav-text">Installation</span>
        <span className="nav-description">4-step setup process</span>
      </div>
    </a>
    
    <a href="#running-simulations" className="nav-item nav-item--simulations">
      <div className="nav-item-icon">
        <FaRocket />
      </div>
      <div className="nav-item-content">
        <span className="nav-text">Running Simulations</span>
        <span className="nav-description">Basic workflow & structure</span>
      </div>
    </a>
    
    <a href="#configuration" className="nav-item nav-item--configuration">
      <div className="nav-item-icon">
        <FaCogs />
      </div>
      <div className="nav-item-content">
        <span className="nav-text">Configuration</span>
        <span className="nav</a>-description">Models & parameters</span>
      </div>
    </a>
    
    <a href="#output" className="nav-item </div>nav-item--output">
      <div className="nav-item-icon">
        <FaChartBar />
      </div>
      <div className="nav-item-content">
        <span className="nav-text">Output & Visualization</span>
        <span className="nav-description">VTK files & analysis</span>
      </div>
    </a>
    
    <a href="#troubleshooting" className="nav-item nav-item--troubleshooting">
      <div className="nav-item-icon">
        <FaWrench />
      </div>
      <div className="nav-item-content">
        <span className="nav-text">Troubleshooting</span>
        <span className="nav-descripti</div>on">Common issues & solutions</span>
      </div>
    </a>
  </div>
</nav>

      {/* Prerequisites Section */
      <section id="prerequisites" className="guide-section">
        <div className="section-header">
          <h2>
            <FaClipboardList /> Prerequisites
          </h2>
          <p className="section-subtitle">Essential requirements before getting started</p>
        </div>
        <div className="requirements-container">
          <div className="requirement-card">
            <h3>
              <FaLaptop /> System Requirements
            </h3>
            <ul className="requirement-list">
              <li>
                <span className="req-label"><strong> Operating System: </strong> </span>
                <span className="req-value">Windows 10+, Linux, or macOS</span>
              </li>
              <li>
                <span className="req-label"><strong>GPU: </strong></span>
                <span className="req-value">OpenCL-compatible graphics card (NVIDIA, AMD, or Intel)</span>
              </li>
              <li>
                <span className="req-label"><strong>Memory: </strong></span>
                <span className="req-value">Depends on simulated grid size. 8GB+ VRAM is recommended.</span>
              </li>
              <li>
                <span className="req-label"><strong>Storage: </strong></span>
                <span className="req-value">Depends on grid size and steps stored.</span>
              </li>
            </ul>
          </div>
          <div className="requirement-card">
            <h3>
              <FaTools /> Required Software
            </h3>
            <ul className="software-list">
              <li className="software-item">
                <strong>Rust Toolchain</strong>
                <p>
                  Install from{" "}
                  <a href="https://rustup.rs/" target="_blank" rel="noopener noreferrer">
                    rustup.rs <FaExternalLinkAlt />
                  </a>
                </p>
              </li>
              <li className="software-item">
                <strong>Git</strong>
                <p>For repository cloning and version control</p>
                <span className="version-badge">Latest</span>
              </li>
              <li className="software-item">
                <strong>OpenCL Drivers</strong>
                <p>GPU-specific drivers for acceleration</p>
              </li>
            </ul>
          </div>
        </div>
        <div className="driver-section">
          <h3>
            <FaGamepad /> OpenCL Driver Installation
          </h3>
          <div className="driver-grid">
            <div className="driver-card nvidia">
              <div className="driver-header">
                <h4>NVIDIA</h4>
                <span className="driver-icon green">●</span>
              </div>
              <p>
                Install CUDA Toolkit<br />
                <a href="https://developer.nvidia.com/cuda-downloads" target="_blank" rel="noopener noreferrer">
                  CUDA Downloads <FaExternalLinkAlt />
                </a>
              </p>
              <p>Para verificar drivers</p>
              <div className="verification-cmd">
                <pre>
                  <code style={{ whiteSpace: 'pre-line' }}>
                    {`# To verify installed versions
                    nvidia-smi`}
                  </code>
                </pre>
              </div>
            </div>
            <div className="driver-card amd">
              <div className="driver-header">
                <h4>AMD</h4>
              </div>
              <p>
                Install AMD Radeon Software or ROCm<br />
                <a href="https://github.com/ROCm/clr" target="_blank" rel="noopener noreferrer">
                  ROCm <FaExternalLinkAlt />
                </a>
              </p>
              <div className="verification-cmd">
                <pre>
                  <code style={{ whiteSpace: 'pre-line' }}>
                    {`# To verify installed versions
                    clinfo`}
                  </code>
                </pre>
              </div>
            </div>
            <div className="driver-card intel">
              <div className="driver-header">
                <h4>Intel</h4>
              </div>
              <p>
                Install Intel OpenCL Runtime <br></br>
                <a href='https://www.intel.com/content/www/us/en/developer/articles/tool/opencl-drivers.html' target='_blank' rel='noopener noreferrer'>
                  Intel OpenCL Drivers <FaExternalLinkAlt aria-label="External link" />
                </a>
                </p>
              <div className="verification-cmd">
                <pre>
                  <code style={{ whiteSpace: 'pre-line' }}>
                    {`# To verify installed versions
                    clinfo`}
                  </code>
                </pre>
              </div>
            </div>
          </div>
        </div>
      </section>

      /* Installation Section */}
      <section id="installation" className="guide-section">
        <div className="section-header">
          <h2>
            <FaCogs /> Installation
          </h2>
          <p className="section-subtitle">Get CappuSim up and running in 4 simple steps</p>
        </div>
        <ol className="install-timeline">
          <lu className="install-step">
            <div className="step-number"></div>
            <div className="step-content">
              <h3>
                <FaDownload /> Clone the Repository
              </h3>
              <div className="code-block">
                <pre>
                  <code>
{`git clone https://github.com/gustavoverneck/CappuSim.git
cd CappuSim`}
                  </code>
                </pre>
              </div>
              <p className="step-note">Downloads the complete CappuSim source code</p>
            </div>
          </lu>
          <lu className="install-step">
            <div className="step-number"></div>
            <div className="step-content">
              <h3>
                <FaRust /> Install Rust (if needed)
              </h3>
              <div className="code-block">
                <pre>
                  <code>
{`# Install Rust using rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Verify installation
rustc --version
cargo --version`}
                  </code>
                </pre>
              </div>
              <p className="step-note">Should show Rust 1.70+ and corresponding Cargo version</p>
            </div>
          </lu>
          <lu className="install-step">
            <div className="step-number"></div>
            <div className="step-content">
              <h3>
                <FaHammer /> Build the Project
              </h3>
              <div className="code-block">
                <pre>
                  <code>
{`# Release build (recommended for simulations)
cargo build --release

# Debug build (faster compilation, slower execution)
cargo build`}
                  </code>
                </pre>
              </div>
              <div className="build-warning">
                <strong>
                  <FaExclamationTriangle /> Important:
                </strong>{" "}
                Always use <code>--release</code> for production simulations as it provides performance improvements.
              </div>
            </div>
          </lu>
          <lu className="install-step">
            <div className="step-number"></div>
            <div className="step-content">
              <h3>
                <FaCheckCircle /> Verify Installation
              </h3>
              <p>Test by running the comprehensive benchmark suite:</p>
              <div className="code-block">
                <pre>
                  <code>cargo run --release</code>
                </pre>
              </div>
              <p className="step-note">This runs benchmarks across all models and grid sizes to verify your setup</p>
            </div>
          </lu>
        </ol>
      </section>

      {/* Running Simulations Section */}
      <section id="running-simulations" className="guide-section">
        <div className="section-header">
          <h2>
            <FaRocket /> Running Simulations
          </h2>
          <p className="section-subtitle">
            Learn the basic workflow for setting up and running simulations
          </p>
        </div>
        <div className="simulation-workflow">
          <h3>
            <FaEdit /> Basic Simulation Structure
          </h3>
          <p>
            CappuSim simulations are configured by editing the <code>main.rs</code> file. Each example is commented out – simply uncomment the simulation you want to run:
          </p>
          <div className="code-block-large">
            <pre>
              <code>
      {`// Edit src/main.rs and choose a simulation example:
      fn main() {
          // --- 2D Lid-driven Cavity ---
          // let mut lbm = LBM::new(128, 128, 1, "D2Q9".to_string(), 0.1);

          // --- 2D Poiseuille Flow ---
          // let mut lbm = LBM::new(256, 64, 1, "D2Q9".to_string(), 0.05);

          // --- 3D Lid-driven Cavity ---
          // let mut lbm = LBM::new(64, 64, 64, "D3Q19".to_string(), 0.12);

          // --- Run comprehensive benchmark (default) ---
          LBM::benchmark();
      }`}
              </code>
            </pre>
          </div>
          <div className="workflow-steps">
            <h4>
              <FaSyncAlt /> Simulation Workflow
            </h4>
            <ol className="workflow-grid">
              <lu className="workflow-item">
                <span className="workflow-number"></span>
                <div className="workflow-content">
                  <h5>
                    <FaCogs /> Initialize
                  </h5>
                  <ul>
                    <li>Choose lattice model and grid size</li>
                    <li>Create LBM instance</li>
                  </ul>
                </div>
              </lu>
              <lu className="workflow-item">
                <span className="workflow-number">2</span>
                <div className="workflow-content">
                  <h5>
                    <FaEdit /> Set Conditions
                  </h5>
                  <ul>
                    <li>Define boundary conditions</li>
                    <li>Set initial velocity and density</li>
                  </ul>
                </div>
              </lu>
              <lu className="workflow-item">
                <span className="workflow-number"></span>
                <div className="workflow-content">
                  <h5>
                    <FaCogs /> Configure Output
                  </h5>
                  <ul>
                    <li>Enable VTK or CSV output</li>
                    <li>Set output intervals</li>
                  </ul>
                </div>
              </lu>
              <lu className="workflow-item">
                <span className="workflow-number"></span>
                <div className="workflow-content">
                  <h5>
                    <FaPlay /> Run
                  </h5>
                  <ul>
                    <li>Execute simulation for desired time steps</li>
                    <li>Monitor progress in terminal</li>
                  </ul>
                </div>
              </lu>
              <lu className="workflow-item">
                <span className="workflow-number"></span>
                <div className="workflow-content">
                  <h5>
                    <FaFileExport /> Export
                  </h5>
                  <ul>
                    <li>Results saved to <code>.vtk</code> or <code>.csv</code></li>
                    <li>
                      Visualize with&nbsp;
                      <a href="https://www.paraview.org/" target="_blank" rel="noopener noreferrer">ParaView <FaExternalLinkAlt /></a>
                      &nbsp;or Python
                    </li>
                  </ul>
                </div>
              </lu>
            </ol>
          </div>
        </div>
      </section>

      {/* Configuration Section */}
      <section id="configuration" className="guide-section">
        <div className="section-header">
          <h2>
            <FaCogs /> Configuration
          </h2>
          <p className="section-subtitle">Understanding lattice models, boundary conditions, and simulation parameters</p>
        </div>
        <div className="config-container">
          <div className="config-section">
            <h3>
              <FaTable /> Lattice Models
            </h3>
            <div className="models-table-container">
              <table className="models-table">
                <thead>
                  <tr>
                    <th>Model</th>
                    <th>Dimensions</th>
                    <th>Velocities</th>
                    <th>Use Case</th>
                    <th>Performance</th>
                  </tr>
                </thead>
                <tbody>
                  <tr>
                    <td>
                      <code className="model-code">D2Q9</code>
                    </td>
                    <td>2D</td>
                    <td>9 directions</td>
                    <td>Most 2D simulations</td>
                    <td>
                      <span className="perf-badge high">Excellent</span>
                    </td>
                  </tr>
                  <tr>
                    <td>
                      <code className="model-code">D3Q7</code>
                    </td>
                    <td>3D</td>
                    <td>7 directions</td>
                    <td>Fast prototyping, large-scale flows, educational demos</td>
                    <td>
                      <span className="perf-badge high">Very Fast</span>
                    </td>
                  </tr>
                  <tr>
                    <td>
                      <code className="model-code">D3Q15</code>
                    </td>
                    <td>3D</td>
                    <td>15 directions</td>
                    <td>Pipe flows, mixing, moderate Reynolds number turbulence</td>
                    <td>
                      <span className="perf-badge medium">Good</span>
                    </td>
                  </tr>
                  <tr>
                    <td>
                      <code className="model-code">D3Q19</code>
                    </td>
                    <td>3D</td>
                    <td>19 directions</td>
                    <td>Complex geometries, porous media, biofluid dynamics</td>
                    <td>
                      <span className="perf-badge medium">Good</span>
                    </td>
                  </tr>
                  <tr>
                    <td>
                      <code className="model-code">D3Q27</code>
                    </td>
                    <td>3D</td>
                    <td>27 directions</td>
                    <td>High-fidelity research, detailed vorticity analysis, publication quality</td>
                    <td>
                      <span className="perf-badge low">Slower</span>
                    </td>
                  </tr>
                </tbody>
              </table>
            </div>
          </div>
          <div className="config-section">
            <h3>
              <FaShieldAlt /> Boundary Conditions
            </h3>
            <p className="section-description">
              Define how fluid interacts with domain boundaries and obstacles.
            </p>
            <div className="boundary-grid">
              <div className="boundary-card solid">
                <h4><code>FLAG_SOLID</code> — No-Slip</h4>
                <p>
                  Solid walls with zero velocity at the boundary. Fluid particles bounce back, creating realistic wall interactions.
                </p>
                <ul>
                  <li><b>Behavior:</b> Zero velocity, full reflection</li>
                  <li><b>Used for:</b> Walls, obstacles, solid boundaries</li>
                  <li><b>Examples:</b> Pipe walls, airfoil surfaces, channel boundaries</li>
                </ul>
              </div>
              <div className="boundary-card fluid">
                <h4><code>FLAG_FLUID</code> — Interior</h4>
                <p>
                  Main fluid domain where lattice Boltzmann equations are solved. Velocity and density computed from neighboring cells.
                </p>
                <ul>
                  <li><b>Behavior:</b> Full LBM computation</li>
                  <li><b>Used for:</b> Main fluid domain, bulk flow regions</li>
                  <li><b>Examples:</b> Interior flow, mixing regions, free stream</li>
                </ul>
              </div>
              <div className="boundary-card equilibrium">
                <h4><code>FLAG_EQ</code> — Prescribed</h4>
                <p>
                  Boundaries with prescribed velocity or density values. Forces equilibrium distribution based on specified conditions.
                </p>
                <ul>
                  <li><b>Behavior:</b> Fixed velocity/density</li>
                  <li><b>Used for:</b> Inlets, outlets, moving walls</li>
                  <li><b>Examples:</b> Inlet velocities, pressure outlets, lid-driven cavity</li>
                </ul>
              </div>
            </div>
          </div>
          <div className="config-section">
            <h3>
              <FaChartBar /> Simulation Parameters
            </h3>
            <p className="section-description">
              Key parameters that control simulation behavior and accuracy.
            </p>
            <div className="params-grid">
              <div className="param-card">
                <h4>Grid Size (nx, ny, nz)</h4>
                <ul>
                  <li><b>Description:</b> Number of lattice points in each spatial direction. Determines simulation resolution and computational cost.</li>
                  <li><b>Typical Range:</b> 64–512 per dimension</li>
                  <li><b>Memory Impact:</b> O(nx × ny × nz × velocities)</li>
                  <li><b>Tip:</b> Start with 128×128 for 2D, 64×64×64 for 3D testing</li>
                </ul>
              </div>

              <div className="param-card">
                <h4>Viscosity (ν)</h4>
                <ul>
                  <li><b>Description:</b> Kinematic viscosity controlling fluid behavior and Reynolds number. Lower values create more turbulent flow.</li>
                  <li><b>Typical Range:</b> 0.01–0.2 (lattice units)</li>
                  <li><b>Reynolds Number:</b> Re = UL/ν</li>
                  <li><b>Tip:</b> Higher viscosity = more stable, lower Reynolds number</li>
                </ul>
              </div>

              <div className="param-card">
                <h4>Time Steps</h4>
                <ul>
                  <li><b>Description:</b> Number of LBM iterations to perform. More steps allow flow to develop and reach steady state.</li>
                  <li><b>Typical Range:</b> 1,000–100,000+ steps</li>
                  <li><b>Convergence:</b> ~10–50 flow-through times</li>
                  <li><b>Tip:</b> Monitor residuals to determine when flow converges</li>
                </ul>
              </div>

              <div className="param-card">
                <h4>Output Interval</h4>
                <ul>
                  <li><b>Description:</b> Frequency of VTK file generation for visualization. Balance between temporal resolution and storage space.</li>
                  <li><b>Typical Range:</b> Every 10–1000 steps</li>
                  <li><b>File Size:</b> ~10–100MB per VTK file</li>
                  <li><b>Tip:</b> Higher frequency for transient analysis, lower for steady flows</li>
                </ul>
              </div>
            </div>
          </div>
        </div>
      </section>

      {/* Output & Visualization Section */}
      <section id="output" className="guide-section">
        <div className="section-header">
          <h2>
            <FaChartBar /> Output & Visualization
          </h2>
          <p className="section-subtitle">Understanding output formats and visualization workflows</p>
        </div>
        <div className="output-container">
          <div className="output-formats">
            <h3>Output Formats</h3>
            <p>CappuSim supports multiple output formats for different analysis workflows.</p>
            <div className="format-grid">
              <div className="format-card vtk">
                <h4>VTK Files</h4>
                <ul>
                  <li><b>Description:</b> Scientific visualization format. Contains velocity, pressure, and density fields on structured grids.</li>
                  <li><b>How to generate:</b> <code>lbm.set_output_vtk(true)</code></li>
                  <li><b>Location:</b> <code>output/</code> directory</li>
                  <li><b>Contents:</b> Velocity field, density, pressure, grid coordinates</li>
                  <li><b>Compatible software:</b> ParaView, VisIt, Mayavi, Tecplot</li>
                </ul>
              </div>
              <div className="format-card csv">
                <h4>CSV Files</h4>
                <ul>
                  <li><b>Description:</b> Tabular format for quantitative analysis and custom plotting.</li>
                  <li><b>How to generate:</b> <code>lbm.output_to("results.csv")</code></li>
                  <li><b>Timing:</b> On-demand or at simulation end</li>
                  <li><b>Contents:</b> Grid coordinates, velocity components, density, derived quantities</li>
                  <li><b>Compatible software:</b> Excel, Python/Pandas, MATLAB, R, Origin</li>
                </ul>
              </div>
            </div>
          </div>
          <div className="visualization-section">
            <h3>Visualization Examples</h3>
            <div className="viz-grid">
              <div className="viz-card paraview">
                <h4>ParaView Workflow</h4>
                <ul>
                  <li><b>Step 1:</b> Open ParaView</li>
                  <li><b>Step 2:</b> Go to <b>File &rarr; Open</b> and select VTK files from <code>output/</code></li>
                  <li><b>Step 3:</b> Click <b>Apply</b> to load data</li>
                  <li><b>Step 4:</b> Add filters:
                    <ul>
                      <li><b>Glyph:</b> for velocity vectors</li>
                      <li><b>Stream Tracer:</b> for streamlines</li>
                      <li><b>Contour:</b> for pressure or vorticity</li>
                    </ul>
                  </li>
                  <li><b>Step 5:</b> Use the <b>Play</b> button to animate time series</li>
                </ul>
              </div>
              <div className="viz-card python">
                <h4>Python Analysis</h4>
                <p>Example for plotting velocity magnitude from CSV output:</p>
                <div className="code-block-large">
                  <pre>
                    <code>
{`import pandas as pd
import matplotlib.pyplot as plt
import numpy as np

# Load simulation data
data = pd.read_csv('output.csv')

# Extract velocity components
u = data['velocity_x'].values
v = data['velocity_y'].values

# Create velocity magnitude plot
velocity_mag = np.sqrt(u**2 + v**2)
plt.figure(figsize=(12, 8))
plt.contourf(velocity_mag.reshape(ny, nx))
plt.colorbar(label='Velocity Magnitude')
plt.title('Velocity Field')
plt.show()`}
                    </code>
                  </pre>
                </div>
              </div>
            </div>
          </div>
        </div>
      </section>

      {/* Troubleshooting Section */}
      <section id="troubleshooting" className="guide-section">
        <div className="section-header">
          <h2>
            <FaWrench /> Troubleshooting
          </h2>
        </div>
        <div className="troubleshooting-container">
          <div className="issue-card critical">
            <div className="issue-header">
              <h3>
                <FaBan /> OpenCL Device Not Found
              </h3>
            </div>
            <div className="issue-content">
              <div className="symptoms">
                <h4>
                  <FaSearch /> Symptoms:
                </h4>
                <p>"No suitable OpenCL device found" error</p>
              </div>
              <div className="solutions">
                <h4>
                  <FaCheckCircle /> Solutions:
                </h4>
                <ul className="solution-list">
                  <li>Install/update GPU drivers</li>
                  <li>Verify OpenCL with <code>clinfo</code></li>
                  <li>Install OpenCL runtime for your GPU vendor</li>
                  <li>For Intel GPUs, ensure OpenCL is enabled in BIOS</li>
                </ul>
              </div>
            </div>
          </div>
          <div className="issue-card high">
            <div className="issue-header">
              <h3>
                <FaExclamationTriangle /> Compilation Errors
              </h3>
            </div>
            <div className="issue-content">
              <div className="symptoms">
                <h4>
                  <FaSearch /> Symptoms:
                </h4>
                <p>Build fails with linker errors</p>
              </div>
              <div className="solutions">
                <h4>
                  <FaCheckCircle /> Solutions:
                </h4>
                <ul className="solution-list">
                  <li>Update Rust: <code>rustup update</code></li>
                  <li>Clean build: <code>cargo clean && cargo build --release</code></li>
                  <li>
                    Install OpenCL headers (Linux: <code>sudo apt install opencl-headers</code>)
                  </li>
                  <li>Check OpenCL library paths</li>
                </ul>
              </div>
            </div>
          </div>
          <div className="issue-card medium">
            <div className="issue-header">
              <h3>
                <FaTachometerAlt /> Performance Issues
              </h3>
            </div>
            <div className="issue-content">
              <div className="symptoms">
                <h4>
                  <FaSearch /> Symptoms:
                </h4>
                <p>Slow simulation execution</p>
              </div>
              <div className="solutions">
                <h4>
                  <FaCheckCircle /> Solutions:
                </h4>
                <ul className="solution-list">
                  <li>Always use <code>cargo run --release</code></li>
                  <li>Verify GPU is being used (not CPU fallback)</li>
                  <li>Start with smaller grid sizes for testing</li>
                  <li>Monitor GPU memory usage</li>
                  <li>
                    Check GPU utilization with <code>nvidia-smi</code> or equivalent
                  </li>
                </ul>
              </div>
            </div>
          </div>
          <div className="issue-card medium">
            <div className="issue-header">
              <h3>
                <FaMemory /> Memory Errors
              </h3>
            </div>
            <div className="issue-content">
              <div className="symptoms">
                <h4>
                  <FaSearch /> Symptoms:
                </h4>
                <p>Out of memory errors with large grids</p>
              </div>
              <div className="solutions">
                <h4>
                  <FaCheckCircle /> Solutions:
                </h4>
                <ul className="solution-list">
                  <li>Reduce grid dimensions (nx, ny, nz)</li>
                  <li>Check available GPU memory</li>
                  <li>Use single precision if available</li>
                  <li>Consider distributed computing for very large problems</li>
                </ul>
              </div>
            </div>
          </div>
        </div>
        <div className="support-section">
          <h3>
            <FaLifeRing /> Getting Help
          </h3>
          <p>
            For additional support, email: <a href="mailto:gustavoverneck@gmail.com">gustavoverneck@gmail.com</a>
          </p>
          <div className="support-tips">
            <h4>
              <FaSearch /> Self-Help Tips:
            </h4>
            <ul>
              <li>
                Review the example code in <code>src/main.rs</code>
              </li>
              <li>
                Examine the solver implementation in <code>src/solver/</code>
              </li>
              <li>Run the benchmark suite to verify your setup</li>
            </ul>
          </div>
        </div>
      </section>
    </div>
  );
};

export default UsageGuide;