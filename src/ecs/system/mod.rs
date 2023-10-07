

mod renderer;

pub use renderer::IRenderer;

use super::World;




pub fn draw_meshes(world: &World, renderer: &mut impl IRenderer) -> Result<(), wgpu::SurfaceError>
{
    renderer.render(&world.meshes)

}