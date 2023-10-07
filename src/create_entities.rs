


use super::ecs::{
    World,
    Entity,
    component::LiveStates,
    component::FarmStates,
    component::HouseStates,
    component::ColoredMesh,
};

pub fn create_agent2(world: &mut World) {
    let entity_index = world.add_entity(Entity::new());
    world.add(entity_index, ColoredMesh::new());
}

pub fn create_agent(world: &mut World) {
    let entity_index = world.add_entity(Entity::new());
    world.add(entity_index, ColoredMesh::new());
    world.add(entity_index, LiveStates::new());
}

pub fn create_farm(world: &mut World) {
    let entity_index = world.add_entity(Entity::new());
    world.add(entity_index, ColoredMesh::new());
    world.add(entity_index, FarmStates::new());
}

pub fn create_house(world: &mut World) {
    let entity_index = world.add_entity(Entity::new());
    world.add(entity_index, ColoredMesh::new());
    world.add(entity_index, HouseStates::new());
}