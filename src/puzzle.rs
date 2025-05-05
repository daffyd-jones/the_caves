//puzzle.rs
//
use crate::enemy::Enemy;
use crate::enums::{Cells, NPCWrap, PuzzleType};
use crate::item::Item;
use rand::prelude::SliceRandom;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn make_maze_map() -> Vec<Vec<Cells>> {
    let (m_width, m_height) = (300, 202);
    let mut rng = rand::thread_rng();
    let mut cells = vec![vec![Cells::Wall; m_width]; m_height];
    let mut small_cells = vec![vec![Cells::Wall; 75]; 50];

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

                if nnx < 75
                    && nny < 50
                    && cells[nny][nnx] == Cells::Wall
                    && cells[ny][nx] == Cells::Wall
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
    let start_x = 75 / 2;
    let start_y = 50 / 2;
    carve_passages(start_x, start_y, &mut small_cells, &mut rng);

    small_cells[32][0] = Cells::Empty;
    small_cells[32][0] = Cells::Empty;
    small_cells[32][0] = Cells::Empty;
    small_cells[32][74] = Cells::Empty;
    small_cells[32][74] = Cells::Empty;
    small_cells[32][74] = Cells::Empty;
    small_cells[0][23] = Cells::Empty;
    small_cells[0][24] = Cells::Empty;
    small_cells[0][24] = Cells::Empty;
    small_cells[49][24] = Cells::Empty;
    small_cells[49][24] = Cells::Empty;
    small_cells[49][24] = Cells::Empty;

    for y in 0..50 {
        for x in 0..75 {
            let cell = small_cells[y][x];
            for dy in 0..4 {
                for dx in 0..4 {
                    cells[y * 4 + dy][x * 4 + dx] = cell;
                }
            }
        }
    }
    cells.clone()
}

