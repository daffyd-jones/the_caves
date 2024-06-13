use crossterm::event::{poll, read, Event, KeyCode};
use crossterm::terminal;
use std::io::stdout;
use std::time::Duration;
use rand::Rng;
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::layout::{Layout, Constraint, Direction, Margin};
use ratatui::style::{Color, Style};
use ratatui::text::{Text, Span};

use std::collections::HashMap;


#[macro_use]
extern crate lazy_static;
extern crate log;

use log::{Record, Level, Metadata, SetLoggerError, LevelFilter};
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::sync::Mutex;

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

// Define the Cell enum
#[derive(Clone, Copy, PartialEq)]
enum Cell {
    Empty,
    Wall,
    Player,
    Tunnel,
}

// Define the Map struct
struct Map {
    cells: Vec<Vec<Cell>>,
    px: usize,
    py: usize,
    tunnels: HashMap<(usize, usize), (usize, usize)>,
    viewport_x: usize,
    viewport_y: usize,
    viewport_width: usize,
    viewport_height: usize,
    gen_x: i32,
    gen_y: i32,
}

const MAP_W: usize = 300;
const MAP_H: usize = 200;

impl Map {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut cells = vec![vec![Cell::Wall; MAP_W]; MAP_H];
        for _ in 0..800 {
            let x = rng.gen_range(0..MAP_W - 9);
            let y = rng.gen_range(0..MAP_H - 9);
            for i in x..x+9 {
                for j in y..y+9 {
                    cells[j][i] = Cell::Empty;
                }
            }
        }
        let mut px = 0;
        let mut py = 0;
        let x_centre = MAP_W/2;
        let y_centre = MAP_H/2;
        loop {
            px = rng.gen_range(x_centre-20..x_centre+20);
            py = rng.gen_range(y_centre-10..y_centre+10);
            if cells[py][px] == Cell::Empty {
               cells[py][px] = Cell::Player;
               break;
            }
        }

        let mut tunnels = HashMap::new();
        for _ in 0..100 {
            let (x1, y1, x2, y2) = loop {
                let x1 = rng.gen_range(0..MAP_W);
                let y1 = rng.gen_range(0..MAP_H);
                let x2 = rng.gen_range(0..MAP_W);
                let y2 = rng.gen_range(0..MAP_H);
                if cells[y1][x1] == Cell::Empty && cells[y2][x2] == Cell::Empty {
                    break (x1, y1, x2, y2);
                }
            };
            cells[y1][x1] = Cell::Tunnel;
            cells[y2][x2] = Cell::Tunnel;
            tunnels.insert((x1, y1), (x2, y2));
            tunnels.insert((x2, y2), (x1, y1));
        }

        let viewport_x = px.clone() - 30;
        let viewport_y = py.clone() - 30;
        let viewport_width = 0;
        let viewport_height = 0;

