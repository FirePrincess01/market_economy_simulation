//! Deferred shader drawing colored objects
//!

mod pipeline;
mod g_buffer;
mod mesh;
mod deferred_color_shader_draw;

mod vertex;
mod vertex_buffer;
mod instance;
mod instance_buffer;

mod entity_buffer;
mod entity_buffer_slice;

pub use pipeline::Pipeline;
pub use g_buffer::GBuffer;
pub use entity_buffer::EntityBuffer;
pub use mesh::Mesh;
pub use deferred_color_shader_draw::DeferredShaderDraw;

pub use vertex::Vertex;
pub use vertex_buffer::VertexBuffer;
pub use instance::Instance;
pub use instance_buffer::InstanceBuffer;

pub use wgpu_renderer::vertex_color_shader::IndexBuffer;

pub use wgpu_renderer::vertex_color_shader::CameraBindGroupLayout;
