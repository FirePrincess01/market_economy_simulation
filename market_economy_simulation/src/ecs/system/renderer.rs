

use super::super::component::ColoredMesh;



pub trait IRenderer {
    fn render(&mut self, meshes: &[ColoredMesh]) -> Result<(), wgpu::SurfaceError>;
}