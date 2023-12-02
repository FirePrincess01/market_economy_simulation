
mod ecs;
mod create_entities;
mod renderer;
mod deferred_color_shader;
mod deferred_light_shader;
mod geometry;
mod performance_monitor;
mod ground_plane;

use ecs::system::DrawAgents;
use ground_plane::GroundPlaneMesh;
use wgpu_renderer::{default_window, vertex_texture_shader};
use winit::event::{WindowEvent, KeyboardInput, VirtualKeyCode, ElementState};
use rusttype;

#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;

use crate::ground_plane::{GroundPlane, GroundResource};


struct MarketEconomySimulation {
    size: winit::dpi::PhysicalSize<u32>,
    scale_factor: f32,

    renderer: renderer::Renderer,
    world: ecs::World,

    _ground_plane: ground_plane::GroundPlane,
    ground_plane_mesh: GroundPlaneMesh,

    draw_agents: ecs::system::DrawAgents,

    // performance monitor
    performance_monitor: performance_monitor::PerformanceMonitor,

    // show the entity index
    mouse_pos_y: u32,
    mouse_pos_x: u32,
    entity_index: u32,
    font: rusttype::Font<'static>,
    entity_index_label: wgpu_renderer::label::Label,
    entity_index_mesh: wgpu_renderer::label::LabelMesh,
}

impl MarketEconomySimulation {
    pub async fn new(window: &winit::window::Window) -> Self 
    {
        let size = window.inner_size();
        let scale_factor = window.scale_factor() as f32;

        let mut renderer = renderer::Renderer::new(window).await;
        let mut world = ecs::World::new();

        let max_agents = 50000;
        let nr_agents =    50000;
        for _i in 0..nr_agents {
            create_entities::create_agent(&mut world);
        }

        // ground plane
        let ground_plane_width = 100;
        let ground_plane_height = 100;
        let mut ground_plane = GroundPlane::new(ground_plane_width, ground_plane_height);
        ground_plane.generate_resource(0.005, GroundResource::Red);
        ground_plane.generate_resource(0.01, GroundResource::Blue);
        ground_plane.generate_resource(0.001, GroundResource::Green);

        // ground plane mesh
        let ground_plane_mesh = GroundPlaneMesh::new(
            &mut renderer.wgpu_renderer, 
            &ground_plane);

        // agents
        let draw_agents = DrawAgents::new(&mut renderer.wgpu_renderer, max_agents);

        // performance monitor
        let performance_monitor = performance_monitor::PerformanceMonitor::new(&mut renderer.wgpu_renderer);

        // show the entity index
        let mouse_pos_y =  0;
        let mouse_pos_x =  0;

        let font = wgpu_renderer::freefont::create_font_free_mono();
        let entity_index_label =  wgpu_renderer::label::Label::new(
            &font, 
            32.0, 
            "47114711"
        );
        let mut entity_index_instance = vertex_texture_shader::Instance::zero();
        entity_index_instance.position.x = 20.0;
        entity_index_instance.position.y = 120.0;
        let entity_index_mesh =  wgpu_renderer::label::LabelMesh::new(
            &mut renderer.wgpu_renderer, 
            entity_index_label.get_image(), 
            &renderer.texture_bind_group_layout,
            &entity_index_instance);

        Self {
            size,
            scale_factor,

            renderer,
            world,

            _ground_plane: ground_plane,
            ground_plane_mesh,

            draw_agents,
            
            performance_monitor,

            mouse_pos_y,
            mouse_pos_x,
            entity_index: 0,
            font,
            entity_index_label,
            entity_index_mesh,
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

        // update entity index label
        let text = self.entity_index.to_string();
        self.entity_index_label.update(&self.font, &text);
        self.entity_index_mesh.update_texture(&mut self.renderer.wgpu_renderer.queue(), self.entity_index_label.get_image());

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
                WindowEvent::CursorMoved { position, .. } => {
                    let pos = apply_scale_factor(*position, self.scale_factor);
                    
                    self.mouse_pos_y = pos.y as u32;
                    self.mouse_pos_x = pos.x as u32;
                    true
                }
                _ => false,
            };
        self.performance_monitor.watch.stop(2);

        res
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {

        // will be used for the next frame
        self.entity_index = self.renderer.read_entity_index(self.mouse_pos_y, self.mouse_pos_x);

        // render current frame
        self.renderer.render(   
            &self.ground_plane_mesh,
            &self.ground_plane_mesh,
            &self.draw_agents, 
            &self.entity_index_mesh,
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