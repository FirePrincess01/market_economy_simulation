//! Deferred shader pipeline drawing light stage
//!

mod deferred_light_sphere_shader_draw;
mod pipeline;

pub use deferred_light_sphere_shader_draw::DeferredLightSphereShaderDraw;
pub use crate::deferred_light_shader::GBufferBindGroupLayout;
pub use crate::deferred_light_shader::Mesh;
pub use pipeline::Pipeline;

pub use crate::deferred_light_shader::Instance;
pub use crate::deferred_light_shader::InstanceBuffer;
pub use crate::deferred_light_shader::Vertex;
pub use crate::deferred_light_shader::VertexBuffer;

pub use crate::deferred_light_shader::IndexBuffer;

pub use wgpu_renderer::vertex_color_shader::CameraBindGroupLayout;
