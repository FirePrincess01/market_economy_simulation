

mod renderer;

pub use renderer::IRenderer;
use wgpu_renderer::{vertex_color_shader::Instance, vertex_color_shader::InstanceRaw, renderer::WgpuRendererInterface};

use crate::deferred_color_shader::DeferredShaderMeshDraw;
use crate::geometry;

use super::super::deferred_color_shader::Mesh;

use super::World;


pub fn move_agents(world: &mut World) 
{
    let iter = World::filter(&world.entities, &mut world.positions, &world.live_stats);

    let mut i = 0;
    let max_y = 150;
    for (pos, _live) in iter 
    {
        let x = i % max_y;
        let y = i / max_y;

        pos.pos[0] = x as f32 * 0.4;
        pos.pos[1] = y as f32 * 0.4;
        pos.pos[2] = 0.0;

        i += 1;
    }
}

pub struct DrawAgents {
    instances_host: Vec<InstanceRaw>,
    mesh: Mesh,
}

impl DrawAgents {
    pub fn new(wgpu_renderer: &mut impl WgpuRendererInterface, max_agents: usize) -> Self {
        
        let circle = geometry::Circle::new(0.15, 16);
        let instances_host =  vec![InstanceRaw::new(); max_agents];

        let mesh = Mesh::new(wgpu_renderer.device(), 
            &circle.vertices, 
            &circle.colors, 
            &circle.indices, 
            &instances_host);
        
        Self {
            instances_host,
            mesh,
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

        self.mesh.update_instance_buffer(wgpu_renderer.queue(), &self.instances_host);
    }

}

impl DeferredShaderMeshDraw for DrawAgents {
    fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        self.mesh.draw(render_pass);
    }
}

