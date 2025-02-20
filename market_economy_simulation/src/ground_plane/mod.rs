//! The ground field

mod ground_field;
mod ground_plane_mesh;

use ground_field::GroundField;
pub use ground_field::GroundResource;
pub use ground_plane_mesh::GroundPlaneMesh;

pub struct Location {
    pub y: u32,
    pub x: u32,
}

pub struct GroundPlane {
    width: usize,
    height: usize,
    size: usize,
    fields: Vec<GroundField>,
}

impl GroundPlane {
    pub fn new(width: usize, height: usize) -> Self {
        let size = width * height;

        let mut fields = Vec::with_capacity(size);
        for _i in 0..size {
            let ground_field = GroundField {
                resource: GroundResource::None,
                entity_index: 0,
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

    pub fn generate_resource(&mut self, probability: f64, resource: GroundResource) {
        let mut rng = fastrand::Rng::new();
        for elem in &mut self.fields {
            let rand = rng.f64();
            if rand < probability {
                elem.resource = resource;
            }
        }
    }

    pub fn get(&self, y: usize, x: usize) -> &GroundField {
        let index = y * self.width + x;

        &self.fields[index]
    }

    pub fn get_mut(&mut self, y: usize, x: usize) -> &mut GroundField {
        let index = y * self.width + x;

        &mut self.fields[index]
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
