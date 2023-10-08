

mod mesh;
mod g_buffer;
mod pipeline;
mod deferred_color_shader_draw;

pub use mesh::Mesh;
pub use deferred_color_shader_draw::DeferredShaderMeshDraw;

pub use g_buffer::GBufferFormat;
pub use g_buffer::GBuffer;

pub use wgpu_renderer::vertex_color_shader::Vertex;
pub use wgpu_renderer::vertex_color_shader::VertexBuffer;
pub use wgpu_renderer::vertex_color_shader::Color;
pub use wgpu_renderer::vertex_color_shader::ColorBuffer;

pub use wgpu_renderer::vertex_color_shader::IndexBuffer;
pub use wgpu_renderer::vertex_color_shader::Instance;
pub use wgpu_renderer::vertex_color_shader::InstanceRaw;
pub use wgpu_renderer::vertex_color_shader::InstanceBuffer;

pub use wgpu_renderer::vertex_color_shader::CameraBindGroupLayout;
pub use wgpu_renderer::vertex_color_shader::CameraUniform;
pub use wgpu_renderer::vertex_color_shader::CameraUniformBuffer;