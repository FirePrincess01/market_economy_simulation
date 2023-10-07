

mod renderer;

pub use renderer::IRenderer;
use wgpu_renderer::{vertex_color_shader::Instance, vertex_color_shader::InstanceBuffer, vertex_color_shader::InstanceRaw, renderer::WgpuRendererInterface};

use super::World;


pub fn move_agents(world: &mut World) 
{
    let iter = World::filter(&world.entities, &mut world.positions, &world.live_stats);

    let mut i = 0;
    for (pos, _live) in iter 
    {

        pos.pos[0] = i as f32 * 0.2;
        pos.pos[1] = i as f32 * 0.2;

        i += 1;
    }
}

struct DrawAgents {
    instances_host: Vec<InstanceRaw>,
    instances_device: InstanceBuffer
}

impl DrawAgents {
    pub fn new(wgpu_renderer: &mut impl WgpuRendererInterface) -> Self {
        
        let instances_host = Vec::<InstanceRaw>::new();
        let instances_hose =  vec![InstanceRaw::new(); 1000];

        let instances_device = InstanceBuffer::new(wgpu_renderer.device(), &instances_host);
        
        Self {

        }
    }

    pub fn update(&mut self, world: &mut World, wgpu_renderer: &mut impl WgpuRendererInterface)
    {
        let iter = World::filter(&world.entities, &mut world.positions, &world.live_stats);

        self.instances_host.clear();
        for (pos, _live) in iter 
        {
            let instance = Instance{ 
                position: glam::Vec3::new(pos.pos[0], pos.pos[1], pos.pos[2]), 
                rotation: glam::Quat::IDENTITY };

            self.instances_host.push(instance.to_raw());
        }  

        self.instances_device.update(wgpu_renderer.queue(), &self.instances_host);
    }

    pub fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>)
    {

    }
}


pub fn draw(world: &mut World, renderer: &mut impl IRenderer) -> Result<(), wgpu::SurfaceError>
{
    let iter = World::filter(&world.entities, &mut world.positions, &world.live_stats);

    let mut i = 0;
    for (pos, _live) in iter 
    {

        pos.pos[0] = i as f32 * 0.2;
        pos.pos[1] = i as f32 * 0.2;

        i += 1;
    }


    renderer.render(&world.meshes)

}