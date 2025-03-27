# ▶️ Usage Guide

This guide walks you through how to compile, run, and visualize LatteLBM simulations.

---

## ⚙️ Build the Project

Make sure you have:
- `Rust` (https://rust-lang.org)
- An OpenCL-compatible GPU and drivers

```bash
git clone https://github.com/your-username/latte-lbm.git
cd latte-lbm
cargo build --release
```

## 🧪 Running Simulations
Each example is a separate `main()` function in `main.rs`. You can swap between them by commenting/uncommenting.

```bash
cargo run
```

### Run an example
To run an example, uncomment the others.

## 🧾 Output Format
Results are saved in `output/data_XXXX.csv`. Each file contains:

``` csv
x, y, z, rho, ux, uy, uz, v, q
```
You can open this in Python, Paraview, or other tools for analysis.

## 🧊 Boundary Flags

| Flag         | Description                          |
|--------------|--------------------------------------|
| `FLAG_FLUID` | Normal fluid node (default)         |
| `FLAG_SOLID` | Bounce-back (no-slip wall)          |
| `FLAG_EQ`    | Equilibrium (fixed velocity/density)|

You set these in `lbm.set_conditions(...)` closures in each example.