        Self { cells, px, py, tunnels, viewport_x, viewport_y, viewport_width, viewport_height, gen_x: 0, gen_y: 0}
    }

    fn set_viewport(&mut self, h: usize, w:usize) {
        self.viewport_height = h;
        self.viewport_width = w;
        self.viewport_y = (self.cells.len()/2) - (h/2);
        self.viewport_x = (self.cells[0].len()/2) - (w/2);
    }

    fn fill_map(&mut self, cells: Vec<Vec<Cell>>, sx: usize, ex: usize, sy: usize, ey: usize) {
        log::info!("sx: {}, ex: {}, sy: {}, ey: {}", sx, ex, sy, ey);
        for j in sy..=ey {
            for i in sx..=ex {
                self.cells[j][i] = cells[j][i];
            }
        }
    }

    fn map_fill(&mut self) {
        let mut rng = rand::thread_rng();
        let mut t_cells = vec![vec![Cell::Wall; MAP_W]; MAP_H];
        for _ in 0..800 {
            let x = rng.gen_range(0..MAP_W-9);
            let y = rng.gen_range(0..MAP_H-9);
            for i in x..x+9 {
                for j in y..y+9 {
                    t_cells[j][i] = Cell::Empty;
                }
            }
        }

        //add tunnels


        let y_max = self.cells.len() - 1;
        let x_max = self.cells[0].len() - 1;

        let (mut sx, mut ex, mut sy, mut ey) = {
            if self.gen_x > 0 && self.gen_y == 0 {
                // log::info!("gen_x: {}", self.gen_x);
                (0_usize, self.gen_x as usize, 0_usize, 0_usize)
            } else if self.gen_x < 0 && self.gen_y == 0 {
                // log::info!("gen_x: {}", self.gen_x);
                ((x_max as i32 + self.gen_x) as usize, x_max, 0_usize, 0_usize)
            } else if self.gen_y > 0 && self.gen_x == 0 {
                // log::info!("gen_x: {}", self.gen_y);
                (0_usize, 0_usize, 0_usize, self.gen_y as usize)
            } else if self.gen_y < 0 && self.gen_x == 0 {
                // log::info!("gen_x: {}", self.gen_y);
                (0_usize, 0_usize, (y_max as i32 + self.gen_y) as usize, y_max)
            } else if self.gen_x > 0 && self.gen_y > 0 {
                (0_usize, self.gen_x as usize, 0_usize, self.gen_y as usize)
            } else if self.gen_x > 0 && self.gen_y < 0 {
                (0_usize, self.gen_x as usize, (y_max as i32 + self.gen_y) as usize, y_max)
            } else if self.gen_x < 0 && self.gen_y > 0 {
                ((x_max as i32 + self.gen_x) as usize, x_max, 0_usize, self.gen_y as usize)
            } else if self.gen_x < 0 && self.gen_y < 0 {
                ((x_max as i32 + self.gen_x) as usize, x_max, (y_max as i32 + self.gen_y) as usize, y_max)
            } else {(0_usize, 0_usize, 0_usize, 0_usize)}
        };

        match (sx, ex, sy, ey) {
            (0, _, 0, 0) => self.fill_map(t_cells.clone(), sx, ex, sy, y_max),
            (_, _, 0, 0) => self.fill_map(t_cells.clone(), sx, ex, sy, y_max),
            (0, 0, 0, _) => self.fill_map(t_cells.clone(), sx, x_max, sy, ey),
            (0, 0, _, _) => self.fill_map(t_cells.clone(), sx, x_max, sy, ey),
            (0, _, 0, _) => {
                self.fill_map(t_cells.clone(), 0, x_max, 0, ey);
                self.fill_map(t_cells.clone(), 0, ex, 0, y_max);
            },
            (0, _, _, _) => {
                self.fill_map(t_cells.clone(), 0, x_max, sy, ey);
                self.fill_map(t_cells.clone(), 0, ex, 0, y_max);
            },
            (_, _, 0, _) => {
                self.fill_map(t_cells.clone(), sx, x_max, 0, ey);
                self.fill_map(t_cells.clone(), sx, ex, 0, y_max);
            },
            (_, _, _, _) => {
                self.fill_map(t_cells.clone(), 0, x_max, sy, ey);
                self.fill_map(t_cells.clone(), sx, ex, 0, y_max);
            }
        }


        self.gen_x = 0;
        self.gen_y = 0;
    }

    fn shift(&mut self, direction: &str) {
        let (dx, dy): (isize, isize) = match direction {
            "UP" => (0, 1),
            "DN" => (0, -1),
            "LF" => (1, 0),
            "RT" => (-1, 0),
            _ => panic!("Invalid direction"),
        };

        self.gen_x += dx as i32;
        self.gen_y += dy as i32;
        //put check and map_fill function

        let mut new_cells = vec![vec![Cell::Empty; self.cells[0].len()]; self.cells.len()];
        for (y, row) in self.cells.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                let new_x = (x as isize + dx) as usize;
                let new_y = (y as isize + dy) as usize;
                if new_x < self.cells[0].len() && new_y < self.cells.len() {
                    new_cells[new_y][new_x] = cell;
                }
            }
        }
        self.cells = new_cells;

        let mut new_tunnels = HashMap::new();
        for ((kx, ky), (vx, vy)) in &self.tunnels {
            let a = (*kx as isize + dx);
            let b = (*ky as isize + dy);
            let c = (*vx as isize + dx);
            let d = (*vy as isize + dy);
            // let logout = format!("\na: {0} b: {1} c: {2} d: {3}", a, b, c, d);

            // log::info!("{}", logout);
            if a >= 0 && a < self.cells[0].len().try_into().unwrap() && b >= 0 && b < self.cells.len().try_into().unwrap()
                && c >= 0 && c < self.cells[0].len().try_into().unwrap() && d >= 0 && d < self.cells.len().try_into().unwrap() {
                    new_tunnels.insert((a as usize, b as usize), (c as usize, d as usize));
                } else {}
            // new_tunnels.insert(((*kx as isize + dx) as usize, (*ky as isize + dy) as usize), ((*vx as isize + dx) as usize, (*vy as isize + dy) as usize));
        }
        self.tunnels = new_tunnels;
        if self.gen_x.abs() >= 50 || self.gen_y.abs() >= 50 {
            // log::info!("Gen Shift");
            self.map_fill();
        }
    }

    fn center_player(&mut self, x: usize, y: usize) {

        let dx = (self.cells[0].len() / 2) as isize - x as isize;
        let dy = (self.cells.len() / 2) as isize - y as isize;
        // self.px = (self.cells[0].len() / 2);
        // self.py = (self.cells.len() / 2);
        self.gen_x += dx as i32;
        self.gen_y += dy as i32;

        // Create a new 2D vector with the same dimensions as self.cells
        let mut new_cells = vec![vec![Cell::Empty; self.cells[0].len()]; self.cells.len()];

        // Iterate over each cell in self.cells
        for (y, row) in self.cells.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                // Calculate the new position of the cell
                let new_x = (x as isize + dx) as usize;
                let new_y = (y as isize + dy) as usize;

                //If the new position is within the bounds of the map, move the cell to the new position
                if new_x < self.cells[0].len() && new_y < self.cells.len() {
                    new_cells[new_y][new_x] = cell;
                } else {}
            }
        }

        // Replace self.cells with new_cells
        self.cells = new_cells;

        // log::info!("\n--------------------\n");

        let mut new_tunnels = HashMap::new();
        for ((kx, ky), (vx, vy)) in &self.tunnels {
            let a = (*kx as isize + dx);
            let b = (*ky as isize + dy);
            let c = (*vx as isize + dx);
            let d = (*vy as isize + dy);
            // let logout = format!("\na: {0} b: {1} c: {2} d: {3}", a, b, c, d);

            //Go over this!!!!!!
            // log::info!("{}", logout);
            if a >= 0 && a < self.cells[0].len().try_into().unwrap() && b >= 0 && b < self.cells.len().try_into().unwrap()
                && c >= 0 && c < self.cells[0].len().try_into().unwrap() && d >= 0 && d < self.cells.len().try_into().unwrap() {
                    new_tunnels.insert((a as usize, b as usize), (c as usize, d as usize));
                } else {
                    if a < 0 || a >= self.cells[0].len().try_into().unwrap() || b < 0 || b >= self.cells.len().try_into().unwrap() {
                        if c >= 0 && c < self.cells[0].len().try_into().unwrap() && d >= 0 && d < self.cells.len().try_into().unwrap() {
                            self.cells[d as usize][c as usize] = Cell::Empty;
                        }
                        // self.cells[c as usize][d as usize] = Cell::Empty;
                    } else if  c < 0 || c >= self.cells[0].len().try_into().unwrap() || d < 0 || d >= self.cells.len().try_into().unwrap() {
                        if a >= 0 && a < self.cells[0].len().try_into().unwrap() && b >= 0 && b < self.cells.len().try_into().unwrap() {
                            self.cells[b as usize][a as usize] = Cell::Empty;
                        } else {}
                        // self.cells[a as usize][b as usize] = Cell::Empty;
                    } else {}
                }

            // new_tunnels.insert(((*kx as isize + dx) as usize, (*ky as isize + dy) as usize), ((*vx as isize + dx) as usize, (*vy as isize + dy) as usize));
        }
        self.tunnels = new_tunnels;
        if self.gen_x.abs() >= 50 || self.gen_y.abs() >= 50 {
            self.map_fill();
        }
    }


}

