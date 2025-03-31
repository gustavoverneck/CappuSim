#![allow(non_snake_case)]
#![allow(clippy::upper_case_acronyms)]

use super::lbm::LBM;

impl LBM {
    pub fn velocity_to_u(&self) -> Vec<f32> {
        self.velocity
            .iter()
            .flat_map(|v| vec![v.x, v.y, v.z])
            .collect()
    }

    // pub fn u_to_velocity(&mut self, flat_velocity_data: Vec<f32>) {
    //     self.velocity = flat_velocity_data
    //         .chunks(3)
    //         .map(|chunk| Velocity {
    //             x: chunk[0],
    //             y: chunk[1],
    //             z: chunk[2],
    //         })
    //         .collect();
    // }
}

pub fn n_from_xyz(x: &usize, y: &usize, z: &usize, Nx: &usize, Ny: &usize) -> usize {
    z * (Nx * Ny) + y * Nx + x
}
pub fn xyz_from_n(n: &usize, Nx: &usize, Ny: &usize) -> (usize, usize, usize) {
    let x = *n % Nx;
    let y = (*n / Nx) % Ny;
    let z = *n / (Ny * Nx);
    (x, y, z)
}
