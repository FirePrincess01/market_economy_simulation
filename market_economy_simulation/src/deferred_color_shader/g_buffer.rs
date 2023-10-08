use wgpu_renderer::renderer::WgpuRendererInterface;

use crate::deferred_light_shader::GBufferBindGroupLayout;


pub struct GBufferFormat {
    pub position: wgpu::TextureFormat,
    pub normal: wgpu::TextureFormat,
    // pub albedo: wgpu::TextureFormat,
    // pub specular: wgpu::TextureFormat,
}

impl GBufferFormat {
    pub fn new() -> Self {

        let position = wgpu::TextureFormat::Rgba16Float;
        let normal = wgpu::TextureFormat::Rgba16Float;
        // let albedo = wgpu::TextureFormat::Rgba8UnormSrgb;
        // let specular = wgpu::TextureFormat::R16Float;

        Self {
            position,
            normal,
            // albedo,
            // specular,
        }
    }
}

pub struct GBufferTexture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
}

impl GBufferTexture {
    pub fn new(device: &wgpu::Device, 
        surface_width: u32, surface_height: u32,
        format: wgpu::TextureFormat,
        label: &str) -> Self
    {
        let size = wgpu::Extent3d {
            width: surface_width,
            height: surface_height,
            depth_or_array_layers: 1,
        };

        let desc = wgpu::TextureDescriptor {
            label: Some(label),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format:format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: Default::default(),
        };
        let texture = device.create_texture(&desc);

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(
            &wgpu::SamplerDescriptor {
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter: wgpu::FilterMode::Nearest,
                min_filter: wgpu::FilterMode::Nearest,
                mipmap_filter: wgpu::FilterMode::Nearest,
                ..Default::default()
            }
        );

        Self {
            texture,
            view,
            sampler,
        }
    }
}

pub struct GBuffer {
    format: GBufferFormat,

    pub position: GBufferTexture,
    pub normal: GBufferTexture,
    // pub albedo: GBufferTexture,
    // pub specular: GBufferTexture,

    pub bind_group: wgpu::BindGroup,
}

impl GBuffer {
    pub fn new(wgpu_renderer: &mut impl WgpuRendererInterface,
         g_buffer_bind_group_layout: &GBufferBindGroupLayout,
         surface_width: u32, 
         surface_height: u32) -> Self 
    {
        let format = GBufferFormat::new();

        let (position, normal, ) = 
        Self::create_buffers(wgpu_renderer.device(), surface_width, surface_height, &format);

        let bind_group = wgpu_renderer.device().create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: g_buffer_bind_group_layout.get(),
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&position.view), 
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&position.sampler), 
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: wgpu::BindingResource::TextureView(&normal.view), 
                    },
                    wgpu::BindGroupEntry {
                        binding: 3,
                        resource: wgpu::BindingResource::Sampler(&normal.sampler),
                    },
                ],
                label: Some("texture_bind_group"),
            }
        );

        Self {
            format,
            
            position,
            normal,
            // albedo,
            // specular,

            bind_group,

        }
    }

    fn create_buffers(device: &wgpu::Device, 
        surface_width: u32, surface_height: u32,
        format: &GBufferFormat) 
        -> (GBufferTexture, GBufferTexture)
    {
        let position = GBufferTexture::new(device, 
            surface_width, surface_height, 
            format.position, 
            "GBuffer Position");

        let normal = GBufferTexture::new(device, 
            surface_width, surface_height, 
            format.normal, 
            "GBuffer Normal");

        // let albedo = GBufferTexture::new(device, 
        //     surface_width, surface_height, 
        //     format.albedo, 
        //     "GBuffer Albedo");

        // let specular = GBufferTexture::new(device, 
        //     surface_width, surface_height, 
        //     format.specular, 
        //     "GBuffer Specular");

        (position, normal)
    }

    pub fn resize(&mut self, device: &wgpu::Device, surface_width: u32, surface_height: u32) {
        let (position, normal, ) = 
            Self::create_buffers(device, surface_width, surface_height, &self.format);

        self.position = position;
        self.normal = normal;
        // self.albedo = albedo;
        // self.specular = specular;
    }

    pub fn get_format(&self) -> &GBufferFormat {
        &self.format
    }

}