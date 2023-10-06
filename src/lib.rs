
mod ecs;
mod create_entities;
mod default_window;
mod renderer;


struct MarketEconomySimulation {
    renderer: renderer::Renderer,
    world: ecs::World,
}

impl MarketEconomySimulation {
    pub async fn new(window: &winit::window::Window) -> Self 
    {
        let renderer = renderer::Renderer::new(window).await;
        let world = ecs::World::new();

        Self {
            renderer,
            world,
        }
    }
}

impl default_window::DefaultWindowApp for MarketEconomySimulation 
{
    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.renderer.resize(new_size);
    }

    fn update_scale_factor(&mut self, scale_factor: f32) {
        
    }

    fn update(&mut self, dt: instant::Duration) {
        self.renderer.update(dt);
    }

    fn input(&mut self, event: &winit::event::WindowEvent) -> bool {
        false
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        ecs::system::draw(&self.world, &mut self.renderer)
    }
}



#[cfg_attr(target_arch="wasm32", wasm_bindgen(start))]
pub async fn run()
{
    let default_window = default_window::DefaultWindow::new();
    let app = MarketEconomySimulation::new(&default_window.window).await;

    default_window::run(default_window, app);
}