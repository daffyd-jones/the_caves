//puzzle.rs
//
use crate::enemy::Enemy;
use crate::enums::{Cells, EnvInter, NPCWrap, PuzzlePiece, PuzzleType};
use crate::item::Item;
use crate::parsing::{parse_map, tile_to_chars};
use log::info;
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

// ruin puzzle

const RUIN_ROOM_BLANK: &str = r#"
________________________________
________________________________
________________________________
________________________________
________________________________
________________________________
________________________________
________________________________
________________________________
________________________________
________________________________
________________________________
"#;

const PALETTE: &str = r#"
empty: ' . , ' * |
wall: ▒ |
other ▓ ░ ~ |
pipes:
═ ║ ╣ ╠ ╩ ╦ ╗ ╝ ╚ ╔ ╬
┐ └ ┴ ┬ ├ ─ ┼ ┘ ┌ ┤ │
ʬ ỻ Π Ħ ʭ ṑ ⑁                   
ж ѧ π
ᘉ ᘈ ᘍ ᘊ
≡ ° × ¤ ¸ ¨ · ■ ¦ ± ¡ ø Ø ©
"#;

const RUIN_ROOM_B: &str = r#"
▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒____▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
"#;

const RUIN_ROOM_T: &str = r#"
▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒____▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
"#;

const RUIN_ROOM_L: &str = r#"
▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
_________________________________________▒
_________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
"#;

const RUIN_ROOM_R: &str = r#"
▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒_________________________________________
▒_________________________________________
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
"#;

const RUIN_ROOM_TR: &str = r#"
▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒____▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒_________________________________________
▒_________________________________________
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
"#;

const RUIN_ROOM_TBR: &str = r#"
▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒____▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒_________________________________________
▒_________________________________________
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒____▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
"#;

const RUIN_ROOM_TL: &str = r#"
▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒____▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
_________________________________________▒
_________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
"#;

const RUIN_ROOM_LRT: &str = r#"
▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒____▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
__________________________________________
__________________________________________
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
"#;

const RUIN_ROOM_TBL: &str = r#"
▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒____▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
_________________________________________▒
_________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒____▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
"#;

const RUIN_ROOM_BR: &str = r#"
▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒_________________________________________
▒_________________________________________
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒____▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
"#;

const RUIN_ROOM_LRB: &str = r#"
▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
__________________________________________
__________________________________________
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒____▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
"#;

const RUIN_ROOM_BL: &str = r#"
▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
_________________________________________▒
_________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒____▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
"#;

const RUIN_ROOM_ALL: &str = r#"
▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒____▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
__________________________________________
__________________________________________
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒____▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
"#;

const RUIN_ROOM_TB: &str = r#"
▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒____▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒____▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
"#;

const RUIN_ROOM_LR: &str = r#"
▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
__________________________________________
__________________________________________
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒________________________________________▒
▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
"#;

const RUIN_ROOM_X: &str = r#"
▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
"#;

#[derive(Debug, Clone, PartialEq, Eq, Copy, PartialOrd, Ord)]
enum RuinRoom {
    B,
    T,
    L,
    R,
    TB,
    LR,
    TL,
    TR,
    BL,
    BR,
    TBL,
    TBR,
    LRT,
    LRB,
    All,
    Default,
    Null,
}

const RUIN_TOP_OPEN: [RuinRoom; 8] = [
    RuinRoom::B,
    RuinRoom::TB,
    RuinRoom::BL,
    RuinRoom::BR,
    RuinRoom::TBR,
    RuinRoom::TBL,
    RuinRoom::LRB,
    RuinRoom::All,
];

const RUIN_TOP_BLOCKED: [RuinRoom; 7] = [
    RuinRoom::T,
    RuinRoom::L,
    RuinRoom::R,
    RuinRoom::LR,
    RuinRoom::TL,
    RuinRoom::TR,
    RuinRoom::LRT,
];

const RUIN_LEFT_OPEN: [RuinRoom; 8] = [
    RuinRoom::R,
    RuinRoom::LR,
    RuinRoom::TR,
    RuinRoom::BR,
    RuinRoom::TBR,
    RuinRoom::LRT,
    RuinRoom::LRB,
    RuinRoom::All,
];

const RUIN_LEFT_BLOCKED: [RuinRoom; 7] = [
    RuinRoom::L,
    RuinRoom::T,
    RuinRoom::B,
    RuinRoom::TB,
    RuinRoom::TL,
    RuinRoom::BL,
    RuinRoom::TBL,
];

