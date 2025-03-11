//! Global settings of the application
//!


use crate::renderer;

pub struct Settings {
    /// On mobile, memory mapped read may may have extremly bad performance.
    /// To still render some demo application, it can be deactivated here.
    pub enable_memory_mapped_read: bool,

    /// Waits for the renderloop to finish before continuing with the regular execution.
    /// This is for determining how much calculating is done by the GPU, otherwise it will
    /// be done in the background.
    pub wait_for_renderloop_to_finish: bool,

    /// Size of the terrain map in both x and y dimension
    pub map_size: usize,
}

impl Settings {
    pub fn new() -> Self {
        Self {
            enable_memory_mapped_read: true,
            wait_for_renderloop_to_finish: false,
            map_size: 300,
        }
    }

    pub fn get_renderer_settings(&self) -> renderer::RendererSettings {
        renderer::RendererSettings{
            enable_memory_mapped_read: self.enable_memory_mapped_read,
            wait_for_renderloop_to_finish: self.wait_for_renderloop_to_finish,
        }
    }

    pub fn get_server_settings(&self) ->  market_economy_simulation_server::game_logic::GameLogicSettings 
    {
        market_economy_simulation_server::game_logic::GameLogicSettings {
            map_size: self.map_size,
        }
    }
}



