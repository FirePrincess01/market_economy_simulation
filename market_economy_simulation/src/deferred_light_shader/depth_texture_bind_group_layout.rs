//! A bind group for the depth texture
//!

use wgpu_renderer::wgpu_renderer::depth_texture::DepthTexture;

pub struct DepthTextureBindGroup {
    bind_group_layout: wgpu::BindGroupLayout,
    bind_group: wgpu::BindGroup,
}

impl DepthTextureBindGroup {
    pub fn new(device: &wgpu::Device, depth_texture: &DepthTexture) -> Self {
        // Texture
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    // depth texture
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: false },
                    },
                    count: None,
                },
            ],
            label: Some("depth texture bind_group_layout"),
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&depth_texture.view),
            }],
            label: Some("depth texture bind group"),
        });

        Self { bind_group_layout, bind_group }
    }

    pub fn get_layout(&self) -> &wgpu::BindGroupLayout {
        &self.bind_group_layout
    }

    pub fn bind<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_bind_group(2, &self.bind_group, &[]);
    }
}
