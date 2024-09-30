//map

// mod enums;
use crate::enums::{Cells};

use std::collections::HashMap;
// use rand::Rng;
// use std::io::stdout;
use rand::{Rng};
use rand::prelude::SliceRandom;
use std::vec::Vec;
// use std::collections::HashMap;


// Define the Map struct
#[derive(Clone, Debug)]
pub struct Map {
    pub cells: Vec<Vec<Cells>>,
    pub px: usize,
    pub py: usize,
    pub tunnels: HashMap<(usize, usize), (usize, usize)>,
    pub dead_tunnels: HashMap<(usize, usize), (usize, usize)>,
    pub viewport_x: usize,
    pub viewport_y: usize,
    pub viewport_width: usize,
    pub viewport_height: usize,
    pub gen_x: i32,
    pub gen_y: i32,
}

pub const MAP_W: usize = 600;
pub const MAP_H: usize = 400;

fn format_hashmap(hashmap: HashMap<(usize, usize), (usize, usize)>) -> String {
    let mut result = String::new();
    for ((a, b), (c, d)) in hashmap {
        result.push_str(&format!("{} {} | {} {}\n", a, b, c, d));
    }
    result
}

impl Map {

    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut cells = vec![vec![Cells::Wall; MAP_W]; MAP_H];
        let mut small_cells = vec![vec![Cells::Wall; 150]; 100];

        fn carve_passages(start_x: usize, start_y: usize, cells: &mut Vec<Vec<Cells>>, rng: &mut rand::rngs::ThreadRng) {
            let mut stack = vec![(start_x, start_y)];
            let directions: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

            while let Some((x, y)) = stack.pop() {
                let mut directions = directions.to_vec();
                directions.shuffle(rng);

                for &(dx, dy) in &directions {
                    let nx = x.wrapping_add(dx as usize);
                    let ny = y.wrapping_add(dy as usize);
                    let nnx = nx.wrapping_add(dx as usize);
                    let nny = ny.wrapping_add(dy as usize);

                    if nnx < 150 && nny < 100 && cells[nny][nnx] == Cells::Wall && cells[ny][nx] == Cells::Wall {
                        cells[y][x] = Cells::Empty;
                        cells[ny][nx] = Cells::Empty;
                        cells[nny][nnx] = Cells::Empty;
                        stack.push((nnx, nny));
                    }
                }
            }
        }

        // Start carving from the center of the map
        let start_x = 150 / 2;
        let start_y = 100 / 2;
        carve_passages(start_x, start_y, &mut small_cells, &mut rng);

        for y in 0..100 {
            for x in 0..150 {
                let cell = small_cells[y][x];
                for dy in 0..4 {
                    for dx in 0..4 {
                        cells[y * 4 + dy][x * 4 + dx] = cell;
                    }
                }
            }
        }

        for _ in 0..(MAP_H*MAP_W)/400 {
            let x = rng.gen_range(0..(MAP_W - 12) / 4) * 4;
            let y = rng.gen_range(0..(MAP_H - 12) / 4) * 4;
            for i in x..x+12 {
                for j in y..y+12 {
                    cells[j][i] = Cells::Empty;
                }
            }
        }

        for _ in 0..(MAP_H*MAP_W)/10 {
            let x1 = rng.gen_range(0..MAP_W);
            let y1 = rng.gen_range(0..MAP_H);
            if cells[y1][x1] == Cells::Empty {
                let temp = {
                    let fl_type = rng.gen_range(0..5);
                    match fl_type {
                        0 => Cells::Dirt1,
                        1 => Cells::Dirt2,
                        2 => Cells::Grass1,
                        3 => Cells::Grass2,
                        4 => Cells::Rock,
                        _ => {
                            log::info!("Wrong rand num");
                            Cells::Empty
                        }
                    }
                };
                cells[y1][x1] = temp;
            }
        }

        let mut px = 0;
        let mut py = 0;
        let x_centre = MAP_W/2;
        let y_centre = MAP_H/2;
        loop {
            px = rng.gen_range(x_centre-20..x_centre+20);
            py = rng.gen_range(y_centre-10..y_centre+10);
            if cells[py][px] == Cells::Empty {
               // cells[py][px] = Cells::Player;
               break;
            }
        }

        let tunnels = HashMap::new();
        let dead_tunnels = HashMap::new();

        let viewport_x = px.clone() - 30;
        let viewport_y = py.clone() - 30;
        let viewport_width = 0;
        let viewport_height = 0;

        Self { cells, px, py, tunnels, dead_tunnels, viewport_x, viewport_y, viewport_width, viewport_height, gen_x: 0, gen_y: 0 }
    }

