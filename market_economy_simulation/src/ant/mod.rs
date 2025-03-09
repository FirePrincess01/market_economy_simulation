use wgpu_renderer::renderer::WgpuRendererInterface;

use crate::{deferred_color_shader::{self, DeferredShaderDraw}, deferred_light_shader::{self, DeferredLightShaderDraw}, geometry};

pub struct Ant {
    // host data
    circle: geometry::Circle,
    circle_light: geometry::Circle,

    circle_instance: deferred_color_shader::Instance,
    circle_light_instance: deferred_light_shader::Instance,

    // gpu data
    circle_mesh: deferred_color_shader::Mesh,
    circle_light_mesh: deferred_light_shader::Mesh,
}

impl Ant {
    pub fn new(wgpu_renderer: &mut dyn WgpuRendererInterface) -> Self {
        let light_radius = 10.0;

        let circle = geometry::Circle::new(0.15, 16);
        let circle_light = geometry::Circle::new(light_radius, 16);
        let circle_instance = deferred_color_shader::Instance{
            position: [22.0, 19.0, 5.0],
            color: [0.0, 0.0, 1.0],
            entity: [34, 0, 0],
        };
        let circle_light_instance = deferred_light_shader::Instance{
            position: circle_instance.position,
            intensity: [0.1, 1.0, 0.0],
        };

        let circle_mesh = deferred_color_shader::Mesh::new(
            wgpu_renderer.device(),
            &circle.deferred_vertices,
            &circle.indices,
            &[circle_instance],
        );

        let circle_light_mesh = deferred_light_shader::Mesh::new(
            wgpu_renderer.device(),
            &circle_light.vertices,
            &circle_light.indices,
            &[circle_light_instance],
        );

        Self {
            circle,
            circle_light,
            circle_instance,
            circle_light_instance,
            circle_mesh,
            circle_light_mesh,
        }
    }
}

impl DeferredShaderDraw for Ant {
    fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        self.circle_mesh.draw(render_pass);
    }
}

impl DeferredLightShaderDraw for Ant {
    fn draw_lights<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        self.circle_light_mesh.draw(render_pass);
    }
}
