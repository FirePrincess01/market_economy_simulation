//! Deferred shader pipeline drawing light stage
//!

use wgpu_renderer::vertex_color_shader;
use wgpu_renderer::wgpu_renderer::depth_texture::DepthTexture;

use crate::deferred_color_shader;

use super::CameraBindGroupLayout;
use super::DeferredLightShaderDraw;
use super::GBufferBindGroupLayout;
use super::Instance;
use super::Vertex;

/// A general purpose shader using vertices, colors and an instance matrix
pub struct Pipeline {
    render_pipeline: wgpu::RenderPipeline,
}

impl Pipeline {
    pub fn new(
        device: &wgpu::Device,
        camera_bind_group_layout: &CameraBindGroupLayout,
        g_buffer_bind_group_layout: &GBufferBindGroupLayout,
        surface_format: wgpu::TextureFormat,
        use_ambient_shader: bool,
    ) -> Self {
        let shader_source = match use_ambient_shader {
            true => include_str!("shader_ambient_light.wgsl"),
            false => include_str!("shader_point_light.wgsl"),
        };

        // Shader
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Deferred Light Shader"),
            source: wgpu::ShaderSource::Wgsl(shader_source.into()),
        });

        // Pipeline
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Deferred Light Render Pipeline Layout"),
                bind_group_layouts: &[
                    camera_bind_group_layout.get(),
                    g_buffer_bind_group_layout.get(),
                ],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Deferred Light Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[Vertex::desc(), Instance::desc()],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw, // counter-clockwise direction
                cull_mode: match use_ambient_shader {
                    true => Some(wgpu::Face::Back),
                    false => Some(wgpu::Face::Front),
                },
                // cull_mode: None,
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: DepthTexture::DEPTH_FORMAT,
                depth_write_enabled: false,
                depth_compare: match use_ambient_shader {
                    true => wgpu::CompareFunction::Less,
                    false => wgpu::CompareFunction::Always,
                },
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        });

        Self { render_pipeline }
    }

    pub fn draw<'a>(
        &self,
        render_pass: &mut wgpu::RenderPass<'a>,
        camera: &'a vertex_color_shader::CameraUniformBuffer,
        g_buffer: &'a deferred_color_shader::GBuffer,
        mesh: &'a dyn DeferredLightShaderDraw,
    ) {
        render_pass.set_pipeline(&self.render_pipeline);
        camera.bind(render_pass);
        g_buffer.bind(render_pass);
        mesh.draw_lights(render_pass);
    }
}
