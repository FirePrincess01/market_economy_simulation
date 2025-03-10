

pub mod uv_sphere;
pub use uv_sphere::UVSphere;


pub struct MeshData {
    pub positions: Vec<cgmath::Vector3<f32>>,
    pub normals: Vec<cgmath::Vector3<f32>>,
    pub indices: Vec<u16>,
}

pub trait MeshDataInterface {
    fn get_mesh_data(&self) -> &MeshData;
}