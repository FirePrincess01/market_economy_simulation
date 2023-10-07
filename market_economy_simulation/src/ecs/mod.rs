
pub mod component;
pub mod system;
pub mod entity_iterator;

#[cfg(test)]
mod tests;

use entity_iterator::EntityIterator;

use component::Component;
use component::LiveStates;
use component::FarmStates;
use component::HouseStates;
use component::ColoredMesh;
use component::Position;

pub struct Entity {
    pub live_stat: Option<usize>,
    pub farm_stat: Option<usize>,
    pub house_stat: Option<usize>,
    pub mesh: Option<usize>,
    pub position: Option<usize>,
}

impl Entity {
    pub fn new() -> Self 
    {
        Self { 
            live_stat: None, 
            farm_stat: None,
            house_stat: None,
            mesh: None,
            position: None,
        }
    }
}

pub struct World {
    pub entities: Vec<Entity>,

    pub live_stats: Vec<LiveStates>,
    pub farm_stats: Vec<FarmStates>,
    pub house_stats: Vec<HouseStates>,
    pub meshes: Vec<ColoredMesh>,
    pub positions: Vec<Position>,
}

impl World {
    pub fn new() -> Self
    {
        Self {
            entities: Vec::new(),

            live_stats: Vec::new(),
            farm_stats: Vec::new(),
            house_stats: Vec::new(),
            meshes: Vec::new(),
            positions: Vec::new(),
        }
    }

    pub fn add_entity(&mut self, entity: Entity) -> usize
    {
        let index = self.entities.len();
        self.entities.push(entity);

        index
    }

    pub fn add<T: Component>(&mut self, entity_index: usize, mut value: T)
    {
        // add entity_index to value
        value.set_entity_index(entity_index);
        let value_index = value.move_to_world(self);

        // add value_index to entity
        let entity = &mut self.entities[entity_index];
        T::set_value_index(entity, Some(value_index));
    }

    pub fn filter<'a, T1, T2>(entities: &'a Vec<Entity>, data: &'a mut Vec<T1>, filter1: &'a Vec<T2>) -> EntityIterator<'a, T1, T2>     {
        EntityIterator::new(entities, data, filter1)
    }

}

