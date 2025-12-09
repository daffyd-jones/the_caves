//map

// mod enums;
use crate::enums::Cells;

use std::collections::HashMap;
// use rand::Rng;
// use std::io::stdout;
use rand::Rng;
use rand::{prelude::SliceRandom, thread_rng};
use std::vec::Vec;
// use std::collections::HashMap;
use serde::{Deserialize, Serialize};

// Define the Map struct
#[derive(Clone, Debug)]
pub struct Map {
    pub cells: Vec<Vec<Cells>>,
    pub fill_cells: HashMap<u8, Vec<Vec<Cells>>>,
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

        for _ in 0..120 {
            let x = rng.gen_range(0..135);
            let y = rng.gen_range(0..85);

            let wall_cell = [Cells::Wall, Cells::Wall2, Cells::Wall3, Cells::Wall4]
                .choose(&mut rng)
                .unwrap_or(&Cells::Wall);

            for j in 0..8 {
                for i in 0..8 {
                    small_cells[y + j][x + i] = *wall_cell;
                }
            }
        }

        fn carve_passages(
            start_x: usize,
            start_y: usize,
            cells: &mut Vec<Vec<Cells>>,
            rng: &mut rand::rngs::ThreadRng,
        ) {
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

                    if nnx < 150
                        && nny < 100
                        && (cells[nny][nnx] == Cells::Wall
                            || cells[nny][nnx] == Cells::Wall2
                            || cells[nny][nnx] == Cells::Wall3
                            || cells[nny][nnx] == Cells::Wall4)
                        && (cells[ny][nx] == Cells::Wall
                            || cells[ny][nx] == Cells::Wall2
                            || cells[ny][nx] == Cells::Wall3
                            || cells[ny][nx] == Cells::Wall4)
                    {
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
        let ul_cells = [
            Cells::ULCorner1,
            Cells::ULCorner2,
            Cells::ULCorner3,
            Cells::ULCorner4,
            Cells::ULCorner5,
        ];
        let ur_cells = [
            Cells::URCorner1,
            Cells::URCorner2,
            Cells::URCorner3,
            Cells::URCorner4,
            Cells::URCorner5,
        ];
        let dl_cells = [
            Cells::DLCorner1,
            Cells::DLCorner2,
            Cells::DLCorner3,
            Cells::DLCorner4,
            Cells::DLCorner5,
        ];
        let dr_cells = [
            Cells::DRCorner1,
            Cells::DRCorner2,
            Cells::DRCorner3,
            Cells::DRCorner4,
            Cells::DRCorner5,
        ];
        for y in 0..100 {
            for x in 0..150 {
                let cell = small_cells[y][x];
                let neighbors = if y > 0 && y < 99 && x > 0 && x < 148 {
                    (
                        small_cells[y - 1][x],
                        small_cells[y + 1][x],
                        small_cells[y][x - 1],
                        small_cells[y][x + 1],
                    )
                } else {
                    (Cells::Null, Cells::Null, Cells::Null, Cells::Null)
                };
                for dy in 0..4 {
                    for dx in 0..4 {
                        if cell == Cells::Wall
                            || cell == Cells::Wall2
                            || cell == Cells::Wall3
                            || cell == Cells::Wall4
                        {
                            match neighbors {
                                (Cells::Empty, _, Cells::Empty, _) => {
                                    if dx == 0 && dy == 0 {
                                        cells[y * 4 + dy][x * 4 + dx] =
                                            *ul_cells.choose(&mut rng).unwrap_or(&Cells::Empty);
                                        // cells[y * 4 + dy][x * 4 + dx] = Cells::Empty;
                                        continue;
                                    }
                                }
                                (Cells::Empty, _, _, Cells::Empty) => {
                                    if dx == 3 && dy == 0 {
                                        cells[y * 4 + dy][x * 4 + dx] =
                                            *ur_cells.choose(&mut rng).unwrap_or(&Cells::Empty);
                                        // cells[y * 4 + dy][x * 4 + dx] = Cells::Empty;
                                        continue;
                                    }
                                }
                                (_, Cells::Empty, Cells::Empty, _) => {
                                    if dx == 0 && dy == 3 {
                                        cells[y * 4 + dy][x * 4 + dx] =
                                            *dl_cells.choose(&mut rng).unwrap_or(&Cells::Empty);
                                        // cells[y * 4 + dy][x * 4 + dx] = Cells::Empty;
                                        continue;
                                    }
                                }
                                (_, Cells::Empty, _, Cells::Empty) => {
                                    if dx == 3 && dy == 3 {
                                        cells[y * 4 + dy][x * 4 + dx] =
                                            *dr_cells.choose(&mut rng).unwrap_or(&Cells::Empty);
                                        // cells[y * 4 + dy][x * 4 + dx] = Cells::Empty;
                                        continue;
                                    }
                                }
                                (u, d, l, r)
                                    if u != Cells::Empty
                                        && d != Cells::Empty
                                        && l != Cells::Empty
                                        && r != Cells::Empty =>
                                {
                                    let cell_ch = if rng.gen_range(0..15) == 0 {
                                        [Cells::Broken1, Cells::Broken3, Cells::Roots]
                                            .choose(&mut rng)
                                            .unwrap_or(&Cells::Broken4)
                                    } else {
                                        &cell
                                    };
                                    cells[y * 4 + dy][x * 4 + dx] = *cell_ch;
                                    continue;
                                    // ℰ
                                }
                                _ => {}
                            }
                        }
                        cells[y * 4 + dy][x * 4 + dx] = cell;
                    }
                }
            }
        }

        for _ in 0..(MAP_H * MAP_W) / 400 {
            let x = rng.gen_range(0..(MAP_W - 12) / 4) * 4;
            let y = rng.gen_range(0..(MAP_H - 12) / 4) * 4;
            for i in x..x + 12 {
                for j in y..y + 12 {
                    cells[j][i] = Cells::Empty;
                }
            }
        }

        for _ in 0..(MAP_H * MAP_W) / 8 {
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
        let x_centre = MAP_W / 2;
        let y_centre = MAP_H / 2;
        loop {
            px = rng.gen_range(x_centre - 20..x_centre + 20);
            py = rng.gen_range(y_centre - 10..y_centre + 10);
            if cells[py][px] == Cells::Empty {
                break;
            }
        }
        let mut fill_cells = HashMap::new();
        let mut v_flip = vec![vec![Cells::Wall; cells[0].len()]; cells.len()];
        let mut h_flip = vec![vec![Cells::Wall; cells[0].len()]; cells.len()];
        let mut d_flip = vec![vec![Cells::Wall; cells[0].len()]; cells.len()];

        for y in 0..cells.len() {
            for x in 0..cells[0].len() {
                v_flip[y][cells[0].len() - x - 1] = {
                    if ul_cells.contains(&cells[y][x]) {
                        *dl_cells.choose(&mut rng).unwrap_or(&Cells::Empty)
                    } else if ur_cells.contains(&cells[y][x]) {
                        *dr_cells.choose(&mut rng).unwrap_or(&Cells::Empty)
                    } else if dl_cells.contains(&cells[y][x]) {
                        *ul_cells.choose(&mut rng).unwrap_or(&Cells::Empty)
                    } else if dr_cells.contains(&cells[y][x]) {
                        *ur_cells.choose(&mut rng).unwrap_or(&Cells::Empty)
                    } else {
                        cells[y][x]
                    }
                };
                h_flip[cells.len() - y - 1][x] = {
                    if ul_cells.contains(&cells[y][x]) {
                        *ur_cells.choose(&mut rng).unwrap_or(&Cells::Empty)
                    } else if ur_cells.contains(&cells[y][x]) {
                        *ul_cells.choose(&mut rng).unwrap_or(&Cells::Empty)
                    } else if dl_cells.contains(&cells[y][x]) {
                        *dr_cells.choose(&mut rng).unwrap_or(&Cells::Empty)
                    } else if dr_cells.contains(&cells[y][x]) {
                        *dl_cells.choose(&mut rng).unwrap_or(&Cells::Empty)
                    } else {
                        cells[y][x]
                    }
                };
                d_flip[cells.len() - y - 1][cells[0].len() - x - 1] = {
                    if ul_cells.contains(&cells[y][x]) {
                        *dr_cells.choose(&mut rng).unwrap_or(&Cells::Empty)
                    } else if ur_cells.contains(&cells[y][x]) {
                        *dl_cells.choose(&mut rng).unwrap_or(&Cells::Empty)
                    } else if dl_cells.contains(&cells[y][x]) {
                        *ur_cells.choose(&mut rng).unwrap_or(&Cells::Empty)
                    } else if dr_cells.contains(&cells[y][x]) {
                        *ul_cells.choose(&mut rng).unwrap_or(&Cells::Empty)
                    } else {
                        cells[y][x]
                    }
                };
                // h_flip[cells.len() - y - 1][x] = cells[y][x];
                // d_flip[cells.len() - y - 1][cells[0].len() - x - 1] = cells[y][x];
            }
        }

        fill_cells.insert(0, v_flip);
        fill_cells.insert(1, h_flip);
        fill_cells.insert(2, d_flip);

        let tunnels = HashMap::new();
        let dead_tunnels = HashMap::new();

        let viewport_x = px - 30;
        let viewport_y = py - 30;
        let viewport_width = 0;
        let viewport_height = 0;

        Self {
            cells,
            fill_cells,
            px,
            py,
            tunnels,
            dead_tunnels,
            viewport_x,
            viewport_y,
            viewport_width,
            viewport_height,
            gen_x: 0,
            gen_y: 0,
        }
    }

    pub fn set_viewport(&mut self, h: usize, w: usize) {
        self.viewport_height = h;
        self.viewport_width = w;
        self.viewport_y = (self.cells.len() / 2) - (h / 2);
        self.viewport_x = (self.cells[0].len() / 2) - (w / 2);
    }

    pub fn get_viewport(&mut self) -> (usize, usize, usize, usize) {
        (
            self.viewport_x,
            self.viewport_y,
            self.viewport_width,
            self.viewport_height,
        )
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
                    _ => ' ', // Cells::Player => '&',
                              // Cells::Enemy => '!',
                };
                map_string.push_str(&symbol.to_string());
            }
            map_string.push('\n');
        }
        map_string
    }

    fn fill_map(&mut self, cells: Vec<Vec<Cells>>, sx: usize, ex: usize, sy: usize, ey: usize) {
        for j in sy..=ey {
            for i in sx..=ex {
                self.cells[j][i] = cells[j][i];
            }
        }
    }

    fn map_fill(&mut self) {
        let mut rng = rand::thread_rng();
        // let mut t_cells = vec![vec![Cells::Wall; MAP_W]; MAP_H];
        // let mut small_cells = vec![vec![Cells::Wall; 150]; 100];

        // for _ in 0..120 {
        //     let x = rng.gen_range(0..135);
        //     let y = rng.gen_range(0..85);

        //     let wall_cell = [Cells::Wall, Cells::Wall2, Cells::Wall3, Cells::Wall4]
        //         .choose(&mut rng)
        //         .unwrap_or(&Cells::Wall);

        //     for j in 0..8 {
        //         for i in 0..8 {
        //             small_cells[y + j][x + i] = *wall_cell;
        //         }
        //     }
        // }

        // fn carve_passages(
        //     start_x: usize,
        //     start_y: usize,
        //     cells: &mut Vec<Vec<Cells>>,
        //     rng: &mut rand::rngs::ThreadRng,
        // ) {
        //     let mut stack = vec![(start_x, start_y)];
        //     let directions: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

        //     while let Some((x, y)) = stack.pop() {
        //         let mut directions = directions.to_vec();
        //         directions.shuffle(rng);

        //         for &(dx, dy) in &directions {
        //             let nx = x.wrapping_add(dx as usize);
        //             let ny = y.wrapping_add(dy as usize);
        //             let nnx = nx.wrapping_add(dx as usize);
        //             let nny = ny.wrapping_add(dy as usize);

        //             if nnx < 150
        //                 && nny < 100
        //                 && (cells[nny][nnx] == Cells::Wall
        //                     || cells[nny][nnx] == Cells::Wall2
        //                     || cells[nny][nnx] == Cells::Wall3
        //                     || cells[nny][nnx] == Cells::Wall4)
        //                 && (cells[ny][nx] == Cells::Wall
        //                     || cells[ny][nx] == Cells::Wall2
        //                     || cells[ny][nx] == Cells::Wall3
        //                     || cells[ny][nx] == Cells::Wall4)
        //             {
        //                 cells[y][x] = Cells::Empty;
        //                 cells[ny][nx] = Cells::Empty;
        //                 cells[nny][nnx] = Cells::Empty;
        //                 stack.push((nnx, nny));
        //             }
        //         }
        //     }
        // }

        // // Start carving from the center of the map
        // let start_x = 150 / 2;
        // let start_y = 100 / 2;
        // carve_passages(start_x, start_y, &mut small_cells, &mut rng);

        // for y in 0..100 {
        //     for x in 0..150 {
        //         let cell = small_cells[y][x];
        //         let neighbors = if y > 0 && y < 99 && x > 0 && x < 148 {
        //             (
        //                 small_cells[y - 1][x],
        //                 small_cells[y + 1][x],
        //                 small_cells[y][x - 1],
        //                 small_cells[y][x + 1],
        //             )
        //         } else {
        //             (Cells::Null, Cells::Null, Cells::Null, Cells::Null)
        //         };
        //         for dy in 0..4 {
        //             for dx in 0..4 {
        //                 if cell == Cells::Wall
        //                     || cell == Cells::Wall2
        //                     || cell == Cells::Wall3
        //                     || cell == Cells::Wall4
        //                 {
        //                     match neighbors {
        //                         (Cells::Empty, _, Cells::Empty, _) => {
        //                             if dx == 0 && dy == 0 {
        //                                 t_cells[y * 4 + dy][x * 4 + dx] = Cells::Empty;
        //                                 continue;
        //                             }
        //                         }
        //                         (Cells::Empty, _, _, Cells::Empty) => {
        //                             if dx == 3 && dy == 0 {
        //                                 t_cells[y * 4 + dy][x * 4 + dx] = Cells::Empty;
        //                                 continue;
        //                             }
        //                         }
        //                         (_, Cells::Empty, Cells::Empty, _) => {
        //                             if dx == 0 && dy == 3 {
        //                                 t_cells[y * 4 + dy][x * 4 + dx] = Cells::Empty;
        //                                 continue;
        //                             }
        //                         }
        //                         (_, Cells::Empty, _, Cells::Empty) => {
        //                             if dx == 3 && dy == 3 {
        //                                 t_cells[y * 4 + dy][x * 4 + dx] = Cells::Empty;
        //                                 continue;
        //                             }
        //                         }
        //                         _ => {}
        //                     }
        //                 }
        //                 t_cells[y * 4 + dy][x * 4 + dx] = cell;
        //             }
        //         }
        //     }
        // }

        // for _ in 0..(MAP_H * MAP_W) / 300 {
        //     let x = rng.gen_range(0..(MAP_W - 12) / 4) * 4;
        //     let y = rng.gen_range(0..(MAP_H - 12) / 4) * 4;
        //     for i in x..x + 12 {
        //         for j in y..y + 12 {
        //             t_cells[j][i] = Cells::Empty;
        //         }
        //     }
        // }

        // for _ in 0..(MAP_H * MAP_W) / 10 {
        //     let x1 = rng.gen_range(0..MAP_W);
        //     let y1 = rng.gen_range(0..MAP_H);
        //     if t_cells[y1][x1] == Cells::Empty {
        //         let temp = {
        //             let fl_type = rng.gen_range(0..5);
        //             match fl_type {
        //                 0 => Cells::Dirt1,
        //                 1 => Cells::Dirt2,
        //                 2 => Cells::Grass1,
        //                 3 => Cells::Grass2,
        //                 4 => Cells::Rock,
        //                 _ => {
        //                     log::info!("Wrong rand num");
        //                     Cells::Empty
        //                 }
        //             }
        //         };
        //         t_cells[y1][x1] = temp;
        //     }
        // }

        let map_k = rng.gen_range(0..3);
        let mut t_cells = self.fill_cells.get(&map_k).unwrap().clone();

        let y_max = self.cells.len() - 1;
        let x_max = self.cells[0].len() - 1;
        let gen_x = match self.gen_x {
            x if x < 0 => self.gen_x + (-4 - self.gen_y % 4),
            x if x > 0 => self.gen_x + (4 - self.gen_x % 4),
            _ => self.gen_x,
        };
        let gen_y = match self.gen_y {
            x if x < 0 => self.gen_y + (-4 - self.gen_y % 4),
            x if x > 0 => self.gen_y + (4 - self.gen_y % 4),
            _ => self.gen_y,
        };

        let (sx, ex, sy, ey) = {
            if gen_x > 0 && gen_y == 0 {
                // log::ingen_x: {}", gen_x);
                (0_usize, (gen_x - 1) as usize, 0_usize, 0_usize)
            } else if gen_x < 0 && gen_y == 0 {
                // log::ingen_x: {}", gen_x);
                ((x_max as i32 + gen_x) as usize, x_max, 0_usize, 0_usize)
            } else if gen_y > 0 && gen_x == 0 {
                // log::ingen_x: {}", gen_y);
                (0_usize, 0_usize, 0_usize, (gen_y - 1) as usize)
            } else if gen_y < 0 && gen_x == 0 {
                // log::ingen_x: {}", gen_y);
                (0_usize, 0_usize, (y_max as i32 + gen_y) as usize, y_max)
            } else if gen_x > 0 && gen_y > 0 {
                //-------------
                (0_usize, (gen_x - 1) as usize, 0_usize, (gen_y - 1) as usize)
            } else if gen_x > 0 && gen_y < 0 {
                (
                    0_usize,
                    (gen_x - 1) as usize,
                    (y_max as i32 + gen_y) as usize,
                    y_max,
                )
            } else if gen_x < 0 && gen_y > 0 {
                (
                    (x_max as i32 + gen_x) as usize,
                    x_max,
                    0_usize,
                    (gen_y - 1) as usize,
                )
            } else if gen_x < 0 && gen_y < 0 {
                (
                    (x_max as i32 + gen_x) as usize,
                    x_max,
                    (y_max as i32 + gen_y) as usize,
                    y_max,
                )
            } else {
                (0_usize, 0_usize, 0_usize, 0_usize)
            }
        };
        // log::info!("sx: {}, ex: {}, sy: {}, ey: {}", sx, ex, sy, ey);
        match (sx, ex, sy, ey) {
            (0, _, 0, 0) => self.fill_map(t_cells.clone(), sx + 4, ex + 4, sy, y_max),
            (_, _, 0, 0) => self.fill_map(t_cells.clone(), sx - 4, ex - 4, sy, y_max),
            (0, 0, 0, _) => self.fill_map(t_cells.clone(), sx, x_max, sy + 4, ey + 4),
            (0, 0, _, _) => self.fill_map(t_cells.clone(), sx, x_max, sy - 4, ey - 4),
            (0, _, 0, _) => {
                self.fill_map(t_cells.clone(), 0, x_max, 0 + 4, ey + 4);
                self.fill_map(t_cells.clone(), 0 + 4, ex + 4, 0, y_max);
            }
            (0, _, _, _) => {
                self.fill_map(t_cells.clone(), 0, x_max, sy - 4, ey - 4);
                self.fill_map(t_cells.clone(), 0 + 4, ex + 4, 0, y_max);
            }
            (_, _, 0, _) => {
                self.fill_map(t_cells.clone(), 0, x_max, 0 + 4, ey + 4);
                self.fill_map(t_cells.clone(), sx - 4, ex - 4, 0, y_max);
            }
            (_, _, _, _) => {
                self.fill_map(t_cells.clone(), 0, x_max, sy - 4, ey - 4);
                self.fill_map(t_cells.clone(), sx - 4, ex - 4, 0, y_max);
            }
        }

        self.gen_x = 0;
        self.gen_y = 0;

        match (gen_x, gen_y) {
            (a, b) if a < 0 && b == 0 => {
                for i in t_cells.iter_mut() {
                    i.rotate_right(gen_x.unsigned_abs() as usize);
                }
            }
            (a, b) if a > 0 && b == 0 => {
                for i in t_cells.iter_mut() {
                    i.rotate_left(gen_x.unsigned_abs() as usize);
                }
            }
            (a, b) if a == 0 && b < 0 => {
                t_cells.rotate_right(gen_y.unsigned_abs() as usize);
            }
            (a, b) if a == 0 && b > 0 => {
                t_cells.rotate_left(gen_y.unsigned_abs() as usize);
            }
            (a, b) if a < 0 && b < 0 => {
                t_cells.rotate_right(gen_y.unsigned_abs() as usize);
                for i in t_cells.iter_mut() {
                    i.rotate_right(gen_x.unsigned_abs() as usize);
                }
            }
            (a, b) if a > 0 && b < 0 => {
                t_cells.rotate_right(gen_y.unsigned_abs() as usize);
                for i in t_cells.iter_mut() {
                    i.rotate_left(gen_x.unsigned_abs() as usize);
                }
            }
            (a, b) if a < 0 && b > 0 => {
                t_cells.rotate_left(gen_y.unsigned_abs() as usize);
                for i in t_cells.iter_mut() {
                    i.rotate_right(gen_x.unsigned_abs() as usize);
                }
            }
            (a, b) if a > 0 && b > 0 => {
                t_cells.rotate_left(gen_y.unsigned_abs() as usize);
                for i in t_cells.iter_mut() {
                    i.rotate_left(gen_x.unsigned_abs() as usize);
                }
            }
            _ => {}
        }
        self.fill_cells.insert(map_k, t_cells.clone());
    }

    pub fn shift(&mut self, direction: &str) {
        let (dx, dy): (isize, isize) = match direction {
            "UP" => (0, 1),
            "DN" => (0, -1),
            "LF" => (1, 0),
            "RT" => (-1, 0),
            _ => panic!("Invalid direction"),
        };

        if dx > 0 && self.gen_x < 0
            || dx < 0 && self.gen_x > 0
            || dy > 0 && self.gen_y < 0
            || dy < 0 && self.gen_y > 0
        {
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

        if self.gen_x.abs() >= (MAP_W / 5).try_into().unwrap()
            || self.gen_y.abs() >= (MAP_H / 4).try_into().unwrap()
        {
            self.map_fill();
        } else {
            // self.tunnels = new_tunnels;
        }
    }

    fn replace_dead_tunnels(
        &mut self,
        mut new_tunnels: HashMap<(usize, usize), (usize, usize)>,
    ) -> HashMap<(usize, usize), (usize, usize)> {
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
        let x_st = MAP_W / 6;
        let y_st = MAP_H / 6;
        for ((a, b), (c, d)) in &self.dead_tunnels {
            if *a < self.cells[0].len() && *b < self.cells.len() {
                let (x, y) = loop {
                    let x = rng.gen_range(x_st..MAP_W - x_st);
                    let y = rng.gen_range(y_st..MAP_H - y_st);
                    if x < start_col
                        || x > end_col && y < start_row
                        || y > end_row
                            && self.cells[y][x] == Cells::Empty
                            && !new_tunnels.contains_key(&(x, y))
                    {
                        break (x, y);
                    }
                };
                new_tunnels.insert((*a, *b), (x, y));
                new_tunnels.insert((x, y), (*a, *b));
                self.cells[*b][*a] = Cells::Tunnel;
                self.cells[y][x] = Cells::Tunnel;
                // ab_count += 1;
                // abt_count += 1;
            } else if *c < self.cells[0].len() && *d < self.cells.len() {
                // cdt_count += 1;
            } else {
                // et_count += 1;
                // log::info!("else tn: {}, {}, {}, {}", a, b, c, d);
                if !placed_tunnels.contains(&(*c, *d)) {
                    let (x1, y1, x2, y2) = loop {
                        let x1 = rng.gen_range(x_st..MAP_W - x_st);
                        let x2 = rng.gen_range(x_st..MAP_W - x_st);
                        let y1 = rng.gen_range(y_st..MAP_H - y_st);
                        let y2 = rng.gen_range(y_st..MAP_H - y_st);
                        if x1 < start_col
                            || x1 > end_col && y1 < start_row
                            || y1 > end_row && x2 < start_col
                            || x2 > end_col && y2 < start_row
                            || y2 > end_row
                                && self.cells[y1][x1] == Cells::Empty
                                && !new_tunnels.contains_key(&(x1, y1))
                                && self.cells[y2][x2] == Cells::Empty
                                && !new_tunnels.contains_key(&(x2, y2))
                        {
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
                } else {
                }
            }
        }
        self.cells = new_cells;
        //---
        self.map_fill();
    }
}
