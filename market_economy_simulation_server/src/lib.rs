//! Contains the game logic
//! Intended to be easily ported to be used in a multiplayer session,
//! if this feature may ever be implemented

pub mod game_logic;
pub mod point_lights;
pub mod terrain;

use std::sync::mpsc;
use std::thread::{self};
use std::time::Duration;

use game_logic::game_logic_interface::{
    GameLogicInterface, GameLogicMessageCritical, GameLogicMessageHeavy, GameLogicMessageLight,
    GameLogicMessageMedium, GameLogicMessageRequest,
};
use game_logic::{GameLogic, GameLogicSettings};

pub struct GameLogicSingleThreaded {
    game_logic: Box<GameLogic>,

    channel_0_tx: mpsc::Sender<GameLogicMessageRequest>,
    channel_1_rx: mpsc::Receiver<GameLogicMessageHeavy>,
    channel_2_rx: mpsc::Receiver<GameLogicMessageMedium>,
    channel_3_rx: mpsc::Receiver<GameLogicMessageLight>,
    channel_4_rx: mpsc::Receiver<GameLogicMessageCritical>,
}

impl GameLogicSingleThreaded {
    pub fn new(settings: GameLogicSettings) -> Self {
        let (channel_0_tx, channel_0_rx) = mpsc::channel();
        let (channel_1_tx, channel_1_rx) = mpsc::channel();
        let (channel_2_tx, channel_2_rx) = mpsc::channel();
        let (channel_3_tx, channel_3_rx) = mpsc::channel();
        let (channel_4_tx, channel_4_rx) = mpsc::channel();

        let game_logic = Box::new(GameLogic::new(
            settings,
            channel_0_rx,
            channel_1_tx,
            channel_2_tx,
            channel_3_tx,
            channel_4_tx,
        ));

        Self {
            game_logic,
            channel_0_tx,
            channel_1_rx,
            channel_2_rx,
            channel_3_rx,
            channel_4_rx,
        }
    }

    fn update(&mut self) {
        self.game_logic.update();
    }
}

impl GameLogicInterface for GameLogicSingleThreaded {
    fn get_heavy_messages(&self) -> &mpsc::Receiver<GameLogicMessageHeavy> {
        &self.channel_1_rx
    }

    fn get_medium_messages(
        &self,
    ) -> &mpsc::Receiver<game_logic::game_logic_interface::GameLogicMessageMedium> {
        &self.channel_2_rx
    }

    fn get_light_messages(&self) -> &mpsc::Receiver<GameLogicMessageLight> {
        &self.channel_3_rx
    }

    fn get_critical_messages(&self) -> &mpsc::Receiver<GameLogicMessageCritical> {
        &self.channel_4_rx
    }

    fn send_messages(&self) -> &mpsc::Sender<GameLogicMessageRequest> {
        &self.channel_0_tx
    }
}

pub struct GameLogicMultiThreaded {
    _game_logic: thread::JoinHandle<()>,

    channel_0_tx: mpsc::Sender<GameLogicMessageRequest>,
    channel_1_rx: mpsc::Receiver<GameLogicMessageHeavy>,
    channel_2_rx: mpsc::Receiver<GameLogicMessageMedium>,
    channel_3_rx: mpsc::Receiver<GameLogicMessageLight>,
    channel_4_rx: mpsc::Receiver<GameLogicMessageCritical>,
}

impl GameLogicMultiThreaded {
    pub fn new(settings: GameLogicSettings) -> Self {
        let (channel_0_tx, channel_0_rx) = mpsc::channel();
        let (channel_1_tx, channel_1_rx) = mpsc::channel();
        let (channel_2_tx, channel_2_rx) = mpsc::channel();
        let (channel_3_tx, channel_3_rx) = mpsc::channel();
        let (channel_4_tx, channel_4_rx) = mpsc::channel();

        let game_logic = thread::spawn(move || {
            let mut game_logic = GameLogic::new(
                settings,
                channel_0_rx,
                channel_1_tx,
                channel_2_tx,
                channel_3_tx,
                channel_4_tx,
            );

            loop {
                let start_time = instant::Instant::now();
                game_logic.update();
                let stop_time = instant::Instant::now();

                let time_passed = stop_time - start_time;

                let interval = Duration::from_millis(16);
                if time_passed < interval {
                    thread::sleep(interval - time_passed);
                }
            }
        });

        Self {
            _game_logic: game_logic,
            channel_0_tx,
            channel_1_rx,
            channel_2_rx,
            channel_3_rx,
            channel_4_rx,
        }
    }
}

