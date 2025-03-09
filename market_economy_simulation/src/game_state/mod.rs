//! Mirrors and synchronises the state of the game_server
//!

mod terrain;

use wgpu_renderer::renderer::WgpuRendererInterface;

use crate::deferred_terrain_shader;

pub struct GameState {
    // values form the game server
    _terrain_server: market_economy_simulation_server::terrain::Terrain,

    // derived values
    _terrain_vertices: Vec<deferred_terrain_shader::Vertex>,
    _terrain_indices: Vec<u32>,
    _terrain_instance: deferred_terrain_shader::Instance,
    pub terrain_mesh: deferred_terrain_shader::Mesh,
}

impl GameState {
    pub fn new(
        renderer: &mut dyn WgpuRendererInterface,
        terrain_server: market_economy_simulation_server::terrain::Terrain,
    ) -> Self {
        let (terrain_vertices, terrain_indices) = terrain::create_vertices(&terrain_server);
        let terrain_instance = deferred_terrain_shader::Instance {
            position: [0.0, 0.0, 1.0],
            color: [0.1, 0.1, 0.1],
            entity: [72, 0, 0],
            color_heighlights: [0.3, 0.05, 0.3],
        };
        let terrain_mesh = deferred_terrain_shader::Mesh::new(
            renderer.device(),
            &terrain_vertices,
            &terrain_indices,
            &[terrain_instance],
        );

        Self {
            _terrain_server: terrain_server,
            _terrain_vertices: terrain_vertices,
            _terrain_indices: terrain_indices,
            _terrain_instance: terrain_instance,
            terrain_mesh,
        }
    }
}
