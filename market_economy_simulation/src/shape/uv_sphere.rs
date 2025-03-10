use cgmath::Zero;

use super::{MeshData, MeshDataInterface};

pub struct UVSphere {
    mesh_data: MeshData,
}

impl UVSphere {
    pub fn new(radius: f32, n: usize) -> Self {

        let mut grid: Vec<cgmath::Vector3<f32>> = Vec::new();
        grid.resize(n*n, cgmath::Vector3::zero());

        for j in 0..n {
            let alpha = -90.0 + 180.0 / (n) as f32 * j as f32;  
            let current_radius = radius * alpha.sin();

            let z = -radius + j as f32 * (2.0 * radius / n as f32);

            for i in 0..n {
                let beta = 360.0 / n as f32 * i as f32;

                let x = current_radius * beta.cos();
                let y = current_radius * beta.sin();

                grid[j * n + i] = cgmath::Vector3::new(x, y, z);
            }
        }




        let positions: Vec<[f32; 3]> = Vec::new();
        let normals: Vec<[f32; 3]> = Vec::new();
        let indices: Vec<u16> = Vec::new();

        Self {
            mesh_data: MeshData {
                positions,
                normals,
                indices,
            },
        }
    }
}

impl MeshDataInterface for UVSphere {
    fn get_mesh_data(&self) -> &MeshData {
        &self.mesh_data
    }
}
