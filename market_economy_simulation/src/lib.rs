
mod ecs;
mod create_entities;
mod default_window;
mod renderer;

#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;
use wgpu_renderer::{performance_monitor, vertex_color_shader};
use winit::event::{WindowEvent, KeyboardInput};


struct MarketEconomySimulation {
    size: winit::dpi::PhysicalSize<u32>,
    scale_factor: f32,

    renderer: renderer::Renderer,
    world: ecs::World,

    // performance monitor
    // watch: performance_monitor::Watch<4>,
    // graph_host: performance_monitor::Graph,
    // graph_device: vertex_color_shader::Mesh,
}

impl MarketEconomySimulation {
    pub async fn new(window: &winit::window::Window) -> Self 
    {
        let size = window.inner_size();
        let scale_factor = window.scale_factor() as f32;

        let renderer = renderer::Renderer::new(window).await;
        let mut world = ecs::World::new();

        for i in 0..100 {
            create_entities::create_agent(&mut world);
        }

        // performance monitor
        // const WATCHPOINTS_SIZE: usize  = 4;
        // let watch: performance_monitor::Watch<WATCHPOINTS_SIZE> = performance_monitor::Watch::new(); 
        // let graph_host = performance_monitor::Graph::new(WATCHPOINTS_SIZE);
        // let graph_instance = vertex_color_shader::Instance{
        //     position: glam::Vec3::ZERO,
        //     rotation: glam::Quat::IDENTITY,
        // };
        // let graph_instances = [graph_instance];
        // let graph_device = vertex_color_shader::Mesh::new(
        //     wgpu_renderer.device(),
        //     graph_host.vertices.as_slice(),
        //     graph_host.colors.as_slice(),
        //     graph_host.indices.as_slice(),
        //     &graph_instances,
        // );

        Self {
            size,
            scale_factor,

            renderer,
            world,
        }
    }
}

fn apply_scale_factor(position: winit::dpi::PhysicalPosition<f64>, scale_factor: f32) 
-> winit::dpi::PhysicalPosition<f64> 
{
    cfg_if::cfg_if! {
        // apply scale factor for the web
        if #[cfg(target_arch = "wasm32")] {
            let mut res = position;
            res.x = res.x / scale_factor as f64;
            res.y = res.y / scale_factor as f64;
            res
        }
        else {
            position
        }
    }
}

impl default_window::DefaultWindowApp for MarketEconomySimulation 
{
    fn get_size(&self) -> winit::dpi::PhysicalSize<u32> {
        self.size
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        self.renderer.resize(new_size);
    }

    fn update_scale_factor(&mut self, scale_factor: f32) {
        self.scale_factor = scale_factor;
    }

    fn update(&mut self, dt: instant::Duration) {
        self.renderer.update(dt);
        ecs::system::move_agents(&mut self.world);
    }

    fn input(&mut self, event: &winit::event::WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        virtual_keycode: Some(key),
                        state,
                        ..
                    },
                ..
            } => self.renderer.process_keyboard(*key, *state),
            WindowEvent::MouseWheel { delta, .. } => {
                self.renderer.process_scroll(delta);
                true
            }
            _ => false,
        }
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        ecs::system::draw_meshes(&self.world, &mut self.renderer)
    }


}



#[cfg_attr(target_arch="wasm32", wasm_bindgen(start))]
pub async fn run()
{
    let default_window = default_window::DefaultWindow::new();
    let app = MarketEconomySimulation::new(&default_window.window).await;

    default_window::run(default_window, app);
}