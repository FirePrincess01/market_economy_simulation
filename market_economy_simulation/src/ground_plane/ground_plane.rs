//! The ground field

// use rand::Rng;
use fastrand;

#[derive(Copy, Clone, PartialEq)]
pub enum GroundResource {
    None,
    Red,
    Green,
    Blue,
}

pub struct GroundField {
    pub resource: GroundResource,
    pub entity_index: Option<usize>,
    pub index: usize,
}


pub struct GroundPlane
{
    width: usize,
    height: usize,
    size: usize,
    fields: Vec<GroundField>,
}

impl GroundPlane {
    pub fn new(width: usize, height: usize) -> Self 
    { 
        let size = width * height;

        let mut fields = Vec::new();
        fields.reserve(size);
        for i in 0..size {
            let ground_field = GroundField { 
                resource: GroundResource::None, 
                entity_index: None, 
                index: i 
            };
            fields.push(ground_field)
        }

        Self { 
            width,
            height,
            size, 
            fields,
        } 
    }

    pub fn generate_resource(&mut self, probability: f64, resource: GroundResource)
    {
        let mut rng = fastrand::Rng::new();
        for elem in &mut self.fields {
            let rand = rng.f64();
            if rand < probability {
                elem.resource = resource;
            }
        }
    }

    pub fn get(&self, y: usize, x: usize) -> &GroundField   
    {
        let index = y * self.width + x;

        &self.fields[index]
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn size(&self) -> usize {
        self.size
    }
}