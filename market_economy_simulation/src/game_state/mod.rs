//! Mirrors and synchronises the state of the game_server
//!

mod terrain;

use wgpu_renderer::renderer::WgpuRendererInterface;

use crate::deferred_terrain_shader;

pub struct GameState {
    // values form the game server
    terrain_server: market_economy_simulation_server::terrain::Terrain,

    // derived values
    terrain_vertices: Vec<deferred_terrain_shader::Vertex>,
    terrain_indices: Vec<u32>,
    terrain_instance: deferred_terrain_shader::Instance,
    pub terrain_mesh: deferred_terrain_shader::Mesh,
}

impl GameState {
    pub fn new(
        renderer: &mut dyn WgpuRendererInterface,
        terrain_server: market_economy_simulation_server::terrain::Terrain,
    ) -> Self {
        let (terrain_vertices, terrain_indices) = terrain::create_vertices(&terrain_server);
        let terrain_instance = deferred_terrain_shader::Instance {
            position: [10.0, 10.0, 1.0],
            color: [0.1, 0.1, 0.1],
            entity: [72, 0, 0],
            color_heighlights: [1.0, 0.2, 1.0],
        };
        let terrain_mesh = deferred_terrain_shader::Mesh::new(
            renderer.device(),
            &terrain_vertices,
            &terrain_indices,
            &[terrain_instance],
        );

        Self {
            terrain_server,
            terrain_vertices,
            terrain_indices,
            terrain_instance,
            terrain_mesh,
        }
    }
}