struct Player {
    x: usize,
    y: usize,
    health: usize,
}

impl Player {
    fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            health: 100,
        }
    }

    fn move_up(&mut self) {
        self.y -= 1;
    }

    fn move_down(&mut self) {
        self.y += 1;
    }

    fn move_left(&mut self) {
        self.x -= 1;
    }

    fn move_right(&mut self) {
        self.x += 1;
    }
}

// Define the GameState struct
struct GameState {
    terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
    map: Map,
    player: Player,
    // changes: HashMap <(usize, usize), char>
}

impl GameState {
    fn new() -> Self {
        let stdout = stdout();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal.clear().unwrap();
        terminal.hide_cursor().unwrap();
        let map = Map::new();
        let x = map.px.clone();
        let y = map.py.clone();
        let player = Player::new(x, y);
        Self {
            terminal,
            map,
            player,
        }
    }

    fn collision(&mut self, dir: &str) -> bool {
        match dir {
            "UP" => {
                self.map.cells[self.player.y - 1][self.player.x] == Cell::Wall
            },
            "DN" => {
                self.map.cells[self.player.y + 1][self.player.x] == Cell::Wall
            },
            "LF" => {
                self.map.cells[self.player.y][self.player.x - 1] == Cell::Wall
            },
            "RT" => {
                self.map.cells[self.player.y][self.player.x + 1] == Cell::Wall
            },
            _ => false
        }
    }

