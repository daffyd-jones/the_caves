//puzzle.rs
//
use crate::enemy::Enemy;
use crate::enums::{Cells, EnvInter, NPCWrap, PuzzleType};
use crate::item::Item;
use crate::parsing::{parse_map, tile_to_chars};
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

// enum RuinRoom {
//     B,
//     T,
//     L,
//     R,
//     TB,
//     LR,
//     TL,
//     TR,
//     BL,
//     BR,
//     TBL,
//     TBR,
//     LRT,
//     LRB,
//     All,
//     Default,
//     Null,
// }

enum PDoorHV {
    Vert,
    Horiz,
}

struct PuzzleKey {
    set: u8,
}

struct PuzzleDoor {
    orient: PDoorHV,
    idxs: Vec<(usize, usize)>,
    set: u8,
}

fn map_room_doors(room: RuinRoom, set: u8, pos: (i8, i8)) -> Option<PuzzleDoor> {
    match (room, pos) {
        (RuinRoom::B, pos) if pos.1 < 0 => Some(PuzzleDoor {
            orient: PDoorHV::Horiz,
            idxs: vec![(20, 15), (21, 15), (22, 15), (23, 15)],
            set,
        }),
        (RuinRoom::T, pos) if pos.1 > 0 => Some(PuzzleDoor {
            orient: PDoorHV::Horiz,
            idxs: vec![(20, 0), (21, 0), (22, 0), (23, 0)],
            set,
        }),
        (RuinRoom::L, pos) if pos.0 > 0 => Some(PuzzleDoor {
            orient: PDoorHV::Vert,
            idxs: vec![(0, 7), (0, 8)],
            set,
        }),
        (RuinRoom::R, pos) if pos.0 < 0 => Some(PuzzleDoor {
            orient: PDoorHV::Vert,
            idxs: vec![(15, 7), (15, 8)],
            set,
        }),
        RuinRoom::TB => {
            vec![
                PuzzleDoor {
                    orient: PDoorHV::Horiz,
                    idxs: vec![(20, 0), (21, 0), (22, 0), (23, 0)],
                    set,
                },
                PuzzleDoor {
                    orient: PDoorHV::Horiz,
                    idxs: vec![(20, 15), (21, 15), (22, 15), (23, 15)],
                    set,
                },
            ]
        }
        RuinRoom::LR => {
            vec![
                PuzzleDoor {
                    orient: PDoorHV::Vert,
                    idxs: vec![(0, 7), (0, 8)],
                    set,
                },
                PuzzleDoor {
                    orient: PDoorHV::Vert,
                    idxs: vec![(15, 7), (15, 8)],
                    set,
                },
            ]
        }
        RuinRoom::TL => {
            vec![
                PuzzleDoor {
                    orient: PDoorHV::Horiz,
                    idxs: vec![(20, 0), (21, 0), (22, 0), (23, 0)],
                    set,
                },
                PuzzleDoor {
                    orient: PDoorHV::Vert,
                    idxs: vec![(0, 7), (0, 8)],
                    set,
                },
            ]
        }
        RuinRoom::TR => {
            vec![
                PuzzleDoor {
                    orient: PDoorHV::Horiz,
                    idxs: vec![(20, 0), (21, 0), (22, 0), (23, 0)],
                    set,
                },
                PuzzleDoor {
                    orient: PDoorHV::Vert,
                    idxs: vec![(15, 7), (15, 8)],
                    set,
                },
            ]
        }
        RuinRoom::BL => {
            vec![
                PuzzleDoor {
                    orient: PDoorHV::Horiz,
                    idxs: vec![(20, 15), (21, 15), (22, 15), (23, 15)],
                    set,
                },
                PuzzleDoor {
                    orient: PDoorHV::Vert,
                    idxs: vec![(0, 7), (0, 8)],
                    set,
                },
            ]
        }
        RuinRoom::BR => {
            vec![
                PuzzleDoor {
                    orient: PDoorHV::Horiz,
                    idxs: vec![(20, 15), (21, 15), (22, 15), (23, 15)],
                    set,
                },
                PuzzleDoor {
                    orient: PDoorHV::Vert,
                    idxs: vec![(15, 7), (15, 8)],
                    set,
                },
            ]
        }
        RuinRoom::TBL => {
            vec![
                PuzzleDoor {
                    orient: PDoorHV::Horiz,
                    idxs: vec![(20, 0), (21, 0), (22, 0), (23, 0)],
                    set,
                },
                PuzzleDoor {
                    orient: PDoorHV::Horiz,
                    idxs: vec![(20, 15), (21, 15), (22, 15), (23, 15)],
                    set,
                },
                PuzzleDoor {
                    orient: PDoorHV::Vert,
                    idxs: vec![(0, 7), (0, 8)],
                    set,
                },
            ]
        }
        RuinRoom::TBR => {
            vec![
                PuzzleDoor {
                    orient: PDoorHV::Horiz,
                    idxs: vec![(20, 0), (21, 0), (22, 0), (23, 0)],
                    set,
                },
                PuzzleDoor {
                    orient: PDoorHV::Horiz,
                    idxs: vec![(20, 15), (21, 15), (22, 15), (23, 15)],
                    set,
                },
                PuzzleDoor {
                    orient: PDoorHV::Vert,
                    idxs: vec![(15, 7), (15, 8)],
                    set,
                },
            ]
        }
        RuinRoom::LRT => {
            vec![
                PuzzleDoor {
                    orient: PDoorHV::Vert,
                    idxs: vec![(0, 7), (0, 8)],
                    set,
                },
                PuzzleDoor {
                    orient: PDoorHV::Vert,
                    idxs: vec![(15, 7), (15, 8)],
                    set,
                },
                PuzzleDoor {
                    orient: PDoorHV::Horiz,
                    idxs: vec![(20, 0), (21, 0), (22, 0), (23, 0)],
                    set,
                },
            ]
        }
        RuinRoom::LRB => {
            vec![
                PuzzleDoor {
                    orient: PDoorHV::Vert,
                    idxs: vec![(0, 7), (0, 8)],
                    set,
                },
                PuzzleDoor {
                    orient: PDoorHV::Vert,
                    idxs: vec![(15, 7), (15, 8)],
                    set,
                },
                PuzzleDoor {
                    orient: PDoorHV::Horiz,
                    idxs: vec![(20, 15), (21, 15), (22, 15), (23, 15)],
                    set,
                },
            ]
        }
        RuinRoom::All => {
            vec![
                PuzzleDoor {
                    orient: PDoorHV::Vert,
                    idxs: vec![(0, 7), (0, 8)],
                    set,
                },
                PuzzleDoor {
                    orient: PDoorHV::Vert,
                    idxs: vec![(15, 7), (15, 8)],
                    set,
                },
                PuzzleDoor {
                    orient: PDoorHV::Horiz,
                    idxs: vec![(20, 0), (21, 0), (22, 0), (23, 0)],
                    set,
                },
                PuzzleDoor {
                    orient: PDoorHV::Horiz,
                    idxs: vec![(20, 15), (21, 15), (22, 15), (23, 15)],
                    set,
                },
            ]
        }
        _ => todo!(),
    }
}

