//! Contains the device buffers to render an object with this shader
//!

use wgpu_renderer::shape;

use super::Instance;
use super::Vertex;

use super::HeightmapTexture;
use super::IndexBuffer;
use super::InstanceBuffer;
use super::Texture;
use super::VertexBuffer;

/// A general purpose shader using vertices, colors and an instance matrix
pub struct Mesh {
    vertex_buffer: VertexBuffer<Vertex>,
    // texture_index: usize,
    // heightmap_texture: HeightmapTexture,
    index_buffer: IndexBuffer<u32>,
    // instance_buffer: InstanceBuffer<Instance>,

    // max_instances: u32,
    // nr_instances: u32,
}

#[allow(unused)]
impl Mesh {
    pub fn new(
        device: &wgpu::Device,
        vertices: &[Vertex],
        indices: &[u32],
        // instances: &[Instance],
    ) -> Self {
        let vertex_buffer = VertexBuffer::new(device, vertices);
        // let heightmap_texture = HeightmapTexture::new(
        //     device,
        //     heightmap_bind_group_layout,
        //     heightmap2d.data,
        //     heightmap2d.width,
        //     heightmap2d.height,
        //     Some("Heightmap Texture"),
        // );
        let index_buffer = IndexBuffer::new(device, indices);

        // let instance_buffer = InstanceBuffer::new(device, instances);

        // let max_instances = instances.len() as u32;
        // let nr_instances = instances.len() as u32;

        Self {
            vertex_buffer,
            // texture_index,
            // heightmap_texture,
            index_buffer,
            // instance_buffer,
            // max_instances,
            // nr_instances,
        }
    }

    pub fn from_shape(
        device: &wgpu::Device,
        shape: &shape::MeshDataTriangles,
        // instances: &[Instance],
    ) -> Self {
        let vertices = &shape.positions;
        let normals = &shape.normals;
        let indices = &shape.indices;

        assert_eq!(vertices.len(), normals.len());

        let len = vertices.iter().len();
        let mut mesh_vertices = Vec::with_capacity(len);

        for i in 0..len {
            mesh_vertices.push(Vertex {
                position: vertices[i].into(),
            });
        }

        Self::new(device, &mesh_vertices, indices)
    }

    pub fn _update_vertex_buffer(&mut self, queue: &wgpu::Queue, vertices: &[Vertex]) {
        self.vertex_buffer.update(queue, vertices);
    }

    // pub fn _set_texture_index(&mut self, texture_index: usize) {
    //     self.texture_index = texture_index;
    // }

    // pub fn update_heightmap_texture(&mut self, queue: &wgpu::Queue, heightmap: &[Heightmap]) {
    //     self.heightmap_texture.update(queue, heightmap);
    // }

    // pub fn update_instance_buffer(&mut self, queue: &wgpu::Queue, instances: &[Instance]) {
    //     self.instance_buffer.update(queue, instances);
    //     self.nr_instances = u32::min(instances.len() as u32, self.max_instances);
    // }

    pub fn draw<'a>(
        &'a self,
        render_pass: &mut wgpu::RenderPass<'a>,
        heightmap_texture: &'a HeightmapTexture,
        texture: &'a Texture,
        instance: &'a InstanceBuffer<Instance>,
    ) {
        self.vertex_buffer.bind(render_pass);
        heightmap_texture.bind(render_pass);
        texture.bind(render_pass);
        self.index_buffer.bind(render_pass);
        instance.bind_slot(render_pass, 1);

        render_pass.draw_indexed(0..self.index_buffer.size(), 0, 0..1);
    }

    pub fn bind<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        self.vertex_buffer.bind(render_pass);
        self.index_buffer.bind(render_pass);
    }

    pub fn draw_indexed<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.draw_indexed(0..self.index_buffer.size(), 0, 0..1);
    }
}

// impl DeferredHeightMapShaderDraw for Mesh {
//     fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>, heightmap_texture: &'a HeightmapTexture, texture: &'a Texture) {
//         self.vertex_buffer.bind(render_pass);
//         heightmap_texture.bind(render_pass);
//         texture.bind(render_pass);
//         self.index_buffer.bind(render_pass);
//         self.instance_buffer.bind_slot(render_pass, 1);

//         render_pass.draw_indexed(
//             0..self.index_buffer.size(),
//             0,
//             0..self.instance_buffer.size(),
//         );
//     }
// }
