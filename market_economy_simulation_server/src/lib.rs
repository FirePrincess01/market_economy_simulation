//! Contains the game logic
//! Intended to be easyly ported to be used in a multiplayer session,
//! if this feature may ever be implemented

pub mod game_logic;
pub mod terrain;

use std::sync::mpsc;
use std::thread::{self};
use std::time::Duration;

use game_logic::GameLagic;
use game_logic::game_logic_interface::{
    GameLogicInterface, GameLogicMessageCritical, GameLogicMessageHeavy, GameLogicMessageLight,
    GameLogicMessageRequest,
};

pub struct GameLogicSingleThreaded {
    game_logic: GameLagic,

    channel_0_tx: mpsc::Sender<GameLogicMessageRequest>,
    channel_1_rx: mpsc::Receiver<GameLogicMessageHeavy>,
    channel_2_rx: mpsc::Receiver<GameLogicMessageLight>,
    channel_3_rx: mpsc::Receiver<GameLogicMessageCritical>,
}

impl GameLogicSingleThreaded {
    pub fn new() -> Self {
        let (channel_0_tx, channel_0_rx) = mpsc::channel();
        let (channel_1_tx, channel_1_rx) = mpsc::channel();
        let (channel_2_tx, channel_2_rx) = mpsc::channel();
        let (channel_3_tx, channel_3_rx) = mpsc::channel();

        let game_logic = GameLagic::new(channel_0_rx, channel_1_tx, channel_2_tx, channel_3_tx);

        Self {
            game_logic,
            channel_0_tx,
            channel_1_rx,
            channel_2_rx,
            channel_3_rx,
        }
    }

    fn update(&mut self) {
        self.game_logic.update();
    }
}

impl Default for GameLogicSingleThreaded {
    fn default() -> Self {
        Self::new()
    }
}

impl GameLogicInterface for GameLogicSingleThreaded {
    fn get_heavy_messages(&self) -> &mpsc::Receiver<GameLogicMessageHeavy> {
        &self.channel_1_rx
    }

    fn get_light_messages(&self) -> &mpsc::Receiver<GameLogicMessageLight> {
        &self.channel_2_rx
    }

    fn get_critical_messages(&self) -> &mpsc::Receiver<GameLogicMessageCritical> {
        &self.channel_3_rx
    }

    fn send_messages(&self) -> &mpsc::Sender<GameLogicMessageRequest> {
        &self.channel_0_tx
    }
}

pub struct GameLogicMultiThreaded {
    _game_logic: thread::JoinHandle<()>,

    channel_0_tx: mpsc::Sender<GameLogicMessageRequest>,
    channel_1_rx: mpsc::Receiver<GameLogicMessageHeavy>,
    channel_2_rx: mpsc::Receiver<GameLogicMessageLight>,
    channel_3_rx: mpsc::Receiver<GameLogicMessageCritical>,
}

impl GameLogicMultiThreaded {
    pub fn new() -> Self {
        let (channel_0_tx, channel_0_rx) = mpsc::channel();
        let (channel_1_tx, channel_1_rx) = mpsc::channel();
        let (channel_2_tx, channel_2_rx) = mpsc::channel();
        let (channel_3_tx, channel_3_rx) = mpsc::channel();

        let game_logic = thread::spawn(move || {
            let mut game_logic =
                GameLagic::new(channel_0_rx, channel_1_tx, channel_2_tx, channel_3_tx);

            loop {
                game_logic.update();
                thread::sleep(Duration::from_millis(20));
            }
        });

        Self {
            _game_logic: game_logic,
            channel_0_tx,
            channel_1_rx,
            channel_2_rx,
            channel_3_rx,
        }
    }
}

impl Default for GameLogicMultiThreaded {
    fn default() -> Self {
        Self::new()
    }
}

impl GameLogicInterface for GameLogicMultiThreaded {
    fn get_heavy_messages(&self) -> &mpsc::Receiver<GameLogicMessageHeavy> {
        &self.channel_1_rx
    }

    fn get_light_messages(&self) -> &mpsc::Receiver<GameLogicMessageLight> {
        &self.channel_2_rx
    }

    fn get_critical_messages(&self) -> &mpsc::Receiver<GameLogicMessageCritical> {
        &self.channel_3_rx
    }

    fn send_messages(&self) -> &mpsc::Sender<GameLogicMessageRequest> {
        &self.channel_0_tx
    }
}

enum GameLogicExecution {
    #[allow(dead_code)]
    SingleThreaded(GameLogicSingleThreaded),
    #[allow(dead_code)]
    Multithreaded(GameLogicMultiThreaded),
}

pub struct GameLogicServer {
    server: GameLogicExecution,
}

impl GameLogicServer {
    pub fn new() -> Self {
        #[allow(clippy::needless_late_init)]
        let server: GameLogicExecution;
        cfg_if::cfg_if! {
            // apply scale factor for the web
            if #[cfg(target_arch = "wasm32")] {
                server =
                GameLogicExecution::SingleThreaded(GameLogicSingleThreaded::new())
            }
            else {
                server =
                GameLogicExecution::Multithreaded(GameLogicMultiThreaded::new())
                // GameLogicExecution::SingleThreaded(GameLogicSingleThreaded::new())
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

impl Default for GameLogicServer {
    fn default() -> Self {
        Self::new()
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