fn make_ruin_key(map: Vec<Vec<RuinRoom>>) {
    let center = (3, 3);
    let mut doors = Vec::new();
    // let mut keys = Vec::new();
    for d in 0..3 {
        match d {
            // 0 => {
            //     let mut depth_rooms = HashMap::new();
            //     depth_rooms.insert(
            //         (center.0, center.1),
            //         map_room_doors(map[center.1][center.0], d),
            //     );
            //     doors.push(depth_rooms);
            // }
            0 => {
                let room_idx = [(0, -1), (-1, 0), (1, 0), (0, 1)];
                let mut depth_rooms = HashMap::new();
                for (y, x) in room_idx {
                    depth_rooms.insert(
                        ((center.0 as i8 + x) as usize, (center.1 as i8 + y) as usize),
                        map_room_doors(
                            map[(center.1 as i8 + y) as usize][(center.0 as i8 + x) as usize],
                            d,
                            (y, x),
                        ),
                    );
                }
                doors.push(depth_rooms);
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
                let mut depth_rooms = HashMap::new();
                for (y, x) in room_idx {
                    // depth_rooms.insert((x, y), map_room_doors(map[center.1 + y][center.0 + x], d));
                    depth_rooms.insert(
                        ((center.0 as i8 + x) as usize, (center.1 as i8 + y) as usize),
                        map_room_doors(
                            map[(center.1 as i8 + y) as usize][(center.0 as i8 + x) as usize],
                            d,
                            (y, x),
                        ),
                    );
                }
                doors.push(depth_rooms);
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
                let mut depth_rooms = HashMap::new();
                for (y, x) in room_idx {
                    // depth_rooms.insert((x, y), map_room_doors(map[center.1 + y][center.0 + x], d));
                    depth_rooms.insert(
                        ((center.0 as i8 + x) as usize, (center.1 as i8 + y) as usize),
                        map_room_doors(
                            map[(center.1 as i8 + y) as usize][(center.0 as i8 + x) as usize],
                            d,
                            (y, x),
                        ),
                    );
                }
                doors.push(depth_rooms);
            }
            _ => todo!(),
        }
    }

    // for y in -3..3 {
    //     for x in -3..3 {
    //         if
    //     }
    // }
}

fn make_ruin() -> (
    Vec<Vec<Cells>>,
    HashMap<(usize, usize), NPCWrap>,
    HashMap<(usize, usize), Item>,
    HashMap<(usize, usize), EnvInter>,
) {
    let cells = vec![vec![Cells::Empty; 294]; 112];
    let (ruin_map, rooms): (String, Vec<Vec<RuinRoom>>) = build_ruin();
    parse_map(&ruin_map, cells)
}

#[derive(Clone, Debug, PartialEq)]
//#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Puzzle {
    pub ptype: PuzzleType,
    pub pos: (i16, i16),
    pub map: Vec<Vec<Cells>>,
    pub portals: HashMap<(usize, usize), (usize, usize)>,
    pub items: HashMap<(usize, usize), Item>,
    pub enemies: HashMap<(usize, usize), Enemy>,
    pub npcs: HashMap<(usize, usize), NPCWrap>,
    pub prize: Item,
    pub prop_pass: bool,
}

impl Puzzle {
    pub fn new(
        ptype: PuzzleType,
        pos: (i16, i16),
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

    pub fn new_maze(pos: (i16, i16)) -> Self {
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

    pub fn new_ruin(pos: (i16, i16)) -> Self {
        let (map, npcs, items, env_inters) = make_ruin();
        let portals = HashMap::new();
        // let items = HashMap::new();
        let enemies = HashMap::new();
        // let npcs = HashMap::new();
        let prize = Item::new_health_potion(112, 42);
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

    pub fn new_teleport(pos: (i16, i16)) -> Self {
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

    pub fn get_pos(&mut self) -> (i16, i16) {
        self.pos.clone()
    }

    pub fn set_pos(&mut self, tpos: (i16, i16)) {
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
