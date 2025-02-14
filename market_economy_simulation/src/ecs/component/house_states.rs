use super::Component;

#[allow(dead_code)]
pub struct HouseStates {
    pub entity_index: usize,

    residents: u32,
}

#[allow(dead_code)]
impl HouseStates {
    pub fn new() -> Self {
        Self {
            entity_index: 0,

            residents: 0,
        }
    }
}

impl Component for HouseStates {
    fn get_entity_index(&self) -> usize {
        self.entity_index
    }

    fn set_entity_index(&mut self, entity_index: usize) {
        self.entity_index = entity_index;
    }

    fn get_value_index(entity: &crate::ecs::Entity) -> Option<usize> {
        entity.house_stat
    }

    fn set_value_index(entity: &mut crate::ecs::Entity, value_index: Option<usize>) {
        entity.house_stat = value_index;
    }

    fn move_to_world(self, world: &mut crate::ecs::World) -> usize {
        let index = world.house_stats.len();
        world.house_stats.push(self);

        index
    }
}
