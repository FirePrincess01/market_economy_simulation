

use super::Component;

pub struct Position {
    pub entity_index: usize,
    pub pos: [f32; 3]
}

#[allow(dead_code)]
impl Position {
    pub fn new() -> Self {
        Self {
            entity_index: 0,
            pos: [0.0; 3],
        }
    }
}

impl Component for Position {

    fn get_entity_index(&self) -> usize {
        self.entity_index
    }

    fn set_entity_index(&mut self, entity_index: usize) {
        self.entity_index = entity_index;
    }


    fn get_value_index(entity: &crate::ecs::Entity) -> Option<usize> {
        entity.position
    }

    fn set_value_index(entity: &mut crate::ecs::Entity, value_index: Option<usize>) {
        entity.position = value_index;
    }


    fn move_to_world(self, world: &mut crate::ecs::World) -> usize {
        let index = world.positions.len();
        world.positions.push(self);

        index
    }
}