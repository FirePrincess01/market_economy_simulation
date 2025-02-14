//! Deferred shader drawing colored objects
//!

mod deferred_color_shader_draw;
mod g_buffer;
mod mesh;
mod pipeline;

mod instance;
mod instance_buffer;
mod vertex;
mod vertex_buffer;

mod entity_buffer;
mod entity_buffer_slice;

pub use deferred_color_shader_draw::DeferredShaderDraw;
pub use entity_buffer::EntityBuffer;
pub use g_buffer::GBuffer;
pub use mesh::Mesh;
pub use pipeline::Pipeline;

pub use instance::Instance;
pub use instance_buffer::InstanceBuffer;
pub use vertex::Vertex;
pub use vertex_buffer::VertexBuffer;

pub use wgpu_renderer::vertex_color_shader::IndexBuffer;

pub use wgpu_renderer::vertex_color_shader::CameraBindGroupLayout;
