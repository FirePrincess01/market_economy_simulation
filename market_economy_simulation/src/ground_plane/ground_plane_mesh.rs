//! Mesh of the ground field

use wgpu_renderer::wgpu_renderer::WgpuRendererInterface;

use crate::{
    deferred_color_shader::{self, DeferredShaderDraw},
    deferred_light_shader::{self, DeferredLightShaderDraw},
    geometry,
    ground_plane::GroundResource,
};

use super::GroundPlane;

pub struct GroundPlaneMesh {
    mesh: deferred_color_shader::Mesh,
    mesh_light: deferred_light_shader::Mesh,
}

impl GroundPlaneMesh {
    pub fn new(
        wgpu_renderer: &mut dyn WgpuRendererInterface,
        ground_plane: &GroundPlane,
        scale: f32,
    ) -> Self {
        // ground plane mesh
        let quad_size = 1.0 * scale;
        let quad = geometry::Quad::new(quad_size);

        let mut instances: Vec<deferred_color_shader::Instance> =
            Vec::with_capacity(ground_plane.size());

        for y in 0..ground_plane.height() {
            for x in 0..ground_plane.width() {
                let field = ground_plane.get(y, x);

                let color = match field.resource {
                    GroundResource::None => [0.2, 0.2, 0.2],
                    GroundResource::Red => [0.3, 0.0, 0.0],
                    GroundResource::Green => [0.0, 0.3, 0.0],
                    GroundResource::Blue => [0.0, 0.0, 0.3],
                };

                let instance = deferred_color_shader::Instance {
                    position: [quad_size * x as f32, quad_size * y as f32, 0.0],
                    color,
                    entity: [field.entity_index as u32, 0, 0],
                };
                instances.push(instance);
            }
        }

        let mesh = deferred_color_shader::Mesh::new(
            wgpu_renderer.device(),
            &quad.deferred_vertices,
            &quad.indices,
            &instances,
        );

        // ground plane light
        let ground_plane_light_quad = geometry::Quad::new(ground_plane.width() as f32 * quad_size);

        const INSTANCES: &[deferred_light_shader::Instance] = &[deferred_light_shader::Instance {
            position: [0.0, 0.0, 1.0],
            light_color: [0.0, 0.4, 0.0],
            radius: 0.0,
            linear: 0.0,
            quadratic: 0.0,
        }];

        let mesh_light = deferred_light_shader::Mesh::new(
            wgpu_renderer.device(),
            &ground_plane_light_quad.vertices,
            &ground_plane_light_quad.indices,
            INSTANCES,
        );

        Self { mesh, mesh_light }
    }
}

impl DeferredShaderDraw for GroundPlaneMesh {
    fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        self.mesh.draw(render_pass);
    }
}

impl DeferredLightShaderDraw for GroundPlaneMesh {
    fn draw_lights<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        self.mesh_light.draw_lights(render_pass);
    }
}
