use crate::enemy::Enemy;
use crate::enums::{Cells, Door, Enemies, EnvInter, Items, Location, PuzzlePiece, ShopItem};
use crate::item::Item;
use crate::puzzle::{PDoorHV, PuzzleDoor, PuzzleKey};
use rand::prelude::SliceRandom;
use rand::Rng;
use std::collections::HashMap;

fn place_enemies(map: Vec<Vec<Cells>>) -> HashMap<(usize, usize), Enemy> {
    let mut enemies = HashMap::new();
    let mut rng = rand::thread_rng();
    let etype = Enemies::Bug;
    let m_h = map.len() - 1;
    let m_w = map[0].len() - 1;
    for i in 0..50 {
        loop {
            // let y = rng.gen_range(10..m_h-10);
            let (x, y) = if i % 2 == 0 {
                let x = gen_broken_range(
                    &mut rng,
                    10,
                    (m_w / 3) as i32,
                    (m_w / 3) as i32 * 2,
                    (m_w - 10) as i32,
                ) as usize;
                let y = rng.gen_range(10..m_h - 10);
                (x, y)
            } else {
                let x = rng.gen_range(10..m_w - 10);
                let y = gen_broken_range(
                    &mut rng,
                    10,
                    (m_h / 3) as i32,
                    (m_h / 3) as i32 * 2,
                    (m_h - 10) as i32,
                ) as usize;
                (x, y)
            };
            if map[y][x] == Cells::Empty {
                // let mut temp_vec = Vec::new();
                // temp_vec.push(Items::BugBits);
                let temp_vec = vec![Items::Guts];
                let e_temp = Enemy::new(etype, "Bug".to_string(), (x, y), 20, 15, 5, 5, temp_vec);
                enemies.insert((x, y), e_temp);
                break;
            }
        }
    }
    enemies
}

pub fn gen_broken_range<R: Rng>(
    rng: &mut R,
    start1: i32,
    end1: i32,
    start2: i32,
    end2: i32,
) -> i32 {
    let range1_len = end1 - start1;
    let range2_len = end2 - start2;
    let total_len = range1_len + range2_len;

    let rand_val = rng.gen_range(0..total_len);

    if rand_val < range1_len {
        start1 + rand_val
    } else {
        start2 + (rand_val - range1_len)
    }
}

pub fn init_items(
    map: Vec<Vec<Cells>>,
    enemies: HashMap<(usize, usize), Enemy>,
) -> HashMap<(usize, usize), Item> {
    let mut items = HashMap::new();
    let mut rng = rand::thread_rng();
    let types = vec![Items::Rock, Items::EdibleRoot];
    let m_h = map.len() - 1;
    let m_w = map[0].len() - 1;
    for i in 0..200 {
        loop {
            let (x, y) = if i % 2 == 0 {
                let x = gen_broken_range(
                    &mut rng,
                    10,
                    (m_w / 3) as i32,
                    (m_w / 3) as i32 * 2,
                    (m_w - 10) as i32,
                ) as usize;
                let y = rng.gen_range(10..m_h - 10);
                (x, y)
            } else {
                let x = rng.gen_range(10..m_w - 10);
                let y = gen_broken_range(
                    &mut rng,
                    10,
                    (m_h / 3) as i32,
                    (m_h / 3) as i32 * 2,
                    (m_h - 10) as i32,
                ) as usize;
                (x, y)
            };
            // let x = rng.gen_range(10..m_w-10);
            // let y = rng.gen_range(10..m_h-10);
            if map[y][x] == Cells::Empty && !enemies.contains_key(&(x, y)) {
                if let Some(i_type) = types.choose(&mut rng) {
                    let itm = match i_type {
                        Items::EdibleRoot => Item::new_edible_root(x, y),
                        Items::Rock => Item::new_rock(x, y),
                        _ => todo!(),
                    };
                    items.insert((x, y), itm);
                    break;
                }
            }
        }
    }
    items
}

pub const COLLISION_CELLS: [Cells; 38] = [
    Cells::Wall,
    Cells::Wall2,
    Cells::Wall3,
    Cells::Wall4,
    Cells::MwH,
    Cells::MwV,
    Cells::MwVL,
    Cells::MwVR,
    Cells::MwHU,
    Cells::MwHD,
    Cells::MwUL,
    Cells::MwUR,
    Cells::MwDL,
    Cells::MwDR,
    Cells::MwCR,
    Cells::SwH,
    Cells::SwV,
    Cells::SwVL,
    Cells::SwVR,
    Cells::SwHU,
    Cells::SwHD,
    Cells::SwUL,
    Cells::SwUR,
    Cells::SwDL,
    Cells::SwDR,
    Cells::SwCR,
    Cells::LBrce,
    Cells::RBrce,
    Cells::LParen,
    Cells::RParen,
    Cells::GenCur,
    Cells::Water,
    Cells::Item,
    Cells::Cong,
    Cells::Log,
    Cells::Clinic,
    Cells::GPost,
    Cells::CPost,
];

