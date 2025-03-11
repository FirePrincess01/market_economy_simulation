//! Contains the textures with all the entity indices
//!

use std::{
    mem,
    sync::{atomic::AtomicBool, Arc},
};

use wgpu_renderer::wgpu_renderer::WgpuRendererInterface;

use super::entity_buffer_slice::EntityBufferSlice;

pub struct MousePosition {
    pub x: u32,
    pub y: u32,
}

pub struct EntityBuffer {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    // pub bind_group: wgpu::BindGroup,
    pub output_buffer: wgpu::Buffer,

    unpadded_bytes_per_row: u32,
    padded_bytes_per_row: u32,
    height: u32,
    size: wgpu::Extent3d,

    pixel_val: u32,
    mouse_position: MousePosition,

    enable_memory_mapped_read: bool,
    memory_in_copying: bool,
    memory_in_mapping: bool,
    memory_is_mapped: Arc<AtomicBool>,
}

impl EntityBuffer {
    pub const FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8Unorm;

    pub fn new(
        wgpu_renderer: &mut dyn WgpuRendererInterface,
        // entity_buffer_bind_group_layout: &EntityBufferBindGroupLayout,
        surface_width: u32,
        surface_height: u32,
        enable_memory_mapped_read: bool,
    ) -> Self {
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
        let pixel_size = mem::size_of::<[u8; 4]>() as u32;
        let align = wgpu::COPY_BYTES_PER_ROW_ALIGNMENT;
        let unpadded_bytes_per_row = pixel_size * surface_width;
        let padding = (align - unpadded_bytes_per_row % align) % align;
        let padded_bytes_per_row = unpadded_bytes_per_row + padding;

        // create a buffer to copy the texture to so we can get the data
        let buffer_size = (padded_bytes_per_row * surface_height) as wgpu::BufferAddress;
        let buffer_desc = wgpu::BufferDescriptor {
            size: 4,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            label: Some("Entity Output Buffer"),
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

            pixel_val: 0,
            mouse_position: MousePosition { x: 0, y: 0 },

            // buffer_slice: None,
            enable_memory_mapped_read,
            memory_in_copying: false,
            memory_in_mapping: false,
            memory_is_mapped: Arc::from(AtomicBool::new(false)),
        }
    }

    pub fn copy_texture_to_buffer(
        &mut self,
        encoder: &mut wgpu::CommandEncoder,
        mouse_position: MousePosition,
    ) {
        let memory_is_mapped = self
            .memory_is_mapped
            .load(std::sync::atomic::Ordering::Relaxed);

        if !memory_is_mapped && !self.memory_in_copying {
            self.memory_in_copying = true;

            let size = wgpu::Extent3d {
                width: 1,
                height: 1,
                depth_or_array_layers: 1,
            };

            let pos_x: u32 = mouse_position.x.min(self.size.width - 1);
            let pos_y = mouse_position.y.min(self.size.height - 1);

            encoder.copy_texture_to_buffer(
                wgpu::TexelCopyTextureInfo {
                    aspect: wgpu::TextureAspect::All,
                    texture: &self.texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d {
                        x: pos_x,
                        y: pos_y,
                        z: 0,
                    },
                },
                wgpu::TexelCopyBufferInfo {
                    buffer: &self.output_buffer,
                    layout: wgpu::TexelCopyBufferLayout {
                        offset: 0,
                        bytes_per_row: None,
                        rows_per_image: None,
                    },
                },
                size,
            );

            self.mouse_position = mouse_position;
        }
    }

    pub fn _map_buffer_async_old(&self) -> EntityBufferSlice {
        let buffer_slice = self.output_buffer.slice(..);
        buffer_slice.map_async(wgpu::MapMode::Read, move |_| ());

        let entity_buffer_slice = EntityBufferSlice::new(
            &self.output_buffer,
            buffer_slice,
            self.size,
            self.unpadded_bytes_per_row,
            self.padded_bytes_per_row,
        );

        entity_buffer_slice
    }

    pub fn map_buffer_async(&mut self) {
        if !self.memory_in_mapping {
            self.memory_in_mapping = true;

            // memory mapping the buffer is extremly slow on the smartphone
            if self.enable_memory_mapped_read {
                let buffer_slice: wgpu::BufferSlice<'_> = self.output_buffer.slice(0..4);
                let memory_is_mapped = self.memory_is_mapped.clone();
                buffer_slice.map_async(wgpu::MapMode::Read, move |_| {
                    memory_is_mapped.store(true, std::sync::atomic::Ordering::SeqCst);
                });
            }
        }
    }

    pub fn read_pixel(&mut self) -> u32 {

        let memory_is_mapped = self
            .memory_is_mapped
            .load(std::sync::atomic::Ordering::Relaxed);

        if memory_is_mapped {
            let buffer_slice: wgpu::BufferSlice<'_> = self.output_buffer.slice(0..4);

            {
                let buffer_view = buffer_slice.get_mapped_range();

                let chunk = buffer_view.chunks(4).next().unwrap();
                assert_eq!(chunk.len(), 4);

                let val = Self::as_u32_le(chunk);

                self.pixel_val = val;
            }

            self.output_buffer.unmap();

            // reset all variables
            self.memory_is_mapped
                .store(false, std::sync::atomic::Ordering::SeqCst);
            self.memory_in_mapping = false;
            self.memory_in_copying = false;
        }

        self.pixel_val 
    }

    fn as_u32_le(array: &[u8]) -> u32 {
        (array[0] as u32)
            + ((array[1] as u32) << 8)
            + ((array[2] as u32) << 16)
            + ((array[3] as u32) << 24)
    }
}


