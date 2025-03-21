mod animated_object;
mod ant;
mod base_factory;
mod create_entities;
mod debug_overlay;
mod deferred_animation_shader;
mod deferred_color_shader;
mod deferred_heightmap_shader;
mod deferred_light_shader;
mod deferred_light_sphere_shader;
mod deferred_terrain_shader;
mod ecs;
mod ecs2;
mod fxaa_shader;
mod game_state;
mod geometry;
mod ground_plane;
mod performance_monitor;
mod point_light_storage;
mod renderer;
mod selector;
mod settings;
mod terrain_storage;
mod world_mesh;

use animated_object::wgpu_animated_object_renderer::{
    WgpuAnimatedObjectRenderer, WgpuAnimatedObjectStorage,
};
use debug_overlay::DebugOverlay;
use market_economy_simulation_server::game_logic::game_logic_interface::{
    GameLogicInterface, GameLogicMessageHeavy, GameLogicMessageLight, GameLogicMessageMedium,
};
use point_light_storage::{PointLightIndex, PointLightInterface};
use selector::Selector;
use terrain_storage::TerrainStorage;
use wgpu_renderer::{
    default_application::{DefaultApplication, DefaultApplicationInterface},
    performance_monitor::watch,
    vertex_texture_shader,
    wgpu_renderer::WgpuRendererInterface,
};
use winit::event::{ElementState, WindowEvent};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

const WATCH_POINTS_SIZE: usize = 7;

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
    watch_fps: watch::Watch<WATCH_POINTS_SIZE>,
    performance_monitor_fps: performance_monitor::PerformanceMonitor<WATCH_POINTS_SIZE>,
    performance_monitor_ups: performance_monitor::PerformanceMonitor<WATCH_POINTS_SIZE>,

    // show the entity index
    mouse_pos_y: u32,
    mouse_pos_x: u32,
    entity_index: u32,
    font: rusttype::Font<'static>,

    debug_overlay: DebugOverlay,

    game_logic: market_economy_simulation_server::GameLogicServer,
    ant: ant::Ant,

    ambient_light_quad: deferred_light_shader::Mesh, // Quad running the global ambient light shader
    point_light_storage: point_light_storage::PointLightStorage,

    terrain_storage: TerrainStorage,

    selector: Selector,
}

