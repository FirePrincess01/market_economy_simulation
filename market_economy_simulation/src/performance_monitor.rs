use wgpu_renderer::{
    performance_monitor,
    vertex_color_shader::{self, VertexColorShaderDraw},
    vertex_texture_shader::{TextureBindGroupLayout, VertexTextureShaderDraw},
    wgpu_renderer::WgpuRendererInterface,
};

const WATCHPOINTS_SIZE: usize = 5;

pub struct PerformanceMonitor {
    pub watch: performance_monitor::Watch<5>,
    graph_host: performance_monitor::Graph,
    graph_device: vertex_color_shader::Mesh,

    label_30fps: wgpu_renderer::label::LabelMesh,
    label_60fps: wgpu_renderer::label::LabelMesh,
    label_120fps: wgpu_renderer::label::LabelMesh,

    pub show: bool,
}

impl PerformanceMonitor {
    pub fn new(
        wgpu_renderer: &mut dyn WgpuRendererInterface,
        texture_bind_group_layout: &TextureBindGroupLayout,
        font: &rusttype::Font<'static>,
    ) -> Self {
        let watch: performance_monitor::Watch<WATCHPOINTS_SIZE> = performance_monitor::Watch::new();
        let graph_host = performance_monitor::Graph::new(WATCHPOINTS_SIZE);

        let graph_device = vertex_color_shader::Mesh::new(
            wgpu_renderer.device(),
            graph_host.vertices.as_slice(),
            graph_host.colors.as_slice(),
            graph_host.indices.as_slice(),
            &[vertex_color_shader::Instance {
                position: glam::Vec3::ZERO,
                rotation: glam::Quat::IDENTITY,
            }],
        );

        let scale = 20.0;
        let label_30fps_host = wgpu_renderer::label::Label::new(font, scale, "30 fps");
        let label_60fps_host = wgpu_renderer::label::Label::new(font, scale, "60 fps");
        let label_120fps_host = wgpu_renderer::label::Label::new(font, scale, "120 fps");

        let label_30fps = wgpu_renderer::label::LabelMesh::new(
            wgpu_renderer,
            label_30fps_host.get_image(),
            texture_bind_group_layout,
            &vertex_color_shader::Instance {
                position: glam::Vec3::new(graph_host.get_width() as f32 + 5.0, graph_host.get_height_30fps() - scale / 2.0, 0.0),
                rotation: glam::Quat::IDENTITY,
            },
        );

        let label_60fps = wgpu_renderer::label::LabelMesh::new(
            wgpu_renderer,
            label_60fps_host.get_image(),
            texture_bind_group_layout,
            &vertex_color_shader::Instance {
                position: glam::Vec3::new(graph_host.get_width() as f32 + 5.0, graph_host.get_height_60fps() - scale / 2.0, 0.0),
                rotation: glam::Quat::IDENTITY,
            },
        );

        let label_120fps = wgpu_renderer::label::LabelMesh::new(
            wgpu_renderer,
            label_120fps_host.get_image(),
            texture_bind_group_layout,
            &vertex_color_shader::Instance {
                position: glam::Vec3::new(graph_host.get_width() as f32 + 5.0, graph_host.get_height_120fps() - scale / 2.0, 0.0),
                rotation: glam::Quat::IDENTITY,
            },
        );

        Self {
            watch,
            graph_host,
            graph_device,

            label_30fps,
            label_60fps,
            label_120fps,

            show: true,
        }
    }

    pub fn update(&mut self, wgpu_renderer: &mut dyn WgpuRendererInterface) {
        self.watch.update();
        self.watch.update_viewer(&mut self.graph_host);
        self.graph_device
            .update_vertex_buffer(wgpu_renderer.queue(), self.graph_host.vertices.as_slice());
    }
}

impl VertexColorShaderDraw for PerformanceMonitor {
    fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        if self.show {
            self.graph_device.draw(render_pass);
        }
    }
}

impl VertexTextureShaderDraw for PerformanceMonitor {
    fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        if self.show {
            // self.label_30fps.draw(render_pass);
            self.label_60fps.draw(render_pass);
            self.label_120fps.draw(render_pass);
        }
    }
}
