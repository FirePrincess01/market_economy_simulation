

use super::Component;

pub struct FarmStates {
    pub entity_index: usize,

    workers: u32,
    progress: u32,
}

impl FarmStates {
    pub fn new() -> Self 
    {
        Self {
            entity_index: 0,

            workers: 0,
            progress: 100,
        }
    }
}

impl Component for FarmStates {

    fn get_entity_index(&self) -> usize {
        self.entity_index
    }

    fn set_entity_index(&mut self, entity_index: usize) {
        self.entity_index = entity_index;
    }


    fn get_value_index(entity: &crate::ecs::Entity) -> Option<usize> {
        entity.farm_stat
    }

    fn set_value_index(entity: &mut crate::ecs::Entity, value_index: Option<usize>) {
        entity.farm_stat = value_index;
    }

    fn move_to_world(self, world: &mut crate::ecs::World) -> usize {
        let index = world.farm_stats.len();
        world.farm_stats.push(self);

        index
    }
}
