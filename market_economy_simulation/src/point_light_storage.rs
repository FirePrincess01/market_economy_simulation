//! Manages the data on the gpu of all the point lights

use market_economy_simulation_server::point_lights::Attenuation;
use wgpu_renderer::{shape::{self, MeshDataInterface}, wgpu_renderer::WgpuRendererInterface};

use crate::{
    deferred_light_shader::{self, DeferredLightShaderDraw},
    deferred_light_sphere_shader::DeferredLightSphereShaderDraw,
};

pub struct PointLightStorage {
    light: deferred_light_shader::Mesh,

    instances: Vec<deferred_light_shader::Instance>,
    is_active: Vec<bool>,
    active_instances: Vec<deferred_light_shader::Instance>,

    render_debug_sphere: bool,

    max_instances: usize,
    nr_instances: usize,
}

impl PointLightStorage {
    pub fn new(
        renderer: &mut dyn WgpuRendererInterface,
        max_instances: usize,
        dbg_point_lights: bool,
    ) -> Self {
        let sphere_raw = shape::UVSphere::new(1.0, 8);
        let sphere = sphere_raw.triangles();

        let mut instances: Vec<deferred_light_shader::Instance> = Vec::new();
        instances.resize(max_instances, deferred_light_shader::Instance::new());

        let mut is_active = Vec::new();
        is_active.resize(max_instances, false);

        let active_instances = instances.clone();

        let light = deferred_light_shader::Mesh::from_shape(renderer.device(), sphere, &instances);

        Self {
            light,
            instances,
            is_active,
            active_instances,
            render_debug_sphere: dbg_point_lights,
            max_instances,
            nr_instances: 0,
        }
    }

    /// Copies the data from the host to the device
    pub fn update(&mut self, renderer: &mut dyn WgpuRendererInterface) {
        let instances = &self.instances;
        let is_active = &self.is_active;
        let active_instances = &mut self.active_instances;
        let max_instances = self.max_instances;
        let nr_instances = self.nr_instances;

        // sanity check
        assert_eq!(instances.len(), max_instances);
        assert_eq!(is_active.len(), max_instances);
        assert_eq!(active_instances.len(), max_instances);
        assert!(nr_instances <= max_instances);

        let mut j = 0;
        for i in 0..nr_instances {
            if is_active[i] {
                active_instances[j] = instances[i];
                j += 1;
            }
        }

        self.light
            .update_instance_buffer(renderer.queue(), &active_instances[0..j]);
    }
}

#[allow(unused)]
pub trait PointLightInterface {
    fn add_light(
        &mut self,
        position: cgmath::Vector3<f32>,
        color: cgmath::Vector3<f32>,
        attenuation: Attenuation,
    ) -> Option<PointLightIndex>;

    fn set_light(
        &mut self,
        index: PointLightIndex,
        position: cgmath::Vector3<f32>,
        color: cgmath::Vector3<f32>,
        attenuation: Attenuation,
    );

    fn set_position(&mut self, index: &PointLightIndex, position: cgmath::Vector3<f32>);
    fn set_color(&mut self, index: &PointLightIndex, color: cgmath::Vector3<f32>);
    fn set_active(&mut self, index: &PointLightIndex, is_active: bool);
}

impl PointLightInterface for PointLightStorage {
    fn add_light(
        &mut self,
        position: cgmath::Vector3<f32>,
        color: cgmath::Vector3<f32>,
        attenuation: Attenuation,
    ) -> Option<PointLightIndex> {
        let max_instances = self.max_instances;
        let nr_instances = &mut self.nr_instances;
        let instances = &mut self.instances;
        let is_active = &mut self.is_active;

        if *nr_instances >= max_instances {
            return None;
        }

        let attenuation = &ATTENUATION_DATA[attenuation as usize];
        let radius = calculate_volume_radius(&cgmath::Vector3::new(1.0, 1.0, 1.0), attenuation);

        instances[*nr_instances] = deferred_light_shader::Instance {
            position: position.into(),
            light_color: color.into(),
            radius: radius as f32,
            linear: attenuation.linear,
            quadratic: attenuation.quadratic,
        };
        is_active[*nr_instances] = true;

        let res = PointLightIndex {
            instance_index: *nr_instances,
        };

        *nr_instances += 1;

        Some(res)
    }

