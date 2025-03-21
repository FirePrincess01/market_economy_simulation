//! Prints some debug info to the screen
//!

use cgmath::Zero;
use wgpu_renderer::{
    vertex_texture_shader::{self, VertexTextureShaderDraw},
    wgpu_renderer::WgpuRendererInterface,
};

pub struct DebugOverlay {
    entity_index_label: wgpu_renderer::label::Label,
    entity_index_mesh: wgpu_renderer::label::LabelMesh,

    entity_kind_label: wgpu_renderer::label::Label,
    entity_kind_mesh: wgpu_renderer::label::LabelMesh,

    x_coord_label: wgpu_renderer::label::Label,
    x_coord_mesh: wgpu_renderer::label::LabelMesh,

    y_coord_label: wgpu_renderer::label::Label,
    y_coord_mesh: wgpu_renderer::label::LabelMesh,

    z_coord_label: wgpu_renderer::label::Label,
    z_coord_mesh: wgpu_renderer::label::LabelMesh,
}

impl DebugOverlay {
    pub fn new(
        renderer: &mut dyn WgpuRendererInterface,
        texture_bind_group_layout: &vertex_texture_shader::TextureBindGroupLayout,
        font: &rusttype::Font<'static>,
        pos_0: cgmath::Vector3<f32>,
    ) -> Self {
        let scale = 16.0;

        // entity index
        let entity_index_label = wgpu_renderer::label::Label::new(font, scale, "          ");
        let entity_index_mesh = wgpu_renderer::label::LabelMesh::new(
            renderer,
            entity_index_label.get_image(),
            texture_bind_group_layout,
            &vertex_texture_shader::Instance {
                position: pos_0,
                rotation: cgmath::Quaternion::zero(),
            },
        );

        let entity_kind_label = wgpu_renderer::label::Label::new(font, scale, "          ");
        let entity_kind_mesh = wgpu_renderer::label::LabelMesh::new(
            renderer,
            entity_index_label.get_image(),
            texture_bind_group_layout,
            &vertex_texture_shader::Instance {
                position: pos_0
                    + cgmath::Vector3 {
                        x: 0.0,
                        y: scale + 2.0,
                        z: 0.0,
                    },
                rotation: cgmath::Quaternion::zero(),
            },
        );

        let x_coord_label = wgpu_renderer::label::Label::new(font, scale, "          ");
        let x_coord_mesh = wgpu_renderer::label::LabelMesh::new(
            renderer,
            entity_index_label.get_image(),
            texture_bind_group_layout,
            &vertex_texture_shader::Instance {
                position: pos_0
                    + cgmath::Vector3 {
                        x: 0.0,
                        y: (scale + 2.0) * 4.0,
                        z: 0.0,
                    },
                rotation: cgmath::Quaternion::zero(),
            },
        );

        let y_coord_label = wgpu_renderer::label::Label::new(font, scale, "          ");
        let y_coord_mesh = wgpu_renderer::label::LabelMesh::new(
            renderer,
            entity_index_label.get_image(),
            texture_bind_group_layout,
            &vertex_texture_shader::Instance {
                position: pos_0
                    + cgmath::Vector3 {
                        x: 0.0,
                        y: (scale + 2.0) * 3.0,
                        z: 0.0,
                    },
                rotation: cgmath::Quaternion::zero(),
            },
        );

        let z_coord_label = wgpu_renderer::label::Label::new(font, scale, "          ");
        let z_coord_mesh = wgpu_renderer::label::LabelMesh::new(
            renderer,
            entity_index_label.get_image(),
            texture_bind_group_layout,
            &vertex_texture_shader::Instance {
                position: pos_0
                    + cgmath::Vector3 {
                        x: 0.0,
                        y: (scale + 2.0) * 2.0,
                        z: 0.0,
                    },
                rotation: cgmath::Quaternion::zero(),
            },
        );

        Self {
            entity_index_label,
            entity_index_mesh,
            entity_kind_label,
            entity_kind_mesh,
            x_coord_label,
            x_coord_mesh,
            y_coord_label,
            y_coord_mesh,
            z_coord_label,
            z_coord_mesh,
        }
    }

    pub fn update_entity(
        &mut self,
        renderer: &mut dyn WgpuRendererInterface,
        font: &rusttype::Font<'static>,
        entity: u32,
    ) {
        let entity_index = entity & 0x00FFFFFF;
        let entity_kind = (entity & 0xFF000000) >> 24;

        let text = format!("e: {}", entity_index);
        self.entity_index_label.update(font, &text);
        self.entity_index_mesh
            .update_texture(renderer.queue(), self.entity_index_label.get_image());

        let text = format!("kind: {}", entity_kind);
        self.entity_kind_label.update(font, &text);
        self.entity_kind_mesh
            .update_texture(renderer.queue(), self.entity_kind_label.get_image());
    }

    pub fn update_coord(
        &mut self,
        renderer: &mut dyn WgpuRendererInterface,
        font: &rusttype::Font<'static>,
        pos: &cgmath::Vector3<f32>,
    ) {
        {
            let text = format!("x: {}", pos.x);
            self.x_coord_label.update(font, &text);
            self.x_coord_mesh
                .update_texture(renderer.queue(), self.x_coord_label.get_image());
        }
        {
            let text = format!("y: {}", pos.y);
            self.y_coord_label.update(font, &text);
            self.y_coord_mesh
                .update_texture(renderer.queue(), self.y_coord_label.get_image());
        }
        {
            let text = format!("z: {}", pos.z);
            self.z_coord_label.update(font, &text);
            self.z_coord_mesh
                .update_texture(renderer.queue(), self.z_coord_label.get_image());
        }
    }
}

impl VertexTextureShaderDraw for DebugOverlay {
    fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        self.entity_index_mesh.draw(render_pass);
        self.entity_kind_mesh.draw(render_pass);
        self.x_coord_mesh.draw(render_pass);
        self.y_coord_mesh.draw(render_pass);
        self.z_coord_mesh.draw(render_pass);
    }
}
