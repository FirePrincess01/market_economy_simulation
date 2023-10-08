//! Contains the device buffers to render an object with this shader
//!

use wgpu_renderer::vertex_color_shader::{

    Vertex,
    Color,
    InstanceRaw,
    
    VertexBuffer,
    ColorBuffer,
    IndexBuffer,
    InstanceBuffer,
};

/// A general purpose shader using vertices, colors and an instance matrix
pub struct Mesh
{
    vertex_buffer: VertexBuffer,
    color_buffer: ColorBuffer,
    index_buffer: IndexBuffer,
    instance_buffer: InstanceBuffer,
    max_instances: u32,
    nr_instances: u32,
}

#[allow(dead_code)]
impl Mesh
{
    pub fn new(device: &wgpu::Device, 
        vertices: &[Vertex],
        colors: &[Color],
        indices: &[u32],
        instances: &[InstanceRaw]) -> Self
    {
        let vertex_buffer = VertexBuffer::new(device, vertices);
        let color_buffer = ColorBuffer::new(device, colors);
        let index_buffer = IndexBuffer::new(device, indices);

        let instance_buffer = InstanceBuffer::new(device, &instances);

        let max_instances = instances.len() as u32;
        let nr_instances = instances.len() as u32;

        Self {
            vertex_buffer,
            color_buffer,
            index_buffer,
            instance_buffer,
            max_instances,
            nr_instances,
        }
    }

    pub fn update_vertex_buffer(&mut self, queue: &wgpu::Queue, vertices: &[Vertex])
    {   
        self.vertex_buffer.update(queue, vertices);
    }

    pub fn update_color_buffer(&mut self, queue: &wgpu::Queue, colors: &[Color])
    {
        self.color_buffer.update(queue, colors);
    }

    pub fn update_instance_buffer(&mut self, queue: &wgpu::Queue, instances: &[InstanceRaw])
    {
        self.instance_buffer.update(queue, instances);
        self.nr_instances = u32::min(instances.len() as u32, self.max_instances);
    }

    pub fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>)
    {
        self.vertex_buffer.bind(render_pass);
        self.color_buffer.bind(render_pass);
        self.index_buffer.bind(render_pass);
        self.instance_buffer.bind(render_pass);

        render_pass.draw_indexed(0..self.index_buffer.size(), 0, 0..self.nr_instances);
    }
}