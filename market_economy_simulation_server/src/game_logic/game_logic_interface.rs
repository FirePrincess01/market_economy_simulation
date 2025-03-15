use std::sync::mpsc;

use wgpu_renderer::performance_monitor::watch;

use crate::{point_lights, terrain};

pub const WATCH_POINT_SIZE: usize = 7;

pub enum GameLogicMessageRequest {
    GetTerrain, // Requests the terrain heightmap
}

pub enum GameLogicMessageHeavy {
    Terrain(terrain::Terrain), // The terrain heightmap data
}

pub enum GameLogicMessageLight {
    UpdatePointLight(point_lights::PointLight), // updates the data of a point light
    UpdateWatchPoints(watch::WatchViewerData<WATCH_POINT_SIZE>),
}

pub enum GameLogicMessageCritical {}

pub trait GameLogicInterface {
    fn get_heavy_messages(&self) -> &mpsc::Receiver<GameLogicMessageHeavy>;
    fn get_light_messages(&self) -> &mpsc::Receiver<GameLogicMessageLight>;
    fn get_critical_messages(&self) -> &mpsc::Receiver<GameLogicMessageCritical>;
    fn send_messages(&self) -> &mpsc::Sender<GameLogicMessageRequest>;
}
