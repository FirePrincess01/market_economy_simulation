


pub enum GroundResource {
    Red,
    Green,
    Blue,
}

pub struct GroundField {
    pub resource: Option<GroundResource>,
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
                resource: None, 
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