
use super::Component;
use wgpu_renderer::vertex_color_shader;

pub struct ColoredMesh {
    pub entity_index: usize,

    // mesh: vertex_color_shader::Mesh,
}

impl ColoredMesh {
    pub fn new() -> Self 
    {
        Self {
            entity_index: 0,
        }
    }
}

impl Component for ColoredMesh {

    fn get_entity_index(&self) -> usize {
        self.entity_index
    }

    fn set_entity_index(&mut self, entity_index: usize) {
        self.entity_index = entity_index;
    }


    fn get_value_index(entity: &crate::ecs::Entity) -> Option<usize> {
        entity.mesh
    }

    fn set_value_index(entity: &mut crate::ecs::Entity, value_index: Option<usize>) {
        entity.mesh = value_index;
    }


    fn move_to_world(self, world: &mut crate::ecs::World) -> usize {
        let index = world.meshes.len();
        world.meshes.push(self);

        index
    }
}