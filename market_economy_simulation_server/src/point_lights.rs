use std::sync::mpsc;

use crate::{game_logic::game_logic_interface::GameLogicMessageLight, terrain::Terrain};

pub struct PointLights {
    lights: Vec<PointLight>,

    requires_update: bool,
}

impl PointLights {
    pub fn new(terrain: &Terrain) -> Self {
        let mut lights = Vec::new();

        let gradient = colorous::TURBO;

        let mut id = 0;
        for x in (0..terrain.size_x).step_by(10) {
            for y in (0..terrain.size_y).step_by(10) {
                let position = cgmath::Vector3::new(
                    (x as f32 - terrain.size_x as f32 / 2.0) * terrain.distance,
                    // (y as f32 - terrain.size_y as f32 / 2.0) * terrain.distance,
                    (y as f32) * terrain.distance,
                    terrain.heights[y * terrain.size_x + x] + 5.0,
                );

                let color = gradient.eval_rational(id % 100, 100);

                let color: cgmath::Vector3<f32> = cgmath::Vector3::new(
                    color.r as f32 / 255.0,
                    color.g as f32 / 255.0,
                    color.b as f32 / 255.0,
                );

                lights.push(PointLight {
                    id: id as u32,
                    position,
                    color,
                    attenuation: Attenuation::_100,
                });
                id += 1;
            }
        }

        Self {
            lights,
            requires_update: true,
        }
    }

    pub fn update(&mut self, channel: &mpsc::Sender<GameLogicMessageLight>) {
        if self.requires_update {
            for elem in &mut self.lights {
                elem.position.x += 0.02;
                // elem.color.x = (elem.color.x + 0.001) % 1.0;

                let res = channel.send(GameLogicMessageLight::UpdatePointLight(elem.clone()));
                match res {
                    Ok(_) => {}
                    Err(err) => {
                        // println!("{}", err)
                    }
                }
            }
        }
    }
}

#[derive(Clone)]
pub struct PointLight {
    pub id: u32,

    pub position: cgmath::Vector3<f32>,
    pub color: cgmath::Vector3<f32>,
    pub attenuation: Attenuation,
}

#[derive(Clone, Copy)]
pub enum Attenuation {
    _7 = 0,
    _13 = 1,
    _20 = 2,
    _32 = 3,
    _50 = 4,
    _65 = 5,
    _100 = 6,
    _160 = 7,
    _200 = 8,
    _325 = 9,
    _600 = 10,
    _3250 = 11,
}
