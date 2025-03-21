//! Global settings of the application
//!

use crate::{renderer, terrain_storage::TerrainSettings};

pub struct Settings {
    /// On mobile, memory mapped read may may have extremely bad performance.
    /// To still render some demo application, it can be deactivated here.
    pub enable_memory_mapped_read: bool,

    /// Waits for the render loop to finish before continuing with the regular execution.
    /// This is for determining how much calculating is done by the GPU, otherwise it will
    /// be done in the background.
    pub wait_for_render_loop_to_finish: bool,

    // enables vertical sync, limiting the fps to the refresh rate of the display (60 fps)
    pub enable_vertical_sync: bool,

    // Fast approximate anti-aliasing. Enables an edge blurring post processing filter
    pub enable_fxaa: bool,

    // The resolution of the window
    pub window_resolution: (u32, u32),

    // enables multithreading if available (no available for the web)
    pub enable_multithreading: bool,

    /// Size of the terrain map in both x and y dimension
    pub map_size: usize,

    /// Maximum number of instances of the point lights
    pub max_point_light_instances: usize,

    // Draws a sphere around the point lights for debugging purposes
    pub dbg_point_lights: bool,

    // The number of tiles in the terrain
    pub nr_tiles: usize,

    // The maximum depth of the lod quad tree
    pub max_depth: usize,
}

impl Settings {
    pub fn new() -> Self {
        Self {
            // render settings
            enable_memory_mapped_read: true,
            wait_for_render_loop_to_finish: true,
            enable_vertical_sync: false,
            enable_fxaa: true,
            window_resolution: (1920 / 2, 1080 / 2),

            // game server settings
            map_size: 10,
            enable_multithreading: true,

            // miscellaneous
            // max_point_light_instances: 65536,
            max_point_light_instances: 16348,
            dbg_point_lights: false,

            // maximum for the web
            nr_tiles: 64,
            max_depth: 8,

            // nr_tiles: 8,
            // max_depth: 1,
        }
    }

    pub fn get_renderer_settings(&self) -> renderer::RendererSettings {
        renderer::RendererSettings {
            enable_memory_mapped_read: self.enable_memory_mapped_read,
            wait_for_render_loop_to_finish: self.wait_for_render_loop_to_finish,
            enable_vertical_sync: self.enable_vertical_sync,
            enable_fxaa: self.enable_fxaa,
            window_resolution: self.window_resolution,
        }
    }

    pub fn get_server_settings(
        &self,
    ) -> market_economy_simulation_server::game_logic::GameLogicSettings {
        market_economy_simulation_server::game_logic::GameLogicSettings {
            map_size: self.map_size,
            enable_multithreading: self.enable_multithreading,
        }
    }

    pub fn get_terrain_settings(&self) -> TerrainSettings {
        TerrainSettings {
            nr_tiles: self.nr_tiles,
            max_depth: self.max_depth,
        }
    }
}
