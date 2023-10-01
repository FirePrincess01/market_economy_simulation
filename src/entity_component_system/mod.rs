
pub mod component;
pub mod system;

use component::LiveStates;
use component::FarmStates;
use component::HouseStates;
use component::Mesh;


pub struct Entity {
    pub live_stat: Option<usize>,
    pub farm_stat: Option<usize>,
    pub house_stat: Option<usize>,
    pub mesh: Option<usize>,
}

impl Entity {
    pub fn new() -> Self 
    {
        Self { 
            live_stat: None, 
            farm_stat: None,
            house_stat: None,
            mesh: None 
        }
    }
}

pub struct World {
    pub entities: Vec<Entity>,

    pub live_stats: Vec<LiveStates>,
    pub farm_stats: Vec<FarmStates>,
    pub house_stats: Vec<HouseStates>,
    pub meshes: Vec<Mesh>,
}

impl World {
    pub fn new() -> Self
    {
        Self {
            entities: Vec::new(),

            live_stats: Vec::new(),
            farm_stats: Vec::new(),
            house_stats: Vec::new(),
            meshes: Vec::new(),
        }
    }

    pub fn add_entity(&mut self, entity: Entity) -> usize
    {
        let index = self.entities.len();
        self.entities.push(entity);

        index
    }

    pub fn add_live_states(&mut self, entity: usize, mut component: LiveStates) 
    {
        // set entity index in component
        component.entity = entity; 

        // add component
        let index = self.live_stats.len();
        self.live_stats.push(component);
        
        // set component index in entity
        self.entities[entity].live_stat = Some(index);
    }

    pub fn add_farm_states(&mut self, entity: usize, mut component: FarmStates) 
    {
        // set entity index in component
        component.entity = entity;

        // add component
        let index = self.farm_stats.len();
        self.farm_stats.push(component);
        
        // set component index in entity
        self.entities[entity].farm_stat = Some(index);
    }

    pub fn add_house_states(&mut self, entity: usize, mut component: HouseStates) 
    {
        // set entity index in component
        component.entity = entity;

        // add component
        let index = self.house_stats.len();
        self.house_stats.push(component);
        
        // set component index in entity
        self.entities[entity].house_stat = Some(index);
    }

    pub fn add_mesh(&mut self, entity: usize, mut component: Mesh) 
    {
        // set entity index in component
        component.entity = entity;

        // add component
        let index = self.meshes.len();
        self.meshes.push(component);
        
        // set component index in entity
        self.entities[entity].mesh = Some(index);
    }



}