impl MarketEconomySimulation {
    pub fn new(
        renderer_interface: &mut dyn WgpuRendererInterface,
        size: winit::dpi::PhysicalSize<u32>,
        scale_factor: f32,
    ) -> Self {
        let settings = settings::Settings::new();

        let renderer =
            renderer::Renderer::new(renderer_interface, settings.get_renderer_settings());

        // font
        let font = wgpu_renderer::freefont::create_font_free_mono();

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
        let watch_fps = watch::Watch::new();
        let performance_monitor_fps = performance_monitor::PerformanceMonitor::new(
            renderer_interface,
            &renderer.texture_bind_group_layout,
            &font,
            colorous::RAINBOW,
            true,
            "60 fps",
        );
        let performance_monitor_ups = performance_monitor::PerformanceMonitor::new(
            renderer_interface,
            &renderer.texture_bind_group_layout,
            &font,
            colorous::PLASMA,
            false,
            "60 ups",
        );

        // Mouse position
        let mouse_pos_y = 0;
        let mouse_pos_x = 0;

        // Debug Overlay
        let debug_overlay = DebugOverlay::new(
            renderer_interface,
            &renderer.texture_bind_group_layout,
            &font,
            cgmath::Vector3 {
                x: 20.0,
                y: 120.0,
                z: 0.0,
            },
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
        let game_logic =
            market_economy_simulation_server::GameLogicServer::new(settings.get_server_settings());

        // create ant
        let ant = ant::Ant::new(renderer_interface);

        let ambient_light_quad_vertices = geometry::Quad::new(2.0);
        let ambient_light_quad_instance = deferred_light_shader::Instance {
            position: [-1.0, -1.0, 0.1],
            light_color: [0.4, 0.4, 0.4],
            radius: 0.0,
            linear: 0.0,
            quadratic: 0.0,
        };
        let ambient_light_quad = deferred_light_shader::Mesh::new(
            renderer_interface.device(),
            &ambient_light_quad_vertices.vertices,
            &ambient_light_quad_vertices.indices,
            &[ambient_light_quad_instance],
        );

        // point light storage
        let point_light_storage = point_light_storage::PointLightStorage::new(
            renderer_interface,
            settings.max_point_light_instances,
            settings.dbg_point_lights,
        );

        // terrain storage
        let terrain_storage = TerrainStorage::new(
            settings.get_terrain_settings(),
            renderer_interface,
            &renderer.texture_bind_group_layout,
        );

        // selector
        let selector = Selector::new();

        Self {
            _settings: settings,

            size,
            scale_factor,

            renderer,

            _world: world,
            // world_mesh,
            animated_object_storage,

            watch_fps,
            performance_monitor_fps,
            performance_monitor_ups,

            mouse_pos_y,
            mouse_pos_x,
            entity_index: 0,
            font,

            debug_overlay,

            game_logic,

            ant,

            ambient_light_quad,
            point_light_storage,

            terrain_storage,

            selector,
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
        self.debug_overlay
            .update_entity(renderer_interface, &self.font, self.entity_index);

        self.game_logic.update();

        self.watch_fps.start(3, "Update data");
        {
            let light_messages = self.game_logic.get_light_messages();
            for msg in light_messages.try_iter() {
                match msg {
                    GameLogicMessageLight::UpdatePointLight(point_light) => {
                        let index = PointLightIndex {
                            instance_index: point_light.id as usize,
                        };
                        self.point_light_storage.set_light(
                            index,
                            point_light.position,
                            point_light.color,
                            point_light.attenuation,
                        );
                    }
                }
            }

            let medium_messages = self.game_logic.get_medium_messages();
            for msg in medium_messages.try_iter() {
                match msg {
                    GameLogicMessageMedium::UpdateWatchPoints(watch_viewer_data) => {
                        self.performance_monitor_ups.update_from_data(
                            renderer_interface,
                            &self.font,
                            &watch_viewer_data,
                        );
                    }
                }
            }

            let heavy_messages = self.game_logic.get_heavy_messages();
            for msg in heavy_messages.try_iter() {
                match msg {
                    GameLogicMessageHeavy::Terrain(height_map) => {
                        self.terrain_storage.update_height_map(
                            renderer_interface,
                            &self.renderer.heightmap_bind_group_layout,
                            height_map,
                        );
                    }
                }
            }

            self.point_light_storage.update(renderer_interface);

            self.terrain_storage
                .update_view_position(&self.renderer.get_view_position());
            self.terrain_storage
                .submit_requests(self.game_logic.send_messages());
        }
        self.watch_fps.stop(3);

        self.watch_fps.start(4, "Select entity");
        {
            self.selector.update_entity(self.entity_index);
            self.selector.update_view(
                &self.renderer.camera,
                &self.renderer.projection,
                &cgmath::Vector2::new(self.mouse_pos_x, self.mouse_pos_y),
            );

            let res = self.selector.find_selection(
                &self.terrain_storage.height_map_details,
                &self.terrain_storage.height_maps,
            );
            if let Some(res) = res {
                match res {
                    selector::Result::Terrain(triangle) => {
                        self.debug_overlay.update_coord(
                            renderer_interface,
                            &self.font,
                            &triangle.p,
                        );
                    }
                }
            }
        }
        self.watch_fps.stop(4);

        self.watch_fps.start(5, "Update animations");
        {
            self.animated_object_storage.update(renderer_interface, &dt);
        }
        self.watch_fps.stop(5);

        self.watch_fps.update();
        self.performance_monitor_fps.update_from_data(
            renderer_interface,
            &self.font,
            &self.watch_fps.get_viewer_data(),
        );
    }

    fn input(&mut self, event: &winit::event::WindowEvent) -> bool {
        self.watch_fps.start(2, "Process user inputs");
        let res = match event {
            WindowEvent::KeyboardInput {
                event:
                    winit::event::KeyEvent {
                        physical_key:
                            winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::F1),
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } => {
                self.performance_monitor_ups.show = false;
                self.performance_monitor_fps.show = !self.performance_monitor_fps.show;
                true
            }
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
                self.performance_monitor_fps.show = false;
                self.performance_monitor_ups.show = !self.performance_monitor_ups.show;
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
        self.watch_fps.stop(2);

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
            &self.animated_object_storage,
            &self.point_light_storage,
            &mut self.terrain_storage,
            &self.ant,
            &self.debug_overlay,
            &self.ambient_light_quad,
            &[
                &mut self.performance_monitor_fps,
                &mut self.performance_monitor_ups,
            ],
            &mut self.watch_fps,
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
