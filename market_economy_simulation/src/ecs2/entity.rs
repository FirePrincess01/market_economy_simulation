use super::{entity_interface::EntityInterface, components::Components};


pub enum Entity {
    GroundPlane{y: usize, x: usize},
    BaseFactory,
    // Player(BaseFactory),
    // Factory(BaseFactory),
    // Agent(BaseFactory),
}


impl Entity {
    pub fn visit<'a>(&'a self, components: &'a Components) -> &'a dyn EntityInterface 
    {
        match self {
            Entity::GroundPlane { x, y } => components.ground_plane.get(*y, *x),
            Entity::BaseFactory => &components.base_factory,
            // Entity::Player(elem) => elem,
            // Entity::Factory(elem) => elem,
            // Entity::Agent(elem) => elem,
        }
    }
}



