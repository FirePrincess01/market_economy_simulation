//! Contains the device buffers to render an object with this shader
//!

use super::animation_uniform_buffer::AnimationUniformBuffer;
use super::AnimationBindGroupLayout;
use super::AnimationUniform;
use super::DeferredAnimationShaderDraw;
use super::Instance;
use super::Vertex;

use super::IndexBuffer;

use super::InstanceBuffer;
use super::VertexBuffer;

/// A general purpose shader using vertices, colors and an instance matrix
pub struct Mesh {
    vertex_buffer: VertexBuffer<Vertex>,
    animation_buffer: AnimationUniformBuffer,
    index_buffer: IndexBuffer,
    instance_buffer: InstanceBuffer,
    max_instances: u32,
    nr_instances: u32,
}

#[allow(dead_code)]
impl Mesh {
    pub fn new<'a>(
        wgpu_renderer: &'a mut dyn wgpu_renderer::renderer::WgpuRendererInterface,
        animation_bind_group_layout: &AnimationBindGroupLayout,
        vertices: &[Vertex],
        animation: &AnimationUniform,
        indices: &[u32],
        instances: &[Instance],
    ) -> Self {
        let vertex_buffer = VertexBuffer::new(wgpu_renderer.device(), vertices);

        let mut animation_buffer =
            AnimationUniformBuffer::new(wgpu_renderer.device(), animation_bind_group_layout);
        animation_buffer.update(wgpu_renderer.queue(), &animation);

        let index_buffer = IndexBuffer::new(wgpu_renderer.device(), indices);

        let instance_buffer = InstanceBuffer::new(wgpu_renderer.device(), &instances);

        let max_instances = instances.len() as u32;
        let nr_instances = instances.len() as u32;

        Self {
            vertex_buffer,
            animation_buffer,
            index_buffer,
            instance_buffer,
            max_instances,
            nr_instances,
        }
    }

    pub fn update_vertex_buffer(&mut self, queue: &wgpu::Queue, vertices: &[Vertex]) {
        self.vertex_buffer.update(queue, vertices);
    }

    pub fn update_animation_buffer(
        &mut self,
        queue: &wgpu::Queue,
        animation_uniform: &AnimationUniform,
    ) {
        self.animation_buffer.update(queue, animation_uniform);
    }

    pub fn update_instance_buffer(&mut self, queue: &wgpu::Queue, instances: &[Instance]) {
        self.instance_buffer.update(queue, instances);
        self.nr_instances = u32::min(instances.len() as u32, self.max_instances);
    }
}

impl DeferredAnimationShaderDraw for Mesh {
    fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        self.vertex_buffer.bind(render_pass);
        self.animation_buffer.bind(render_pass);
        self.index_buffer.bind(render_pass);
        self.instance_buffer.bind(render_pass);

        render_pass.draw_indexed(0..self.index_buffer.size(), 0, 0..self.nr_instances);
    }
}
