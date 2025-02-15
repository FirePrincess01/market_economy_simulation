use collada::document::ColladaDocument;

use crate::{
    deferred_color_shader::{self, DeferredShaderDraw},
    deferred_light_shader,
};

use super::animated_object_renderer::{AnimatedObjectRenderer, AnimatedObjectRendererResult};

pub struct AnimatedObject {
    pub is_visible: bool,
    // pub update_position: bool,
    // pub update_text: bool,
    pub x: f32,
    pub y: f32,
    pub z: f32,

    pub instance: deferred_color_shader::Instance,

    pub mesh: deferred_color_shader::Mesh,
    // pub mes_light: deferred_light_shader::Mesh,
}

pub struct WgpuAnimatedObjectStorage {
    pub elements: Vec<AnimatedObject>,
}

impl WgpuAnimatedObjectStorage {
    pub fn new() -> Self {
        let elements = Vec::new();

        Self { elements }
    }

    pub fn draw<'b>(&'b self, render_pass: &mut wgpu::RenderPass<'b>) {
        for elem in &self.elements {
            elem.mesh.draw(render_pass);
        }
    }
}

pub struct WgpuAnimatedObjectRenderer<'a> {
    pub storage: &'a mut WgpuAnimatedObjectStorage,

    // wgpu renderer
    // pub font: &'a rusttype::Font<'static>,
    pub wgpu_renderer: &'a mut dyn wgpu_renderer::renderer::WgpuRendererInterface,
    // pub texture_bind_group_layout: &'a wgpu_renderer::vertex_texture_shader::TextureBindGroupLayout,
}

impl<'a> AnimatedObjectRenderer for WgpuAnimatedObjectRenderer<'a> {
    fn from_collada(
        &mut self,
        xml_string: &str,
    ) -> super::animated_object_renderer::AnimatedObjectRendererResult {
        let collada_document = ColladaDocument::from_str(xml_string).unwrap();

        let obj_set = collada_document.get_obj_set().unwrap();
        let obj = &obj_set.objects[0];

        let id = &obj.id;
        let name = &obj.name;
        let vertices = &obj.vertices;
        let joint_weights = &obj.joint_weights;
        let tex_vertices = &obj.tex_vertices;
        let normals = &obj.normals;

        let geometry = &obj.geometry[0];
        let smooth_shading = geometry.smooth_shading_group;
        let mesh = &geometry.mesh[0];

        let mut deferred_vertices: Vec<deferred_color_shader::Vertex> = Vec::new();
        let mut deferred_indices: Vec<u32> = Vec::new();

        match mesh {
            collada::PrimitiveElement::Polylist(polylist) => todo!(),
            collada::PrimitiveElement::Triangles(triangles) => {
                let indices = &triangles.vertices;
                let tex_indices = triangles.tex_vertices.as_ref().unwrap();
                let normal_indices = triangles.normals.as_ref().unwrap();
                let material = &triangles.material;

                println!("");
                println!("{}", name);

                println!("");
                println!("vertices len: {:?}", vertices.len());
                // println!("vertices: {:?}", vertices);

                println!("");
                println!("tex_vertices len: {:?}", tex_vertices.len());
                // println!("tex_vertices: {:?}", tex_vertices);

                println!("");
                println!("normals len: {:?}", normals.len());
                // println!("normals: {:?}", normals);

                println!("");
                println!("indices len: {:?}", indices.len());
                // println!("indices: {:?}", indices);

                println!("");
                println!("tex_indices len: {:?}", tex_indices.len());
                // println!("tex_indices: {:?}", tex_indices);
                println!("");

                println!("normal_indices len: {:?}", normal_indices.len());
                // println!("normal_indices: {:?}", normal_indices);
                println!("");

                println!("material: {:?}", material);

                assert!(normal_indices.len() == indices.len());
                assert!(tex_indices.len() == indices.len());

                for i in 0..indices.len() {
                    let indices = indices[i];
                    let normal_indices = normal_indices[i];
                    let tex_indices = tex_indices[i];

                    let vertex0 = vertices[indices.0];
                    let vertex1 = vertices[indices.1];
                    let vertex2 = vertices[indices.2];

                    let normal0 = normals[normal_indices.0];
                    let normal1 = normals[normal_indices.1];
                    let normal2 = normals[normal_indices.2];

                    let tex_coordinate_0 = tex_vertices[tex_indices.0];
                    let tex_coordinate_1 = tex_vertices[tex_indices.1];
                    let tex_coordinate_2 = tex_vertices[tex_indices.2];

                    let indices_0 = i * 3 + 0;
                    let indices_1 = i * 3 + 1;
                    let indices_2 = i * 3 + 2;

                    let deferred_vertex0 = deferred_color_shader::Vertex {
                        position: [vertex0.x as f32, vertex0.y as f32, vertex0.z as f32],
                        normal: [normal0.x as f32, normal0.y as f32, normal0.z as f32],
                    };
                    let deferred_vertex1 = deferred_color_shader::Vertex {
                        position: [vertex1.x as f32, vertex1.y as f32, vertex1.z as f32],
                        normal: [normal1.x as f32, normal1.y as f32, normal1.z as f32],
                    };
                    let deferred_vertex2 = deferred_color_shader::Vertex {
                        position: [vertex2.x as f32, vertex2.y as f32, vertex2.z as f32],
                        normal: [normal2.x as f32, normal2.y as f32, normal2.z as f32],
                    };

                    deferred_vertices.push(deferred_vertex0);
                    deferred_vertices.push(deferred_vertex1);
                    deferred_vertices.push(deferred_vertex2);

                    deferred_indices.push(indices_0 as u32);
                    deferred_indices.push(indices_1 as u32);
                    deferred_indices.push(indices_2 as u32);
                }
            }
        }

        let instance = deferred_color_shader::Instance {
            position: [100.0, 100.0, 10.0],
            color: [0.5, 0.5, 0.8],
            entity: [99, 0, 0],
        };

        let mesh = deferred_color_shader::Mesh::new(
            self.wgpu_renderer.device(),
            &deferred_vertices,
            &deferred_indices,
            &[instance],
        );

        let element = AnimatedObject {
            is_visible: true,
            x: 0.0,
            y: 0.0,
            z: 0.0,
            instance,
            mesh,
        };

        let render_index = self.storage.elements.len();
        self.storage.elements.push(element);

        AnimatedObjectRendererResult { index: render_index }
    }

    fn set_object_position(&mut self, index: usize, x: f32, y: f32, z: f32) {
        let elem = &mut self.storage.elements[index];

        elem.x = x;
        elem.y = y;
        elem.z = y;

        elem.instance.position[0] = elem.x as f32;
        elem.instance.position[1] = elem.y as f32;
        elem.instance.position[2] = elem.y as f32;
        elem.mesh.update_instance_buffer(self.wgpu_renderer.queue(), &[elem.instance]);
    }

}
