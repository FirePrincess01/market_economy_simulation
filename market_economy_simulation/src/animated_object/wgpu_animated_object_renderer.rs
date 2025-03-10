use wgpu_renderer::renderer::WgpuRendererInterface;

use crate::{
    animated_object::animated_model::animation::Animation,
    deferred_animation_shader::{self, DeferredAnimationShaderDraw},
};

use super::{
    animated_model::skeleton::Skeleton, animated_object_renderer::AnimatedObjectRendererResult,
    gltf_importer::GltfImporter,
};

pub struct AnimatedObject {
    pub _is_visible: bool,
    // pub update_position: bool,
    pub _x: f32,
    pub _y: f32,
    pub _z: f32,

    pub skeleton: Skeleton,
    pub animation: Animation,

    pub _instance: deferred_animation_shader::Instance,
    pub animation_uniform: deferred_animation_shader::AnimationUniform,

    mesh: deferred_animation_shader::Mesh,
}

pub struct WgpuAnimatedObjectStorage {
    pub elements: Vec<AnimatedObject>,
}

impl WgpuAnimatedObjectStorage {
    pub fn new() -> Self {
        let elements = Vec::new();

        Self { elements }
    }

    pub fn update(
        &mut self,
        renderer_interface: &mut dyn WgpuRendererInterface,
        dt: &instant::Duration,
    ) {
        for elem in &mut self.elements {
            let skeleton = &elem.skeleton;

            elem.animation.increment_time(dt);
            elem.animation
                .update_animation_uniform(skeleton, &mut elem.animation_uniform);
            elem.mesh
                .update_animation_buffer(renderer_interface.queue(), &elem.animation_uniform);
        }
    }
}

impl DeferredAnimationShaderDraw for WgpuAnimatedObjectStorage {
    fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        for elem in &self.elements {
            elem.mesh.draw(render_pass);
        }
    }
}

pub struct WgpuAnimatedObjectRenderer<'a> {
    pub storage: &'a mut WgpuAnimatedObjectStorage,

    // wgpu renderer
    pub wgpu_renderer: &'a mut dyn wgpu_renderer::renderer::WgpuRendererInterface,
    pub animation_bind_group_layout: &'a deferred_animation_shader::AnimationBindGroupLayout,
}

impl WgpuAnimatedObjectRenderer<'_> {
    pub fn create_from_glb(
        &mut self,
        glb_bin: &[u8],
    ) -> super::animated_object_renderer::AnimatedObjectRendererResult {
        let animation_data = GltfImporter::create(glb_bin);

        let skeleton = Skeleton::new(&animation_data);
        let animation_0 = Animation::new(animation_data.animations[0].clone());
        let animation_uniform = deferred_animation_shader::AnimationUniform::zero();

        let instance = deferred_animation_shader::Instance {
            position: [0.0, 20.0, 5.0],
            color: [0.5, 0.5, 0.8],
            entity: [99, 0, 0],
        };

        let mesh = deferred_animation_shader::Mesh::from_animation_data(
            self.wgpu_renderer,
            self.animation_bind_group_layout,
            &animation_data,
            &[instance],
        );

        let element = AnimatedObject {
            _is_visible: true,
            _x: 0.0,
            _y: 0.0,
            _z: 0.0,

            skeleton,
            animation: animation_0,

            _instance: instance,
            animation_uniform,

            mesh,
        };

        let render_index = self.storage.elements.len();
        self.storage.elements.push(element);

        AnimatedObjectRendererResult {
            _index: render_index,
        }
    }
}