fn build_ruin() -> (String, Vec<Vec<RuinRoom>>) {
    let mut rng = rand::thread_rng();
    let mut cells = vec![vec![' '; 294]; 112];
    let mut temp = vec![vec![RuinRoom::Default; 7]; 7];
    // temp[0][0] = Field::OutCornerUL;
    for j in (0..temp.len()) {
        for i in (0..temp[0].len()) {
            let up = if j > 0 {
                temp[j - 1][i]
            } else {
                RuinRoom::Null
            };
            let left = if i > 0 {
                temp[j][i - 1]
            } else {
                RuinRoom::Null
            };
            temp[j][i] = {
                match (up, left) {
                    (RuinRoom::Null, RuinRoom::Null) => {
                        *[RuinRoom::BR].choose(&mut rng).unwrap_or(&RuinRoom::BR)
                    }
                    (up, left) if j == temp.len() - 1 && i == temp[0].len() - 1 => {
                        if RUIN_TOP_OPEN.contains(&up) && RUIN_LEFT_OPEN.contains(&left) {
                            RuinRoom::TL
                        } else if RUIN_LEFT_OPEN.contains(&left) {
                            RuinRoom::L
                        } else if RUIN_TOP_OPEN.contains(&up) {
                            RuinRoom::T
                        } else {
                            RuinRoom::Default
                        }
                    }
                    (RuinRoom::Null, left) => {
                        if i == temp[0].len() - 1 && RUIN_LEFT_OPEN.contains(&left) {
                            RuinRoom::BL
                        } else if i == temp[0].len() - 1 && RUIN_LEFT_BLOCKED.contains(&left) {
                            RuinRoom::B
                        } else if i == (temp[0].len() - 1) / 2 {
                            if RUIN_LEFT_OPEN.contains(&left) {
                                *[RuinRoom::TL, RuinRoom::TBL, RuinRoom::LRT, RuinRoom::All]
                                    .choose(&mut rng)
                                    .unwrap_or(&RuinRoom::TL)
                            } else {
                                *[RuinRoom::TR, RuinRoom::TBR, RuinRoom::TB]
                                    .choose(&mut rng)
                                    .unwrap_or(&RuinRoom::TR)
                            }
                        } else {
                            if RUIN_LEFT_OPEN.contains(&left) {
                                *[
                                    RuinRoom::L,
                                    RuinRoom::BL,
                                    RuinRoom::LR,
                                    RuinRoom::BL,
                                    RuinRoom::LR,
                                    RuinRoom::LRB,
                                ]
                                .choose(&mut rng)
                                .unwrap_or(&RuinRoom::BL)
                            } else {
                                RuinRoom::BR
                            }
                        }
                    }
                    (up, RuinRoom::Null) => {
                        if j == temp.len() - 1 && RUIN_TOP_OPEN.contains(&up) {
                            RuinRoom::TR
                        } else if j == temp.len() - 1 && RUIN_TOP_BLOCKED.contains(&up) {
                            RuinRoom::R
                        } else if j == (temp.len() - 1) / 2 {
                            if RUIN_TOP_OPEN.contains(&up) {
                                *[RuinRoom::TL, RuinRoom::TBL, RuinRoom::LRT, RuinRoom::All]
                                    .choose(&mut rng)
                                    .unwrap_or(&RuinRoom::TBR)
                            } else {
                                *[RuinRoom::BL, RuinRoom::LRB, RuinRoom::LR]
                                    .choose(&mut rng)
                                    .unwrap_or(&RuinRoom::BL)
                            }
                        } else {
                            if RUIN_TOP_OPEN.contains(&up) {
                                *[
                                    RuinRoom::T,
                                    RuinRoom::TR,
                                    RuinRoom::TB,
                                    RuinRoom::TR,
                                    RuinRoom::TB,
                                    RuinRoom::TBR,
                                ]
                                .choose(&mut rng)
                                .unwrap_or(&RuinRoom::TR)
                            } else {
                                RuinRoom::BR
                            }
                        }
                    }
                    (up, left) if i == temp[0].len() - 1 => {
                        if j == (temp.len() - 1) / 2 {
                            if RUIN_TOP_OPEN.contains(&up) && RUIN_LEFT_OPEN.contains(&left) {
                                RuinRoom::All
                            } else if RUIN_TOP_OPEN.contains(&up)
                                && RUIN_LEFT_BLOCKED.contains(&left)
                            {
                                *[RuinRoom::TR, RuinRoom::TBR]
                                    .choose(&mut rng)
                                    .unwrap_or(&RuinRoom::TBR)
                                // RuinRoom::TR
                            } else if RUIN_TOP_BLOCKED.contains(&up)
                                && RUIN_LEFT_OPEN.contains(&left)
                            {
                                *[RuinRoom::LR, RuinRoom::LRB]
                                    .choose(&mut rng)
                                    .unwrap_or(&RuinRoom::TBR)
                                // RuinRoom::LR
                            } else {
                                RuinRoom::BR
                            }
                        } else {
                            if RUIN_TOP_OPEN.contains(&up) && RUIN_LEFT_OPEN.contains(&left) {
                                *[RuinRoom::TL, RuinRoom::TBL]
                                    .choose(&mut rng)
                                    .unwrap_or(&RuinRoom::TBL)
                            } else if RUIN_TOP_OPEN.contains(&up)
                                && RUIN_LEFT_BLOCKED.contains(&left)
                            {
                                *[RuinRoom::T, RuinRoom::TB, RuinRoom::TB]
                                    .choose(&mut rng)
                                    .unwrap_or(&RuinRoom::TB)
                            } else if RUIN_TOP_BLOCKED.contains(&up)
                                && RUIN_LEFT_OPEN.contains(&left)
                            {
                                RuinRoom::BL
                            } else {
                                RuinRoom::B
                            }
                        }
                    }
                    (up, left) if j == temp.len() - 1 => {
                        if i == (temp[0].len() - 1) / 2 {
                            if RUIN_TOP_OPEN.contains(&up) && RUIN_LEFT_OPEN.contains(&left) {
                                RuinRoom::All
                            } else if RUIN_TOP_BLOCKED.contains(&up)
                                && RUIN_LEFT_OPEN.contains(&left)
                            {
                                *[RuinRoom::BL, RuinRoom::LRB]
                                    .choose(&mut rng)
                                    .unwrap_or(&RuinRoom::LRB)
                            } else if RUIN_TOP_OPEN.contains(&up)
                                && RUIN_LEFT_BLOCKED.contains(&left)
                            {
                                *[RuinRoom::TB, RuinRoom::TBR]
                                    .choose(&mut rng)
                                    .unwrap_or(&RuinRoom::TBR)
                            } else {
                                RuinRoom::All
                            }
                        } else {
                            if RUIN_TOP_OPEN.contains(&up) && RUIN_LEFT_OPEN.contains(&left) {
                                *[RuinRoom::TL, RuinRoom::LRT]
                                    .choose(&mut rng)
                                    .unwrap_or(&RuinRoom::LRT)
                            } else if RUIN_TOP_BLOCKED.contains(&up)
                                && RUIN_LEFT_OPEN.contains(&left)
                            {
                                *[RuinRoom::L, RuinRoom::LR, RuinRoom::LR]
                                    .choose(&mut rng)
                                    .unwrap_or(&RuinRoom::LR)
                            } else if RUIN_TOP_OPEN.contains(&up)
                                && RUIN_LEFT_BLOCKED.contains(&left)
                            {
                                RuinRoom::TR
                            } else {
                                RuinRoom::R
                            }
                        }
                    }
                    (up, left) if RUIN_TOP_OPEN.contains(&up) && RUIN_LEFT_OPEN.contains(&left) => {
                        *[RuinRoom::All, RuinRoom::LRT, RuinRoom::TBL]
                            .choose(&mut rng)
                            .unwrap_or(&RuinRoom::All)
                    }
                    (up, left)
                        if RUIN_TOP_OPEN.contains(&up) && RUIN_LEFT_BLOCKED.contains(&left) =>
                    {
                        *[
                            RuinRoom::T,
                            RuinRoom::TR,
                            RuinRoom::TB,
                            RuinRoom::TR,
                            RuinRoom::TB,
                            RuinRoom::TBR,
                        ]
                        .choose(&mut rng)
                        .unwrap_or(&RuinRoom::TR)
                    }
                    (up, left)
                        if RUIN_TOP_BLOCKED.contains(&up) && RUIN_LEFT_OPEN.contains(&left) =>
                    {
                        *[
                            RuinRoom::L,
                            RuinRoom::BL,
                            RuinRoom::LR,
                            RuinRoom::BL,
                            RuinRoom::LR,
                            RuinRoom::LRB,
                        ]
                        .choose(&mut rng)
                        .unwrap_or(&RuinRoom::BL)
                    }
                    (up, left)
                        if RUIN_TOP_BLOCKED.contains(&up) && RUIN_LEFT_BLOCKED.contains(&left) =>
                    {
                        *[RuinRoom::BR, RuinRoom::BR, RuinRoom::B, RuinRoom::R]
                            .choose(&mut rng)
                            .unwrap_or(&RuinRoom::BR)
                    }
                    _ => RuinRoom::Default,
                }
            }
        }
    }
    // println!("{:?}", temp);
    for j in 0..temp.len() {
        for i in 0..temp[0].len() {
            let patch = match temp[j][i] {
                // RuinRoom::Horz => STREAM_HORZ,
                // RuinRoom::Vert => STREAM_VERT,
                RuinRoom::T => RUIN_ROOM_T,
                RuinRoom::B => RUIN_ROOM_B,
                RuinRoom::L => RUIN_ROOM_L,
                RuinRoom::R => RUIN_ROOM_R,
                RuinRoom::TL => RUIN_ROOM_TL,
                RuinRoom::TR => RUIN_ROOM_TR,
                RuinRoom::BL => RUIN_ROOM_BL,
                RuinRoom::BR => RUIN_ROOM_BR,
                RuinRoom::TB => RUIN_ROOM_TB,
                RuinRoom::LR => RUIN_ROOM_LR,
                RuinRoom::TBL => RUIN_ROOM_TBL,
                RuinRoom::TBR => RUIN_ROOM_TBR,
                RuinRoom::LRT => RUIN_ROOM_LRT,
                RuinRoom::LRB => RUIN_ROOM_LRB,
                RuinRoom::All => RUIN_ROOM_ALL,
                _ => RUIN_ROOM_X,
            };
            let patch_chars = tile_to_chars(patch);
            for y in 0..16 {
                for x in 0..42 {
                    cells[j * 16 + y][i * 42 + x] = patch_chars[y][x];
                }
            }
        }
    }
    (
        std::iter::once("Null|Null|Null".to_string())
            .chain(cells.iter().map(|row| row.iter().collect::<String>()))
            .collect::<Vec<String>>()
            .join("\n"),
        temp,
    )
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDoorHV {
    Vert,
    Horiz,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PuzzleKey {
    pub set: u8,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PuzzleDoor {
    pub id: String,
    pub orient: PDoorHV,
    pub idxs: Vec<(usize, usize)>,
    pub set: u8,
}

// impl PuzzlePiece {
//     pub fn move_piece(&mut self, dir: &str) {
//         match self {
//             PuzzlePiece::PuzzleDoor(door) => {
//                 let idxs = door.idxs.clone();
//                 match dir {
//                     "UP" => {
//                         // let mut temp = Vec::new();
//                         for (i, (x, y)) in idxs.iter().enumerate() {
//                             door.idxs[i] = (*x, *y + 1);
//                         }
//                     }
//                     "DN" => {
//                         let mut temp = Vec::new();
//                         for (x, y) in idxs {
//                             temp.push((x, y - 1));
//                         }
//                     }
//                     "LF" => {
//                         let mut temp = Vec::new();
//                         for (x, y) in idxs {
//                             temp.push((x + 1, y));
//                         }
//                     }
//                     "RT" => {
//                         let mut temp = Vec::new();
//                         for (x, y) in idxs {
//                             temp.push((x - 1, y));
//                         }
//                     }
//                     _ => todo!(),
//                 };
//             }
//             PuzzlePiece::PuzzleKey(key) => {}
//         }
//     }
// }

fn map_room_doors(room: RuinRoom, set: u8, pos: (i8, i8)) -> Option<PuzzleDoor> {
    let (xoff, yoff) = (((3 + pos.0) as usize * 42), ((3 + pos.1) as usize * 16));
    match (room, pos) {
        (RuinRoom::B, pos) if pos.1 < 0 => Some(PuzzleDoor {
            id: format!(
                "{:?}-{:?}-{:?}-{:?}",
                (xoff + 19, yoff + 15),
                (xoff + 20, yoff + 15),
                (xoff + 21, yoff + 15),
                (xoff + 22, yoff + 15)
            ),
            orient: PDoorHV::Horiz,
            idxs: vec![
                (xoff + 19, yoff + 15),
                (xoff + 20, yoff + 15),
                (xoff + 21, yoff + 15),
                (xoff + 22, yoff + 15),
            ],
            set,
        }),
        (RuinRoom::T, pos) if pos.1 > 0 => Some(PuzzleDoor {
            id: format!(
                "{:?}-{:?}-{:?}-{:?}",
                (xoff + 19, yoff),
                (xoff + 20, yoff),
                (xoff + 21, yoff),
                (xoff + 22, yoff)
            ),
            orient: PDoorHV::Horiz,
            idxs: vec![
                (xoff + 19, yoff),
                (xoff + 20, yoff),
                (xoff + 21, yoff),
                (xoff + 22, yoff),
            ],
            set,
        }),
        (RuinRoom::L, pos) if pos.0 > 0 => Some(PuzzleDoor {
            id: format!("{:?}-{:?}", (xoff + 0, yoff + 7), (xoff + 0, yoff + 8)),
            orient: PDoorHV::Vert,
            idxs: vec![(xoff + 0, yoff + 7), (xoff + 0, yoff + 8)],
            set,
        }),
        (RuinRoom::R, pos) if pos.0 < 0 => Some(PuzzleDoor {
            id: format!("{:?}-{:?}", (xoff + 41, yoff + 7), (xoff + 41, yoff + 8)),
            orient: PDoorHV::Vert,
            idxs: vec![(xoff + 41, yoff + 7), (xoff + 41, yoff + 8)],
            set,
        }),
        (RuinRoom::TB, pos) if pos.1 > 0 => Some(PuzzleDoor {
            id: format!(
                "{:?}-{:?}-{:?}-{:?}",
                (xoff + 19, yoff),
                (xoff + 20, yoff),
                (xoff + 21, yoff),
                (xoff + 22, yoff)
            ),
            orient: PDoorHV::Horiz,
            idxs: vec![
                (xoff + 19, yoff),
                (xoff + 20, yoff),
                (xoff + 21, yoff),
                (xoff + 22, yoff),
            ],
            set,
        }),
        (RuinRoom::TB, pos) if pos.1 < 0 => Some(PuzzleDoor {
            id: format!(
                "{:?}-{:?}-{:?}-{:?}",
                (xoff + 19, yoff + 15),
                (xoff + 20, yoff + 15),
                (xoff + 21, yoff + 15),
                (xoff + 22, yoff + 15)
            ),
            orient: PDoorHV::Horiz,
            idxs: vec![
                (xoff + 19, yoff + 15),
                (xoff + 20, yoff + 15),
                (xoff + 21, yoff + 15),
                (xoff + 22, yoff + 15),
            ],
            set,
        }),
        (RuinRoom::LR, pos) if pos.0 > 0 => Some(PuzzleDoor {
            id: format!("{:?}-{:?}", (xoff + 0, yoff + 7), (xoff + 0, yoff + 8)),
            orient: PDoorHV::Vert,
            idxs: vec![(xoff + 0, yoff + 7), (xoff + 0, yoff + 8)],
            set,
        }),
        (RuinRoom::LR, pos) if pos.0 < 0 => Some(PuzzleDoor {
            id: format!("{:?}-{:?}", (xoff + 41, yoff + 7), (xoff + 41, yoff + 8)),
            orient: PDoorHV::Vert,
            idxs: vec![(xoff + 41, yoff + 7), (xoff + 41, yoff + 8)],
            set,
        }),
        (RuinRoom::TL, pos) if pos.1 > 0 => Some(PuzzleDoor {
            id: format!(
                "{:?}-{:?}-{:?}-{:?}",
                (xoff + 19, yoff),
                (xoff + 20, yoff),
                (xoff + 21, yoff),
                (xoff + 22, yoff)
            ),
            orient: PDoorHV::Horiz,
            idxs: vec![
                (xoff + 19, yoff),
                (xoff + 20, yoff),
                (xoff + 21, yoff),
                (xoff + 22, yoff),
            ],
            set,
        }),
        (RuinRoom::TL, pos) if pos.0 > 0 => Some(PuzzleDoor {
            id: format!("{:?}-{:?}", (xoff + 0, yoff + 7), (xoff + 0, yoff + 8)),
            orient: PDoorHV::Vert,
            idxs: vec![(xoff + 0, yoff + 7), (xoff + 0, yoff + 8)],
            set,
        }),
        (RuinRoom::TR, pos) if pos.1 > 0 => Some(PuzzleDoor {
            id: format!(
                "{:?}-{:?}-{:?}-{:?}",
                (xoff + 19, yoff),
                (xoff + 20, yoff),
                (xoff + 21, yoff),
                (xoff + 22, yoff)
            ),
            orient: PDoorHV::Horiz,
            idxs: vec![
                (xoff + 19, yoff),
                (xoff + 20, yoff),
                (xoff + 21, yoff),
                (xoff + 22, yoff),
            ],
            set,
        }),
        (RuinRoom::TR, pos) if pos.0 < 0 => Some(PuzzleDoor {
            id: format!("{:?}-{:?}", (xoff + 41, yoff + 7), (xoff + 41, yoff + 8)),
            orient: PDoorHV::Vert,
            idxs: vec![(xoff + 41, yoff + 7), (xoff + 41, yoff + 8)],
            set,
        }),
        (RuinRoom::BL, pos) if pos.1 < 0 => Some(PuzzleDoor {
            id: format!(
                "{:?}-{:?}-{:?}-{:?}",
                (xoff + 19, yoff + 15),
                (xoff + 20, yoff + 15),
                (xoff + 21, yoff + 15),
                (xoff + 22, yoff + 15)
            ),
            orient: PDoorHV::Horiz,
            idxs: vec![
                (xoff + 19, yoff + 15),
                (xoff + 20, yoff + 15),
                (xoff + 21, yoff + 15),
                (xoff + 22, yoff + 15),
            ],
            set,
        }),
        (RuinRoom::BL, pos) if pos.0 > 0 => Some(PuzzleDoor {
            id: format!("{:?}-{:?}", (xoff + 0, yoff + 7), (xoff + 0, yoff + 8)),
            orient: PDoorHV::Vert,
            idxs: vec![(xoff + 0, yoff + 7), (xoff + 0, yoff + 8)],
            set,
        }),
        (RuinRoom::BR, pos) if pos.1 < 0 => Some(PuzzleDoor {
            id: format!(
                "{:?}-{:?}-{:?}-{:?}",
                (xoff + 19, yoff + 15),
                (xoff + 20, yoff + 15),
                (xoff + 21, yoff + 15),
                (xoff + 22, yoff + 15)
            ),
            orient: PDoorHV::Horiz,
            idxs: vec![
                (xoff + 19, yoff + 15),
                (xoff + 20, yoff + 15),
                (xoff + 21, yoff + 15),
                (xoff + 22, yoff + 15),
            ],
            set,
        }),
        (RuinRoom::BR, pos) if pos.0 < 0 => Some(PuzzleDoor {
            id: format!("{:?}-{:?}", (xoff + 41, yoff + 7), (xoff + 41, yoff + 8)),
            orient: PDoorHV::Vert,
            idxs: vec![(xoff + 41, yoff + 7), (xoff + 41, yoff + 8)],
            set,
        }),
        (RuinRoom::TBL, pos) if pos.1 > 0 => Some(PuzzleDoor {
            id: format!(
                "{:?}-{:?}-{:?}-{:?}",
                (xoff + 19, yoff),
                (xoff + 20, yoff),
                (xoff + 21, yoff),
                (xoff + 22, yoff)
            ),
            orient: PDoorHV::Horiz,
            idxs: vec![
                (xoff + 19, yoff),
                (xoff + 20, yoff),
                (xoff + 21, yoff),
                (xoff + 22, yoff),
            ],
            set,
        }),
        (RuinRoom::TBL, pos) if pos.1 < 0 => Some(PuzzleDoor {
            id: format!(
                "{:?}-{:?}-{:?}-{:?}",
                (xoff + 19, yoff + 15),
                (xoff + 20, yoff + 15),
                (xoff + 21, yoff + 15),
                (xoff + 22, yoff + 15)
            ),
            orient: PDoorHV::Horiz,
            idxs: vec![
                (xoff + 19, yoff + 15),
                (xoff + 20, yoff + 15),
                (xoff + 21, yoff + 15),
                (xoff + 22, yoff + 15),
            ],
            set,
        }),
        (RuinRoom::TBL, pos) if pos.0 > 0 => Some(PuzzleDoor {
            id: format!("{:?}-{:?}", (xoff + 0, yoff + 7), (xoff + 0, yoff + 8)),
            orient: PDoorHV::Vert,
            idxs: vec![(xoff + 0, yoff + 7), (xoff + 0, yoff + 8)],
            set,
        }),
        (RuinRoom::TBR, pos) if pos.1 > 0 => Some(PuzzleDoor {
            id: format!(
                "{:?}-{:?}-{:?}-{:?}",
                (xoff + 19, yoff),
                (xoff + 20, yoff),
                (xoff + 21, yoff),
                (xoff + 22, yoff)
            ),
            orient: PDoorHV::Horiz,
            idxs: vec![
                (xoff + 19, yoff),
                (xoff + 20, yoff),
                (xoff + 21, yoff),
                (xoff + 22, yoff),
            ],
            set,
        }),
        (RuinRoom::TBR, pos) if pos.1 < 0 => Some(PuzzleDoor {
            id: format!(
                "{:?}-{:?}-{:?}-{:?}",
                (xoff + 19, yoff + 15),
                (xoff + 20, yoff + 15),
                (xoff + 21, yoff + 15),
                (xoff + 22, yoff + 15)
            ),
            orient: PDoorHV::Horiz,
            idxs: vec![
                (xoff + 19, yoff + 15),
                (xoff + 20, yoff + 15),
                (xoff + 21, yoff + 15),
                (xoff + 22, yoff + 15),
            ],
            set,
        }),
        (RuinRoom::TBR, pos) if pos.0 < 0 => Some(PuzzleDoor {
            id: format!("{:?}-{:?}", (xoff + 41, yoff + 7), (xoff + 41, yoff + 8)),
            orient: PDoorHV::Vert,
            idxs: vec![(xoff + 41, yoff + 7), (xoff + 41, yoff + 8)],
            set,
        }),
        (RuinRoom::LRT, pos) if pos.0 > 0 => Some(PuzzleDoor {
            id: format!("{:?}-{:?}", (xoff + 0, yoff + 7), (xoff + 0, yoff + 8)),
            orient: PDoorHV::Vert,
            idxs: vec![(xoff + 0, yoff + 7), (xoff + 0, yoff + 8)],
            set,
        }),
        (RuinRoom::LRT, pos) if pos.0 < 0 => Some(PuzzleDoor {
            id: format!("{:?}-{:?}", (xoff + 41, yoff + 7), (xoff + 41, yoff + 8)),
            orient: PDoorHV::Vert,
            idxs: vec![(xoff + 41, yoff + 7), (xoff + 41, yoff + 8)],
            set,
        }),
        (RuinRoom::LRT, pos) if pos.1 > 0 => Some(PuzzleDoor {
            id: format!(
                "{:?}-{:?}-{:?}-{:?}",
                (xoff + 19, yoff),
                (xoff + 20, yoff),
                (xoff + 21, yoff),
                (xoff + 22, yoff)
            ),
            orient: PDoorHV::Horiz,
            idxs: vec![
                (xoff + 19, yoff),
                (xoff + 20, yoff),
                (xoff + 21, yoff),
                (xoff + 22, yoff),
            ],
            set,
        }),
        (RuinRoom::LRB, pos) if pos.0 > 0 => Some(PuzzleDoor {
            id: format!("{:?}-{:?}", (xoff + 0, yoff + 7), (xoff + 0, yoff + 8)),
            orient: PDoorHV::Vert,
            idxs: vec![(xoff + 0, yoff + 7), (xoff + 0, yoff + 8)],
            set,
        }),
        (RuinRoom::LRB, pos) if pos.0 < 0 => Some(PuzzleDoor {
            id: format!("{:?}-{:?}", (xoff + 41, yoff + 7), (xoff + 41, yoff + 8)),
            orient: PDoorHV::Vert,
            idxs: vec![(xoff + 41, yoff + 7), (xoff + 41, yoff + 8)],
            set,
        }),
        (RuinRoom::LRB, pos) if pos.1 < 0 => Some(PuzzleDoor {
            id: format!(
                "{:?}-{:?}-{:?}-{:?}",
                (xoff + 19, yoff + 15),
                (xoff + 20, yoff + 15),
                (xoff + 21, yoff + 15),
                (xoff + 22, yoff + 15)
            ),
            orient: PDoorHV::Horiz,
            idxs: vec![
                (xoff + 19, yoff + 15),
                (xoff + 20, yoff + 15),
                (xoff + 21, yoff + 15),
                (xoff + 22, yoff + 15),
            ],
            set,
        }),
        (RuinRoom::All, pos) if pos.0 > 0 => Some(PuzzleDoor {
            id: format!("{:?}-{:?}", (xoff + 0, yoff + 7), (xoff + 0, yoff + 8)),
            orient: PDoorHV::Vert,
            idxs: vec![(xoff + 0, yoff + 7), (xoff + 0, yoff + 8)],
            set,
        }),
        (RuinRoom::All, pos) if pos.0 < 0 => Some(PuzzleDoor {
            id: format!("{:?}-{:?}", (xoff + 41, yoff + 7), (xoff + 41, yoff + 8)),
            orient: PDoorHV::Vert,
            idxs: vec![(xoff + 41, yoff + 7), (xoff + 41, yoff + 8)],
            set,
        }),
        (RuinRoom::All, pos) if pos.1 > 0 => Some(PuzzleDoor {
            id: format!(
                "{:?}-{:?}-{:?}-{:?}",
                (xoff + 19, yoff),
                (xoff + 20, yoff),
                (xoff + 21, yoff),
                (xoff + 22, yoff)
            ),
            orient: PDoorHV::Horiz,
            idxs: vec![
                (xoff + 19, yoff),
                (xoff + 20, yoff),
                (xoff + 21, yoff),
                (xoff + 22, yoff),
            ],
            set,
        }),
        (RuinRoom::All, pos) if pos.1 < 0 => Some(PuzzleDoor {
            id: format!(
                "{:?}-{:?}-{:?}-{:?}",
                (xoff + 19, yoff + 15),
                (xoff + 20, yoff + 15),
                (xoff + 21, yoff + 15),
                (xoff + 22, yoff + 15)
            ),
            orient: PDoorHV::Horiz,
            idxs: vec![
                (xoff + 19, yoff + 15),
                (xoff + 20, yoff + 15),
                (xoff + 21, yoff + 15),
                (xoff + 22, yoff + 15),
            ],
            set,
        }),
        _ => None,
        // _ => todo!(),
    }
}

// fn map_room_doors(room: RuinRoom, set: u8, pos: (i8, i8)) -> Option<PuzzleDoor> {
//     let (xoff, yoff) = (((3 + pos.0) as usize * 42), ((3 + pos.1) as usize * 16));
//     match (room, pos) {
//         (RuinRoom::B, pos) if pos.1 < 0 => Some(PuzzleDoor {
//             id: format!(
//                 "{:?}-{:?}-{:?}-{:?}",
//                 (xoff + 19, yoff + 15),
//                 (xoff + 20, yoff + 15),
//                 (xoff + 21, yoff + 15),
//                 (xoff + 22, yoff + 15)
//             ),
//             orient: PDoorHV::Horiz,
//             idxs: vec![
//                 (xoff + 19, yoff + 15),
//                 (xoff + 20, yoff + 15),
//                 (xoff + 21, yoff + 15),
//                 (xoff + 22, yoff + 15),
//             ],
//             set,
//         }),
//         (RuinRoom::T, pos) if pos.1 > 0 => Some(PuzzleDoor {
//             id: format!(
//                 "{:?}-{:?}-{:?}-{:?}",
//                 (xoff + 19, yoff),
//                 (xoff + 20, yoff),
//                 (xoff + 21, yoff),
//                 (xoff + 22, yoff)
//             ),
//             orient: PDoorHV::Horiz,
//             idxs: vec![
//                 (xoff + 19, yoff),
//                 (xoff + 20, yoff),
//                 (xoff + 21, yoff),
//                 (xoff + 22, yoff),
//             ],
//             set,
//         }),
//         (RuinRoom::L, pos) if pos.0 > 0 => Some(PuzzleDoor {
//             id: format!("{:?}-{:?}", (xoff + 0, yoff + 7), (xoff + 0, yoff + 8)),
//             orient: PDoorHV::Vert,
//             idxs: vec![(xoff + 0, yoff + 7), (xoff + 0, yoff + 8)],
//             set,
//         }),
//         (RuinRoom::R, pos) if pos.0 < 0 => Some(PuzzleDoor {
//             id: format!("{:?}-{:?}", (xoff + 41, yoff + 7), (xoff + 41, yoff + 8)),
//             orient: PDoorHV::Vert,
//             idxs: vec![(xoff + 41, yoff + 7), (xoff + 41, yoff + 8)],
//             set,
//         }),
//         (RuinRoom::TB, pos) if pos.1 > 0 => Some(PuzzleDoor {
//             id: format!(
//                 "{:?}-{:?}-{:?}-{:?}",
//                 (xoff + 19, yoff),
//                 (xoff + 20, yoff),
//                 (xoff + 21, yoff),
//                 (xoff + 22, yoff)
//             ),
//             orient: PDoorHV::Horiz,
//             idxs: vec![
//                 (xoff + 19, yoff),
//                 (xoff + 20, yoff),
//                 (xoff + 21, yoff),
//                 (xoff + 22, yoff),
//             ],
//             set,
//         }),
//         (RuinRoom::TB, pos) if pos.1 < 0 => Some(PuzzleDoor {
//             id: format!(
//                 "{:?}-{:?}-{:?}-{:?}",
//                 (xoff + 19, yoff + 15),
//                 (xoff + 20, yoff + 15),
//                 (xoff + 21, yoff + 15),
//                 (xoff + 22, yoff + 15)
//             ),
//             orient: PDoorHV::Horiz,
//             idxs: vec![
//                 (xoff + 19, yoff + 15),
//                 (xoff + 20, yoff + 15),
//                 (xoff + 21, yoff + 15),
//                 (xoff + 22, yoff + 15),
//             ],
//             set,
//         }),
//         (RuinRoom::LR, pos) if pos.0 > 0 => Some(PuzzleDoor {
//             id: format!("{:?}-{:?}", (xoff + 0, yoff + 7), (xoff + 0, yoff + 8)),
//             orient: PDoorHV::Vert,
//             idxs: vec![(xoff + 0, yoff + 7), (xoff + 0, yoff + 8)],
//             set,
//         }),
//         (RuinRoom::LR, pos) if pos.0 < 0 => Some(PuzzleDoor {
//             id: format!("{:?}-{:?}", (xoff + 41, yoff + 7), (xoff + 41, yoff + 8)),
//             orient: PDoorHV::Vert,
//             idxs: vec![(xoff + 41, yoff + 7), (xoff + 41, yoff + 8)],
//             set,
//         }),
//         (RuinRoom::TL, pos) if pos.1 > 0 => Some(PuzzleDoor {
//             id: format!(
//                 "{:?}-{:?}-{:?}-{:?}",
//                 (xoff + 19, yoff),
//                 (xoff + 20, yoff),
//                 (xoff + 21, yoff),
//                 (xoff + 22, yoff)
//             ),
//             orient: PDoorHV::Horiz,
//             idxs: vec![
//                 (xoff + 19, yoff),
//                 (xoff + 20, yoff),
//                 (xoff + 21, yoff),
//                 (xoff + 22, yoff),
//             ],
//             set,
//         }),
//         (RuinRoom::TL, pos) if pos.0 > 0 => Some(PuzzleDoor {
//             id: format!("{:?}-{:?}", (xoff + 0, yoff + 7), (xoff + 0, yoff + 8)),
//             orient: PDoorHV::Vert,
//             idxs: vec![(xoff + 0, yoff + 7), (xoff + 0, yoff + 8)],
//             set,
//         }),
//         (RuinRoom::TR, pos) if pos.1 > 0 => Some(PuzzleDoor {
//             id: format!(
//                 "{:?}-{:?}-{:?}-{:?}",
//                 (xoff + 19, yoff),
//                 (xoff + 20, yoff),
//                 (xoff + 21, yoff),
//                 (xoff + 22, yoff)
//             ),
//             orient: PDoorHV::Horiz,
//             idxs: vec![
//                 (xoff + 19, yoff),
//                 (xoff + 20, yoff),
//                 (xoff + 21, yoff),
//                 (xoff + 22, yoff),
//             ],
//             set,
//         }),
//         (RuinRoom::TR, pos) if pos.0 < 0 => Some(PuzzleDoor {
//             id: format!("{:?}-{:?}", (xoff + 41, yoff + 7), (xoff + 41, yoff + 8)),
//             orient: PDoorHV::Vert,
//             idxs: vec![(xoff + 41, yoff + 7), (xoff + 41, yoff + 8)],
//             set,
//         }),
//         (RuinRoom::BL, pos) if pos.1 < 0 => Some(PuzzleDoor {
//             id: format!(
//                 "{:?}-{:?}-{:?}-{:?}",
//                 (xoff + 19, yoff + 15),
//                 (xoff + 20, yoff + 15),
//                 (xoff + 21, yoff + 15),
//                 (xoff + 22, yoff + 15)
//             ),
//             orient: PDoorHV::Horiz,
//             idxs: vec![
//                 (xoff + 19, yoff + 15),
//                 (xoff + 20, yoff + 15),
//                 (xoff + 21, yoff + 15),
//                 (xoff + 22, yoff + 15),
//             ],
//             set,
//         }),
//         (RuinRoom::BL, pos) if pos.0 > 0 => Some(PuzzleDoor {
//             id: format!("{:?}-{:?}", (xoff + 0, yoff + 7), (xoff + 0, yoff + 8)),
//             orient: PDoorHV::Vert,
//             idxs: vec![(xoff + 0, yoff + 7), (xoff + 0, yoff + 8)],
//             set,
//         }),
//         (RuinRoom::BR, pos) if pos.1 < 0 => Some(PuzzleDoor {
//             id: format!(
//                 "{:?}-{:?}-{:?}-{:?}",
//                 (xoff + 19, yoff + 15),
//                 (xoff + 20, yoff + 15),
//                 (xoff + 21, yoff + 15),
//                 (xoff + 22, yoff + 15)
//             ),
//             orient: PDoorHV::Horiz,
//             idxs: vec![
//                 (xoff + 19, yoff + 15),
//                 (xoff + 20, yoff + 15),
//                 (xoff + 21, yoff + 15),
//                 (xoff + 22, yoff + 15),
//             ],
//             set,
//         }),
//         (RuinRoom::BR, pos) if pos.0 < 0 => Some(PuzzleDoor {
//             id: format!("{:?}-{:?}", (xoff + 41, yoff + 7), (xoff + 41, yoff + 8)),
//             orient: PDoorHV::Vert,
//             idxs: vec![(xoff + 41, yoff + 7), (xoff + 41, yoff + 8)],
//             set,
//         }),
//         (RuinRoom::TBL, pos) if pos.1 > 0 => Some(PuzzleDoor {
//             id: format!(
//                 "{:?}-{:?}-{:?}-{:?}",
//                 (xoff + 19, yoff),
//                 (xoff + 20, yoff),
//                 (xoff + 21, yoff),
//                 (xoff + 22, yoff)
//             ),
//             orient: PDoorHV::Horiz,
//             idxs: vec![
//                 (xoff + 19, yoff),
//                 (xoff + 20, yoff),
//                 (xoff + 21, yoff),
//                 (xoff + 22, yoff),
//             ],
//             set,
//         }),
//         (RuinRoom::TBL, pos) if pos.1 < 0 => Some(PuzzleDoor {
//             id: format!(
//                 "{:?}-{:?}-{:?}-{:?}",
//                 (xoff + 19, yoff + 15),
//                 (xoff + 20, yoff + 15),
//                 (xoff + 21, yoff + 15),
//                 (xoff + 22, yoff + 15)
//             ),
//             orient: PDoorHV::Horiz,
//             idxs: vec![
//                 (xoff + 19, yoff + 15),
//                 (xoff + 20, yoff + 15),
//                 (xoff + 21, yoff + 15),
//                 (xoff + 22, yoff + 15),
//             ],
//             set,
//         }),
//         (RuinRoom::TBL, pos) if pos.0 > 0 => Some(PuzzleDoor {
//             id: format!("{:?}-{:?}", (xoff + 0, yoff + 7), (xoff + 0, yoff + 8)),
//             orient: PDoorHV::Vert,
//             idxs: vec![(xoff + 0, yoff + 7), (xoff + 0, yoff + 8)],
//             set,
//         }),
//         (RuinRoom::TBR, pos) if pos.1 > 0 => Some(PuzzleDoor {
//             id: format!(
//                 "{:?}-{:?}-{:?}-{:?}",
//                 (xoff + 19, yoff),
//                 (xoff + 20, yoff),
//                 (xoff + 21, yoff),
//                 (xoff + 22, yoff)
//             ),
//             orient: PDoorHV::Horiz,
//             idxs: vec![
//                 (xoff + 19, yoff),
//                 (xoff + 20, yoff),
//                 (xoff + 21, yoff),
//                 (xoff + 22, yoff),
//             ],
//             set,
//         }),
//         (RuinRoom::TBR, pos) if pos.1 < 0 => Some(PuzzleDoor {
//             id: format!(
//                 "{:?}-{:?}-{:?}-{:?}",
//                 (xoff + 19, yoff + 15),
//                 (xoff + 20, yoff + 15),
//                 (xoff + 21, yoff + 15),
//                 (xoff + 22, yoff + 15)
//             ),
//             orient: PDoorHV::Horiz,
//             idxs: vec![
//                 (xoff + 19, yoff + 15),
//                 (xoff + 20, yoff + 15),
//                 (xoff + 21, yoff + 15),
//                 (xoff + 22, yoff + 15),
//             ],
//             set,
//         }),
//         (RuinRoom::TBR, pos) if pos.0 < 0 => Some(PuzzleDoor {
//             id: format!("{:?}-{:?}", (xoff + 41, yoff + 7), (xoff + 41, yoff + 8)),
//             orient: PDoorHV::Vert,
//             idxs: vec![(xoff + 41, yoff + 7), (xoff + 41, yoff + 8)],
//             set,
//         }),
//         (RuinRoom::LRT, pos) if pos.0 > 0 => Some(PuzzleDoor {
//             id: format!("{:?}-{:?}", (xoff + 0, yoff + 7), (xoff + 0, yoff + 8)),
//             orient: PDoorHV::Vert,
//             idxs: vec![(xoff + 0, yoff + 7), (xoff + 0, yoff + 8)],
//             set,
//         }),
//         (RuinRoom::LRT, pos) if pos.0 < 0 => Some(PuzzleDoor {
//             id: format!("{:?}-{:?}", (xoff + 41, yoff + 7), (xoff + 41, yoff + 8)),
//             orient: PDoorHV::Vert,
//             idxs: vec![(xoff + 41, yoff + 7), (xoff + 41, yoff + 8)],
//             set,
//         }),
//         (RuinRoom::LRT, pos) if pos.1 > 0 => Some(PuzzleDoor {
//             id: format!(
//                 "{:?}-{:?}-{:?}-{:?}",
//                 (xoff + 19, yoff),
//                 (xoff + 20, yoff),
//                 (xoff + 21, yoff),
//                 (xoff + 22, yoff)
//             ),
//             orient: PDoorHV::Horiz,
//             idxs: vec![
//                 (xoff + 19, yoff),
//                 (xoff + 20, yoff),
//                 (xoff + 21, yoff),
//                 (xoff + 22, yoff),
//             ],
//             set,
//         }),
//         (RuinRoom::LRB, pos) if pos.0 > 0 => Some(PuzzleDoor {
//             id: format!("{:?}-{:?}", (xoff + 0, yoff + 7), (xoff + 0, yoff + 8)),
//             orient: PDoorHV::Vert,
//             idxs: vec![(xoff + 0, yoff + 7), (xoff + 0, yoff + 8)],
//             set,
//         }),
//         (RuinRoom::LRB, pos) if pos.0 < 0 => Some(PuzzleDoor {
//             id: format!("{:?}-{:?}", (xoff + 41, yoff + 7), (xoff + 41, yoff + 8)),
//             orient: PDoorHV::Vert,
//             idxs: vec![(xoff + 41, yoff + 7), (xoff + 41, yoff + 8)],
//             set,
//         }),
//         (RuinRoom::LRB, pos) if pos.1 < 0 => Some(PuzzleDoor {
//             id: format!(
//                 "{:?}-{:?}-{:?}-{:?}",
//                 (xoff + 19, yoff + 15),
//                 (xoff + 20, yoff + 15),
//                 (xoff + 21, yoff + 15),
//                 (xoff + 22, yoff + 15)
//             ),
//             orient: PDoorHV::Horiz,
//             idxs: vec![
//                 (xoff + 19, yoff + 15),
//                 (xoff + 20, yoff + 15),
//                 (xoff + 21, yoff + 15),
//                 (xoff + 22, yoff + 15),
//             ],
//             set,
//         }),
//         (RuinRoom::All, pos) if pos.0 > 0 => Some(PuzzleDoor {
//             id: format!("{:?}-{:?}", (xoff + 0, yoff + 7), (xoff + 0, yoff + 8)),
//             orient: PDoorHV::Vert,
//             idxs: vec![(xoff + 0, yoff + 7), (xoff + 0, yoff + 8)],
//             set,
//         }),
//         (RuinRoom::All, pos) if pos.0 < 0 => Some(PuzzleDoor {
//             id: format!("{:?}-{:?}", (xoff + 41, yoff + 7), (xoff + 41, yoff + 8)),
//             orient: PDoorHV::Vert,
//             idxs: vec![(xoff + 41, yoff + 7), (xoff + 41, yoff + 8)],
//             set,
//         }),
//         (RuinRoom::All, pos) if pos.1 > 0 => Some(PuzzleDoor {
//             id: format!(
//                 "{:?}-{:?}-{:?}-{:?}",
//                 (xoff + 19, yoff),
//                 (xoff + 20, yoff),
//                 (xoff + 21, yoff),
//                 (xoff + 22, yoff)
//             ),
//             orient: PDoorHV::Horiz,
//             idxs: vec![
//                 (xoff + 19, yoff),
//                 (xoff + 20, yoff),
//                 (xoff + 21, yoff),
//                 (xoff + 22, yoff),
//             ],
//             set,
//         }),
//         (RuinRoom::All, pos) if pos.1 < 0 => Some(PuzzleDoor {
//             id: format!(
//                 "{:?}-{:?}-{:?}-{:?}",
//                 (xoff + 19, yoff + 15),
//                 (xoff + 20, yoff + 15),
//                 (xoff + 21, yoff + 15),
//                 (xoff + 22, yoff + 15)
//             ),
//             orient: PDoorHV::Horiz,
//             idxs: vec![
//                 (xoff + 19, yoff + 15),
//                 (xoff + 20, yoff + 15),
//                 (xoff + 21, yoff + 15),
//                 (xoff + 22, yoff + 15),
//             ],
//             set,
//         }),
//         _ => None,
//         // _ => todo!(),
//     }
// }

fn make_ruin_key(
    map: Vec<Vec<RuinRoom>>,
) -> (
    HashMap<(usize, usize), PuzzleDoor>,
    HashMap<(usize, usize), PuzzleKey>,
) {
    let center = (3, 3);
    let mut doors = HashMap::new();
    // let mut keys = Vec::new();
    for d in 0..3 {
        match d {
            0 => {
                let room_idx = [(0, -1), (-1, 0), (1, 0), (0, 1)];
                for (x, y) in room_idx {
                    if let Some(door) = map_room_doors(
                        map[(center.1 as i8 + y) as usize][(center.0 as i8 + x) as usize],
                        d,
                        (x, y),
                    ) {
                        doors.insert(
                            ((center.0 as i8 + x) as usize, (center.1 as i8 + y) as usize),
                            door,
                        );
                    }
                }
            }
            1 => {
                let room_idx = [
                    (-1, -2),
                    (0, -2),
                    (1, -2),
                    (-2, -1),
                    (-2, 0),
                    (-2, 1),
                    (2, 1),
                    (2, 0),
                    (2, -1),
                    (-1, 2),
                    (0, 2),
                    (1, 2),
                ];
                for (x, y) in room_idx {
                    if let Some(door) = map_room_doors(
                        map[(center.1 as i8 + y) as usize][(center.0 as i8 + x) as usize],
                        d,
                        (x, y),
                    ) {
                        doors.insert(
                            ((center.0 as i8 + x) as usize, (center.1 as i8 + y) as usize),
                            door,
                        );
                    }
                }
            }
            2 => {
                let room_idx = [
                    (-3, -2),
                    (-3, -1),
                    (-3, 0),
                    (-3, 1),
                    (-3, 2),
                    (3, -2),
                    (3, -1),
                    (3, 0),
                    (3, 1),
                    (3, 2),
                    (-2, -3),
                    (-1, -3),
                    (0, -3),
                    (1, -3),
                    (2, -3),
                    (-2, 3),
                    (-1, 3),
                    (0, 3),
                    (1, 3),
                    (2, 3),
                ];
                for (x, y) in room_idx {
                    if let Some(door) = map_room_doors(
                        map[(center.1 as i8 + y) as usize][(center.0 as i8 + x) as usize],
                        d,
                        (x, y),
                    ) {
                        doors.insert(
                            ((center.0 as i8 + x) as usize, (center.1 as i8 + y) as usize),
                            door,
                        );
                    }
                }
            }
            _ => todo!(),
        }
    }

    let mut keys = HashMap::new();
    let set2 = [
        (-3, 0),
        (3, 0),
        (0, -3),
        (0, 3),
        (-2, -2),
        (2, -2),
        (-2, 2),
        (2, 2),
    ];

    let set1 = [(-3, -3), (-3, 3), (3, -3), (3, 3)];

    for (x, y) in set2 {
        keys.insert(
            (
                ((center.0 as i8 + x) as usize * 42 + 21),
                ((center.1 as i8 + y) as usize * 16 + 8),
            ),
            PuzzleKey { set: 2 },
        );
    }

    for (x, y) in set1 {
        keys.insert(
            (
                ((center.0 as i8 + x) as usize * 42 + 21),
                ((center.1 as i8 + y) as usize * 16 + 8),
            ),
            PuzzleKey { set: 1 },
        );
    }
    (doors, keys)
}

fn place_map(map: Vec<Vec<Cells>>, mut nmap: Vec<Vec<Cells>>) -> Vec<Vec<Cells>> {
    for j in 0..map.len() {
        for i in 0..map[0].len() {
            nmap[j + 2][i + 4] = map[j][i];
        }
    }
    nmap
}

fn make_ruin() -> (
    Vec<Vec<Cells>>,
    HashMap<(usize, usize), NPCWrap>,
    HashMap<(usize, usize), Item>,
    HashMap<(usize, usize), EnvInter>,
) {
    let cells = vec![vec![Cells::Empty; 294]; 112];
    let (map, npcs, items, env_inters) = parse_map(&build_ruin().0, cells);
    let nmap = place_map(map, vec![vec![Cells::Empty; 302]; 116]);
    (nmap, npcs, items, env_inters)
}

fn make_key_ruin() -> (
    Vec<Vec<Cells>>,
    HashMap<(usize, usize), NPCWrap>,
    HashMap<(usize, usize), Item>,
    HashMap<(usize, usize), EnvInter>,
    HashMap<(usize, usize), PuzzleDoor>,
    HashMap<(usize, usize), PuzzleKey>,
) {
    let cells = vec![vec![Cells::Empty; 294]; 112];
    let (ruin_map, rooms): (String, Vec<Vec<RuinRoom>>) = build_ruin();
    let (doors, keys) = make_ruin_key(rooms);
    let (map, npcs, items, env_inters) = parse_map(&ruin_map, cells);
    let nmap = place_map(map, vec![vec![Cells::Empty; 302]; 116]);
    (nmap, npcs, items, env_inters, doors, keys)
}

type Doors = HashMap<(usize, usize), PuzzleDoor>;

#[derive(Clone, Debug, PartialEq)]
pub enum Puzzle {
    Maze {
        pos: (i16, i16),
        map: Vec<Vec<Cells>>,
        items: HashMap<(usize, usize), Item>,
        enemies: HashMap<(usize, usize), Enemy>,
        npcs: HashMap<(usize, usize), NPCWrap>,
        prize: Item,
        prop_pass: bool,
    },
    Ruin {
        pos: (i16, i16),
        map: Vec<Vec<Cells>>,
        items: HashMap<(usize, usize), Item>,
        enemies: HashMap<(usize, usize), Enemy>,
        npcs: HashMap<(usize, usize), NPCWrap>,
        prize: Item,
        prop_pass: bool,
    },
    Flip {
        pos: (i16, i16),
        map: Vec<Vec<Cells>>,
        items: HashMap<(usize, usize), Item>,
        enemies: HashMap<(usize, usize), Enemy>,
        npcs: HashMap<(usize, usize), NPCWrap>,
        prize: Item,
        prop_pass: bool,
    },
    KeyRuin {
        pos: (i16, i16),
        map: Vec<Vec<Cells>>,
        items: HashMap<(usize, usize), Item>,
        enemies: HashMap<(usize, usize), Enemy>,
        npcs: HashMap<(usize, usize), NPCWrap>,
        doors: Doors,
        keys: HashMap<(usize, usize), PuzzleKey>,
        prize: Item,
        prop_pass: bool,
    },
}

impl Puzzle {
    pub fn new_maze(pos: (i16, i16)) -> Self {
        let map = make_maze_map();
        let nmap = place_map(map, vec![vec![Cells::Empty; 304]; 206]);
        let items = HashMap::new();
        let enemies = HashMap::new();
        let npcs = HashMap::new();
        let prize = Item::new_health_potion(150, 100);
        Self::Maze {
            pos,
            map: nmap,
            items,
            enemies,
            npcs,
            prize,
            prop_pass: false,
        }
    }

    pub fn new_ruin(pos: (i16, i16)) -> Self {
        let (map, npcs, items, env_inters) = make_ruin();
        let enemies = HashMap::new();
        let prize = Item::new_health_potion(112, 42);
        Self::Ruin {
            pos,
            map,
            items,
            enemies,
            npcs,
            prize,
            prop_pass: false,
        }
    }

    pub fn new_flip(pos: (i16, i16)) -> Self {
        let map = make_teleport_map();
        let nmap = place_map(map, vec![vec![Cells::Empty; 308]; 204]);
        let items = HashMap::new();
        let enemies = HashMap::new();
        let npcs = HashMap::new();
        let prize = Item::new_health_potion(150, 100);
        Self::Flip {
            pos,
            map: nmap,
            items,
            enemies,
            npcs,
            prize,
            prop_pass: false,
        }
    }

    pub fn new_key_ruin(pos: (i16, i16)) -> Self {
        let (map, npcs, items, env_inters, doors, keys) = make_key_ruin();
        let enemies = HashMap::new();
        // log::info!("doors:\n{:#?}\nkeys:{:#?}", doors, keys);
        let prize = Item::new_health_potion(112, 42);
        Self::KeyRuin {
            pos,
            map,
            items,
            enemies,
            npcs,
            doors,
            keys,
            prize,
            prop_pass: false,
        }
    }

    pub fn get_pos(&mut self) -> (i16, i16) {
        match self {
            Self::Maze { pos, .. } => *pos,
            Self::Ruin { pos, .. } => *pos,
            Self::Flip { pos, .. } => *pos,
            Self::KeyRuin { pos, .. } => *pos,
        }
    }

    pub fn set_pos(&mut self, tpos: (i16, i16)) {
        match self {
            Self::Maze { pos, .. } => *pos = tpos,
            Self::Ruin { pos, .. } => *pos = tpos,
            Self::Flip { pos, .. } => *pos = tpos,
            Self::KeyRuin { pos, .. } => *pos = tpos,
        }
    }

    pub fn get_map(&mut self) -> Vec<Vec<Cells>> {
        match self {
            Self::Maze { map, .. } => map.clone(),
            Self::Ruin { map, .. } => map.clone(),
            Self::Flip { map, .. } => map.clone(),
            Self::KeyRuin { map, .. } => map.clone(),
        }
    }

    pub fn is_prop_pass(&mut self) -> bool {
        match self {
            Self::Maze { prop_pass, .. } => *prop_pass,
            Self::Ruin { prop_pass, .. } => *prop_pass,
            Self::Flip { prop_pass, .. } => *prop_pass,
            Self::KeyRuin { prop_pass, .. } => *prop_pass,
        }
    }

    pub fn get_items(&mut self) -> HashMap<(usize, usize), Item> {
        match self {
            Self::Maze { items, .. } => items.clone(),
            Self::Ruin { items, .. } => items.clone(),
            Self::Flip { items, .. } => items.clone(),
            Self::KeyRuin { items, .. } => items.clone(),
        }
    }

    pub fn get_npcs(&mut self) -> HashMap<(usize, usize), NPCWrap> {
        match self {
            Self::Maze { npcs, .. } => npcs.clone(),
            Self::Ruin { npcs, .. } => npcs.clone(),
            Self::Flip { npcs, .. } => npcs.clone(),
            Self::KeyRuin { npcs, .. } => npcs.clone(),
        }
    }

    pub fn get_doors(&mut self) -> Option<Doors> {
        match self {
            Self::KeyRuin { doors, .. } => Some(doors.clone()),
            _ => None,
        }
    }

    pub fn get_keys(&mut self) -> Option<HashMap<(usize, usize), PuzzleKey>> {
        match self {
            Self::KeyRuin { keys, .. } => Some(keys.clone()),
            _ => None,
        }
    }

    pub fn toggle_ppass(&mut self) {
        match self {
            Self::Maze { prop_pass, .. } => *prop_pass = !*prop_pass,
            Self::Ruin { prop_pass, .. } => *prop_pass = !*prop_pass,
            Self::Flip { prop_pass, .. } => *prop_pass = !*prop_pass,
            Self::KeyRuin { prop_pass, .. } => *prop_pass = !*prop_pass,
        }
    }

    // pub fn get_portals(&mut self) -> HashMap<(usize, usize), (usize, usize)> {
    //     self.portals.clone()
    // }
}
