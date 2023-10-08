


use super::ecs::{
    World,
    Entity,
    component::LiveStates,
    component::FarmStates,
    component::HouseStates,
    component::ColoredMesh,
    component::Position,
};


pub fn create_agent(world: &mut World) {
    let entity_index = world.add_entity(Entity::new());
    // world.add(entity_index, ColoredMesh::new(1));
    world.add(entity_index, LiveStates::new());
    world.add(entity_index, Position::new());
}

pub fn _create_farm(world: &mut World) {
    let entity_index = world.add_entity(Entity::new());
    world.add(entity_index, ColoredMesh::new(1));
    world.add(entity_index, FarmStates::new());
}

pub fn _create_house(world: &mut World) {
    let entity_index = world.add_entity(Entity::new());
    world.add(entity_index, ColoredMesh::new(1));
    world.add(entity_index, HouseStates::new());
}