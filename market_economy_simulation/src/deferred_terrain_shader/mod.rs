//! Deferred shader drawing animated objects
//!

// mod deferred_animation_shader_draw;
mod mesh;
mod pipeline;
mod vertex;

// mod animation_bind_group_layout;
// mod animation_uniform;
// mod animation_uniform_buffer;

// pub use animation_bind_group_layout::AnimationBindGroupLayout;
// pub use animation_uniform::AnimationUniform;
// pub use deferred_animation_shader_draw::DeferredAnimationShaderDraw;
pub use mesh::Mesh;
pub use pipeline::Pipeline;
pub use vertex::Vertex;

pub use super::deferred_color_shader::Instance;
pub use super::deferred_color_shader::InstanceBuffer;
pub use super::deferred_color_shader::VertexBuffer;

pub use wgpu_renderer::vertex_color_shader::IndexBuffer;