fn make_teleport_map() -> Vec<Vec<Cells>> {
    let (m_width, m_height) = (300, 200);
    let mut rng = rand::thread_rng();
    let mut cells = vec![vec![Cells::Empty; m_width]; m_height];
    let (sec_w, sec_h) = ((m_width - 0) / 5, (m_height - 0) / 5);

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

                if nnx < 15
                    && nny < 10
                    && cells[nny][nnx] == Cells::Wall
                    && cells[ny][nx] == Cells::Wall
                {
                    cells[y][x] = Cells::Empty;
                    cells[ny][nx] = Cells::Empty;
                    cells[nny][nnx] = Cells::Empty;
                    stack.push((nnx, nny));
                }
            }
        }
    }

    let round_secs = [
        ((0, 0), (60, 0), (0, 40), (60, 40)),
        ((180, 0), (240, 0), (180, 40), (240, 40)),
        ((0, 120), (60, 120), (0, 160), (60, 160)),
        ((180, 120), (240, 120), (180, 160), (240, 160)),
        ((180, 80), (240, 80), (0, 80), (60, 80)),
        ((120, 0), (120, 120), (120, 40), (120, 160)),
    ];

    let sm_sec_w = sec_w / 4;
    let sm_sec_h = sec_h / 4;
    let start_x = sm_sec_w / 2;
    let start_y = sm_sec_h / 2;
    for i in 0..6 {
        let mut sec_cells = vec![vec![Cells::Wall; sm_sec_w]; sec_h / 4];
        carve_passages(start_x, start_y, &mut sec_cells, &mut rng);
        let mut sec_cells_vflip = vec![vec![Cells::Wall; sm_sec_w]; sm_sec_h];
        let mut sec_cells_hflip = vec![vec![Cells::Wall; sm_sec_w]; sm_sec_h];
        let mut sec_cells_dflip = vec![vec![Cells::Wall; sm_sec_w]; sm_sec_h];
        for y in 0..sec_cells.len() {
            for x in 0..sec_cells[0].len() {
                sec_cells_vflip[y][sec_cells[0].len() - x - 1] = sec_cells[y][x];
                sec_cells_hflip[sec_cells.len() - y - 1][x] = sec_cells[y][x];
                sec_cells_dflip[sec_cells.len() - y - 1][sec_cells[0].len() - x - 1] =
                    sec_cells[y][x];
            }
        }

        let (a, b, c, d) = round_secs[i];
        for y in 0..sm_sec_h {
            for x in 0..sm_sec_w {
                let cell = sec_cells[y][x];
                for dy in 0..4 {
                    for dx in 0..4 {
                        cells[y * 4 + dy + a.1][x * 4 + dx + a.0] = cell;
                    }
                }
            }
        }
        for y in 0..sm_sec_h {
            for x in 0..sm_sec_w {
                let cell = sec_cells_vflip[y][x];
                for dy in 0..4 {
                    for dx in 0..4 {
                        cells[y * 4 + dy + b.1][x * 4 + dx + b.0] = cell;
                    }
                }
            }
        }
        for y in 0..sm_sec_h {
            for x in 0..sm_sec_w {
                let cell = sec_cells_hflip[y][x];
                for dy in 0..4 {
                    for dx in 0..4 {
                        cells[y * 4 + dy + c.1][x * 4 + dx + c.0] = cell;
                    }
                }
            }
        }
        for y in 0..sm_sec_h {
            for x in 0..sm_sec_w {
                let cell = sec_cells_dflip[y][x];
                for dy in 0..4 {
                    for dx in 0..4 {
                        cells[y * 4 + dy + d.1][x * 4 + dx + d.0] = cell;
                    }
                }
            }
        }
    }

    for j in 0..m_height {
        for i in 0..m_width {
            if j % 40 == 0 || i % 60 == 0 {
                cells[j][i] = Cells::Wall;
            }
        }
    }

    for i in 0..4 {
        cells[m_height / 2][i] = Cells::Empty;
        cells[m_height / 2 - 1][i] = Cells::Empty;
        cells[m_height / 2 - 2][i] = Cells::Empty;
        cells[m_height / 2 + 1][i] = Cells::Empty;
        cells[m_height / 2 + 2][i] = Cells::Empty;

        cells[m_height / 2][m_width - 1 - i] = Cells::Empty;
        cells[m_height / 2 - 1][m_width - 1 - i] = Cells::Empty;
        cells[m_height / 2 - 2][m_width - 1 - i] = Cells::Empty;
        cells[m_height / 2 + 1][m_width - 1 - i] = Cells::Empty;
        cells[m_height / 2 + 2][m_width - 1 - i] = Cells::Empty;

        cells[i][m_width / 2] = Cells::Empty;
        cells[i][m_width / 2 - 1] = Cells::Empty;
        cells[i][m_width / 2 - 2] = Cells::Empty;
        cells[i][m_width / 2 + 1] = Cells::Empty;
        cells[i][m_width / 2 + 2] = Cells::Empty;

        cells[m_height - 1 - i][m_width / 2] = Cells::Empty;
        cells[m_height - 1 - i][m_width / 2 - 1] = Cells::Empty;
        cells[m_height - 1 - i][m_width / 2 - 2] = Cells::Empty;
        cells[m_height - 1 - i][m_width / 2 + 1] = Cells::Empty;
        cells[m_height - 1 - i][m_width / 2 + 2] = Cells::Empty;

        cells[120 + i][m_width / 2] = Cells::Empty;
        cells[120 + i][(m_width / 2) - 1] = Cells::Empty;
        cells[120 + i][(m_width / 2) - 2] = Cells::Empty;
        cells[120 + i][(m_width / 2) + 1] = Cells::Empty;
        cells[120 + i][(m_width / 2) + 2] = Cells::Empty;
    }

    cells
}

