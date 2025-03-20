use std::sync::mpsc;

use wgpu_renderer::performance_monitor::watch;

use crate::{
    heightmap_generator::{HeightMap, HeightMapDetails},
    point_lights,
};

pub const WATCH_POINT_SIZE: usize = 7;

pub enum GameLogicMessageRequest {
    GetTerrain(HeightMapDetails), // Requests the terrain heightmap
}

pub enum GameLogicMessageHeavy {
    Terrain(HeightMap), // The terrain heightmap data
}

pub enum GameLogicMessageMedium {
    UpdateWatchPoints(watch::WatchViewerData<WATCH_POINT_SIZE>), // all the data for a point of the performance monitor
}

pub enum GameLogicMessageLight {
    UpdatePointLight(point_lights::PointLight), // updates the data of a point light
}

pub enum GameLogicMessageCritical {}

pub trait GameLogicInterface {
    fn get_heavy_messages(&self) -> &mpsc::Receiver<GameLogicMessageHeavy>;
    fn get_medium_messages(&self) -> &mpsc::Receiver<GameLogicMessageMedium>;
    fn get_light_messages(&self) -> &mpsc::Receiver<GameLogicMessageLight>;
    fn get_critical_messages(&self) -> &mpsc::Receiver<GameLogicMessageCritical>;
    fn send_messages(&self) -> &mpsc::Sender<GameLogicMessageRequest>;
}
