//! Contains the device buffers to render an object with this shader
//!

use crate::shape;

use super::DeferredLightShaderDraw;
use super::Instance;
use super::Vertex;

use super::IndexBuffer;
use super::InstanceBuffer;
use super::VertexBuffer;

/// A general purpose shader using vertices, colors and an instance matrix
pub struct Mesh {
    vertex_buffer: VertexBuffer,
    index_buffer: IndexBuffer<u16>,
    instance_buffer: InstanceBuffer<Instance>,
    max_instances: u32,
    nr_instances: u32,
}

#[allow(dead_code)]
impl Mesh {
    pub fn new(
        device: &wgpu::Device,
        vertices: &[Vertex],
        indices: &[u16],
        instances: &[Instance],
    ) -> Self {
        let vertex_buffer = VertexBuffer::new(device, vertices);
        let index_buffer = IndexBuffer::new(device, indices);

        let instance_buffer = InstanceBuffer::new(device, instances);

        let max_instances = instances.len() as u32;
        let nr_instances = instances.len() as u32;

        Self {
            vertex_buffer,
            index_buffer,
            instance_buffer,
            max_instances,
            nr_instances,
        }
    }

    pub fn from_shape(
        device: &wgpu::Device,
        shape: &shape::MeshData,
        instances: &[Instance],
    ) -> Self {
        let shape = shape.triangulate();

        let vertices = &shape.positions;
        let indices = &shape.indices;

        let len = vertices.iter().len();
        let mut mesh_vertices = Vec::with_capacity(len);

        for vertex in vertices {
            mesh_vertices.push(Vertex {
                position: (*vertex).into(),
            });
        }

        Self::new(device, &mesh_vertices, indices, instances)
    }

    pub fn update_vertex_buffer(&mut self, queue: &wgpu::Queue, vertices: &[Vertex]) {
        self.vertex_buffer.update(queue, vertices);
    }

    pub fn update_instance_buffer(&mut self, queue: &wgpu::Queue, instances: &[Instance]) {
        self.instance_buffer.update(queue, instances);
        self.nr_instances = u32::min(instances.len() as u32, self.max_instances);
    }
}

impl DeferredLightShaderDraw for Mesh {
    fn draw_lights<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        self.vertex_buffer.bind(render_pass);
        self.index_buffer.bind(render_pass);
        self.instance_buffer.bind_slot(render_pass, 1);

        render_pass.draw_indexed(0..self.index_buffer.size(), 0, 0..self.nr_instances);
    }
}
