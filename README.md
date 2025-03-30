
# CappuSim
https://github.com/gustavoverneck/CappuSim/blob/main/README.md
![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white) ![License: GNU v3](https://img.shields.io/badge/License-GNU%20v3-blue.svg)  

<p align="center">
  <img src="https://github.com/user-attachments/assets/fc920e2a-e6f2-48cb-a463-5ead9f9dd9b1" alt="CappuSim banner"/>
</p>

## Description  
**CappuSim** is a Rust software for computational fluid dynamics (CFD) simulations based on the **Lattice Boltzmann Method (LBM)**. This project is part of my Master's Degree in Physics and aims to provide an efficient and flexible tool for LBM simulations, leveraging [opencl](https://www.khronos.org/opencl/) for GPU acceleration.  

The library is **Open Source**, and contributions are highly encouraged. Whether you're an academic researcher or just curious about fluid dynamics, you're welcome to contribute!  

## Current State and road map
**CappuSim** is currently in **alpha** and appears **functional**, but no yet validated. A validated **beta version** will be available soon. Stay tuned!

The possibilities for the future include:
- Temperature
- Forces
- Improve kernels handling
- GUI
- Python interface
- Multiphase fluid
- Moving boundaries
- Plasma extension

## Installation  
To use CappuSim, you will need to install specific versions of **cargo** and **rust**: 

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
To run it, clone the directory and execute it using cargo:

```bash
git clone https://github.com/gustavoverneck/CappuSim
cd CappuSim
cargo build --release
cargo run
```

A package installation will be available in future releases.

## Documentation
The official documentation is under development and can be accessed in [Documentation](https://gustavoverneck.github.io/CappuSim/).  

## License  
This project is licensed under the **GNU General Public License v3.0**. For more details, see the [LICENSE](LICENSE) file.  

## Contributing  
Contributions are welcome! If you want to contribute:  
1. Fork the repository  
2. Create a new branch (`git checkout -b feature/my-feature`)  
3. Commit your changes (`git commit -m 'Add new feature'`)  
4. Push to the branch (`git push origin feature/my-feature`)  
5. Open a pull request  
