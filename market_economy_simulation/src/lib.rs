mod animated_object;
mod ant;
mod base_factory;
mod create_entities;
mod deferred_animation_shader;
mod deferred_color_shader;
mod deferred_light_shader;
mod deferred_terrain_shader;
mod ecs;
mod ecs2;
mod game_state;
mod geometry;
mod ground_plane;
mod performance_monitor;
mod renderer;
mod settings;
mod shape;
mod world_mesh;

use animated_object::wgpu_animated_object_renderer::{
    WgpuAnimatedObjectRenderer, WgpuAnimatedObjectStorage,
};
use market_economy_simulation_server::game_logic::game_logic_interface::{
    GameLogicInterface, GameLogicMessageHeavy, GameLogicMessageRequest,
};
use wgpu_renderer::{
    default_application::{DefaultApplication, DefaultApplicationInterface},
    vertex_texture_shader,
    wgpu_renderer::WgpuRendererInterface,
};
use winit::event::{ElementState, WindowEvent};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

struct MarketEconomySimulation {
    _settings: settings::Settings,

    size: winit::dpi::PhysicalSize<u32>,
    scale_factor: f32,

    renderer: renderer::Renderer,
    _world: ecs2::World,
    // world_mesh: world_mesh::WorldMesh,

    // ant: deferred_color_shader::Mesh,
    animated_object_storage: WgpuAnimatedObjectStorage,

    // performance monitor
    performance_monitor: performance_monitor::PerformanceMonitor,

    // show the entity index
    mouse_pos_y: u32,
    mouse_pos_x: u32,
    entity_index: u32,
    font: rusttype::Font<'static>,
    entity_index_label: wgpu_renderer::label::Label,
    entity_index_mesh: wgpu_renderer::label::LabelMesh,

    game_server: market_economy_simulation_server::GameLogicServer,
    game_state: game_state::GameState,

    ant: ant::Ant,

    ambient_light_quad: deferred_light_shader::Mesh, // Quad running the global ambient light shader
}

impl MarketEconomySimulation {
    pub fn new(
        renderer_interface: &mut dyn WgpuRendererInterface,
        size: winit::dpi::PhysicalSize<u32>,
        scale_factor: f32,
    ) -> Self {
        let settings = settings::Settings::new();

        // let size: winit::dpi::PhysicalSize<u32> = window.inner_size();
        // let scale_factor = window.scale_factor() as f32;

        let renderer =
            renderer::Renderer::new(renderer_interface, settings.get_renderer_settings());

        // world
        let mut world = ecs2::World::new();

        let blue_token = world.resources.blues2.create(0.0, 1.0, 1.0);

        world
            .base_factory
            .add_blue(blue_token, &mut world.resources);
        world
            .base_factory
            .produce(&mut world.resources, &mut world.agents);

        // world mesh
        let _world_mesh = world_mesh::WorldMesh::new(renderer_interface, &world);

        let mut animated_object_storage = WgpuAnimatedObjectStorage::new();

        // performance monitor
        let performance_monitor = performance_monitor::PerformanceMonitor::new(renderer_interface);

        // show the entity index
        let mouse_pos_y = 0;
        let mouse_pos_x = 0;

        let font = wgpu_renderer::freefont::create_font_free_mono();
        let entity_index_label = wgpu_renderer::label::Label::new(&font, 32.0, "47114711");
        let mut entity_index_instance = vertex_texture_shader::Instance::zero();
        entity_index_instance.position.x = 20.0;
        entity_index_instance.position.y = 120.0;

        let entity_index_mesh = wgpu_renderer::label::LabelMesh::new(
            renderer_interface,
            entity_index_label.get_image(),
            &renderer.texture_bind_group_layout,
            &entity_index_instance,
        );

        // create ant
        let mut animated_object_renderer = WgpuAnimatedObjectRenderer {
            storage: &mut animated_object_storage,
            wgpu_renderer: renderer_interface,
            animation_bind_group_layout: &renderer.animation_bind_group_layout,
        };

        let glb_bin = include_bytes!("../res/ant_0_8.glb");
        // let glb_bin = include_bytes!("../res/wiggle_tower2.glb");
        animated_object_renderer.create_from_glb(glb_bin);

        // create game server
        let mut game_server =
            market_economy_simulation_server::GameLogicServer::new(settings.get_server_settings());

        game_server
            .send_messages()
            .send(GameLogicMessageRequest::GetTerrain)
            .unwrap();
        game_server.update();

        // get base values from game server
        #[allow(clippy::needless_late_init)]
        let terrain_server: market_economy_simulation_server::terrain::Terrain;
        let msg: GameLogicMessageHeavy = game_server.get_heavy_messages().recv().unwrap();
        match msg {
            GameLogicMessageHeavy::Terrain(terrain) => {
                terrain_server = terrain;
            }
        }

        // create the game state
        let game_state = game_state::GameState::new(renderer_interface, terrain_server);

        let ant = ant::Ant::new(renderer_interface);

        let ambient_light_quad_vertices = geometry::Quad::new(2.0);
        let amblient_light_quad_instance = deferred_light_shader::Instance {
            position: [-1.0, -1.0, 0.1],
            intensity: [0.4, 0.4, 0.4],
        };
        let ambient_light_quad = deferred_light_shader::Mesh::new(
            renderer_interface.device(),
            &ambient_light_quad_vertices.vertices,
            &ambient_light_quad_vertices.indices,
            &[amblient_light_quad_instance],
        );

        Self {
            _settings: settings,

            size,
            scale_factor,

            renderer,

            _world: world,
            // world_mesh,
            animated_object_storage,

            performance_monitor,

            mouse_pos_y,
            mouse_pos_x,
            entity_index: 0,
            font,
            entity_index_label,
            entity_index_mesh,

            game_server,
            game_state,

            ant,

            ambient_light_quad,
        }
    }
}