impl GameLogicInterface for GameLogicMultiThreaded {
    fn get_heavy_messages(&self) -> &mpsc::Receiver<GameLogicMessageHeavy> {
        &self.channel_1_rx
    }

    fn get_medium_messages(&self) -> &mpsc::Receiver<GameLogicMessageMedium> {
        &self.channel_2_rx
    }

    fn get_light_messages(&self) -> &mpsc::Receiver<GameLogicMessageLight> {
        &self.channel_3_rx
    }

    fn get_critical_messages(&self) -> &mpsc::Receiver<GameLogicMessageCritical> {
        &self.channel_4_rx
    }

    fn send_messages(&self) -> &mpsc::Sender<GameLogicMessageRequest> {
        &self.channel_0_tx
    }
}

enum GameLogicExecution {
    SingleThreaded(GameLogicSingleThreaded),
    #[allow(dead_code)] // unused in wasm
    Multithreaded(GameLogicMultiThreaded),
}

pub struct GameLogicServer {
    server: GameLogicExecution,
}

impl GameLogicServer {
    pub fn new(settings: GameLogicSettings) -> Self {
        #[allow(clippy::needless_late_init)]
        let server: GameLogicExecution;
        cfg_if::cfg_if! {
            // apply scale factor for the web
            if #[cfg(target_arch = "wasm32")] {
                server =
                GameLogicExecution::SingleThreaded(GameLogicSingleThreaded::new(settings))
            }
            else {
                server = match settings.enable_multithreading {
                    true => GameLogicExecution::Multithreaded(GameLogicMultiThreaded::new(settings)),
                    false => GameLogicExecution::SingleThreaded(GameLogicSingleThreaded::new(settings)),
                }
            }
        }

        Self { server }
    }

    pub fn update(&mut self) {
        match &mut self.server {
            GameLogicExecution::SingleThreaded(game_logic_single_threaded) => {
                game_logic_single_threaded.update();
            }
            GameLogicExecution::Multithreaded(_game_logic_multi_threaded) => {
                // update is done in another thread
            }
        }
    }
}

impl GameLogicInterface for GameLogicServer {
    fn get_heavy_messages(&self) -> &mpsc::Receiver<GameLogicMessageHeavy> {
        match &self.server {
            GameLogicExecution::SingleThreaded(game_logic_single_threaded) => {
                game_logic_single_threaded.get_heavy_messages()
            }
            GameLogicExecution::Multithreaded(game_logic_multi_threaded) => {
                game_logic_multi_threaded.get_heavy_messages()
            }
        }
    }

    fn get_medium_messages(&self) -> &mpsc::Receiver<GameLogicMessageMedium> {
        match &self.server {
            GameLogicExecution::SingleThreaded(game_logic_single_threaded) => {
                game_logic_single_threaded.get_medium_messages()
            }
            GameLogicExecution::Multithreaded(game_logic_multi_threaded) => {
                game_logic_multi_threaded.get_medium_messages()
            }
        }
    }

    fn get_light_messages(&self) -> &mpsc::Receiver<GameLogicMessageLight> {
        match &self.server {
            GameLogicExecution::SingleThreaded(game_logic_single_threaded) => {
                game_logic_single_threaded.get_light_messages()
            }
            GameLogicExecution::Multithreaded(game_logic_multi_threaded) => {
                game_logic_multi_threaded.get_light_messages()
            }
        }
    }

    fn get_critical_messages(&self) -> &mpsc::Receiver<GameLogicMessageCritical> {
        match &self.server {
            GameLogicExecution::SingleThreaded(game_logic_single_threaded) => {
                game_logic_single_threaded.get_critical_messages()
            }
            GameLogicExecution::Multithreaded(game_logic_multi_threaded) => {
                game_logic_multi_threaded.get_critical_messages()
            }
        }
    }

    fn send_messages(&self) -> &mpsc::Sender<GameLogicMessageRequest> {
        match &self.server {
            GameLogicExecution::SingleThreaded(game_logic_single_threaded) => {
                game_logic_single_threaded.send_messages()
            }
            GameLogicExecution::Multithreaded(game_logic_multi_threaded) => {
                game_logic_multi_threaded.send_messages()
            }
        }
    }
}