//     pub get_adjacet(&mut self) {
//
//     }

    pub fn set_viewport(&mut self, h: usize, w:usize) {
        self.viewport_height = h;
        self.viewport_width = w;
        self.viewport_y = (self.cells.len()/2) - (h/2);
        self.viewport_x = (self.cells[0].len()/2) - (w/2);
    }

    pub fn get_viewport(&mut self) -> (usize, usize, usize, usize) {
        (self.viewport_x, self.viewport_y, self.viewport_width, self.viewport_height)
    }

    fn map_to_string(&self, cells: &Vec<Vec<Cells>>) -> String {
        let mut map_string = String::new();
        map_string.push('\n');
        for row in cells {
            for cell in row {
                let symbol = match cell {
                    Cells::Empty => ' ',
                    Cells::Dirt1 => '\'',
                    Cells::Dirt2 => '.',
                    Cells::Grass1 => ',',
                    Cells::Grass2 => '\'',
                    Cells::Rock => '*',
                    Cells::Wall => '▒',
                    Cells::Tunnel => '@',
                    // Cells::Player => '&',
                    // Cells::Enemy => '!',
                };
                map_string.push_str(&symbol.to_string());
            }
            map_string.push('\n');
        }
        map_string
    }

    fn fill_map(&mut self, cells: Vec<Vec<Cells>>, sx: usize, ex: usize, sy: usize, ey: usize) {
        // log::info!("sx: {}, ex: {}, sy: {}, ey: {}", sx, ex, sy, ey);
        //  let mapout = self.map_to_string(&self.cells);
        // log::info!("prefill\n{}", mapout);
        for j in sy..=ey {
            for i in sx..=ex {
                self.cells[j][i] = cells[j][i];
            }
        }
        // let mapout = self.map_to_string(&self.cells);
        // log::info!("post\n{}", mapout);
    }

    fn map_fill(&mut self) {
        let mut rng = rand::thread_rng();
        let mut t_cells = vec![vec![Cells::Wall; MAP_W]; MAP_H];
        let mut small_cells = vec![vec![Cells::Wall; 150]; 100];

        fn carve_passages(start_x: usize, start_y: usize, cells: &mut Vec<Vec<Cells>>, rng: &mut rand::rngs::ThreadRng) {
            let mut stack = vec![(start_x, start_y)];
            let directions: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

            while let Some((x, y)) = stack.pop() {
                let mut directions = directions.to_vec();
                directions.shuffle(rng);

                for &(dx, dy) in &directions {
                    let nx = x.wrapping_add(dx as usize);
                    let ny = y.wrapping_add(dy as usize);
                    let nnx = nx.wrapping_add(dx as usize);
                    let nny = ny.wrapping_add(dy as usize);

                    if nnx < 150 && nny < 100 && cells[nny][nnx] == Cells::Wall && cells[ny][nx] == Cells::Wall {
                        cells[y][x] = Cells::Empty;
                        cells[ny][nx] = Cells::Empty;
                        cells[nny][nnx] = Cells::Empty;
                        stack.push((nnx, nny));
                    }
                }
            }
        }

        // Start carving from the center of the map
        let start_x = 150 / 2;
        let start_y = 100 / 2;
        carve_passages(start_x, start_y, &mut small_cells, &mut rng);

        for y in 0..100 {
            for x in 0..150 {
                let cell = small_cells[y][x];
                for dy in 0..4 {
                    for dx in 0..4 {
                        t_cells[y * 4 + dy][x * 4 + dx] = cell;
                    }
                }
            }
        }

        for _ in 0..(MAP_H*MAP_W)/300 {
            let x = rng.gen_range(0..(MAP_W - 12) / 4) * 4;
            let y = rng.gen_range(0..(MAP_H - 12) / 4) * 4;
            for i in x..x+12 {
                for j in y..y+12 {
                    t_cells[j][i] = Cells::Empty;
                }
            }
        }

        for _ in 0..(MAP_H*MAP_W)/10 {
            let x1 = rng.gen_range(0..MAP_W);
            let y1 = rng.gen_range(0..MAP_H);
            if t_cells[y1][x1] == Cells::Empty {
                let temp = {
                    let fl_type = rng.gen_range(0..5);
                    match fl_type {
                        0 => Cells::Dirt1,
                        1 => Cells::Dirt2,
                        2 => Cells::Grass1,
                        3 => Cells::Grass2,
                        4 => Cells::Rock,
                        _ => {
                            log::info!("Wrong rand num");
                            Cells::Empty
                        }
                    }
                };
                t_cells[y1][x1] = temp;
            }
        }

        let y_max = self.cells.len() - 1;
        let x_max = self.cells[0].len() - 1;

        let (sx, ex, sy, ey) = {
            if self.gen_x > 0 && self.gen_y == 0 {
                // log::info!("gen_x: {}", self.gen_x);
                (0_usize, (self.gen_x-1) as usize, 0_usize, 0_usize)
            } else if self.gen_x < 0 && self.gen_y == 0 {
                // log::info!("gen_x: {}", self.gen_x);
                ((x_max as i32 + self.gen_x) as usize, x_max, 0_usize, 0_usize)
            } else if self.gen_y > 0 && self.gen_x == 0 {
                // log::info!("gen_x: {}", self.gen_y);
                (0_usize, 0_usize, 0_usize, (self.gen_y-1) as usize)
            } else if self.gen_y < 0 && self.gen_x == 0 {
                // log::info!("gen_x: {}", self.gen_y);
                (0_usize, 0_usize, (y_max as i32 + self.gen_y) as usize, y_max)
            } else if self.gen_x > 0 && self.gen_y > 0 { //-------------
                (0_usize, (self.gen_x-1) as usize, 0_usize, (self.gen_y-1) as usize)
            } else if self.gen_x > 0 && self.gen_y < 0 {
                (0_usize, (self.gen_x-1) as usize, (y_max as i32 + self.gen_y) as usize, y_max)
            } else if self.gen_x < 0 && self.gen_y > 0 {
                ((x_max as i32 + self.gen_x) as usize, x_max, 0_usize, (self.gen_y-1) as usize)
            } else if self.gen_x < 0 && self.gen_y < 0 {
                ((x_max as i32 + self.gen_x) as usize, x_max, (y_max as i32 + self.gen_y) as usize, y_max)
            } else {(0_usize, 0_usize, 0_usize, 0_usize)}
        };
        // log::info!("sx: {}, ex: {}, sy: {}, ey: {}", sx, ex, sy, ey);
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
                self.fill_map(t_cells.clone(), 0, x_max, 0, ey);
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

    pub fn shift(&mut self, direction: &str) {
        let (dx, dy): (isize, isize) = match direction {
            "UP" => (0, 1),
            "DN" => (0, -1),
            "LF" => (1, 0),
            "RT" => (-1, 0),
            _ => panic!("Invalid direction"),
        };

        if dx > 0 && self.gen_x < 0 || dx < 0 && self.gen_x > 0
            || dy > 0 && self.gen_y < 0 || dy < 0 && self.gen_y > 0 {
                self.map_fill();
        }
        //---
        self.gen_x += dx as i32;
        self.gen_y += dy as i32;
        let mut new_cells = vec![vec![Cells::Dirt1; self.cells[0].len()]; self.cells.len()];
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
        //---

        // ---tobe: tunnel_translate
        // let mut new_tunnels = HashMap::new();
        // for ((kx, ky), (vx, vy)) in &self.tunnels {
        //     let a = (*kx as isize + dx) as usize;
        //     let b = (*ky as isize + dy) as usize;
        //     let c = (*vx as isize + dx) as usize;
        //     let d = (*vy as isize + dy) as usize;
        //     if a < self.cells[0].len() && b < self.cells.len()
        //         && c < self.cells[0].len() && d < self.cells.len() {
        //             new_tunnels.insert((a, b), (c, d));
        //     } else {
        //         self.dead_tunnels.insert((a, b), (c, d));
        //     }
        // }
        //---

        if self.gen_x.abs() >= ((MAP_W/5)).try_into().unwrap() || self.gen_y.abs() >= ((MAP_H/4)).try_into().unwrap() {
            self.map_fill();
            // let temp_tunnels = self.replace_dead_tunnels(new_tunnels.clone());
            // self.tunnels = temp_tunnels;
        } else {
            // self.tunnels = new_tunnels;
        }
    }

    fn replace_dead_tunnels(&mut self, mut new_tunnels: HashMap<(usize, usize), (usize, usize)>) -> HashMap<(usize, usize), (usize, usize)> {
        let mut rng = rand::thread_rng();
        let start_row = self.viewport_y;
        let end_row = (self.viewport_y + self.viewport_height).min(self.cells.len());
        let start_col = self.viewport_x;
        let end_col = (self.viewport_x + self.viewport_width).min(self.cells[0].len());
        // log::info!("prenlen: {}", new_tunnels.len());
        // log::info!("predlen: {}", self.dead_tunnels.len());
        // let mut ab_count = 0;
        // let mut abt_count = 0;
        // let mut cd_count = 0;
        // let mut cdt_count = 0;
        // let mut tt_count = 0;
        // let mut et_count = 0;
        let mut placed_tunnels: Vec<(usize, usize)> = Vec::new();
        let x_st = MAP_W/6;
        let y_st = MAP_H/6;
        for ((a, b), (c, d)) in &self.dead_tunnels {
            if *a < self.cells[0].len() && *b < self.cells.len() {
                let (x, y) = loop {
                    let x = rng.gen_range(x_st..MAP_W-x_st);
                    let y = rng.gen_range(y_st..MAP_H-y_st);
                    if x < start_col || x > end_col && y < start_row || y > end_row
                        && self.cells[y][x] == Cells::Empty && !new_tunnels.contains_key(&(x, y)) {
                        break (x, y);
                    }
                };
                new_tunnels.insert((*a, *b), (x, y));
                new_tunnels.insert((x, y), (*a, *b));
                self.cells[*b as usize][*a as usize] = Cells::Tunnel;
                self.cells[y as usize][x as usize] = Cells::Tunnel;
                // ab_count += 1;
                // abt_count += 1;
            } else if *c < self.cells[0].len() && *d < self.cells.len() {
                // cdt_count += 1;
            } else {
                // et_count += 1;
                // log::info!("else tn: {}, {}, {}, {}", a, b, c, d);
                if !placed_tunnels.contains(&(*c, *d)) {
                    let (x1, y1, x2, y2) = loop {
                        let x1 = rng.gen_range(x_st..MAP_W-x_st);
                        let x2 = rng.gen_range(x_st..MAP_W-x_st);
                        let y1 = rng.gen_range(y_st..MAP_H-y_st);
                        let y2 = rng.gen_range(y_st..MAP_H-y_st);
                        if x1 < start_col || x1 > end_col && y1 < start_row || y1 > end_row
                            && x2 < start_col || x2 > end_col && y2 < start_row || y2 > end_row
                            && self.cells[y1][x1] == Cells::Empty && !new_tunnels.contains_key(&(x1, y1))
                            && self.cells[y2][x2] == Cells::Empty && !new_tunnels.contains_key(&(x2, y2)) {
                            break (x1, y1, x2, y2);
                        }
                    };
                    new_tunnels.insert((x1, y1), (x2, y2));
                    new_tunnels.insert((x2, y2), (x1, y1));
                    self.cells[y1 as usize][x1 as usize] = Cells::Tunnel;
                    self.cells[y2 as usize][x2 as usize] = Cells::Tunnel;
                    placed_tunnels.push((*a, *b));
                    // cd_count += 1;
                }

            }
            // tt_count += 1
        }
        // log::info!("ab_count: {}, cd_count: {} abt_count: {}, cdt_count: {} et_count: {} tt_count: {} ntlen: {}", ab_count, cd_count, abt_count, cdt_count, et_count, tt_count, new_tunnels.len());
        self.dead_tunnels.clear();
        new_tunnels
    }



    pub fn center_player(&mut self, x: usize, y: usize) {
        let dx = (self.cells[0].len() / 2) as isize - x as isize;
        let dy = (self.cells.len() / 2) as isize - y as isize;
        if self.gen_x != 0 || self.gen_y != 0 {
            self.map_fill();
        }
        self.gen_x += dx as i32;
        self.gen_y += dy as i32;
        let mut new_cells = vec![vec![Cells::Grass2; self.cells[0].len()]; self.cells.len()];
        for (y, row) in self.cells.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                let new_x = (x as isize + dx) as usize;
                let new_y = (y as isize + dy) as usize;

                if new_x < self.cells[0].len() && new_y < self.cells.len() {
                    new_cells[new_y][new_x] = cell;
                } else {}
            }
        }
        self.cells = new_cells;
        //---
        self.map_fill();


        // log::info!("\n--------------------\n");

        //---tobe: tunnel_translate
        // let mut new_tunnels = HashMap::new();
        // for ((kx, ky), (vx, vy)) in &self.tunnels {
        //     let a = (*kx as isize + dx) as usize;
        //     let b = (*ky as isize + dy) as usize;
        //     let c = (*vx as isize + dx) as usize;
        //     let d = (*vy as isize + dy) as usize;
        //     if a < self.cells[0].len() && b < self.cells.len()
        //         && c < self.cells[0].len() && d < self.cells.len() {
        //             new_tunnels.insert((a, b), (c, d));
        //     } else {
        //         self.dead_tunnels.insert((a, b), (c, d));
        //     }
        // }
        // //---
        //
        // // log::info!("ntunlen: {}", new_tunnels.len());
        // // log::info!("dtunlen: {}", self.dead_tunnels.len());
        // let tunnel_string = format_hashmap(self.tunnels.clone());
        // log::info!("tunlist:\n{}", tunnel_string);
        //
        // let temp_new_tunnels = self.replace_dead_tunnels(new_tunnels.clone());
        // // log::info!("ttunlen: {}", temp_new_tunnels.len());
        // let tunnel_string = format_hashmap(self.tunnels.clone());
        // log::info!("tunlistpost:\n{}", tunnel_string);
        // self.tunnels = temp_new_tunnels;
    }


}
