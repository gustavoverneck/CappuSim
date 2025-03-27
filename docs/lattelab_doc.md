# 🍵 LatteLab

**LatteLab** is a GPU-accelerated, Rust-based framework for simulating fluid dynamics using the **Lattice Boltzmann Method (LBM)**. It is designed to be modular, readable, and easy to extend for research and academic applications in computational fluid dynamics (CFD).

---

## 📖 Documentation

- [▶️ Usage Guide](usage.md)
- [🧠 LBM Theory](theory.md)

---

## ✨ Features

- Supports multiple lattice models: `D2Q9`, `D3Q7`, `D3Q15`, `D3Q19`, `D3Q27`
- Cell types: `FLUID`, `SOLID`, `EQ` (for fixed conditions)
- Boundary conditions: bounce-back (no-slip), equilibrium (e.g. inlets, moving lids)
- Vorticity and Q-criterion calculation
- `.csv` export for visualization in Python, Paraview, etc.
- 2D and 3D simulation support
- OpenCL acceleration on GPU or CPU
- Generic architecture decoupling numerics from case logic

---

## 🧪 Available Test Cases

You can run a case by editing `main.rs` and switching the activated example.

| Case Name                 | File                          | Description                                |
|--------------------------|-------------------------------|--------------------------------------------|
| Lid-Driven Cavity        | `liddriven_cavity.rs`         | Recirculation driven by moving top wall    |
| Poiseuille Flow          | `poiseuille.rs`               | Pressure-driven channel flow               |
| Taylor-Green Vortex      | `taylor_green_vortex.rs`      | Classical test for viscous dissipation     |
| Von Kármán Vortex Street | `von_karman_vortex.rs`        | Periodic vortex shedding from a cylinder   |

---

## 🛠️ Project Structure

- `main.rs` – entry point for simulations
- `lbm.rs` – solver logic and OpenCL coordination
- `kernels.rs` – OpenCL kernels (collision, streaming, etc.)
- `*.rs` – individual test cases (Poiseuille, cavity, etc.)

---

## 🚀 Requirements

- [Rust](https://rust-lang.org) (with `cargo`)
- OpenCL-compatible GPU or CPU
- Linux, macOS, or Windows

---

## 🤝 Contributing

Contributions are welcome!

- Open issues for bugs, ideas, or requests
- Submit PRs with new test cases, improvements, or kernel enhancements
- Help improve documentation for others to learn and use

---

## 📜 License

GNU General Public License v3.0 © 2025 – Made with ☕, ❤️, and 🧠