    fn set_light(
        &mut self,
        index: PointLightIndex,
        position: cgmath::Vector3<f32>,
        color: cgmath::Vector3<f32>,
        attenuation: Attenuation,
    ) {
        let index = index.instance_index;

        if index >= self.max_instances {
            // index out of bounds
            return;
        }

        if index >= self.nr_instances {
            self.nr_instances = index + 1;
        }

        let attenuation = &ATTENUATION_DATA[attenuation as usize];
        let radius = calculate_volume_radius(&cgmath::Vector3::new(1.0, 1.0, 1.0), attenuation);

        self.instances[index] = deferred_light_shader::Instance {
            position: position.into(),
            light_color: color.into(),
            radius: radius as f32,
            linear: attenuation.linear,
            quadratic: attenuation.quadratic,
        };
        self.is_active[index] = true;
    }

    fn set_position(&mut self, index: &PointLightIndex, position: cgmath::Vector3<f32>) {
        let i = index.instance_index;

        self.instances[i].position = position.into();
    }

    fn set_color(&mut self, index: &PointLightIndex, color: cgmath::Vector3<f32>) {
        let i = index.instance_index;

        self.instances[i].position = color.into();
    }

    fn set_active(&mut self, index: &PointLightIndex, is_active: bool) {
        let i = index.instance_index;

        self.is_active[i] = is_active;
    }
}

impl DeferredLightShaderDraw for PointLightStorage {
    fn draw_lights<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        self.light.draw_lights(render_pass);
    }
}

impl DeferredLightSphereShaderDraw for PointLightStorage {
    fn draw_sphere<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        if self.render_debug_sphere {
            self.light.draw_sphere(render_pass);
        }
    }
}

pub struct PointLightIndex {
    pub instance_index: usize,
}

struct AttenuationData {
    constant: f32,
    linear: f32,
    quadratic: f32,
}

const ATTENUATION_DATA: [AttenuationData; 12] = [
    AttenuationData {
        constant: 1.0,
        linear: 0.7,
        quadratic: 1.8,
    },
    AttenuationData {
        constant: 1.0,
        linear: 0.35,
        quadratic: 0.44,
    },
    AttenuationData {
        constant: 1.0,
        linear: 0.22,
        quadratic: 0.20,
    },
    AttenuationData {
        constant: 1.0,
        linear: 0.14,
        quadratic: 0.07,
    },
    AttenuationData {
        constant: 1.0,
        linear: 0.09,
        quadratic: 0.032,
    },
    AttenuationData {
        constant: 1.0,
        linear: 0.07,
        quadratic: 0.017,
    },
    AttenuationData {
        constant: 1.0,
        linear: 0.045,
        quadratic: 0.075,
    },
    AttenuationData {
        constant: 1.0,
        linear: 0.027,
        quadratic: 0.028,
    },
    AttenuationData {
        constant: 1.0,
        linear: 0.022,
        quadratic: 0.019,
    },
    AttenuationData {
        constant: 1.0,
        linear: 0.014,
        quadratic: 0.0007,
    },
    AttenuationData {
        constant: 1.0,
        linear: 0.07,
        quadratic: 0.0002,
    },
    AttenuationData {
        constant: 1.0,
        linear: 0.0014,
        quadratic: 0.000007,
    },
];

fn calculate_volume_radius(
    light_color: &cgmath::Vector3<f32>,
    attenuation: &AttenuationData,
) -> f64 {
    // https://learnopengl.com/Advanced-Lighting/Deferred-Shading

    // Distance 	Constant 	Linear 	Quadratic
    // 7        	1.0     	0.7 	1.8
    // 13        	1.0     	0.35 	0.44
    // 20        	1.0     	0.22 	0.20
    // 32        	1.0     	0.14 	0.07
    // 50        	1.0     	0.09 	0.032
    // 65        	1.0     	0.07 	0.017
    // 100        	1.0     	0.045 	0.0075
    // 160        	1.0     	0.027 	0.0028
    // 200        	1.0     	0.022 	0.0019
    // 325        	1.0     	0.014 	0.0007
    // 600        	1.0     	0.007 	0.0002
    // 3250        	1.0     	0.0014 	0.000007

    let constant = attenuation.constant as f64;
    let linear = attenuation.linear as f64;
    let quadratic = attenuation.quadratic as f64;

    let light_max = light_color.x.max(light_color.y).max(light_color.z) as f64;

    // let attenuation = 256.0 / 5.0;
    let attenuation = 256.0 / 25.0;

    (-linear + (linear * linear - 4.0 * quadratic * (constant - attenuation * light_max)).sqrt())
        / (2.0 * quadratic)
}
