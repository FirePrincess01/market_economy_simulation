//! Contains the position and the of an object
//!

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Instance {
    pub position: [f32; 3],
    pub color: [f32; 3],
    pub entity: [u32; 3],
    pub color_highlights: [f32; 3],
}

impl Instance {
    pub fn _new() -> Self {
        Self {
            position: [0.0, 0.0, 0.0],
            color: [0.0, 0.0, 0.0],
            entity: [0, 0, 0],
            color_highlights: [0.0, 0.0, 0.0],
        }
    }

    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Instance>() as wgpu::BufferAddress,
            // We need to switch from using a step mode of Vertex to Instance
            // This means that our shaders will only change to use the next
            // instance when the shader stats processing a new instance
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    // While our vertex shader only uses locations 0, and 1 now, in later tutorials we'll
                    // be using 2, 3, and 4, for Vertex. We'll start at slot 5 not conflict with them later
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32x3,
                },
                // A mat4 takes up 4 vertex slots as it is technically 4 vec4s. We need to define a slot
                // for each vec4. We'll have to reassemble the mat4 in the shader.
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 6,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3 * 2]>() as wgpu::BufferAddress,
                    shader_location: 7,
                    format: wgpu::VertexFormat::Uint32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3 * 3]>() as wgpu::BufferAddress,
                    shader_location: 8,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}
