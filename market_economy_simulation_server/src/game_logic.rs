use std::sync::mpsc;

use game_logic_interface::{
    GameLogicMessageCritical, GameLogicMessageHeavy, GameLogicMessageLight, GameLogicMessageRequest,
};

use crate::terrain;

pub mod game_logic_interface;

pub struct GameLogicSettings {
    pub map_size: usize,
}

pub struct GameLagic {
    settings: GameLogicSettings,

    channel_0_rx: mpsc::Receiver<GameLogicMessageRequest>,
    channel_1_tx: mpsc::Sender<GameLogicMessageHeavy>,
    _channel_2_tx: mpsc::Sender<GameLogicMessageLight>,
    _channel_3_tx: mpsc::Sender<GameLogicMessageCritical>,

    terrain: terrain::Terrain,
}

impl GameLagic {
    pub fn new(
        settings: GameLogicSettings,
        channel_0_rx: mpsc::Receiver<GameLogicMessageRequest>,
        channel_1_tx: mpsc::Sender<GameLogicMessageHeavy>,
        channel_2_tx: mpsc::Sender<GameLogicMessageLight>,
        channel_3_tx: mpsc::Sender<GameLogicMessageCritical>,
    ) -> Self {
        let size = settings.map_size;

        let terrain = terrain::Terrain::new(size, size, 1.0);

        Self {
            settings,

            channel_0_rx,
            channel_1_tx,
            _channel_2_tx: channel_2_tx,
            _channel_3_tx: channel_3_tx,

            terrain,
        }
    }

    pub(crate) fn update(&mut self) {
        let res = self.channel_0_rx.try_recv();
        match res {
            Ok(message) => match message {
                GameLogicMessageRequest::GetTerrain => {
                    let res = self
                        .channel_1_tx
                        .send(GameLogicMessageHeavy::Terrain(self.terrain.clone()));
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
}