    fn update(&mut self) -> bool {
        if let Event::Key(event) = read().unwrap() {
            match event.code {
                KeyCode::Up => {
                    if self.collision("UP") {} else {
                        if self.player.y - 1 <= self.map.viewport_y + (self.map.viewport_height/7) {
                            self.map.cells[self.player.y][self.player.x] = Cell::Empty;
                            self.map.cells[self.player.y-1][self.player.x] = Cell::Player;
                            self.map.shift("UP");
                        } else {
                            self.map.cells[self.player.y][self.player.x] = Cell::Empty;
                            self.player.y -= 1;
                            self.map.cells[self.player.y][self.player.x] = Cell::Player;
                        }


                    }
                },
                KeyCode::Down => {
                    if self.collision("DN") {} else {
                        if self.player.y + 1 >= (self.map.viewport_height + self.map.viewport_y) - (self.map.viewport_height/7) {
                            self.map.cells[self.player.y][self.player.x] = Cell::Empty;
                            self.map.cells[self.player.y+1][self.player.x] = Cell::Player;
                            self.map.shift("DN");
                        } else {
                            self.map.cells[self.player.y][self.player.x] = Cell::Empty;
                            self.player.y += 1;
                            self.map.cells[self.player.y][self.player.x] = Cell::Player;
                        }
                    }
                },
                KeyCode::Left => {
                    if self.collision("LF") {} else {
                        if self.player.x - 1 <= self.map.viewport_x + (self.map.viewport_width/7) {
                            self.map.cells[self.player.y][self.player.x] = Cell::Empty;
                            self.map.cells[self.player.y][self.player.x-1] = Cell::Player;
                            self.map.shift("LF");
                        } else {
                            self.map.cells[self.player.y][self.player.x] = Cell::Empty;
                            self.player.x -= 1;
                            self.map.cells[self.player.y][self.player.x] = Cell::Player;
                        }
                    }
                },
                KeyCode::Right => {
                    if self.collision("RT") {} else {
                        if self.player.x + 1 >= (self.map.viewport_width + self.map.viewport_x) - (self.map.viewport_width/7) {
                            self.map.cells[self.player.y][self.player.x] = Cell::Empty;
                            self.map.cells[self.player.y][self.player.x+1] = Cell::Player;
                            self.map.shift("RT");
                        } else {
                            self.map.cells[self.player.y][self.player.x] = Cell::Empty;
                            self.player.x += 1;
                            self.map.cells[self.player.y][self.player.x] = Cell::Player;
                        }


                    }
                },
                KeyCode::Char('q') => return false,
                _ => {}
            }

            if let Some(&(tx, ty)) = self.map.tunnels.get(&(self.player.x, self.player.y)) {
                self.map.cells[self.player.y][self.player.x] = Cell::Tunnel;
                let neighs: Vec<(i32, i32)> = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
                let mut xx = 0;
                let mut yy = 0;
                for &(y, x) in &neighs {
                    let new_x = tx as i32 + x;
                    let new_y = ty as i32 + y;
                    if new_x >= 0 && new_y >= 0 && new_x <= self.map.cells[0].len().try_into().unwrap()
                        && new_y <= self.map.cells.len().try_into().unwrap()
                        && self.map.cells[new_y as usize][new_x as usize] == Cell::Empty {
                        xx = new_x as usize;
                        yy = new_y as usize;
                        break;
                    }
                }
                self.map.cells[yy][xx] = Cell::Player;
                self.map.center_player(xx, yy);
                self.player.x = (self.map.cells[0].len() / 2);
                self.player.y = (self.map.cells.len() / 2);
                // self.player.x = xx;
                // self.player.y = yy;
            }

            true
        } else {
            false
        }
    }

