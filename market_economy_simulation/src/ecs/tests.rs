//! Unit tests


use super::*;

pub fn create_position(world: &mut World) {
    let entity_index = world.add_entity(Entity::new());
    world.add(entity_index, ColoredMesh::new(1));
    world.add(entity_index, Position::new());
}


#[test]
fn simple() -> Result<(), String> 
{
    let mut world = super::World::new();

    create_position(&mut world);

    let iter = World::filter(&world.entities, 
        &mut world.positions, 
        &world.meshes);

    for (pos, mesh) in iter {
        pos.pos[0] = 1.0;
    }

    assert_eq!(world.positions[0].pos[0], 1.0);

    Ok(())
}
