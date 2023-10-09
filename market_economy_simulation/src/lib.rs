
mod ecs;
mod create_entities;
mod renderer;
mod deferred_color_shader;
mod deferred_light_shader;
mod geometry;
mod performance_monitor;

use ecs::system::DrawAgents;
use wgpu_renderer::default_window;
use winit::event::{WindowEvent, KeyboardInput, VirtualKeyCode, ElementState};

#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;


struct MarketEconomySimulation {
    size: winit::dpi::PhysicalSize<u32>,
    scale_factor: f32,

    renderer: renderer::Renderer,
    world: ecs::World,

    draw_agents: ecs::system::DrawAgents,

    // performance monitor
    performance_monitor: performance_monitor::PerformanceMonitor,
}

impl MarketEconomySimulation {
    pub async fn new(window: &winit::window::Window) -> Self 
    {
        let size = window.inner_size();
        let scale_factor = window.scale_factor() as f32;

        let mut renderer = renderer::Renderer::new(window).await;
        let mut world = ecs::World::new();

        let max_agents = 20000;
        let nr_agents =    20000;
        for _i in 0..nr_agents {
            create_entities::create_agent(&mut world);
        }

        let draw_agents = DrawAgents::new(&mut renderer.wgpu_renderer, max_agents);

        // performance monitor
        let performance_monitor = performance_monitor::PerformanceMonitor::new(&mut renderer.wgpu_renderer);

        Self {
            size,
            scale_factor,

            renderer,
            world,

            draw_agents,
            performance_monitor,
        }
    }
}

#[allow(unused)]
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

        self.performance_monitor.watch.start(3);
            ecs::system::move_agents(&mut self.world);
        self.performance_monitor.watch.stop(3);
        
        self.performance_monitor.watch.start(4);
            self.draw_agents.update(&mut self.world, &mut self.renderer.wgpu_renderer);
        self.performance_monitor.watch.stop(4);

        self.performance_monitor.update(&mut self.renderer.wgpu_renderer);
    }

    fn input(&mut self, event: &winit::event::WindowEvent) -> bool {
        self.performance_monitor.watch.start(2);
            let res = match event {
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            virtual_keycode: Some(VirtualKeyCode::F2),
                            state: ElementState::Pressed,
                            ..
                        },
                    ..
                } => { 
                    self.performance_monitor.show = !self.performance_monitor.show;
                    true
                },
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
            };
        self.performance_monitor.watch.stop(2);

        res
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        self.renderer.render(
            &self.draw_agents, 
            &mut self.performance_monitor)
    }


}



#[cfg_attr(target_arch="wasm32", wasm_bindgen(start))]
pub async fn run()
{
    let default_window = default_window::DefaultWindow::new();
    let app = MarketEconomySimulation::new(&default_window.window).await;

    default_window::run(default_window, app);
}