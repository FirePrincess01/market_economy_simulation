//! Contains the meshes for all objects of the world

use wgpu_renderer::wgpu_renderer::WgpuRendererInterface;

use crate::{
    base_factory::BaseFactoryMesh, deferred_color_shader::DeferredShaderDraw,
    deferred_light_shader::DeferredLightShaderDraw, ecs2::World, ground_plane::GroundPlaneMesh,
};

pub struct WorldMesh {
    ground_plane_mesh: GroundPlaneMesh,
    base_factory_mesh: BaseFactoryMesh,
}

impl WorldMesh {
    pub fn new(wgpu_renderer: &mut dyn WgpuRendererInterface, world: &World) -> Self {
        let scale = 10.0;

        // ground plane mesh
        let ground_plane_mesh = GroundPlaneMesh::new(wgpu_renderer, &world.ground_plane, scale);

        // base factory mesh
        let base_factory_mesh = BaseFactoryMesh::new(wgpu_renderer, &world.base_factory);

        Self {
            ground_plane_mesh,
            base_factory_mesh,
        }
    }
}

impl DeferredShaderDraw for WorldMesh {
    fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        self.ground_plane_mesh.draw(render_pass);
        self.base_factory_mesh.draw(render_pass);
    }
}

impl DeferredLightShaderDraw for WorldMesh {
    fn draw_lights<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        self.ground_plane_mesh.draw_lights(render_pass);
        self.base_factory_mesh.draw_lights(render_pass);
    }
}