#[allow(unused)]
fn apply_scale_factor(
    position: winit::dpi::PhysicalPosition<f64>,
    scale_factor: f32,
) -> winit::dpi::PhysicalPosition<f64> {
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

impl DefaultApplicationInterface for MarketEconomySimulation {
    fn create(
        renderer_interface: &mut dyn WgpuRendererInterface,
        size: winit::dpi::PhysicalSize<u32>,
        scale_factor: f32,
    ) -> Self {
        Self::new(renderer_interface, size, scale_factor)
    }

    fn get_size(&self) -> winit::dpi::PhysicalSize<u32> {
        self.size
    }

    fn resize(
        &mut self,
        renderer_interface: &mut dyn WgpuRendererInterface,
        new_size: winit::dpi::PhysicalSize<u32>,
    ) {
        self.size = new_size;
        self.renderer.resize(renderer_interface, new_size);
    }

    fn update_scale_factor(&mut self, scale_factor: f32) {
        self.scale_factor = scale_factor;
    }

    fn update(
        &mut self,
        renderer_interface: &mut dyn WgpuRendererInterface,
        dt: instant::Duration,
    ) {
        self.renderer.update(renderer_interface, dt);

        // update entity index label
        let text = self.entity_index.to_string();
        self.entity_index_label.update(&self.font, &text);
        self.entity_index_mesh.update_texture(
            renderer_interface.queue(),
            self.entity_index_label.get_image(),
        );

        self.performance_monitor.watch.start(3);
        self.game_server.update();
        // ecs::system::move_agents(&mut self.world);
        self.performance_monitor.watch.stop(3);

        self.performance_monitor.watch.start(4);
        self.animated_object_storage.update(renderer_interface, &dt);
        // self.draw_agents.update(&mut self.world, &mut self.renderer.wgpu_renderer);
        self.performance_monitor.watch.stop(4);

        self.performance_monitor.update(renderer_interface);
    }

    fn input(&mut self, event: &winit::event::WindowEvent) -> bool {
        self.performance_monitor.watch.start(2);
        let res = match event {
            WindowEvent::KeyboardInput {
                event:
                    winit::event::KeyEvent {
                        physical_key:
                            winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::F2),
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } => {
                self.performance_monitor.show = !self.performance_monitor.show;
                true
            }
            WindowEvent::KeyboardInput {
                event:
                    winit::event::KeyEvent {
                        // virtual_keycode: Some(key),
                        physical_key: winit::keyboard::PhysicalKey::Code(key),
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

    fn render(
        &mut self,
        renderer_interface: &mut dyn WgpuRendererInterface,
    ) -> Result<(), wgpu::SurfaceError> {
        // will be used for the next frame
        self.entity_index = self.renderer.read_entity_index();

        // render current frame
        self.renderer.render(
            renderer_interface,
            // &self.world_mesh,
            &self.game_state.terrain_mesh,
            &self.animated_object_storage,
            &self.ant,
            &self.entity_index_mesh,
            &self.ambient_light_quad,
            &mut self.performance_monitor,
            deferred_color_shader::entity_buffer::MousePosition {
                x: self.mouse_pos_x,
                y: self.mouse_pos_y,
            },
        )
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub async fn run() {
    let event_loop = winit::event_loop::EventLoop::new().unwrap();
    let mut application: DefaultApplication<MarketEconomySimulation> = DefaultApplication::new();
    event_loop.run_app(&mut application).unwrap();
}
