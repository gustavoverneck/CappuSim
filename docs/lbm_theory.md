# 🧠 Lattice Boltzmann Method (LBM) – Theory

LatteLab simulates fluid dynamics using the Lattice Boltzmann Method (LBM), a mesoscopic approach based on particle distributions.

---

## 🔁 LBM Update Cycle

Each time step performs the following operations:

1. **Collision** – relaxes the distribution toward equilibrium:

   `f[q] ← f[q] - ω (f[q] - f_eq[q])`

3. **Streaming** – shifts each distribution `f[q]` to the neighboring cell along its direction `c[q]`.

4. **Swap** – replaces the old distribution with the streamed one (`f_new → f`) for the next iteration.

---

## ⚙️ Collision and Equilibrium

The equilibrium distribution is defined by a second-order approximation of the Maxwell-Boltzmann distribution:
f_eq[q] = ρ * w[q] * (1 + 3(c·u) + 4.5(c·u)^2 - 1.5(u·u))


Where:

- `ρ` is the local density
- `u` is the macroscopic velocity
- `c[q]` is the discrete lattice velocity in direction `q`
- `w[q]` is the corresponding lattice weight

---

## 📦 Macroscopic Quantities

These are derived from the distribution functions as:

- **Density**:
  `ρ = Σ f[q]`


- **Velocity**:
`u = (1 / ρ) * Σ f[q] * c[q]`


These calculations are performed in the `collision_kernel` before updating the distributions.

---

## 🧱 Boundary Conditions

LatteLBM supports three types of boundary flags per node:

| Flag       | Description                                  |
|------------|----------------------------------------------|
| `FLUID`    | Normal LBM evolution (collision + streaming) |
| `SOLID`    | No-slip wall using bounce-back condition     |
| `EQ`       | Prescribed equilibrium (used for inlets, lids, etc.) |

### Bounce-back (No-slip Wall)

If a distribution tries to stream into a solid node, it is reflected back in the opposite direction:
  f_new[n][opposite[q]] = f[n][q]
  
This effectively enforces zero velocity at the wall.

---

## 📏 Viscosity and Relaxation

The relaxation parameter `ω` controls viscosity:

`ν = (1 / ω - 0.5) / 3`

Guidelines:

- `ω ∈ (0, 2)`
- Higher `ω` → lower viscosity
- Low viscosity → high Reynolds number flows

---

## 🔬 Vorticity and Q-Criterion

LatteLBM computes:

- **Vorticity (‖∇ × u‖)**: local fluid rotation
- **Q-Criterion**: vortex identification

The Q-criterion is defined as:
`Q = 0.5 * (‖W‖² - ‖S‖²)`


Where `W` is the rotation tensor and `S` is the strain-rate tensor.

These metrics are computed on the host (in Rust) after the simulation step.

---

## 🔢 Lattice Models

LatteLBM supports multiple velocity models (lattices), including:

- `D2Q9` — standard 2D model
- `D3Q7`, `D3Q15`, `D3Q19`, `D3Q27` — for 3D

All kernel data (velocity vectors `c[q]`, weights `w[q]`, and opposites) is loaded dynamically based on the selected model at initialization.

---

## 📚 References

- Krüger, T. et al. *The Lattice Boltzmann Method* (Springer, 2017)
- Succi, S. *The Lattice Boltzmann Equation for Fluid Dynamics and Beyond* (Oxford, 2001)
- Mohamad, A. A. *Lattice Boltzmann Method: Fundamentals and Engineering Applications with Computer Codes* (Springer, 2011)
- FluidX3D and other open-source solvers.
