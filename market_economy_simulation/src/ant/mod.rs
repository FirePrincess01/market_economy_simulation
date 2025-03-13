use wgpu_renderer::wgpu_renderer::WgpuRendererInterface;

use crate::{
    deferred_color_shader::{self, DeferredShaderDraw},
    deferred_light_shader::{self, DeferredLightShaderDraw},
    geometry,
    shape::{self, MeshDataInterface},
};

pub struct Ant {
    // host data
    _circle: geometry::Circle,
    _circle_light: geometry::Circle,

    _circle_instance: deferred_color_shader::Instance,
    _circle_light_instance: deferred_light_shader::Instance,

    // gpu data
    _circle_mesh: deferred_color_shader::Mesh,
    _circle_light_mesh: deferred_light_shader::Mesh,

    // sphere
    _sphere: shape::UVSphere,
    _sphere_light: shape::UVSphere,

    _sphere_instance: deferred_color_shader::Instance,
    _sphere_light_instance: deferred_light_shader::Instance,

    // gpu data
    _sphere_mesh: deferred_color_shader::Mesh,
    _sphere_light_mesh: deferred_light_shader::Mesh,
}

impl Ant {
    pub fn new(wgpu_renderer: &mut dyn WgpuRendererInterface) -> Self {
        let light_radius = 10.0;

        let circle = geometry::Circle::new(0.15, 16);
        let circle_light = geometry::Circle::new(light_radius, 16);
        let circle_instance = deferred_color_shader::Instance {
            position: [22.0, 19.0, 5.0],
            color: [0.0, 0.0, 1.0],
            entity: [34, 0, 0],
        };
        let circle_light_instance = deferred_light_shader::Instance {
            position: [
                circle_instance.position[0],
                circle_instance.position[1],
                circle_instance.position[2],
            ],
            light_color: [0.1, 1.0, 0.0],
            radius: 0.0,
            linear: 0.0,
            quadratic: 0.0,
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

        // sphere
        let sphere = shape::UVSphere::new(5.0, 10);
        let sphere_light = shape::UVSphere::new(5.0, 10);
        let sphere_instance = deferred_color_shader::Instance {
            position: [0.0, 19.0, 5.0],
            color: [0.0, 0.0, 1.0],
            entity: [34, 0, 0],
        };
        let sphere_light_instance = deferred_light_shader::Instance {
            position: [
                sphere_instance.position[0],
                sphere_instance.position[1],
                sphere_instance.position[2],
            ],
            light_color: [0.1, 1.0, 0.0],
            radius: 0.0,
            linear: 0.0,
            quadratic: 0.0,
        };

        let sphere_mesh = deferred_color_shader::Mesh::from_shape(
            wgpu_renderer.device(),
            sphere.data(),
            &[sphere_instance],
        );
        let sphere_light_mesh = deferred_light_shader::Mesh::from_shape(
            wgpu_renderer.device(),
            sphere_light.data(),
            &[sphere_light_instance],
        );

        Self {
            _circle: circle,
            _circle_light: circle_light,
            _circle_instance: circle_instance,
            _circle_light_instance: circle_light_instance,
            _circle_mesh: circle_mesh,
            _circle_light_mesh: circle_light_mesh,

            _sphere: sphere,
            _sphere_light: sphere_light,
            _sphere_instance: sphere_instance,
            _sphere_light_instance: sphere_light_instance,
            _sphere_mesh: sphere_mesh,
            _sphere_light_mesh: sphere_light_mesh,
        }
    }
}

impl DeferredShaderDraw for Ant {
    fn draw<'a>(&'a self, _render_pass: &mut wgpu::RenderPass<'a>) {
        // self.circle_mesh.draw(render_pass);
        // self.sphere_mesh.draw(render_pass);
    }
}

impl DeferredLightShaderDraw for Ant {
    fn draw_lights<'a>(&'a self, _render_pass: &mut wgpu::RenderPass<'a>) {
        // self.circle_light_mesh.draw(render_pass);
        // self.sphere_light_mesh.draw_lights(render_pass);
    }
}