pub const COLLISION_INTERS: [EnvInter; 24] = [
    EnvInter::Door(Door::VLocked(0)),
    EnvInter::Door(Door::VLocked(1)),
    EnvInter::Door(Door::VLocked(2)),
    EnvInter::Door(Door::VLocked(3)),
    EnvInter::Door(Door::VLocked(4)),
    EnvInter::Door(Door::VLocked(5)),
    EnvInter::Door(Door::VLocked(6)),
    EnvInter::Door(Door::VLocked(7)),
    EnvInter::Door(Door::VLocked(8)),
    EnvInter::Door(Door::VLocked(9)),
    EnvInter::Door(Door::HLocked(0)),
    EnvInter::Door(Door::HLocked(1)),
    EnvInter::Door(Door::HLocked(2)),
    EnvInter::Door(Door::HLocked(3)),
    EnvInter::Door(Door::HLocked(4)),
    EnvInter::Door(Door::HLocked(5)),
    EnvInter::Door(Door::HLocked(6)),
    EnvInter::Door(Door::HLocked(7)),
    EnvInter::Door(Door::HLocked(8)),
    EnvInter::Door(Door::HLocked(9)),
    EnvInter::Records,
    EnvInter::Clinic,
    EnvInter::GuildPost,
    EnvInter::ChurchPost,
];

// pub const COLLISION_PUZZLE_PIECE: [PuzzlePiece; 3] = [
//     PuzzlePiece::PuzzleDoor(()),
//     PuzzlePiece::PuzzleDoor(()),
//     PuzzlePiece::PuzzleKey(PuzzleKey { set: 0 }),
//     // PuzzlePiece::PuzzleDoor(PuzzleDoor {
//     //     id: String::from(""),
//     //     orient: PDoorHV::Vert,
//     //     idxs: [(0, 0), (0, 0)].to_vec(),
//     //     set: 0,
//     // }),
//     // PuzzlePiece::PuzzleDoor(PuzzleDoor {
//     //     id: "".to_string(),
//     //     orient: PDoorHV::Horiz,
//     //     idxs: [(0, 0), (0, 0), (0, 0), (0, 0)].to_vec(),
//     //     set: 0,
//     // }),
//     // PuzzlePiece::PuzzleKey(PuzzleKey { set: 0 }),
// ];

pub fn in_range(pos1: (i16, i16), pos2: (i16, i16), rad: u16) -> bool {
    let xx = (pos1.0 - pos2.0) as i32;
    let yy = (pos1.1 - pos2.1) as i32;
    let hyp = ((xx.pow(2) + yy.pow(2)) as f64).sqrt() as u16;
    //log::info!("hyp: {}, eCx: {}, ey: {}", e.steps.clone(), x.clone(), y.clone());
    hyp <= rad
}

pub fn get_dir(vec: (i16, i16)) -> (i8, i8) {
    match vec {
        (x, y) if x < 0 && y < 0 => (-1, -1),
        (x, y) if x >= 0 && y < 0 => (1, -1),
        (x, y) if x < 0 && y >= 0 => (-1, 1),
        (x, y) if x >= 0 && y >= 0 => (1, 1),
        _ => (0, 0),
    }
}

pub fn loc_shop_items(dist_fo: (i16, i16), loc: Location) -> HashMap<(usize, usize), ShopItem> {
    match loc {
        Location::Null => HashMap::new(),
        Location::Settlement(mut settle) => {
            let mut itms = HashMap::new();
            if let Some(sitems) = settle.get_all_shop_items() {
                let spos = settle.get_pos();
                for ((x, y), i) in sitems {
                    let nx = (dist_fo.0 + x as i16 + spos.0) as usize;
                    let ny = (dist_fo.1 + y as i16 + spos.1) as usize;
                    // let ipos = i.get_pos();
                    // i.set_pos((nx, ny));
                    itms.insert((nx, ny), i);
                }
                itms
            } else {
                itms
            }
        }
        Location::Puzzle(_puzzle) => HashMap::new(),
        Location::Feature(feat) => {
            let mut itms = HashMap::new();
            let sitems = feat.hermit_shop.stock;
            let fpos = feat.pos;
            for ((x, y), i) in sitems {
                let nx = (dist_fo.0 + x as i16 + fpos.0) as usize;
                let ny = (dist_fo.1 + y as i16 + fpos.1) as usize;
                itms.insert((nx, ny), i);
            }
            itms
        } // Location::Feature(_) => HashMap::new(),
    }
}

pub fn comb_conv(name: String, convo: Vec<String>) -> String {
    let mut conv = "".to_string();
    for (i, c) in convo.into_iter().enumerate() {
        let t = if i % 2 == 0 {
            format!("{}: {}", name, c)
        } else {
            format!("You: {}", c)
        };
        conv.push_str(&t);
        conv.push('#');
    }
    conv
}
