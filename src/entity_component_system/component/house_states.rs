

pub struct HouseStates {
    pub entity: usize,

    residents: u32,
}

impl HouseStates {
    pub fn new() -> Self 
    {
        Self {
            entity: 0,

            residents: 0,
        }
    }
}