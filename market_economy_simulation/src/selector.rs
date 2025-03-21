//! Selects entities with the mouse

mod ray_triangle_intersection;
mod terrain_selector;

use cgmath::Zero;
use terrain_selector::TerrainSelector;

use crate::terrain_storage::terrain_texture_details::TerrainTextureDetails;

pub struct Selector {
    terrain_selector: TerrainSelector,

    view_position: cgmath::Vector3<f32>,
    view_direction: cgmath::Vector3<f32>,

    entity: u32,
}

pub const ENTITY_TERRAIN_BIT: u32 = 1 << 31;

impl Selector {
    pub fn new() -> Self {

        Self {
            terrain_selector: TerrainSelector::new(),
            view_position:  cgmath::Vector3::zero(),
            view_direction: cgmath::Vector3::zero(),
            entity: 0,
        }
    }

    pub fn update_view_position(&mut self, view_position: cgmath::Vector3<f32>) {
        self.view_position = view_position;
    }

    pub fn update_view_direction(&mut self, view_direction: cgmath::Vector3<f32>) {
        self.view_direction = view_direction;
    }

    pub fn update_entity(&mut self, entity: u32) {
        self.entity = entity;
    }

    pub fn find_selection(
        &mut self,
        height_map_details: &[TerrainTextureDetails],
        height_maps: &[Vec<f32>],
    ) -> Option<Result> {
        let entity_bit_mask = self.entity & 0xFF000000;
        let entity_index = (self.entity & 0x00FFFFFF) as usize;

        if entity_bit_mask == ENTITY_TERRAIN_BIT {
            let height_map_detail = &height_map_details[entity_index];
            let height_map = &height_maps[entity_index];

            let res = self.terrain_selector.find_intersection(
                height_map_detail,
                height_map,
                &self.view_position,
                &self.view_direction,
            );

            if let Some(triangle) = res {
                return Some(Result::Terrain(triangle));
            }
        }

        None
    }
}

pub struct Triangle {
    /// First Vertex
    pub v0: cgmath::Vector3<f32>,
    /// Second Vertex
    pub v1: cgmath::Vector3<f32>,
    /// Thrid Vertex
    pub v2: cgmath::Vector3<f32>,

    /// View position
    pub orig: cgmath::Vector3<f32>,
    /// View direction
    pub dir: cgmath::Vector3<f32>,

    /// Distance form the origin to the intersection point p
    pub distance: f32,
    /// Barycentric coordinate 0 inside the triangle
    pub u: f32,
    /// Barycentric coordinate 1 inside the triangle
    pub v: f32,

    /// Point of the intersection
    pub p: cgmath::Vector3<f32>,
}

pub enum Result {
    Terrain(Triangle),
}
