use crate::{ground_plane::GroundPlane, base_factory::BaseFactory};


// struct ResourceRed {
//     z: f32,
//     y: f32,
//     x: f32,
// }

// impl ResourceRed {
//     fn new(z: f32, y: f32, x: f32) 
//     -> Self 
//     { 
//         Self { z, y, x, } 
//     }
// }

// struct ResourceGreen {
//     z: f32,
//     y: f32,
//     x: f32,
// }

pub struct ResourceBlue{
    pub z: f32,
    pub y: f32,
    pub x: f32,
}

// can only be moved
// pub struct RedToken {
//     index: usize, // index into the reds vector
// }

// impl RedToken {
//     fn new(index: usize) -> Self { Self { index } }
// }


// struct GreenToken {
//     index: usize, // index into the reds vector
// }

// impl GreenToken {
//     fn new(index: usize) -> Self { Self { index } }
// }

pub struct BlueToken {
    index: usize, // index into the reds vector
}

impl BlueToken {
    fn new(index: usize) -> Self { Self { index } }

    fn get_index(&self) -> usize { self.index }
}

pub struct Blues {
    elements: Vec<ResourceBlue>,
}

impl Blues {
    pub fn new() -> Self 
    {
        Self {
            elements: Vec::new(),
        }
    }

    pub fn _get(&self, blue_token: &BlueToken) -> &ResourceBlue
    {
        &self.elements[blue_token.get_index()]
    }

    pub fn get_mut(&mut self, blue_token: &BlueToken) -> &mut ResourceBlue
    {
        &mut self.elements[blue_token.get_index()]
    }

    pub fn create(&mut self, z: f32, y: f32, x: f32) -> BlueToken {
        let token = BlueToken::new(self.elements.len());
        let resource = ResourceBlue{ z, y, x };
        self.elements.push(resource);

        token
    }
}

#[allow(dead_code)]
pub struct AgentBlue {
    z: f32,
    y: f32,
    x: f32,
}

#[allow(dead_code)]
pub struct AgentBlueToken {
    index: usize,
}

#[allow(dead_code)]
impl AgentBlueToken {
    pub fn new(index: usize) -> Self { Self { index } }
}

#[allow(dead_code)]
pub struct AgentsBlue {
    elements: Vec<AgentBlue>,
}

impl AgentsBlue {
    pub fn new() -> Self { Self { elements: Vec::new() } }
}

pub struct Components {
    pub ground_plane: GroundPlane,
    pub base_factory: BaseFactory,
    // pub reds: Vec<ResourceRed>,
    // pub greens: Vec<ResourceGreen>,
    
    // pub agents: Vec<Agent>,
}

pub struct Resources2 {
    pub blues2: Blues,
}

#[allow(dead_code)]
impl Resources2 {
    pub fn new(blues2: Blues) -> Self { Self { blues2 } }
}

pub struct Agents {
    pub agent_blue: AgentsBlue, 
}