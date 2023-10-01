

pub struct LiveStates {
    pub entity: usize,

    age: u32,
    health: u32,
    sleep: u32,
    food: u32,
}

impl LiveStates {
    pub fn new() -> Self 
    {
        Self {
            entity: 0,

            age: 0,
            health: 100,
            sleep: 100,
            food: 100,
        }
    }
}

