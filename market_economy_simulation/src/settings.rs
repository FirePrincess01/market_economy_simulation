//! Global settings of the application
//!

use crate::renderer;

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

    /// Size of the terrain map in both x and y dimension
    pub map_size: usize,

    /// Maximum number of instances of the point lights
    pub max_point_light_instances: usize,

    // Draws a sphere around the point lights for debugging purposes
    pub dbg_point_lights: bool,
}

impl Settings {
    pub fn new() -> Self {
        Self {
            enable_memory_mapped_read: true,
            wait_for_render_loop_to_finish: true,
            enable_vertical_sync: false,
            map_size: 300*2,
            // max_point_light_instances: 65536,
            max_point_light_instances: 16348,
            dbg_point_lights: false,
        }
    }

    pub fn get_renderer_settings(&self) -> renderer::RendererSettings {
        renderer::RendererSettings {
            enable_memory_mapped_read: self.enable_memory_mapped_read,
            wait_for_render_loop_to_finish: self.wait_for_render_loop_to_finish,
            is_vsync_enabled: self.enable_vertical_sync,
        }
    }

    pub fn get_server_settings(
        &self,
    ) -> market_economy_simulation_server::game_logic::GameLogicSettings {
        market_economy_simulation_server::game_logic::GameLogicSettings {
            map_size: self.map_size,
        }
    }
}
