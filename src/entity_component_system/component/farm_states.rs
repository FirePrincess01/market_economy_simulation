

pub struct FarmStates {
    pub entity: usize,

    workers: u32,
    progress: u32,
}

impl FarmStates {
    pub fn new() -> Self 
    {
        Self {
            entity: 0,

            workers: 0,
            progress: 100,
        }
    }
}
