use super::Component;

#[allow(dead_code)]
pub struct LiveStates {
    pub entity_index: usize,

    age: u32,
    health: u32,
    sleep: u32,
    food: u32,
}

#[allow(dead_code)]
impl LiveStates {
    pub fn new() -> Self {
        Self {
            entity_index: 0,

            age: 0,
            health: 100,
            sleep: 100,
            food: 100,
        }
    }
}

impl Component for LiveStates {
    fn get_entity_index(&self) -> usize {
        self.entity_index
    }

    fn set_entity_index(&mut self, entity_index: usize) {
        self.entity_index = entity_index;
    }

    fn get_value_index(entity: &crate::ecs::Entity) -> Option<usize> {
        entity.live_stat
    }

    fn set_value_index(entity: &mut crate::ecs::Entity, value_index: Option<usize>) {
        entity.live_stat = value_index;
    }

    fn move_to_world(self, world: &mut crate::ecs::World) -> usize {
        let index = world.live_stats.len();
        world.live_stats.push(self);

        index
    }
}
