
use wgpu_renderer::vertex_color_shader::Vertex as Vertex;
use wgpu_renderer::vertex_color_shader::Color as Color;

use crate::deferred_color_shader;


pub struct Quad {
    pub vertices: Vec<Vertex>,
    pub colors: Vec<Color>,
    pub indices: Vec<u32>,

    pub deferred_vertices: Vec::<deferred_color_shader::Vertex>,
}

impl Quad {
    pub fn new(size: f32) -> Self
    {
        let vertices = vec![
            Vertex { position: [0.0, 0.0, 0.0] }, // A
            Vertex { position: [size, 0.0, 0.0] }, // B
            Vertex { position: [size, size, 0.0] }, // C
            Vertex { position: [0.0, size, 0.0] }, // D
        ];

        let color = Color { color: [0.2, 0.2, 0.2] };
        let colors = vec![
            color, // A
            color, // B
            color, // C
            color, // D
        ];

        let indices = vec![ 
            0, 1, 2,
            2, 3, 0,  
        ];

        let normal = [0.0, 0.0, 1.0];
        let deferred_vertices = vec![
            deferred_color_shader::Vertex { position: [0.0, 0.0, 0.0], normal: normal }, // A
            deferred_color_shader::Vertex { position: [size, 0.0, 0.0], normal: normal }, // B
            deferred_color_shader::Vertex { position: [size, size, 0.0], normal: normal }, // C
            deferred_color_shader::Vertex { position: [0.0, size, 0.0], normal: normal }, // D
        ];

        Self {
            vertices,
            colors,
            indices,

            deferred_vertices,
        }
    }
}