fn place_portals(cells: Vec<Vec<Cells>>) -> HashMap<(usize, usize), (usize, usize)> {
    // let (m_width, m_height) = (300, 200);
    let mut rng = rand::thread_rng();
    // let (sec_w, sec_h) = ((m_width - 1) / 5, (m_height - 1) / 5);

    let round_secs = [
        (0, 0),
        (60, 0),
        (0, 40),
        (60, 40),
        (180, 0),
        (240, 0),
        (180, 40),
        (240, 40),
        (0, 120),
        (60, 120),
        (0, 160),
        (60, 160),
        (180, 120),
        (240, 120),
        (180, 160),
        (240, 160),
        (180, 80),
        (240, 80),
        (0, 80),
        (60, 80),
        (120, 0),
        (120, 120),
        (120, 40),
        (120, 160),
    ];

    //let mut rng = rand::thread_rng();
    let mut portal_ord1: Vec<u32> = (0..24).collect();

    let mut portal_ord2: Vec<u32> = (0..24).collect();
    let mut pairings = Vec::new();
    for i in 0..24 {
        let t = loop {
            let t1 = portal_ord1.choose(&mut rng).unwrap().clone();
            let t2 = portal_ord2.choose(&mut rng).unwrap().clone();
            if t1 == t2 {
                continue;
            }
            println!("{:?}-{:?}::{:?}", i, t1, t2);
            portal_ord1.retain(|&x| x != t1);
            portal_ord2.retain(|&x| x != t2);
            break (t1, t2);
        };
        pairings.push(t);
    }

    let mut portals = HashMap::new();
    for (a, b) in pairings {
        let apos = loop {
            let ax = rng.gen_range(round_secs[a as usize].0..(round_secs[a as usize].0 + 60));
            let ay = rng.gen_range(round_secs[a as usize].1..(round_secs[a as usize].1 + 40));
            if cells[ay][ax] == Cells::Empty {
                break (ax, ay);
            }
        };
        let bpos = loop {
            let bx = rng.gen_range(round_secs[b as usize].0..(round_secs[b as usize].0 + 60));
            let by = rng.gen_range(round_secs[b as usize].1..(round_secs[b as usize].1 + 40));
            if cells[by][bx] == Cells::Empty {
                break (bx, by);
            }
        };
        portals.insert(apos, bpos);
        portals.insert(bpos, apos);
    }

    portals
}

#[derive(Clone, Debug, PartialEq)]
//#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Puzzle {
    ptype: PuzzleType,
    pos: (i64, i64),
    map: Vec<Vec<Cells>>,
    portals: HashMap<(usize, usize), (usize, usize)>,
    items: HashMap<(usize, usize), Item>,
    enemies: HashMap<(usize, usize), Enemy>,
    npcs: HashMap<(usize, usize), NPCWrap>,
    prize: Item,
    prop_pass: bool,
}

impl Puzzle {
    pub fn new(
        ptype: PuzzleType,
        pos: (i64, i64),
        map: Vec<Vec<Cells>>,
        portals: HashMap<(usize, usize), (usize, usize)>,
        items: HashMap<(usize, usize), Item>,
        enemies: HashMap<(usize, usize), Enemy>,
        npcs: HashMap<(usize, usize), NPCWrap>,
        prize: Item,
        prop_pass: bool,
    ) -> Self {
        Self {
            ptype,
            pos,
            map,
            portals,
            items,
            enemies,
            npcs,
            prize,
            prop_pass,
        }
    }

    pub fn new_maze(pos: (i64, i64)) -> Self {
        let map = make_maze_map();
        let portals = HashMap::new();
        let items = HashMap::new();
        let enemies = HashMap::new();
        let npcs = HashMap::new();
        let prize = Item::new_health_potion(150, 100);
        Self {
            ptype: PuzzleType::Maze,
            pos,
            map,
            portals,
            items,
            enemies,
            npcs,
            prize,
            prop_pass: false,
        }
    }

    pub fn new_teleport(pos: (i64, i64)) -> Self {
        let map = make_teleport_map();
        let portals = place_portals(map.clone());
        let items = HashMap::new();
        let enemies = HashMap::new();
        let npcs = HashMap::new();
        let prize = Item::new_health_potion(150, 100);
        Self {
            ptype: PuzzleType::Maze,
            pos,
            map,
            portals,
            items,
            enemies,
            npcs,
            prize,
            prop_pass: false,
        }
    }

    pub fn get_pos(&mut self) -> (i64, i64) {
        self.pos.clone()
    }

    pub fn set_pos(&mut self, tpos: (i64, i64)) {
        self.pos = tpos;
    }

    pub fn get_map(&mut self) -> Vec<Vec<Cells>> {
        self.map.clone()
    }

    pub fn get_ptype(&mut self) -> PuzzleType {
        self.ptype.clone()
    }

    pub fn is_prop_pass(&mut self) -> bool {
        self.prop_pass.clone()
    }

    pub fn toggle_ppass(&mut self) {
        self.prop_pass = !self.prop_pass;
    }

    pub fn get_portals(&mut self) -> HashMap<(usize, usize), (usize, usize)> {
        self.portals.clone()
    }
}
