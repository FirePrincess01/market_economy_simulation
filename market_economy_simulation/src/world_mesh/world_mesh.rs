use wgpu_renderer::renderer::WgpuRendererInterface;

use crate::{ground_plane::GroundPlaneMesh, ecs2::World, deferred_color_shader::DeferredShaderDraw, deferred_light_shader::DeferredLightShaderDraw};



pub struct WorldMesh {
    ground_plane_mesh: GroundPlaneMesh,
}

impl WorldMesh {
    pub fn new(wgpu_renderer: &mut impl WgpuRendererInterface, 
        world: &World,
    ) -> Self
    {
        // ground plane mesh
        let ground_plane_mesh = GroundPlaneMesh::new(
            wgpu_renderer, 
            &world.components.ground_plane);

        Self {
            ground_plane_mesh,
        }
    }
}


impl DeferredShaderDraw for WorldMesh {
    fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        self.ground_plane_mesh.draw(render_pass);
    }
}

impl DeferredLightShaderDraw for WorldMesh {
    fn draw_lights<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        self.ground_plane_mesh.draw_lights(render_pass);
    }
}