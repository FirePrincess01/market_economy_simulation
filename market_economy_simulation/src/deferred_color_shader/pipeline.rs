//! Deferred shader drawing colored objects
//!

use super::entity_buffer::EntityBuffer;
use super::CameraBindGroupLayout;
use super::DeferredShaderDraw;
use super::GBuffer;
use super::Instance;
use super::Vertex;
use wgpu_renderer::vertex_color_shader;
use wgpu_renderer::wgpu_renderer::depth_texture;

/// A general purpose shader using vertices, colors and an instance matrix
pub struct Pipeline {
    render_pipeline: wgpu::RenderPipeline,
}

impl Pipeline {
    pub fn _new_lines(
        device: &wgpu::Device,
        camera_bind_group_layout: &CameraBindGroupLayout,
        surface_format: wgpu::TextureFormat,
    ) -> Self {
        Self::new_parameterized(
            device,
            camera_bind_group_layout,
            surface_format,
            wgpu::PrimitiveTopology::LineList,
        )
    }

    pub fn new(
        device: &wgpu::Device,
        camera_bind_group_layout: &CameraBindGroupLayout,
        surface_format: wgpu::TextureFormat,
    ) -> Self {
        Self::new_parameterized(
            device,
            camera_bind_group_layout,
            surface_format,
            wgpu::PrimitiveTopology::LineList,
        )
    }

    fn new_parameterized(
        device: &wgpu::Device,
        camera_bind_group_layout: &CameraBindGroupLayout,
        _surface_format: wgpu::TextureFormat,
        topology: wgpu::PrimitiveTopology,
    ) -> Self {
        // Shader
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        // Pipeline
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[camera_bind_group_layout.get()],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Deferred Color Render Pipeline"),
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
                targets: &[
                    // Some(wgpu::ColorTargetState {
                    //     format: surface_format,
                    //     blend: None,
                    //     write_mask: wgpu::ColorWrites::ALL,
                    // }),
                    // None,
                    Some(wgpu::ColorTargetState {
                        format: GBuffer::G_BUFFER_FORMAT_POSITION,
                        blend: None,
                        write_mask: wgpu::ColorWrites::ALL,
                    }),
                    Some(wgpu::ColorTargetState {
                        format: GBuffer::G_BUFFER_FORMAT_NORMAL,
                        blend: None,
                        write_mask: wgpu::ColorWrites::ALL,
                    }),
                    Some(wgpu::ColorTargetState {
                        format: GBuffer::G_BUFFER_FORMAT_ALBEDO,
                        blend: None,
                        write_mask: wgpu::ColorWrites::ALL,
                    }),
                    Some(wgpu::ColorTargetState {
                        format: EntityBuffer::FORMAT,
                        blend: None,
                        write_mask: wgpu::ColorWrites::ALL,
                    }),
                ],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology, // wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw, // counter-clockwise direction
                cull_mode: Some(wgpu::Face::Back),
                // cull_mode: None,
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: depth_texture::DepthTexture::DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Always, // for the light orb to shine through
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
        mesh: &'a dyn DeferredShaderDraw,
    ) {
        render_pass.set_pipeline(&self.render_pipeline);
        camera.bind(render_pass);
        mesh.draw(render_pass);
    }
}
