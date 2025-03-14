pub mod sliding_average;
mod sorted_table;

use wgpu_renderer::{
    performance_monitor::{self, watch},
    shape::{self, MeshDataInterface},
    vertex_color_shader::{
        self, vertex_color_shader_draw::VertexColorShaderDrawLines, VertexColorShaderDraw,
    },
    vertex_texture_shader::{TextureBindGroupLayout, VertexTextureShaderDraw},
    wgpu_renderer::WgpuRendererInterface,
};

const WATCHPOINTS_SIZE: usize = 7;

pub struct PerformanceMonitor {
    pub watch: performance_monitor::Watch<WATCHPOINTS_SIZE>,
    graph_host: performance_monitor::Graph,
    graph_device: vertex_color_shader::Mesh,

    // label_30fps: wgpu_renderer::label::LabelMesh,
    label_60fps: wgpu_renderer::label::LabelMesh,
    // label_120fps: wgpu_renderer::label::LabelMesh,

    table: sorted_table::SortedTable,

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

        let scale = 14.0;
        let label_60fps_host = wgpu_renderer::label::Label::new(font, scale, "60 fps");

        let label_60fps = wgpu_renderer::label::LabelMesh::new(
            wgpu_renderer,
            label_60fps_host.get_image(),
            texture_bind_group_layout,
            &vertex_color_shader::Instance {
                position: glam::Vec3::new(
                    graph_host.get_width() as f32 - label_60fps_host.width() as f32,
                    graph_host.get_height_60fps(),
                    0.0,
                ),
                rotation: glam::Quat::IDENTITY,
            },
        );

        // create table
        let table = sorted_table::SortedTable::new(
            wgpu_renderer,
            texture_bind_group_layout,
            font,
            WATCHPOINTS_SIZE,
            graph_host.get_nr_lines(),
            &performance_monitor::Graph::color_gradient(WATCHPOINTS_SIZE),
            scale,
            cgmath::Vector3 {
                x: graph_host.get_width() as f32 + 5.0,
                y: 10.0,
                z: 0.0,
            },
        );

        Self {
            watch,
            graph_host,
            graph_device,

            label_60fps,

            table,
            show: true,
        }
    }

    pub fn update(
        &mut self,
        wgpu_renderer: &mut dyn WgpuRendererInterface,
        font: &rusttype::Font<'static>,
    ) {
        self.watch.update();
        self.watch.update_viewer(&mut self.graph_host);
        self.watch.update_viewer(&mut self.table);
        self.graph_device
            .update_vertex_buffer(wgpu_renderer.queue(), self.graph_host.vertices.as_slice());
        self.table.update_device(wgpu_renderer, font);
    }
}

impl VertexColorShaderDraw for PerformanceMonitor {
    fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        if self.show {
            for elem in &self.table.mesh_colors {
                elem.draw(render_pass);
            }
        }
    }
}

impl VertexColorShaderDrawLines for PerformanceMonitor {
    fn draw_lines<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        if self.show {
            self.graph_device.draw_lines(render_pass);
        }
    }
}

impl VertexTextureShaderDraw for PerformanceMonitor {
    fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        if self.show {
            self.label_60fps.draw(render_pass);

            for elem in &self.table.mesh_percent {
                elem.draw(render_pass);
            }

            for elem in &self.table.mesh_names {
                elem.draw(render_pass);
            }
        }
    }
}
