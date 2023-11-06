use wgpu_renderer::renderer::WgpuRendererInterface;

use crate::deferred_light_shader::GBufferBindGroupLayout;

pub struct GBufferTexture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
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

        Self {
            texture,
            view,
        }
    }
}


pub struct GBuffer {
    pub position: GBufferTexture,
    pub normal: GBufferTexture,
    pub albedo: GBufferTexture,

    pub bind_group: wgpu::BindGroup,
}

impl GBuffer {
    pub const G_BUFFER_FORMAT_POSITION: wgpu::TextureFormat = wgpu::TextureFormat::Rgba16Float;
    pub const G_BUFFER_FORMAT_NORMAL: wgpu::TextureFormat = wgpu::TextureFormat::Rgba16Float;
    pub const G_BUFFER_FORMAT_ALBEDO: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8UnormSrgb;

    pub fn new(wgpu_renderer: &mut impl WgpuRendererInterface,
         g_buffer_bind_group_layout: &GBufferBindGroupLayout,
         surface_width: u32, 
         surface_height: u32) -> Self 
    {
        let position = GBufferTexture::new(wgpu_renderer.device(), 
            surface_width, surface_height, 
            Self::G_BUFFER_FORMAT_POSITION, 
            "GBuffer Position");

        let normal = GBufferTexture::new(wgpu_renderer.device(), 
            surface_width, surface_height, 
            Self::G_BUFFER_FORMAT_NORMAL, 
            "GBuffer Normal");

        let albedo = GBufferTexture::new(wgpu_renderer.device(), 
            surface_width, surface_height, 
            Self::G_BUFFER_FORMAT_ALBEDO, 
            "GBuffer Albedo");


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
                        resource: wgpu::BindingResource::TextureView(&normal.view), 
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: wgpu::BindingResource::TextureView(&albedo.view), 
                    },
                ],
                label: Some("g_buffer_bind_group"),
            }
        );

        Self {
            position,
            normal,
            albedo,

            bind_group,
        }
    }



    pub fn bind<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>,) {
        render_pass.set_bind_group(1, &self.bind_group, &[]);
    }

}