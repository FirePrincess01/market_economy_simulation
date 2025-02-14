//! contains all data of the world

mod entity;
mod entity_interface;
mod world;
mod components;

pub use entity_interface::EntityInterface;
pub use world::World;
pub use components::BlueToken;
pub use components::AgentBlueToken;
pub use components::Blues;
pub use components::Resources2;
pub use components::AgentsBlue;
pub use components::Agents;