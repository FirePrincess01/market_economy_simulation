


use super::entity_component_system::{
    World,
    Entity,
    component::LiveStates,
    component::FarmStates,
    component::HouseStates,
    component::Mesh,
};

pub fn add_agent(world: &mut World) {
    let entity = world.add_entity(Entity::new());
    world.add_mesh(entity, Mesh::new());
    world.add_live_states(entity, LiveStates::new());
}

pub fn add_farm(world: &mut World) {
    let entity = world.add_entity(Entity::new());
    world.add_mesh(entity, Mesh::new());
    world.add_farm_states(entity, FarmStates::new());
}

pub fn add_house(world: &mut World) {
    let entity = world.add_entity(Entity::new());
    world.add_mesh(entity, Mesh::new());
    world.add_house_states(entity, HouseStates::new());
}