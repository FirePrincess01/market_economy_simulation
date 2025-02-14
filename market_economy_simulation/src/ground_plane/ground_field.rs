//! Contains all information of a ground field

use crate::ecs2::EntityInterface;

#[derive(Copy, Clone, PartialEq)]
pub enum GroundResource {
    None,
    Red,
    Green,
    Blue,
}

pub struct GroundField {
    pub resource: GroundResource,
    pub entity_index: usize,
    // pub index: usize,
}

impl EntityInterface for GroundField {
    fn width(&self) -> f32 {
        0.0
    }

    fn height(&self) -> f32 {
        0.0
    }
}
