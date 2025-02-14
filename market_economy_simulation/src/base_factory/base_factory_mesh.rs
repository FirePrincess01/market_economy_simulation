//! Mesh of the base factory

use wgpu_renderer::renderer::WgpuRendererInterface;

use crate::{deferred_color_shader::{self, DeferredShaderDraw}, deferred_light_shader::DeferredLightShaderDraw, geometry};

use super::BaseFactory;

pub struct BaseFactoryMesh {
    mesh: deferred_color_shader::Mesh,
    // mesh_light: deferred_light_shader::Mesh,
}

impl BaseFactoryMesh {
    pub fn new(wgpu_renderer: &mut dyn WgpuRendererInterface, 
        base_factory: &BaseFactory) -> Self 
    {
        let scale = 10.0;

        // factory mesh
        let frame = geometry::Frame::new(scale);

        let location = base_factory.location();
        let instance = deferred_color_shader::Instance{
            position: [location.x as f32 * scale, location.y as f32 * scale, 0.0],
            color: [0.5, 0.5, 0.5],
            entity: [69, 0, 0],
        };


        let mesh = deferred_color_shader::Mesh::new(wgpu_renderer.device(), 
            &frame.deferred_vertices, 
            &frame.indices, 
            &[instance]);

        // // ground plane light
        // let ground_plane_light_quad = geometry::Quad::new(ground_plane.width() as f32 * quad_size);

        // const INSTANCES: &[deferred_light_shader::Instance] = &[ 
        //     deferred_light_shader::Instance{
        //         position: [0.0, 0.0, 0.0],
        //         intensity: [0.0, 0.4, 0.0],
        //     },
        // ];

        // let mesh_light = deferred_light_shader::Mesh::new(wgpu_renderer.device(), 
        // &ground_plane_light_quad.vertices, 
        // &ground_plane_light_quad.indices, 
        // &INSTANCES);

        Self {
            mesh,
            // mesh_light,
        }
    }
}

impl DeferredShaderDraw for BaseFactoryMesh {
    fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        self.mesh.draw(render_pass);
    }
}

impl DeferredLightShaderDraw for BaseFactoryMesh {
    fn draw_lights<'a>(&'a self, _render_pass: &mut wgpu::RenderPass<'a>) {
        // self.mesh_light.draw(render_pass);
    }
}