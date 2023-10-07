
use super::Component;

pub struct ColoredMesh {
    pub entity_index: usize,

    pub mesh_index: usize,

    // mesh: vertex_color_shader::Mesh,
}

impl ColoredMesh {
    pub fn new(mesh_index: usize) -> Self 
    {
        Self {
            entity_index: 0,
            mesh_index,
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