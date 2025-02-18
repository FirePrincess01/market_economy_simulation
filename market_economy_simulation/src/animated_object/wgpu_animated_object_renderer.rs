use cgmath::Vector3;
use collada::document::ColladaDocument;
use wgpu_renderer::renderer::WgpuRendererInterface;

use crate::{animated_object::animated_model::animation::Animation, deferred_animation_shader::{self, DeferredAnimationShaderDraw}};

use super::{animated_model::skeleton::Skeleton, animated_object_renderer::{AnimatedObjectRenderer, AnimatedObjectRendererResult}};

pub struct AnimatedObject {
    pub is_visible: bool,
    // pub update_position: bool,
    // pub update_text: bool,
    pub x: f32,
    pub y: f32,
    pub z: f32,

    pub animation: Animation,

    pub instance: deferred_animation_shader::Instance,
    pub animation_uniform: deferred_animation_shader::AnimationUniform,

    mesh: deferred_animation_shader::Mesh,
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

    pub fn update(
        &mut self,
        renderer_interface: &mut dyn WgpuRendererInterface,
        dt: &instant::Duration,
    ) {
        for elem in &mut self.elements {
            elem.animation.increment_time(dt);
            elem.animation.update_animation_uniform(&mut elem.animation_uniform);
            elem.mesh.update_animation_buffer(renderer_interface.queue(), &elem.animation_uniform);
        }
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
    pub animation_bind_group_layout: &'a deferred_animation_shader::AnimationBindGroupLayout,
    // pub texture_bind_group_layout: &'a wgpu_renderer::vertex_texture_shader::TextureBindGroupLayout,
}

impl<'a> WgpuAnimatedObjectRenderer<'a> {
    fn create_mesh(
        &mut self,
        obj_set: &collada::ObjSet,
        animation_uniform: &deferred_animation_shader::AnimationUniform,
    ) -> (
        deferred_animation_shader::Instance,
        deferred_animation_shader::Mesh,
    ) {
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

        let mut deferred_vertices: Vec<deferred_animation_shader::Vertex> = Vec::new();
        let mut deferred_indices: Vec<u32> = Vec::new();

        match mesh {
            collada::PrimitiveElement::Polylist(polylist) => todo!(),
            collada::PrimitiveElement::Triangles(triangles) => {
                let indices: &Vec<(usize, usize, usize)> = &triangles.vertices;
                let tex_indices: &Vec<(usize, usize, usize)> =
                    triangles.tex_vertices.as_ref().unwrap();
                let normal_indices: &Vec<(usize, usize, usize)> =
                    triangles.normals.as_ref().unwrap();
                let material: &Option<String> = &triangles.material;

                println!("********");
                println!("mesh");
                println!("********");

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

                println!("normal_indices len: {:?}", normal_indices.len());
                // println!("normal_indices: {:?}", normal_indices);

                println!("");
                println!("material: {:?}", material);

                println!("");
                println!("joint_weights len: {:?}", joint_weights.len());
                println!("joint_weights: {:?}", joint_weights);

                println!("");

                assert!(normal_indices.len() == indices.len());
                assert!(tex_indices.len() == indices.len());

                for i in 0..indices.len() {
                    let indices = indices[i];
                    let normal_indices = normal_indices[i];
                    let tex_indices = tex_indices[i];

                    let vertex0 = vertices[indices.0];
                    let vertex1 = vertices[indices.1];
                    let vertex2 = vertices[indices.2];

                    let joint0 = joint_weights[indices.0];
                    let joint1 = joint_weights[indices.1];
                    let joint2 = joint_weights[indices.2];

                    let normal0 = normals[normal_indices.0];
                    let normal1 = normals[normal_indices.1];
                    let normal2 = normals[normal_indices.2];

                    let tex_coordinate_0 = tex_vertices[tex_indices.0];
                    let tex_coordinate_1 = tex_vertices[tex_indices.1];
                    let tex_coordinate_2 = tex_vertices[tex_indices.2];

                    let indices_0 = i * 3 + 0;
                    let indices_1 = i * 3 + 1;
                    let indices_2 = i * 3 + 2;

                    let deferred_vertex0 = deferred_animation_shader::Vertex {
                        position: [vertex0.x as f32, vertex0.y as f32, vertex0.z as f32],
                        normal: [normal0.x as f32, normal0.y as f32, normal0.z as f32],
                        joint_indices: [
                            joint0.joints[0] as u32,
                            joint0.joints[1] as u32,
                            joint0.joints[2] as u32,
                            joint0.joints[3] as u32,
                        ],
                        joint_weights: [
                            joint0.weights[0],
                            joint0.weights[1],
                            joint0.weights[2],
                            joint0.weights[3],
                        ],
                    };
                    let deferred_vertex1 = deferred_animation_shader::Vertex {
                        position: [vertex1.x as f32, vertex1.y as f32, vertex1.z as f32],
                        normal: [normal1.x as f32, normal1.y as f32, normal1.z as f32],
                        joint_indices: [
                            joint1.joints[0] as u32,
                            joint1.joints[1] as u32,
                            joint1.joints[2] as u32,
                            joint1.joints[3] as u32,
                        ],
                        joint_weights: [
                            joint1.weights[0],
                            joint1.weights[1],
                            joint1.weights[2],
                            joint1.weights[3],
                        ],
                    };
                    let deferred_vertex2 = deferred_animation_shader::Vertex {
                        position: [vertex2.x as f32, vertex2.y as f32, vertex2.z as f32],
                        normal: [normal2.x as f32, normal2.y as f32, normal2.z as f32],
                        joint_indices: [
                            joint2.joints[0] as u32,
                            joint2.joints[1] as u32,
                            joint2.joints[2] as u32,
                            joint2.joints[3] as u32,
                        ],
                        joint_weights: [
                            joint2.weights[0],
                            joint2.weights[1],
                            joint2.weights[2],
                            joint2.weights[3],
                        ],
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

        let instance = deferred_animation_shader::Instance {
            position: [100.0, 100.0, 10.0],
            color: [0.5, 0.5, 0.8],
            entity: [99, 0, 0],
        };

        let mesh = deferred_animation_shader::Mesh::new(
            self.wgpu_renderer,
            &self.animation_bind_group_layout,
            &deferred_vertices,
            &*animation_uniform,
            &deferred_indices,
            &[instance],
        );

        (instance, mesh)
    }

    fn create_skeleton(&mut self, collada_skeleton: &collada::Skeleton) -> Skeleton {
        let joints = &collada_skeleton.joints;
        let bind_poses = &collada_skeleton.bind_poses;

        println!("********");
        println!("Skeleton");
        println!("********");

        println!("");
        println!("joints len: {:?}", joints.len());
        println!("joints: {:?}", joints);

        println!("");
        println!("bind_poses len: {:?}", bind_poses.len());
        println!("bind_poses: {:?}", bind_poses);

        let skeleton = Skeleton::new(collada_skeleton);

        skeleton
    }

    fn create_animation(&mut self, skeleton: &Skeleton, collada_animation: &collada::Animation) -> Animation {
        let target: &String = &collada_animation.target;
        let sample_times: &Vec<f32> = &collada_animation.sample_times;
        let sample_poses: &Vec<[[f32; 4]; 4]> = &collada_animation.sample_poses;

        println!("********");
        println!("Animation");
        println!("********");

        println!("");
        println!("target {:}", target);

        println!("");
        println!("sample_times len: {:?}", sample_times.len());
        println!("sample_times: {:?}", sample_times);

        println!("");
        println!("sample_poses len: {:?}", sample_poses.len());
        println!("sample_poses: {:?}", sample_poses);

        let animation = Animation::new(skeleton, target, sample_times, sample_poses);

        animation
    }
}

impl<'a> AnimatedObjectRenderer for WgpuAnimatedObjectRenderer<'a> {
    fn from_collada(
        &mut self,
        xml_string: &str,
    ) -> super::animated_object_renderer::AnimatedObjectRendererResult {
        let collada_document: ColladaDocument = ColladaDocument::from_str(xml_string).unwrap();

        let obj_set: collada::ObjSet = collada_document.get_obj_set().unwrap();
        let collada_skeletons: Vec<collada::Skeleton> = collada_document.get_skeletons().unwrap();
        let collada_animations: Vec<collada::Animation> = collada_document.get_animations().unwrap();

        let collada_skeleton_0 = &collada_skeletons[0];
        let skeleton = self.create_skeleton(collada_skeleton_0);

        let collada_animation_0 = &collada_animations[0];
        let animation = self.create_animation(&skeleton, collada_animation_0);

        let mut animation_uniform = deferred_animation_shader::AnimationUniform::zero();

        animation_uniform.joint_transform[1] = cgmath::Matrix4::from_translation(Vector3{
            x: 2.0,
            y: 0.0,
            z: 0.0,
        }).into();

        let (instance, mesh) = self.create_mesh(&obj_set, &animation_uniform);

        let element = AnimatedObject {
            is_visible: true,
            x: 0.0,
            y: 0.0,
            z: 0.0,
            instance,
            animation_uniform,
            animation,

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
        elem.mesh
            .update_instance_buffer(self.wgpu_renderer.queue(), &[elem.instance]);
    }
}
