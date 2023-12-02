use crate::{ground_plane::{GroundPlane, GroundResource}, base_factory::BaseFactory};

use super::{entity::Entity, components::Components};





pub struct World {
    pub entities: Vec<Entity>,
    pub components: Components,
}

impl World {
    pub fn new() -> Self
    {
        // entities
        let mut entities: Vec<Entity> = Vec::new();

        // ground plane
        let ground_plane_height = 100;
        let ground_plane_width = 100;
        let mut ground_plane = GroundPlane::new(ground_plane_width, ground_plane_height);
        ground_plane.generate_resource(0.005, GroundResource::Red);
        ground_plane.generate_resource(0.01, GroundResource::Blue);
        ground_plane.generate_resource(0.001, GroundResource::Green);

        for y in 0..ground_plane_height {
            for x in 0..ground_plane_width {
                let field = ground_plane.get_mut(y, x);
                field.entity_index = entities.len();
                entities.push(Entity::GroundPlane { y: y, x: x });
            }
        }

        // base factory
        let base_factory = BaseFactory::new(
            entities.len(),
            [10, 10],
        );
        entities.push(Entity::BaseFactory);

        // agents

        // components
        let components = Components{ ground_plane, base_factory: base_factory };

        Self {
            entities,
            components,
        }
    }
}