//! Visit element function for a selector (to implement in the future)

use super::{components::Components, entity_interface::EntityInterface};

pub enum Entity {
    GroundPlane { y: usize, x: usize },
    BaseFactory,
    // Player(BaseFactory),
    // Factory(BaseFactory),
    // Agent(BaseFactory),
}

#[allow(dead_code)]
impl Entity {
    pub fn visit<'a>(&'a self, components: &'a Components) -> &'a dyn EntityInterface {
        match self {
            Entity::GroundPlane { x, y } => components.ground_plane.get(*y, *x),
            Entity::BaseFactory => &components.base_factory,
            // Entity::Player(elem) => elem,
            // Entity::Factory(elem) => elem,
            // Entity::Agent(elem) => elem,
        }
    }
}