    fn draw(&mut self) {

        self.terminal.draw(|f| {
            let size = f.size();
            let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(10),
                    Constraint::Percentage(80),
                    Constraint::Percentage(10)
                ].as_ref()
            )
            .split(f.size());

            let game_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(70),
                    Constraint::Percentage(30)
                ].as_ref()
            )
            .split(chunks[1]);

            let block = Block::default()
            .title("Game")
            .borders(Borders::ALL);
            f.render_widget(block.clone(), game_chunks[0]);

            let block_area = game_chunks[0];
            f.render_widget(block.clone(), block_area);

            let inner_area = block_area.inner(&Margin::default());

            let in_h = inner_area.height as usize;
            let in_w = inner_area.width as usize;


            if in_h != self.map.viewport_height && in_w != self.map.viewport_width {

                self.map.set_viewport(in_h, in_w);
            }

            let mut text = Vec::new();

            let start_row = self.map.viewport_y;
            let end_row = (self.map.viewport_y + in_h).min(self.map.cells.len());
            let start_col = self.map.viewport_x;
            let end_col = (self.map.viewport_x + in_w).min(self.map.cells[0].len());

            for (i, row) in self.map.cells[start_row..end_row].iter().enumerate() {
                let mut line = String::new();
                for (j, &cell) in row[start_col..end_col].iter().enumerate() {
                    let symbol = match cell {
                        Cell::Empty => ' ',
                        Cell::Wall => '#',
                        Cell::Tunnel => '@',
                        Cell::Player => '&',

                    };
                    line.push(symbol);
                }
                text.push(Span::from(line));
            }
            let texts: Text<'_> = text.into_iter().collect();
            let paragraph = Paragraph::new(texts).block(Block::default().borders(Borders::NONE));
            f.render_widget(paragraph, inner_area);

            // Here you can add the content for the right side window
            // For example, let's add another block
            let info_block = Block::default()
            .title("Information")
            .borders(Borders::ALL);
            // f.render_widget(info_block, game_chunks[1]);

            let text = vec![
                Span::styled("px: ", Style::default().fg(Color::White)).into(),
                Span::styled(self.player.x.to_string(), Style::default().fg(Color::Yellow)).into(),
                Span::styled("py: ", Style::default().fg(Color::White)).into(),
                Span::styled(self.player.y.to_string(), Style::default().fg(Color::Yellow)).into(),
                Span::styled("vx: ", Style::default().fg(Color::White)).into(),
                Span::styled(self.map.viewport_x.to_string(), Style::default().fg(Color::Yellow)).into(),
                Span::styled("vy: ", Style::default().fg(Color::White)).into(),
                Span::styled(self.map.viewport_y.to_string(), Style::default().fg(Color::Yellow)).into(),
                Span::styled("vw: ", Style::default().fg(Color::White)).into(),
                Span::styled(self.map.viewport_width.to_string(), Style::default().fg(Color::Yellow)).into(),
                Span::styled("vh: ", Style::default().fg(Color::White)).into(),
                Span::styled(self.map.viewport_height.to_string(), Style::default().fg(Color::Yellow)).into(),
                //scrollpoint
                Span::styled("su: ", Style::default().fg(Color::White)).into(),
                Span::styled((self.map.viewport_y + (self.map.viewport_height/7)).to_string(), Style::default().fg(Color::Yellow)).into(),
                Span::styled("sd: ", Style::default().fg(Color::White)).into(),
                Span::styled(((self.map.viewport_height + self.map.viewport_y) - (self.map.viewport_height/7)).to_string(), Style::default().fg(Color::Yellow)).into(),
                Span::styled("sl: ", Style::default().fg(Color::White)).into(),
                Span::styled((self.map.viewport_x + (self.map.viewport_width/7)).to_string(), Style::default().fg(Color::Yellow)).into(),
                Span::styled("sr: ", Style::default().fg(Color::White)).into(),
                Span::styled(((self.map.viewport_width + self.map.viewport_x) - (self.map.viewport_width/7)).to_string(), Style::default().fg(Color::Yellow)).into(),
                //asdfasdfasdf
                Span::styled("gx: ", Style::default().fg(Color::White)).into(),
                Span::styled((self.map.gen_x).to_string(), Style::default().fg(Color::Yellow)).into(),
                Span::styled("gy: ", Style::default().fg(Color::White)).into(),
                Span::styled((self.map.gen_y).to_string(), Style::default().fg(Color::Yellow)).into(),
            ];

            let paragraph = Paragraph::new(text)
                .block(info_block)
                .wrap(Wrap { trim: true });

            f.render_widget(paragraph, game_chunks[1]);
        }).unwrap();
    }


}

fn main() {
    init().unwrap();
    let mut game_state = GameState::new();

    terminal::enable_raw_mode().unwrap();

    loop {
        game_state.draw();
        if game_state.update() == false {
            break;
        }
    }

    terminal::disable_raw_mode().unwrap();
}
