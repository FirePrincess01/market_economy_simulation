use super::ecs::{
    component::ColoredMesh, component::FarmStates, component::HouseStates, component::LiveStates,
    component::Position, Entity, World,
};

pub fn _create_agent(world: &mut World) {
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
