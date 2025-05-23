use std::sync::mpsc;

use game_logic_interface::{
    GameLogicMessageCritical, GameLogicMessageHeavy, GameLogicMessageLight, GameLogicMessageMedium,
    GameLogicMessageRequest,
};
use wgpu_renderer::performance_monitor::watch;

use crate::ants;
use crate::heightmap_generator;

pub mod game_logic_interface;

pub struct GameLogicSettings {
    // pub map_size: usize,
    pub enable_multithreading: bool,
    pub max_nr_ants: usize,
}

pub struct GameLogic {
    _settings: GameLogicSettings,

    channel_0_rx: mpsc::Receiver<GameLogicMessageRequest>,
    channel_1_tx: mpsc::Sender<GameLogicMessageHeavy>,
    channel_2_tx: mpsc::Sender<GameLogicMessageMedium>,
    channel_3_tx: mpsc::Sender<GameLogicMessageLight>,
    _channel_4_tx: mpsc::Sender<GameLogicMessageCritical>,

    heightmap_generator: heightmap_generator::HeightMapGenerator,
    ants: ants::Ants,
    // terrain: terrain::Terrain,
    // point_lights: point_lights::PointLights,
    watch: watch::Watch<{ game_logic_interface::WATCH_POINT_SIZE }>,
}

impl GameLogic {
    pub fn new(
        settings: GameLogicSettings,
        channel_0_rx: mpsc::Receiver<GameLogicMessageRequest>,
        channel_1_tx: mpsc::Sender<GameLogicMessageHeavy>,
        channel_2_tx: mpsc::Sender<GameLogicMessageMedium>,
        channel_3_tx: mpsc::Sender<GameLogicMessageLight>,
        channel_4_tx: mpsc::Sender<GameLogicMessageCritical>,
    ) -> Self {
        // let size = settings.map_size;

        let heightmap_generator = heightmap_generator::HeightMapGenerator::new();
        let ants = ants::Ants::new(settings.max_nr_ants);

        // let terrain = terrain::Terrain::new(size, size, 1.0);
        // let point_lights = point_lights::PointLights::new(&terrain);

        let watch = watch::Watch::new();

        Self {
            _settings: settings,

            channel_0_rx,
            channel_1_tx,
            channel_2_tx,
            channel_3_tx,
            _channel_4_tx: channel_4_tx,

            heightmap_generator,
            ants,
            // terrain,
            // point_lights,
            watch,
        }
    }

    pub(crate) fn update(&mut self) {
        // update ups viewer
        self.watch.update();
        let _res = self
            .channel_2_tx
            .send(GameLogicMessageMedium::UpdateWatchPoints(
                self.watch.get_viewer_data(),
            ));

        self.watch.start(0, "Process Requests");
        {
            let res = self.channel_0_rx.try_recv();
            match res {
                Ok(message) => match message {
                    GameLogicMessageRequest::GetTerrain(heightmap_details) => {
                        let heightmap = self.heightmap_generator.generate(heightmap_details);
                        let res = self
                            .channel_1_tx
                            .send(GameLogicMessageHeavy::Terrain(heightmap));
                        match res {
                            Ok(_) => {}
                            Err(err) => println!("{}", err),
                        }
                    }
                },
                Err(_err) => {
                    // no message found
                }
            }
        }
        self.watch.stop(0);

        self.watch.start(1, "Update point lights");
        {
            // point lights
            // self.point_lights.update(&self.channel_3_tx);
            self.ants.update(&self.channel_3_tx);
        }
        self.watch.stop(1);
    }
}
