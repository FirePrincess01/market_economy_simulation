mod colored_mesh;
mod farm_states;
mod house_states;
mod life_states;
mod position;

pub use colored_mesh::ColoredMesh;
pub use farm_states::FarmStates;
pub use house_states::HouseStates;
pub use life_states::LiveStates;
pub use position::Position;

use super::{Entity, World};

pub trait Component {
    fn get_entity_index(&self) -> usize;
    fn set_entity_index(&mut self, entity_index: usize);

    fn get_value_index(entity: &Entity) -> Option<usize>;
    fn set_value_index(entity: &mut Entity, value_index: Option<usize>);

    fn move_to_world(self, world: &mut World) -> usize;
}
