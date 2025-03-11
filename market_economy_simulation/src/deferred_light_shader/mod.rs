//! Deferred shader pipeline drawing light stage
//!

mod deferred_light_shader_draw;
mod g_buffer_bind_group_layout;
mod depth_texture_bind_group_layout;
mod mesh;
mod pipeline;

mod instance;
mod instance_buffer;

pub use deferred_light_shader_draw::DeferredLightShaderDraw;
pub use g_buffer_bind_group_layout::GBufferBindGroupLayout;
pub use depth_texture_bind_group_layout::DepthTextureBindGroup;
pub use mesh::Mesh;
pub use pipeline::Pipeline;

pub use instance::Instance;
pub use instance_buffer::InstanceBuffer;
pub use wgpu_renderer::vertex_color_shader::Vertex;
pub use wgpu_renderer::vertex_color_shader::VertexBuffer;

pub use wgpu_renderer::vertex_color_shader::IndexBuffer;

pub use wgpu_renderer::vertex_color_shader::CameraBindGroupLayout;
