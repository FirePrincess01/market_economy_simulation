//! Factory controlled by the player

use crate::{
    ecs2::{AgentBlueToken, Agents, BlueToken, EntityInterface, Resources2},
    ground_plane::Location,
};

pub enum Recipe {
    Agent,
}

#[allow(dead_code)]
pub enum Result {
    Blue(BlueToken),
    Agent(AgentBlueToken),
}

#[allow(dead_code)]
pub struct BaseFactory {
    entity_index: usize,
    location: Location,

    // reds: Vec<RedToken>,
    // greens: Vec<GreenToken>,
    blues: Vec<BlueToken>,

    progress: f64,

    agent: Option<AgentBlueToken>,

    recipe: Recipe,

    result: Vec<Result>,
}

impl BaseFactory {
    pub fn new(entity_index: usize, location: Location) -> Self {
        Self {
            entity_index,
            location,
            // reds: Vec::new(),
            // greens: Vec::new(),
            blues: Vec::new(),
            progress: 0.0,
            agent: None,
            recipe: Recipe::Agent,
            result: Vec::new(),
        }
    }

    pub fn add_blue(&mut self, blue_token: BlueToken, resources: &mut Resources2) {
        let location_offset_y = 0.8;
        let location_offset_x = 0.2;

        let blue = resources.blues2.get_mut(&blue_token);
        blue.z = 0.0;
        blue.y = self.location.y as f32 + location_offset_y;
        blue.x = self.location.x as f32 + location_offset_x;

        self.blues.push(blue_token);
    }

    pub fn produce(
        &mut self,
        // dt: instant::Duration,
        _resources: &mut Resources2,
        _agents: &mut Agents,
    ) {
    }

    pub fn location(&self) -> &Location {
        &self.location
    }
}

impl EntityInterface for BaseFactory {
    fn width(&self) -> f32 {
        1.0
    }

    fn height(&self) -> f32 {
        1.0
    }
}
