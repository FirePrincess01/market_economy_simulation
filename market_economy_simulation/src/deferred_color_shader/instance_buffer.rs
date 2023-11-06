//! GPU memory buffer containing the instances for this shader
//!

use super::Instance;
use wgpu::util::DeviceExt;

pub struct InstanceBuffer {
    buffer: wgpu::Buffer,
    _size: u32,
}

impl InstanceBuffer {
    pub fn new(device: &wgpu::Device, instances: &[Instance]) -> Self
    {
        let buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Instance Buffer"),
                contents: bytemuck::cast_slice(instances),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            }
        );

        let _size = instances.len() as u32;

        Self {
            buffer,
            _size,
        }
    }

    pub fn update(&mut self, queue: &wgpu::Queue, instances: &[Instance])
    {   
        let data = bytemuck::cast_slice(instances);

        if data.len() as u64 <= self.buffer.size() {
            queue.write_buffer(&self.buffer, 0, data);
        }
    }

    pub fn bind<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) 
    {
        render_pass.set_vertex_buffer(1, self.buffer.slice(..));
    }

    pub fn _size(&self) -> u32 {
        self._size
    }

}