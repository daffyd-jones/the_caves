mod enums;
mod map;
mod player;
mod lsystems;
mod enemy;
mod npc;
mod item;
mod gui;
mod settlements;
mod settlement;
mod shop;
mod notebook;

// use crossterm::event::{read, Event, KeyCode};
use ratatui::crossterm::terminal;
// use std::io::stdout;
// use std::time::Duration;
// use rand::Rng;
// use ratatui::Terminal;
// use ratatui::backend::CrosstermBackend;
// use ratatui::prelude::Line;
// use ratatui::widgets::{Block, Borders, Paragraph, Wrap, Padding};
// use ratatui::layout::{Layout, Constraint, Direction, Margin};
// use ratatui::style::{Color, Style};
// use ratatui::text::{Text, Span};

// use std::collections::HashMap;

use std::time::{Duration, Instant};
use std::thread::sleep;

mod gamestate;
use gamestate::{GameState};


#[macro_use]
extern crate lazy_static;
extern crate log;

use log::{Record, Level, Metadata, SetLoggerError, LevelFilter};
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

        SimpleLogger { file: Mutex::new(file) }
    };
}

pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(&*LOGGER)
    .map(|()| log::set_max_level(LevelFilter::Info))
}




fn main() {
    init().unwrap();
    let mut game_state = GameState::new();

    terminal::enable_raw_mode().unwrap();

    // game_state.start_update_threads();
    GameState::start_update_threads(Arc::clone(&game_state));

    let mut previous = Instant::now();
    let timestep = Duration::from_millis(1000 / 15); // 60 updates per second

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
                if game_state.update() == false {
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
