

pub mod uv_sphere;


pub struct MeshData {
    pub positions: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub indices: Vec<u16>,
}

pub trait MeshDataInterface {
    fn get_mesh_data(&self) -> &MeshData;
}