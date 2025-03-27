#gh-dark-mode-only
# 🍵 LatteLab

LatteLab is a GPU-accelerated, OpenCL-based Lattice Boltzmann Method (LBM) simulator written in Rust.

It supports 2D and 3D simulations with multiple lattice models, dynamic boundary conditions, and output for visualization or analysis.

---

## 📖 Documentation

- [▶️ Usage Guide](usage_guide.md)
- [🧠 LBM Theory](lbm_theory.md)
- [📚 LatteLab Documentation](lattelab_doc.md)

---

## ✨ Features

- ⚙️ Models: D2Q9, D3Q7, D3Q15, D3Q19, D3Q27
- 🧱 Solid / Fluid / Equilibrium boundaries
- 🚀 GPU acceleration (OpenCL)
- 📁 Output in CSV for plotting

---

## 🧪 Examples

| Case                    | File                        |
|-------------------------|-----------------------------|
| Lid-Driven Cavity       | `liddriven_cavity.rs`       |
| Poiseuille Flow         | `poiseuille.rs`             |
| Taylor-Green Vortex     | `taylor_green_vortex.rs`    |
| Von Kármán Vortex Street| `von_karman_vortex.rs`      |
