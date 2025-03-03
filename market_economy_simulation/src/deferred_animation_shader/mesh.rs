//! Contains the device buffers to render an object with this shader
//!

use gltf::mesh::util::weights;

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
    instance_buffer: InstanceBuffer,
    max_instances: u32,
    nr_instances: u32,
    nr_vertices: u32,
}

#[allow(dead_code)]
impl Mesh {
    pub fn new(
        wgpu_renderer: &mut dyn wgpu_renderer::renderer::WgpuRendererInterface,
        animation_bind_group_layout: &AnimationBindGroupLayout,
        vertices: &[Vertex],
        animation: &AnimationUniform,
        instances: &[Instance],
    ) -> Self {
        let vertex_buffer = VertexBuffer::new(wgpu_renderer.device(), vertices);

        let mut animation_buffer =
            AnimationUniformBuffer::new(wgpu_renderer.device(), animation_bind_group_layout);
        animation_buffer.update(wgpu_renderer.queue(), animation);

        let instance_buffer = InstanceBuffer::new(wgpu_renderer.device(), instances);

        let max_instances = instances.len() as u32;
        let nr_instances = instances.len() as u32;
        let nr_vertices = vertices.len() as u32;

        Self {
            vertex_buffer,
            animation_buffer,
            instance_buffer,
            max_instances,
            nr_instances,
            nr_vertices,
        }
    }

    pub fn from_animation_data(
        wgpu_renderer: &mut dyn wgpu_renderer::renderer::WgpuRendererInterface,
        animation_bind_group_layout: &AnimationBindGroupLayout,
        animation_data: &crate::animated_object::animated_object_data::AnimatedObjectData,
        instances: &[Instance],
    ) -> Self
    {
        let positions = &animation_data.positions;
        let normals = &animation_data.normals;
        let joints = &animation_data.joints;
        let weights = &animation_data.weights;

        let len = positions.len();
        assert_eq!(normals.len(), len);
        assert_eq!(joints.len(), len);
        assert_eq!(weights.len(), len);

        let mut vertices = Vec::new();
        for i in 0..len {
            let vertex = Vertex {
                position: [positions[i][0], positions[i][1], positions[i][2], 1.0],
                normal: [normals[i][0], normals[i][1], normals[i][2], 0.0],
                joint_indices: [joints[i][0] as u32, joints[i][1] as u32, joints[i][2] as u32, joints[i][3] as u32],
                joint_weights: weights[i],
            };

            vertices.push(vertex);
        }

        let animation_uniform = AnimationUniform::zero();

        Self::new(wgpu_renderer, animation_bind_group_layout, &vertices, &animation_uniform, instances)
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
        self.instance_buffer.bind(render_pass);

        // render_pass.draw_indexed(0..self.index_buffer.size()-1374 +200, 0, 0..self.nr_instances);
        render_pass.draw(0..self.nr_vertices, 0..self.nr_instances);
    }
}
