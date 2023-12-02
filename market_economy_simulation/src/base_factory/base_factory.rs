use crate::ecs2::EntityInterface;



pub struct BaseFactory {
    entity_index: usize,
    location: [u32; 2],
}

impl BaseFactory {
    pub fn new(entity_index: usize, location: [u32; 2]) -> Self 
    {
        Self {
            entity_index,
            location,
        }
    }
}

impl EntityInterface for BaseFactory {
    fn width(&self) -> f32 {
        0.0
    }

    fn height(&self) -> f32 {
        0.0
    }
}