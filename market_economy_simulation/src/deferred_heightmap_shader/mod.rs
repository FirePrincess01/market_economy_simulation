//! A general purpose pipeline using vertices, textures, a heightmap and instances
//!

mod vertex;
mod instance;
mod mesh;
mod pipeline;
mod deferred_heightmap_shader_draw;

pub use mesh::Mesh;
pub use pipeline::Pipeline;
pub use deferred_heightmap_shader_draw::DeferredHeightMapShaderDraw;

pub use wgpu_renderer::vertex_color_shader::IndexBuffer;

pub use vertex::Vertex;
pub use super::deferred_color_shader::VertexBuffer;

pub use instance::Instance;
pub use super::deferred_color_shader::InstanceBuffer;

pub use super::vertex_texture_shader::Texture;
pub use super::vertex_texture_shader::TextureBindGroupLayout;

pub use wgpu_renderer::vertex_heightmap_shader::Heightmap;
pub use wgpu_renderer::vertex_heightmap_shader::Heightmap2D;
pub use wgpu_renderer::vertex_heightmap_shader::HeightmapBindGroupLayout;
pub use wgpu_renderer::vertex_heightmap_shader::HeightmapTexture;

pub use super::deferred_color_shader::EntityBuffer;

pub use crate::deferred_color_shader::GBuffer;

pub use wgpu_renderer::vertex_color_shader::CameraBindGroupLayout;



