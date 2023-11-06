//! Contains the textures with all the entity indices
//!

use std::mem;

use wgpu_renderer::renderer::WgpuRendererInterface;

use super::entity_buffer_slice::EntityBufferSlice;


pub struct EntityBuffer {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    // pub bind_group: wgpu::BindGroup,

    pub output_buffer: wgpu::Buffer,

    unpadded_bytes_per_row: u32,
    padded_bytes_per_row: u32,
    height: u32,
    size: wgpu::Extent3d,
}


impl EntityBuffer {
    pub const FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8Unorm;

    pub fn new(wgpu_renderer: &mut impl WgpuRendererInterface, 
        // entity_buffer_bind_group_layout: &EntityBufferBindGroupLayout,
        surface_width: u32, surface_height: u32,
    ) -> Self
    {
        let size = wgpu::Extent3d {
            width: surface_width,
            height: surface_height,
            depth_or_array_layers: 1,
        };

        let desc = wgpu::TextureDescriptor {
            label: Some("Entity Texture"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
            view_formats: Default::default(),
        };
        let texture = wgpu_renderer.device().create_texture(&desc);

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        // wgpu requires texture -> buffer copies to be aligned using
        // wgpu::COPY_BYTES_PER_ROW_ALIGNMENT. Because of this we'll
        // need to save both the padded_bytes_per_row as well as the
        // unpadded_bytes_per_row
        let pixel_size = mem::size_of::<[u8;4]>() as u32;
        let align = wgpu::COPY_BYTES_PER_ROW_ALIGNMENT;
        let unpadded_bytes_per_row = pixel_size * surface_width;
        let padding = (align - unpadded_bytes_per_row % align) % align;
        let padded_bytes_per_row = unpadded_bytes_per_row + padding;

        // create a buffer to copy the texture to so we can get the data
        let buffer_size = (padded_bytes_per_row * surface_height) as wgpu::BufferAddress;
        let buffer_desc = wgpu::BufferDescriptor {
            size: buffer_size,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            label: Some("Output Buffer"),
            mapped_at_creation: false,
        };
        let output_buffer = wgpu_renderer.device().create_buffer(&buffer_desc);

        Self {
            texture,
            view,
            
            output_buffer,

            unpadded_bytes_per_row,
            padded_bytes_per_row,
            height: surface_height,
            size,
        }
    }

    pub fn copy_texture_to_buffer(&self, encoder: &mut wgpu::CommandEncoder)
    {
        encoder.copy_texture_to_buffer(
            wgpu::ImageCopyTexture {
                aspect: wgpu::TextureAspect::All,
                texture: &self.texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            }, 
            wgpu::ImageCopyBuffer {
                buffer: &self.output_buffer,
                layout: wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(self.padded_bytes_per_row),
                    rows_per_image: Some(self.height),
                }
            },
            self.size
        );
    }

    pub fn map_buffer_async(&self) ->  EntityBufferSlice
    {
        let buffer_slice = self.output_buffer.slice(..);
        buffer_slice.map_async(wgpu::MapMode::Read, move |_| ());

        let entity_buffer_slice = 
            EntityBufferSlice::new(&self.output_buffer,
                buffer_slice, 
                self.size, 
                self.unpadded_bytes_per_row, 
                self.padded_bytes_per_row);

        entity_buffer_slice
    }  
}
