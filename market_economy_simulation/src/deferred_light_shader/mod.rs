//! Deferred shader pipeline drawing light stage
//!

mod deferred_light_shader_draw;
mod depth_texture_bind_group_layout;
mod g_buffer_bind_group_layout;
mod mesh;
mod pipeline;

mod instance;

pub use deferred_light_shader_draw::DeferredLightShaderDraw;
pub use g_buffer_bind_group_layout::GBufferBindGroupLayout;
pub use mesh::Mesh;
pub use pipeline::Pipeline;

pub use instance::Instance;
pub use wgpu_renderer::vertex_color_shader::instance_buffer::InstanceBuffer;
pub use wgpu_renderer::vertex_color_shader::Vertex;
pub use wgpu_renderer::vertex_color_shader::VertexBuffer;

pub use wgpu_renderer::vertex_color_shader::IndexBuffer;

pub use wgpu_renderer::vertex_color_shader::CameraBindGroupLayout;


pub fn calculate_volume_radius(light_color: &cgmath::Vector3<f32>) -> f64 {
    // https://learnopengl.com/Advanced-Lighting/Deferred-Shading


    // Distance 	Constant 	Linear 	Quadratic
    // 7        	1.0     	0.7 	1.8
    // 13        	1.0     	0.35 	0.44
    // 20        	1.0     	0.22 	0.20
    // 32        	1.0     	0.14 	0.07
    // 50        	1.0     	0.09 	0.032
    // 65        	1.0     	0.07 	0.017
    // 100        	1.0     	0.045 	0.0075
    // 160        	1.0     	0.027 	0.0028
    // 200        	1.0     	0.022 	0.0019
    // 325        	1.0     	0.014 	0.0007
    // 600        	1.0     	0.007 	0.0002
    // 3250        	1.0     	0.0014 	0.000007

    let constant = 1.0;
    let linear = 0.7;
    let quadratic = 1.8;

    let light_max = light_color.x.max(light_color.y).max(light_color.z) as f64;

    let attenuation = 256.0 / 5.0;

    let radius = (-linear
        + (linear * linear - 4.0 * quadratic * (constant - attenuation * light_max)).sqrt())
        / (2.0 * quadratic);

    radius
}
