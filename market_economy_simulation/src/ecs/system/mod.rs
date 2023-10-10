


use wgpu_renderer::renderer::WgpuRendererInterface;

use crate::deferred_color_shader::Instance;

use crate::deferred_color_shader::DeferredShaderDraw;
use crate::deferred_light_shader::DeferredLightShaderDraw;
use crate::geometry;

use crate::deferred_color_shader;
use crate::deferred_light_shader;

use super::World;
use super::component::Component;


pub fn move_agents(world: &mut World) 
{
    let iter = World::filter(&world.entities, &mut world.positions, &world.live_stats);

    let mut i = 0;
    // let max_y = 400;
    let max_y = 1000;
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
    instances_host: Vec<Instance>,
    mesh: deferred_color_shader::Mesh,
    // mesh_light: deferred_color_shader::Mesh,
}

impl DrawAgents {
    pub fn new(wgpu_renderer: &mut impl WgpuRendererInterface, max_agents: usize) -> Self {
        
        let circle = geometry::Circle::new(0.15, 16);
        let circle_light = geometry::Circle::new(0.30, 16);
        let instances_host =  vec![Instance::new(); max_agents];

        let mesh = deferred_color_shader::Mesh::new(wgpu_renderer.device(), 
            &circle.deferred_vertices, 
            &circle.indices, 
            &instances_host);

        // let mesh_light = deferred_color_shader::Mesh::new(wgpu_renderer.device(), 
        //     &circle_light.vertices, 
        //     &circle_light.colors, 
        //     &circle_light.indices, 
        //     &instances_host);
        
        Self {
            instances_host,
            mesh,
            // mesh_light,
        }
    }

    pub fn update(&mut self, world: &mut World, wgpu_renderer: &mut impl WgpuRendererInterface)
    {
        let iter = World::filter(&world.entities, &mut world.positions, &world.live_stats);

        self.instances_host.clear();
        for (pos, _live) in iter 
        {
            let instance = Instance{ 
                position: [pos.pos[0], pos.pos[1], pos.pos[2]], 
                color: [0.5, 0.0, 0.5], 
                entity: [pos.get_entity_index() as u32, 0, 0], 
            };

            self.instances_host.push(instance);
        }  

        self.mesh.update_instance_buffer(wgpu_renderer.queue(), &self.instances_host);
        // self.mesh_light.update_instance_buffer(wgpu_renderer.queue(), &self.instances_host);
    }

}

impl DeferredShaderDraw for DrawAgents {
    fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        self.mesh.draw(render_pass);
    }
}

// impl DeferredLightShaderDraw for DrawAgents {
//     fn draw_lights<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
//         self.mesh_light.draw(render_pass);
//     }
// }

