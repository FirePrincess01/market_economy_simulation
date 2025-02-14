//! contains all data of the world

use crate::{ground_plane::{GroundPlane, GroundResource, Location}, base_factory::BaseFactory};

use super::{entity::Entity, components::Agents, Blues, Resources2, AgentsBlue, };

#[allow(unused)]
pub struct World {
    pub entities: Vec<Entity>,

    pub ground_plane: GroundPlane,
    pub base_factory: BaseFactory,

    pub resources: Resources2,
    pub agents: Agents,
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
            Location{ y: 10, x: 10 },
        );
        entities.push(Entity::BaseFactory);

        // resources
        let blues2 = Blues::new();     

        let resources = Resources2{ blues2 };

        // agents
        let agent_blue = AgentsBlue::new();
        let agents = Agents{ agent_blue };

        Self {
            entities,
            ground_plane,
            base_factory,
            resources,
            agents,
        }
    }

}


