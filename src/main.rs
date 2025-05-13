mod enemy;
mod enums;
mod features;
mod gui;
mod gui_utils;
mod item;
mod lsystems;
mod map;
mod nodemap;
mod notebook;
mod npc;
mod npc_utils;
mod player;
mod puzzle;
mod puzzles;
mod settlement;
mod settlements;
mod shop;
mod stats;
mod utils;
use ratatui::crossterm::terminal;
use std::thread::sleep;
use std::time::{Duration, Instant};

mod gamestate;
use gamestate::GameState;

#[macro_use]
extern crate lazy_static;
extern crate log;

use log::{Level, LevelFilter, Metadata, Record, SetLoggerError};
use std::fs::OpenOptions;
use std::io::prelude::*;
// use std::sync::Mutex;
use std::sync::{Arc, Mutex};

struct SimpleLogger {
    file: Mutex<std::fs::File>,
}

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let mut file = self.file.lock().unwrap();
            writeln!(file, "{} - {}", record.level(), record.args()).unwrap();
        }
    }

    fn flush(&self) {}
}

lazy_static! {
    static ref LOGGER: SimpleLogger = {
        let file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open("log.txt")
            .unwrap();

        SimpleLogger {
            file: Mutex::new(file),
        }
    };
}

pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(&*LOGGER).map(|()| log::set_max_level(LevelFilter::Info))
}

fn main() {
    init().unwrap();
    let mut game_state = GameState::new();

    terminal::enable_raw_mode().unwrap();

    // game_state.start_update_threads();
    GameState::start_update_threads(Arc::clone(&game_state));

    let mut previous = Instant::now();
    let timestep = Duration::from_millis(1000 / 15);

    loop {
        let now = Instant::now();
        let elapsed = now - previous;

        if elapsed >= timestep {
            previous = now;
            {
                let mut game_state = game_state.lock().unwrap();
                game_state.draw();
            }
            {
                let mut game_state = game_state.lock().unwrap();
                if !game_state.update() {
                    break;
                }
            }
            // Update game state here
        } else {
            // sleep(Duration::from_millis());
            sleep(timestep - elapsed);
        }
        // game_state.draw();
        // if game_state.update() == false {
        //     break;
        // }
    }

    terminal::disable_raw_mode().unwrap();
}
