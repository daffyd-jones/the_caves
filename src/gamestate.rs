//gamestate
use crate::enemy::Enemy;
use crate::enums::{
    Cells, CompMode, EncOpt, Enemies, EnvInter, FightSteps, GUIMode, GameMode, InterOpt,
    InterSteps, Interactable, ItemOpt, Items, Location, NPCWrap, NPCs, PuzzleType,
};
use crate::map::{Map, MAP_H, MAP_W};
use crate::npc::{new_comm_npc, new_conv_npc, new_shop_npc, BaseNPC, Convo, NPC};
use crate::player::Player;
use crate::puzzles::Puzzles;
//use crate::lsystems::LSystems;
use crate::features::Features;
use crate::gui::GUI;
use crate::item::Item;
use crate::notebook::Notebook;
use crate::settlements::Settlements;
use crate::shop::Shop;

mod enemies;
mod keys;
mod locations;
mod npcs;

use rand::prelude::SliceRandom;
use rand::Rng;
use ratatui::crossterm::event::{poll, read, Event, KeyCode};

// use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
//use serde_json::Result;
//use serde_json::Value;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;

use std::collections::HashMap;
use std::io::{self, BufRead, BufReader, Write};

fn gen_broken_range<R: Rng>(rng: &mut R, start1: i32, end1: i32, start2: i32, end2: i32) -> i32 {
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

fn place_enemies(map: Vec<Vec<Cells>>) -> HashMap<(usize, usize), Enemy> {
    let mut enemies = HashMap::new();
    let mut rng = rand::thread_rng();
    let etype = Enemies::Bug;
    let m_h = map.len() - 1;
    let m_w = map[0].len() - 1;
    for i in 0..100 {
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
                let temp_vec = vec![Items::BugBits];
                let e_temp = Enemy::new(etype, "Bug".to_string(), (x, y), 20, 15, 5, 5, temp_vec);
                enemies.insert((x, y), e_temp);
                break;
            }
        }
    }
    enemies
}

fn place_npcs(map: Vec<Vec<Cells>>) -> HashMap<(usize, usize), NPCWrap> {
    let data1 = fs::read_to_string("src/npcs/npc_names.json");
    // log::info!("{:?}", &data1);
    let names: Vec<String> = match data1 {
        Ok(content) => serde_json::from_str(&content).unwrap(),
        Err(e) => {
            log::info!("{:?}", e);
            Vec::new()
        }
    };

    let data2 = fs::read_to_string("src/npcs/npc_comms.json");
    // log::info!("{:?}", &data2);
    let comms: Vec<String> = match data2 {
        Ok(content) => serde_json::from_str(&content).unwrap(),
        Err(e) => {
            log::info!("{:?}", e);
            Vec::new()
        }
    };

    let data3 = fs::read_to_string("src/npcs/npc_convos.json");
    // log::info!("{:?}", &data3);
    let convos: Vec<Convo> = match data3 {
        Ok(content) => serde_json::from_str(&content).unwrap(),
        Err(e) => {
            log::info!("{:?}", e);
            Vec::new()
        }
    };

    //let data4 = fs::read_to_string("src/npcs/npc_quests.json");
    //log::info!("{:?}", &data4);
    //let quests: HashMap<String, NQuest> = match data4 {
    //    Ok(content) => serde_json::from_str(&content).unwrap(),
    //    Err(e) => {
    //        log::info!("{:?}", e);
    //        HashMap::new()
    //    },
    //};
    let mut npcs = HashMap::new();
    let mut rng = rand::thread_rng();
    let types = vec![NPCs::ConvNPC, NPCs::CommNPC];
    // let types = vec![NPCs::CommNPC, NPCs::ConvNPC, NPCs::QuestNPC];
    let m_h = map.len() - 1;
    let m_w = map[0].len() - 1;
    for i in 0..80 {
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
            if map[y][x] == Cells::Empty {
                if let Some(i_type) = types.choose(&mut rng) {
                    let npc = match i_type {
                        NPCs::CommNPC => {
                            let rnd_comms = {
                                let mut tvec = Vec::new();
                                for _ in 0..4 {
                                    let tidx = rng.gen_range(0..comms.len());
                                    tvec.push(comms[tidx].clone());
                                }
                                tvec
                            };
                            let name = names
                                .choose(&mut rng)
                                .unwrap_or(&"Kevthony".to_string())
                                .clone();

                            //let sname = &names[0];
                            //let comm: Vec<String> = comms.clone();
                            NPCWrap::CommNPC(new_comm_npc(name.to_string(), x, y, rnd_comms))
                        }
                        NPCs::ConvNPC => {
                            let name = names
                                .choose(&mut rng)
                                .unwrap_or(&"Miranda".to_string())
                                .clone();
                            let conv: Convo = convos
                                .choose(&mut rng)
                                .unwrap_or(&convos[0].clone())
                                .clone();
                            //let sname = &names[0];
                            //let conv: Convo = convos[0].clone();
                            NPCWrap::ConvNPC(new_conv_npc(name.to_string(), x, y, conv))
                        }
                        _ => todo!(),
                    };
                    npcs.insert((x, y), npc);
                    break;
                }
            }
        }
    }
    npcs
}

fn init_items(
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

fn map_to_string(cells: &Vec<Vec<Cells>>) -> String {
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
                _ => todo!(),
            };
            map_string.push_str(&symbol.to_string());
        }
        map_string.push('\n');
    }
    map_string
}

fn n_collision(dir: &str, pos: (usize, usize), cells: Vec<Vec<Cells>>) -> bool {
    match dir {
        "UP" => collision_cells.contains(&cells[pos.1 - 1][pos.0]),
        "DN" => collision_cells.contains(&cells[pos.1 + 1][pos.0]),
        "LF" => collision_cells.contains(&cells[pos.1][pos.0 - 1]),
        "RT" => collision_cells.contains(&cells[pos.1][pos.0 + 1]),
        _ => false,
    }
}

fn npc_move(
    mut npc: Box<dyn NPC>,
    map: Vec<Vec<Cells>>,
    mw: usize,
    mh: usize,
    x: usize,
    y: usize,
) -> ((usize, usize), Box<dyn NPC>) {
    let mut rng = rand::thread_rng();
    let dch = rng.gen_range(0..20);
    if dch % 5 == 0 {
        npc.set_steps(dch);
    }
    let pos = if npc.get_steps() < 5 {
        npc.inc_steps();
        // if y == 0 {(x, y)} else {
        if y <= 10 || n_collision("UP", npc.get_pos(), map.clone()) {
            (x, y)
        } else {
            npc.mmove("UP");
            (x, y - 1)
        }
    } else if npc.get_steps() >= 5 && npc.get_steps() < 10 {
        npc.inc_steps();
        // if x == 0 {(x, y)} else {
        if x <= 10 || n_collision("LF", npc.get_pos(), map.clone()) {
            (x, y)
        } else {
            npc.mmove("LF");
            (x - 1, y)
        }
    } else if npc.get_steps() >= 10 && npc.get_steps() < 15 {
        npc.inc_steps();
        // if y >= mh-5 {(x, y)} else {
        if y >= mh - 10 || n_collision("DN", npc.get_pos(), map.clone()) {
            (x, y)
        } else {
            npc.mmove("DN");
            (x, y + 1)
        }
    } else if npc.get_steps() >= 15 && npc.get_steps() < 20 {
        npc.inc_steps();
        // if x >= mw-5 {(x, y)} else {
        if x >= mw - 10 || n_collision("RT", npc.get_pos(), map.clone()) {
            (x, y)
        } else {
            npc.mmove("RT");
            (x + 1, y)
        }
    } else if npc.get_steps() == 20 {
        npc.set_steps(0);
        (x, y)
    } else {
        (x, y)
    };
    (pos, npc)
    // (pos, Box::new(npc))
}

pub fn box_npc(npc: NPCWrap) -> Box<dyn NPC> {
    match npc {
        NPCWrap::CommNPC(comm_npc) => Box::new(comm_npc),
        NPCWrap::ConvNPC(conv_npc) => Box::new(conv_npc),
        NPCWrap::ShopNPC(shop_npc) => Box::new(shop_npc),
        NPCWrap::SpawnNPC(spawn_npc) => Box::new(spawn_npc),
        NPCWrap::TradeNPC(trade_npc) => Box::new(trade_npc),
        _ => todo!(),
    }
}

pub fn wrap_nbox(mut nbox: Box<dyn NPC>) -> NPCWrap {
    match nbox.get_ntype() {
        NPCs::CommNPC => {
            if let Some(comm_npc) = nbox.as_comm_npc() {
                NPCWrap::CommNPC(comm_npc.clone())
            } else {
                NPCWrap::BaseNPC(BaseNPC::new())
            }
        }
        NPCs::ConvNPC => {
            if let Some(conv_npc) = nbox.as_conv_npc() {
                NPCWrap::ConvNPC(conv_npc.clone())
            } else {
                NPCWrap::BaseNPC(BaseNPC::new())
            }
        }
        NPCs::ShopNPC => {
            if let Some(shop_npc) = nbox.as_shop_npc() {
                NPCWrap::ShopNPC(shop_npc.clone())
            } else {
                NPCWrap::BaseNPC(BaseNPC::new())
            }
        }
        NPCs::SpawnNPC => {
            if let Some(spawn_npc) = nbox.as_spawn_npc() {
                NPCWrap::SpawnNPC(spawn_npc.clone())
            } else {
                NPCWrap::BaseNPC(BaseNPC::new())
            }
        }
        NPCs::TradeNPC => {
            if let Some(trade_npc) = nbox.as_trade_npc() {
                NPCWrap::TradeNPC(trade_npc.clone())
            } else {
                NPCWrap::BaseNPC(BaseNPC::new())
            }
        }
        _ => todo!(),
    }
}

fn shift_npc(npc: NPCWrap, pos: (usize, usize)) -> NPCWrap {
    let mut nbox = box_npc(npc);
    nbox.set_pos(pos);
    wrap_nbox(nbox)
}

fn loc_shop_items(dist_fo: (i64, i64), loc: Location) -> HashMap<(usize, usize), Item> {
    match loc {
        Location::Null => HashMap::new(),
        Location::Settlement(mut settle) => {
            let mut itms = HashMap::new();
            if let Some(sitems) = settle.get_all_shop_items() {
                let spos = settle.get_pos();
                for ((x, y), mut i) in sitems {
                    let nx = (dist_fo.0 + x as i64 + spos.0) as usize;
                    let ny = (dist_fo.1 + y as i64 + spos.1) as usize;
                    // let ipos = i.get_pos();
                    i.set_pos((nx, ny));
                    itms.insert((nx, ny), i);
                }
                itms
            } else {
                itms
            }
        }
        Location::Puzzle(_puzzle) => HashMap::new(),
        _ => todo!(),
    }
}

const collision_cells: [Cells; 35] = [
    Cells::Wall,
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

fn in_range(pos1: (i64, i64), pos2: (i64, i64), rad: u16) -> bool {
    let xx = pos1.0 - pos2.0;
    let yy = pos1.1 - pos2.1;
    let hyp = ((xx.pow(2) + yy.pow(2)) as f64).sqrt() as i64;
    //log::info!("hyp: {}, eCx: {}, ey: {}", e.steps.clone(), x.clone(), y.clone());
    hyp.abs() <= rad.into()
    // if hyp.abs() <= rad.into() {
    //     return true;
    // } else {
    //     return false;
    // }
}

fn get_dir(vec: (i64, i64)) -> (i8, i8) {
    match vec {
        (x, y) if x < 0 && y < 0 => (-1, -1),
        (x, y) if x >= 0 && y < 0 => (1, -1),
        (x, y) if x < 0 && y >= 0 => (-1, 1),
        (x, y) if x >= 0 && y >= 0 => (1, 1),
        _ => (0, 0),
    }
}

//#[derive(Serialize, Deserialize, Debug)]
pub struct GameState {
    game_mode: GameMode,
    notebook: Notebook,
    gui: GUI,
    map: Map,
    settles: Settlements,
    puzzles: Puzzles,
    player: Player,
    features: Features,
    dist_fo: (i64, i64),
    comp_head: (i64, i64),
    comp_list: HashMap<(i64, i64), String>,
    comp_mode: CompMode,
    loc_rad: u16,
    depth: u32,
    level: u32,
    //l_systems: LSystems,
    l_rate: u64,
    enemies: HashMap<(usize, usize), Enemy>,
    step_group: u8,
    items: HashMap<(usize, usize), Item>,
    npcs: HashMap<(usize, usize), NPCWrap>,
    env_inters: HashMap<(usize, usize), EnvInter>,
    npc_names: Vec<String>,
    npc_comms: Vec<String>,
    npc_convos: Vec<Convo>,
    npc_spconvos: Vec<Convo>,
    npc_spcomms: Vec<String>,
    npc_trade: Vec<HashMap<String, String>>,
    key_debounce_dur: Duration,
    last_event_time: Instant,
    interactee: Interactable,
    location: Location,
    portals: HashMap<(usize, usize), (usize, usize)>,
    portal_cool: Instant,
    loc_map: Option<Vec<Vec<Cells>>>,
    enc: EncOpt,
}

impl GameState {
    pub fn new() -> Arc<Mutex<Self>> {
        let gui = GUI::new();
        let map = Map::new();
        // let x = map.px.clone();
        // let y = map.py.clone();
        let comp_list = HashMap::new();
        let player = Player::new(309, 196);
        // let player = Player::new(x, y);
        //let mut l_systems = LSystems::new();
        let enemies = place_enemies(map.cells.clone());
        let items = init_items(map.cells.clone(), enemies.clone());
        //let npcs = place_npcs(map.cells.clone());
        let npcs = HashMap::new();
        let env_inters = HashMap::new();

        let data1 = fs::read_to_string("src/npcs/npc_names.json");
        //log::info!("{:?}", &data1);
        let npc_names: Vec<String> = match data1 {
            Ok(content) => serde_json::from_str(&content).unwrap(),
            Err(e) => {
                log::info!("{:?}", e);
                Vec::new()
            }
        };
        let data2 = fs::read_to_string("src/npcs/npc_comms.json");
        //log::info!("{:?}", &data2);
        let npc_comms: Vec<String> = match data2 {
            Ok(content) => serde_json::from_str(&content).unwrap(),
            Err(e) => {
                log::info!("{:?}", e);
                Vec::new()
            }
        };
        let data3 = fs::read_to_string("src/npcs/npc_convos.json");
        //log::info!("{:?}", &data3);
        let npc_convos: Vec<Convo> = match data3 {
            Ok(content) => serde_json::from_str(&content).unwrap(),
            Err(e) => {
                log::info!("{:?}", e);
                Vec::new()
            }
        };
        let data4 = fs::read_to_string("src/npcs/npc_spawn_convos.json");
        //log::info!("{:?}", &data3);
        let npc_spconvos: Vec<Convo> = match data4 {
            Ok(content) => serde_json::from_str(&content).unwrap(),
            Err(e) => {
                log::info!("{:?}", e);
                Vec::new()
            }
        };
        let data5 = fs::read_to_string("src/npcs/npc_spawn_comms.json");
        //log::info!("{:?}", &data3);
        let npc_spcomms: Vec<String> = match data5 {
            Ok(content) => serde_json::from_str(&content).unwrap(),
            Err(e) => {
                log::info!("{:?}", e);
                Vec::new()
            }
        };

        let data6 = fs::read_to_string("src/npcs/npc_trade.json");
        //log::info!("{:?}", &data3);
        let npc_trade: Vec<HashMap<String, String>> = match data6 {
            Ok(content) => serde_json::from_str(&content).unwrap(),
            Err(e) => {
                log::info!("{:?}", e);
                Vec::new()
            }
        };

        let portals = HashMap::new();

        let notebook = Notebook::new().unwrap();
        let l_rate = 100 as u64;

        let settles = Settlements::demo_self();
        let puzzles = Puzzles::demo_self();

        let features = Features::new();

        Arc::new(Mutex::new(GameState {
            game_mode: GameMode::Play,
            notebook,
            gui,
            map,
            settles,
            puzzles,
            player,
            features,
            dist_fo: (0, 0),
            comp_head: (0, 0),
            comp_list,
            comp_mode: CompMode::Search,
            loc_rad: 500,
            depth: 1,
            level: 0,
            //l_systems,
            l_rate,
            enemies,
            step_group: 0,
            items,
            npcs,
            env_inters,
            npc_names,
            npc_comms,
            npc_convos,
            npc_spconvos,
            npc_spcomms,
            npc_trade,
            key_debounce_dur: Duration::from_millis(30),
            last_event_time: Instant::now(),
            interactee: Interactable::Null,
            location: Location::Null,
            portals,
            portal_cool: Instant::now(),
            loc_map: None,
            enc: EncOpt::Null,
        }))
    }

    fn collision(&mut self, dir: &str) -> bool {
        match dir {
            "UP" => {
                let map_coll =
                    collision_cells.contains(&self.map.cells[self.player.y - 1][self.player.x]);
                let item_coll = self.items.contains_key(&(self.player.x, self.player.y - 1));
                map_coll || item_coll
            }
            "DN" => {
                let map_coll =
                    collision_cells.contains(&self.map.cells[self.player.y + 1][self.player.x]);
                let item_coll = self.items.contains_key(&(self.player.x, self.player.y + 1));
                map_coll || item_coll
            }
            "LF" => {
                let map_coll =
                    collision_cells.contains(&self.map.cells[self.player.y][self.player.x - 1]);
                let item_coll = self.items.contains_key(&(self.player.x - 1, self.player.y));
                map_coll || item_coll
            }
            "RT" => {
                let map_coll =
                    collision_cells.contains(&self.map.cells[self.player.y][self.player.x + 1]);
                let item_coll = self.items.contains_key(&(self.player.x + 1, self.player.y));
                map_coll || item_coll
            }
            _ => false,
        }
    }

    fn e_collision(&mut self, dir: &str, entity: Enemy) -> bool {
        match dir {
            "UP" => {
                let map_coll =
                    collision_cells.contains(&self.map.cells[entity.pos.1 - 1][entity.pos.0]);
                let item_coll = self.items.contains_key(&(entity.pos.0, entity.pos.1 - 1));
                map_coll || item_coll
            }
            "DN" => {
                let map_coll =
                    collision_cells.contains(&self.map.cells[entity.pos.1 + 1][entity.pos.0]);
                let item_coll = self.items.contains_key(&(entity.pos.0, entity.pos.1 + 1));
                map_coll || item_coll
            }
            "LF" => {
                let map_coll =
                    collision_cells.contains(&self.map.cells[entity.pos.1][entity.pos.0 - 1]);
                let item_coll = self.items.contains_key(&(entity.pos.0 - 1, entity.pos.1));
                map_coll || item_coll
            }
            "RT" => {
                let map_coll =
                    collision_cells.contains(&self.map.cells[entity.pos.1][entity.pos.0 + 1]);
                let item_coll = self.items.contains_key(&(entity.pos.0 + 1, entity.pos.1));
                map_coll || item_coll
            }
            _ => false,
        }
    }

    fn translate_state(&mut self, dx: i16, dy: i16) {
        let titems = self.items.clone();
        let tnpcs = self.npcs.clone();
        let tenemies = self.enemies.clone();
        let tportals = self.portals.clone();
        let mut nitems = HashMap::new();
        let mut nnpcs = HashMap::new();
        let mut nenemies = HashMap::new();
        let mut nportals = HashMap::new();
        for ((x, y), mut i) in titems {
            let npos = ((x as i16 + dx) as usize, (y as i16 + dy) as usize);
            if npos.0 > MAP_W || npos.1 > MAP_H {
                continue;
            }
            i.set_pos(npos);
            nitems.insert(npos, i);
        }
        for ((x, y), n) in tnpcs {
            let npos = ((x as i16 + dx) as usize, (y as i16 + dy) as usize);
            if npos.0 > MAP_W || npos.1 > MAP_H {
                continue;
            }
            let nnpc = shift_npc(n, npos);
            nnpcs.insert(npos, nnpc);
        }
        for ((x, y), mut e) in tenemies {
            let npos = ((x as i16 + dx) as usize, (y as i16 + dy) as usize);
            if npos.0 > MAP_W || npos.1 > MAP_H {
                continue;
            }
            e.set_pos(npos);
            nenemies.insert(npos, e);
        }
        for ((ix, iy), (ox, oy)) in tportals {
            let nin = ((ix as i16 + dx) as usize, (iy as i16 + dy) as usize);
            let nout = ((ox as i16 + dx) as usize, (oy as i16 + dy) as usize);
            nportals.insert(nin, nout);
        }
        self.items = nitems;
        self.npcs = nnpcs;
        self.enemies = nenemies;
        self.portals = nportals;
    }

    fn shift_items(&mut self, dir: &str) {
        let temp_i = self.items.clone();
        let mut new_i = HashMap::new();
        let mw = self.map.cells[0].len();
        let mh = self.map.cells.len();
        for ((x, y), mut i) in temp_i {
            match dir {
                "UP" => {
                    if y < mh {
                        i.y += 1;
                        new_i.insert((x, y + 1), i.clone());
                    }
                }
                "DN" => {
                    if y > 0 {
                        i.y -= 1;
                        new_i.insert((x, y - 1), i.clone());
                    }
                }
                "LF" => {
                    if x < mw {
                        i.x += 1;
                        new_i.insert((x + 1, y), i.clone());
                    }
                }
                "RT" => {
                    if x > 0 {
                        i.x -= 1;
                        new_i.insert((x - 1, y), i.clone());
                    }
                }
                _ => todo!(),
            };
        }
        self.items = new_i;
    }

    fn shift_portals(&mut self, dir: &str) {
        let temp_p = self.portals.clone();
        let mut new_p = HashMap::new();
        let mw = self.map.cells[0].len();
        let mh = self.map.cells.len();
        for ((ix, iy), (ox, oy)) in temp_p {
            match dir {
                "UP" => {
                    if iy < mh && oy < mh {
                        new_p.insert((ix, iy + 1), (ox, oy + 1));
                    }
                }
                "DN" => {
                    if iy > 0 && oy > 0 {
                        new_p.insert((ix, iy - 1), (ox, oy - 1));
                    }
                }
                "LF" => {
                    if ix < mw && ox < mw {
                        new_p.insert((ix + 1, iy), (ox + 1, oy));
                    }
                }
                "RT" => {
                    if ix > 0 && ox > 0 {
                        new_p.insert((ix - 1, iy), (ox - 1, oy));
                    }
                }
                _ => todo!(),
            };
        }
        self.portals = new_p;
    }

    pub fn shift_env_inters(&mut self, dir: &str) {
        let temp_ei = self.env_inters.clone();
        let mut new_i = HashMap::new();
        let mw = self.map.cells[0].len();
        let mh = self.map.cells.len();
        for ((x, y), i) in temp_ei {
            match dir {
                "UP" => {
                    if y < mh {
                        new_i.insert((x, y + 1), i);
                    }
                }
                "DN" => {
                    if y > 0 {
                        new_i.insert((x, y - 1), i);
                    }
                }
                "LF" => {
                    if x < mw {
                        new_i.insert((x + 1, y), i);
                    }
                }
                "RT" => {
                    if x > 0 {
                        new_i.insert((x - 1, y), i);
                    }
                }
                _ => todo!(),
            };
        }
        self.env_inters = new_i;
    }

    fn start_interact(&mut self) {
        let (px, py) = self.player.pos();
        let adj = vec![
            (px, (py as isize - 1) as usize),
            (px, py + 1),
            ((px as isize - 1) as usize, py),
            (px + 1, py),
        ];
        let mut adj_inter = HashMap::new();
        for (x, y) in &adj {
            if let Some(item) = self.items.get(&(*x, *y)) {
                adj_inter.insert((*x, *y), Some(Interactable::Item(item.clone())));
            }
            if let Some(enemy) = self.enemies.get(&(*x, *y)) {
                adj_inter.insert((*x, *y), Some(Interactable::Enemy(enemy.clone())));
            }
            if let Some(npc) = self.npcs.get(&(*x, *y)) {
                adj_inter.insert((*x, *y), Some(Interactable::NPC(npc.clone())));
            }
            if let Some(env_inter) = self.env_inters.get(&(*x, *y)) {
                adj_inter.insert((*x, *y), Some(Interactable::EnvInter(*env_inter)));
            }
            if self.location != Location::Null {
                let st = loc_shop_items(self.dist_fo, self.location.clone());
                if let Some(sitm) = st.get(&(*x, *y)) {
                    adj_inter.insert((*x, *y), Some(Interactable::ShopItem(sitm.clone())));
                }
            }
        }
        if !adj_inter.is_empty() {
            self.game_mode = GameMode::Interact(InterSteps::AdjOpt);
            self.gui.set_info_mode(GUIMode::Interact);
            self.gui.set_interactable(adj_inter);
        }
    }

    fn get_interactee(&mut self, pos: (usize, usize)) -> Option<Interactable> {
        if let Some(item) = self.items.get(&pos) {
            Some(Interactable::Item(item.clone()))
        } else if let Some(sitem) = loc_shop_items(self.dist_fo, self.location.clone()).get(&pos) {
            Some(Interactable::ShopItem(sitem.clone()))
        } else if let Some(enemy) = self.enemies.get(&pos) {
            Some(Interactable::Enemy(enemy.clone()))
        } else if let Some(npc) = self.npcs.get(&pos) {
            Some(Interactable::NPC(npc.clone()))
        } else if let Some(env_inter) = self.env_inters.get(&pos) {
            Some(Interactable::EnvInter(*env_inter))
        } else {
            Some(Interactable::Null)
        }
    }

    fn confirm_equip(&mut self, mut item: Item) -> bool {
        let msg_str = format!("Would you like to equip the {}?", item.get_sname());
        let iopts = "Yes#No".to_string();
        self.gui.reset_cursor();
        loop {
            self.gui.item_use_draw(
                msg_str.clone(),
                iopts.clone(),
                self.map.clone(),
                self.player.clone(),
                self.portals.clone(),
                self.enemies.clone(),
                self.items.clone(),
                self.npcs.clone(),
                loc_shop_items(self.dist_fo, self.location.clone()),
                self.env_inters.clone(),
            );
            if poll(std::time::Duration::from_millis(100)).unwrap() {
                if let Event::Key(event) = read().unwrap() {
                    // log::info!("keykind {:?}", event.kind.clone());
                    let now = Instant::now();
                    if now.duration_since(self.last_event_time) > self.key_debounce_dur {
                        self.last_event_time = now;
                        match event.code {
                            KeyCode::Up => {
                                self.gui.move_cursor("UP");
                            }
                            KeyCode::Down => {
                                self.gui.move_cursor("DN");
                            }
                            KeyCode::Left => {
                                self.gui.move_cursor("LF");
                            }
                            KeyCode::Right => {
                                self.gui.move_cursor("RT");
                            }
                            KeyCode::Char('p') => self.gui.set_info_mode(GUIMode::Bug),
                            KeyCode::Char('o') => self.gui.set_info_mode(GUIMode::Normal),
                            KeyCode::Char('z') => {
                                self.gui.set_info_mode(GUIMode::Normal);
                                self.game_mode = GameMode::Play;
                            }
                            KeyCode::Char('a') => self.gui.move_cursor("LF"),
                            KeyCode::Char('s') => self.gui.move_cursor("UP"),
                            KeyCode::Char('d') => self.gui.move_cursor("DN"),
                            KeyCode::Char('f') => self.gui.move_cursor("RT"),
                            KeyCode::Enter => {
                                return self.gui.get_ysno();
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    fn inv_use_opt(&mut self, mut item: Item) -> ItemOpt {
        let iopts = item.get_iopts();
        //iopts.remove(&ItemOpt::PickUp);
        let msg_str = format!("What would you like to do with the {}?", item.get_sname());

        let mut useable = false;
        let opts_str = if iopts.contains_key(&InterOpt::Item(ItemOpt::Use)) {
            useable = true;
            "Use#Drop#Back".to_string()
        } else {
            "Drop#Back".to_string()
        };

        self.gui.reset_cursor();
        loop {
            self.gui.item_use_draw(
                msg_str.clone(),
                opts_str.clone(),
                self.map.clone(),
                self.player.clone(),
                self.portals.clone(),
                self.enemies.clone(),
                self.items.clone(),
                self.npcs.clone(),
                loc_shop_items(self.dist_fo, self.location.clone()),
                self.env_inters.clone(),
            );
            if poll(std::time::Duration::from_millis(100)).unwrap() {
                if let Event::Key(event) = read().unwrap() {
                    // log::info!("keykind {:?}", event.kind.clone());
                    let now = Instant::now();
                    if now.duration_since(self.last_event_time) > self.key_debounce_dur {
                        self.last_event_time = now;
                        match event.code {
                            KeyCode::Up => {
                                self.gui.move_cursor("UP");
                            }
                            KeyCode::Down => {
                                self.gui.move_cursor("DN");
                            }
                            KeyCode::Left => {
                                self.gui.move_cursor("LF");
                            }
                            KeyCode::Right => {
                                self.gui.move_cursor("RT");
                            }
                            KeyCode::Char('p') => self.gui.set_info_mode(GUIMode::Bug),
                            KeyCode::Char('o') => self.gui.set_info_mode(GUIMode::Normal),
                            KeyCode::Char('z') => {
                                self.gui.set_info_mode(GUIMode::Normal);
                                self.game_mode = GameMode::Play;
                            }
                            KeyCode::Char('a') => self.gui.move_cursor("LF"),
                            KeyCode::Char('s') => self.gui.move_cursor("UP"),
                            KeyCode::Char('d') => self.gui.move_cursor("DN"),
                            KeyCode::Char('f') => self.gui.move_cursor("RT"),
                            KeyCode::Enter => {
                                let res = self.gui.get_cursor();
                                return match res.0 {
                                    0 if useable => ItemOpt::Use,
                                    0 if !useable => ItemOpt::Drp,
                                    1 if useable => ItemOpt::Drp,
                                    1 if !useable => ItemOpt::Null,
                                    2 if useable => ItemOpt::Null,
                                    _ => todo!(),
                                };
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    fn use_inv_item(&mut self) {
        let (idx, mut item) = self.gui.get_inv_opt();
        //gui, using item

        if item.is_equip() {
            if self.confirm_equip(item.clone()) {
                self.player.add_equip(item.clone());
            }
            return;
        } else {
            match self.inv_use_opt(item.clone()) {
                ItemOpt::Use => {
                    self.player.apply_item_effect(item.clone());
                    self.player.rem_inv_item(idx);
                    self.gui.set_inventory(self.player.get_inventory());
                }
                ItemOpt::Drp => {
                    self.player.rem_inv_item(idx);
                    self.gui.set_inventory(self.player.get_inventory());
                }
                ItemOpt::Null => {
                    return;
                }
                _ => todo!(),
            }
        }
        self.gui.reset_cursor();
        match self.game_mode {
            GameMode::Play => {
                loop {
                    self.gui.item_used_draw(
                        self.map.clone(),
                        self.player.clone(),
                        self.portals.clone(),
                        self.enemies.clone(),
                        self.items.clone(),
                        self.npcs.clone(),
                        loc_shop_items(self.dist_fo, self.location.clone()),
                        self.env_inters.clone(),
                    );
                    if poll(std::time::Duration::from_millis(100)).unwrap() {
                        if let Event::Key(event) = read().unwrap() {
                            // log::info!("keykind {:?}", event.kind.clone());
                            let now = Instant::now();
                            if now.duration_since(self.last_event_time) > self.key_debounce_dur {
                                self.last_event_time = now;
                                match event.code {
                                    KeyCode::Enter => {
                                        break;
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                }
            }
            GameMode::Fight(_) => {
                let itstr = format!("You used the {}", item.clone().get_sname());
                loop {
                    self.gui.encounter_show_content(
                        itstr.clone(),
                        self.map.clone(),
                        self.player.clone(),
                        self.portals.clone(),
                        self.enemies.clone(),
                        self.items.clone(),
                        self.npcs.clone(),
                        loc_shop_items(self.dist_fo, self.location.clone()),
                        self.env_inters.clone(),
                    );
                    if poll(std::time::Duration::from_millis(100)).unwrap() {
                        if let Event::Key(event) = read().unwrap() {
                            // log::info!("keykind {:?}", event.kind.clone());
                            let now = Instant::now();
                            if now.duration_since(self.last_event_time) > self.key_debounce_dur {
                                self.last_event_time = now;
                                match event.code {
                                    KeyCode::Enter => {
                                        break;
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }

    fn set_comp(&mut self) {
        //let curs = self.gui.get_cursor();
        let copt = self.gui.get_comp_opt();
        if copt == "Search" {
            self.comp_mode = CompMode::Search;
            return;
        } else {
            self.comp_mode = CompMode::Location;
        }
        let comp_pos = self
            .comp_list
            .clone()
            .into_iter()
            .find_map(|(pos, name)| {
                // log::info!("copt: {:?}, name: {:?}, pos: {:?}", copt, name, pos);
                if name == copt {
                    Some(pos)
                } else {
                    Some((0, 0))
                }
            })
            .unwrap();
        // log::info!("compass: {:?}", comp_pos);
        self.comp_head = comp_pos;
        // let comp_pos = {
        //     for (pos, name) in self.comp_list.clone() {
        //         match name {
        //             copt => pos,
        //             _ -> todo!(),
        //         }
        //     }
        // };
    }

    fn play_key(&mut self, code: KeyCode) -> bool {
        match code {
            KeyCode::Up => {
                if self.collision("UP") {
                } else {
                    if self.player.y - 1 <= self.map.viewport_y + (self.map.viewport_height / 7) {
                        self.shift_enemies("UP");
                        self.shift_items("UP");
                        self.shift_npcs("UP");
                        self.shift_env_inters("UP");
                        self.shift_portals("UP");
                        self.map.shift("UP");
                        self.dist_fo.1 += 1;
                        self.gui.set_comp_head((
                            self.comp_head.0 - self.dist_fo.0 * -1,
                            self.comp_head.1 - self.dist_fo.1 * -1,
                        ));
                    } else {
                        self.player.y -= 1;
                    }
                }
            }
            KeyCode::Down => {
                if self.collision("DN") {
                } else {
                    if self.player.y + 1
                        >= (self.map.viewport_height + self.map.viewport_y)
                            - (self.map.viewport_height / 7)
                    {
                        self.shift_enemies("DN");
                        self.shift_items("DN");
                        self.shift_npcs("DN");
                        self.shift_env_inters("DN");
                        self.shift_portals("DN");
                        self.map.shift("DN");
                        self.dist_fo.1 -= 1;
                        self.gui.set_comp_head((
                            self.comp_head.0 - self.dist_fo.0 * -1,
                            self.comp_head.1 - self.dist_fo.1 * -1,
                        ));
                    } else {
                        self.player.y += 1;
                    }
                }
            }
            KeyCode::Left => {
                if self.collision("LF") {
                } else {
                    if self.player.x - 1 <= self.map.viewport_x + (self.map.viewport_width / 7) {
                        self.shift_enemies("LF");
                        self.shift_items("LF");
                        self.shift_npcs("LF");
                        self.shift_env_inters("LF");
                        self.shift_portals("LF");
                        self.map.shift("LF");
                        self.dist_fo.0 += 1;
                        self.gui.set_comp_head((
                            self.comp_head.0 - self.dist_fo.0 * -1,
                            self.comp_head.1 - self.dist_fo.1 * -1,
                        ));
                    } else {
                        self.player.x -= 1;
                    }
                }
            }
            KeyCode::Right => {
                if self.collision("RT") {
                } else {
                    if self.player.x + 1
                        >= (self.map.viewport_width + self.map.viewport_x)
                            - (self.map.viewport_width / 7)
                    {
                        self.shift_enemies("RT");
                        self.shift_items("RT");
                        self.shift_npcs("RT");
                        self.shift_env_inters("RT");
                        self.shift_portals("RT");
                        self.map.shift("RT");
                        self.dist_fo.0 -= 1;
                        self.gui.set_comp_head((
                            self.comp_head.0 - self.dist_fo.0 * -1,
                            self.comp_head.1 - self.dist_fo.1 * -1,
                        ));
                    } else {
                        self.player.x += 1;
                    }
                }
            }
            KeyCode::Char('h') => self.gui.toggle_help(),
            KeyCode::Char('p') => self.gui.set_info_mode(GUIMode::Bug),
            KeyCode::Char('o') => self.gui.set_info_mode(GUIMode::Normal),
            KeyCode::Char('q') => self.gui.set_info_mode(GUIMode::Normal),
            KeyCode::Char('w') => {
                self.gui.set_info_mode(GUIMode::Map);
                //self.gui.set_comp_head(self.comp_head);
                self.gui.set_comp_list(self.comp_list.clone());
                self.gui.set_comp_head((
                    self.comp_head.0 - self.dist_fo.0 * -1,
                    self.comp_head.1 - self.dist_fo.1 * -1,
                ));
            }
            KeyCode::Char('e') => {
                self.gui.set_info_mode(GUIMode::Inventory);
                self.gui.set_inventory(self.player.get_inventory());
                self.gui.reset_cursor();
            }
            KeyCode::Char('r') => {
                self.gui.set_info_mode(GUIMode::Notes);
                self.gui.set_notes(self.notebook.get_active_notes());
            }
            KeyCode::Char('a') => self.gui.move_cursor("LF"),
            KeyCode::Char('s') => self.gui.move_cursor("UP"),
            KeyCode::Char('d') => self.gui.move_cursor("DN"),
            KeyCode::Char('f') => self.gui.move_cursor("RT"),
            KeyCode::Char(' ') => self.start_interact(),
            KeyCode::Enter => {
                let gmode = self.gui.get_mode();
                match gmode {
                    GUIMode::Normal => {}
                    GUIMode::Inventory => {
                        //put use_opts here, use_inv_item in opts
                        self.use_inv_item();
                    }
                    GUIMode::Map => {
                        self.set_comp();
                        self.gui.set_comp_list(self.comp_list.clone());
                    }
                    GUIMode::Notes => {
                        self.gui.menu_lvl("DN");
                    }
                    _ => {}
                }
            }
            KeyCode::Backspace => {
                let gmode = self.gui.get_mode();
                match gmode {
                    GUIMode::Normal => {}
                    GUIMode::Inventory => {}
                    GUIMode::Map => {}
                    GUIMode::Notes => {
                        self.gui.menu_lvl("UP");
                    }
                    _ => {}
                }
            }
            KeyCode::Esc => return false,
            _ => {}
        }
        true
    }

    fn select_adj(&mut self) {
        let (pos, st) = self.gui.get_interactee();
        let Some(intee) = self.get_interactee(pos) else {
            todo!()
        };
        self.interactee = intee.clone();
        match intee {
            Interactable::Item(item) => {
                // self.item_opt(item.clone());
                self.gui.set_inter_opt(item.iopts);
            }
            Interactable::ShopItem(sitem) => {}
            Interactable::Enemy(enemy) => {}
            Interactable::NPC(npc) => {}
            Interactable::EnvInter(env_inter) => {}
            Interactable::Null => {}
            _ => todo!(),
        }
    }

    fn pickup_item(&mut self, item: Item) {
        self.player.add_to_inv(item.clone());
        if let Some(itm) = self.items.remove(&(item.x, item.y)) {}
    }

    fn select_opt(&mut self) {
        let (opt, _) = self.gui.get_iopt();
        match opt {
            InterOpt::Item(item_opt) => match item_opt {
                ItemOpt::PickUp => {
                    let Interactable::Item(item) = self.interactee.clone() else {
                        todo!()
                    };
                    self.pickup_item(item);
                }
                ItemOpt::Drp => {}
                ItemOpt::Use => {}
                _ => todo!(),
            },
            _ => todo!(),
        }
    }

    fn enc_key(&mut self, code: KeyCode) -> bool {
        match code {
            KeyCode::Up => {
                self.gui.move_cursor("UP");
            }
            KeyCode::Down => {
                self.gui.move_cursor("DN");
            }
            KeyCode::Left => {
                self.gui.move_cursor("LF");
            }
            KeyCode::Right => {
                self.gui.move_cursor("RT");
            }
            KeyCode::Char('p') => self.gui.set_info_mode(GUIMode::Bug),
            KeyCode::Char('o') => self.gui.set_info_mode(GUIMode::Normal),
            KeyCode::Char('z') => {
                self.gui.set_info_mode(GUIMode::Normal);
                self.game_mode = GameMode::Play;
            }
            KeyCode::Char('a') => self.gui.move_cursor("LF"),
            KeyCode::Char('s') => self.gui.move_cursor("UP"),
            KeyCode::Char('d') => self.gui.move_cursor("DN"),
            KeyCode::Char('f') => self.gui.move_cursor("RT"),
            KeyCode::Enter => {
                match self.game_mode {
                    GameMode::Fight(FightSteps::Open) => {
                        // self.select_adj();
                        // self.game_mode = GameMode::Fight(FightSteps::);
                    }
                    GameMode::Fight(FightSteps::Enemy) => {
                        // self.select_opt();
                        // self.game_mode = GameMode::Fight(FightSteps::Player);
                    }
                    GameMode::Fight(FightSteps::Player) => {
                        // self.select_opt();
                        // self.game_mode = GameMode::Fight(FightSteps::Enemy);
                        // let opt = self.enc.clone();
                        let opt = self.gui.get_enc_opt();
                        self.enc = opt.0.clone();
                        // log::info!("opt1 {:?}", opt.clone());
                        match opt.0 {
                            _ => self.enc_option(),
                            // _ => {},
                        }
                    }
                    GameMode::Fight(FightSteps::Message) => {
                        // self.select_adj();
                        // self.game_mode = GameMode::Play;
                    }
                    _ => {}
                }

                return false;
            }
            KeyCode::Esc => {
                self.game_mode = GameMode::Play;
                return false;
            }
            _ => {}
        }
        true
    }

    fn comm_key(&mut self, code: KeyCode) -> bool {
        match code {
            KeyCode::Up => {
                self.gui.move_cursor("UP");
            }
            KeyCode::Down => {
                self.gui.move_cursor("DN");
            }
            KeyCode::Left => {
                self.gui.move_cursor("LF");
            }
            KeyCode::Right => {
                self.gui.move_cursor("RT");
            }
            KeyCode::Char('p') => self.gui.set_info_mode(GUIMode::Bug),
            KeyCode::Char('o') => self.gui.set_info_mode(GUIMode::Normal),
            KeyCode::Char('z') => {
                self.gui.set_info_mode(GUIMode::Normal);
                self.game_mode = GameMode::Play;
            }
            KeyCode::Char('a') => self.gui.move_cursor("LF"),
            KeyCode::Char('s') => self.gui.move_cursor("UP"),
            KeyCode::Char('d') => self.gui.move_cursor("DN"),
            KeyCode::Char('f') => self.gui.move_cursor("RT"),
            KeyCode::Enter => {
                self.game_mode = GameMode::Play;
                self.gui.set_info_mode(GUIMode::Normal);

                return false;
            }
            // KeyCode::Esc => return false,
            _ => {}
        }
        true
    }

    fn inter_key(&mut self, code: KeyCode) -> bool {
        match code {
            KeyCode::Up => {
                self.gui.move_cursor("UP");
            }
            KeyCode::Down => {
                self.gui.move_cursor("DN");
            }
            KeyCode::Left => {
                self.gui.move_cursor("LF");
            }
            KeyCode::Right => {
                self.gui.move_cursor("RT");
            }
            KeyCode::Char('p') => self.gui.set_info_mode(GUIMode::Bug),
            KeyCode::Char('o') => self.gui.set_info_mode(GUIMode::Normal),
            KeyCode::Char('z') => {
                self.gui.set_info_mode(GUIMode::Normal);
                self.game_mode = GameMode::Play;
            }
            KeyCode::Char('a') => self.gui.move_cursor("LF"),
            KeyCode::Char('s') => self.gui.move_cursor("UP"),
            KeyCode::Char('d') => self.gui.move_cursor("DN"),
            KeyCode::Char('f') => self.gui.move_cursor("RT"),
            KeyCode::Enter => {
                match self.game_mode {
                    GameMode::Interact(InterSteps::AdjOpt) => {
                        self.select_adj();
                        self.game_mode = GameMode::Interact(InterSteps::IntOpt);
                    }
                    GameMode::Interact(InterSteps::IntOpt) => {
                        self.select_opt();
                        self.game_mode = GameMode::Interact(InterSteps::Feedback);
                    }
                    GameMode::Interact(InterSteps::Feedback) => {
                        self.game_mode = GameMode::Play;
                    }
                    _ => self.game_mode = GameMode::Play,
                }

                return false;
            }
            KeyCode::Esc => return false,
            _ => {}
        }
        true
    }

    fn play_update(&mut self) -> bool {
        if poll(std::time::Duration::from_millis(5)).unwrap() {
            if let Event::Key(event) = read().unwrap() {
                // log::info!("keykind {:?}", event.kind.clone());
                let now = Instant::now();
                if now.duration_since(self.last_event_time) > self.key_debounce_dur {
                    self.last_event_time = now;
                    self.play_key(event.code)
                } else {
                    true
                }
            } else {
                true
            }
        } else {
            true
        }
    }

    fn item_interaction(&mut self) -> bool {
        self.gui.reset_cursor();
        loop {
            self.gui.inter_opt_draw(
                self.map.clone(),
                self.player.clone(),
                self.portals.clone(),
                self.enemies.clone(),
                self.items.clone(),
                self.npcs.clone(),
                loc_shop_items(self.dist_fo, self.location.clone()),
                self.env_inters.clone(),
            );
            if poll(std::time::Duration::from_millis(100)).unwrap() {
                if let Event::Key(event) = read().unwrap() {
                    // log::info!("keykind {:?}", event.kind.clone());
                    let now = Instant::now();
                    if now.duration_since(self.last_event_time) > self.key_debounce_dur {
                        self.last_event_time = now;
                        let res = self.inter_key(event.code);
                        if !res {
                            break;
                        }
                    }
                }
            }
        }
        self.gui.reset_cursor();
        loop {
            self.gui.inter_res_draw(
                self.map.clone(),
                self.player.clone(),
                self.portals.clone(),
                self.enemies.clone(),
                self.items.clone(),
                self.npcs.clone(),
                loc_shop_items(self.dist_fo, self.location.clone()),
                self.env_inters.clone(),
            );
            if poll(std::time::Duration::from_millis(100)).unwrap() {
                if let Event::Key(event) = read().unwrap() {
                    // log::info!("keykind {:?}", event.kind.clone());
                    let now = Instant::now();
                    if now.duration_since(self.last_event_time) > self.key_debounce_dur {
                        self.last_event_time = now;
                        let res = self.inter_key(event.code);
                        if !res {
                            break;
                        }
                    }
                }
            }
        }
        self.gui.reset_cursor();
        // self.gui.set_info_mode(GUIMode::Normal);
        true
    }

    fn get_shop_from_item(&mut self, mut item: Item) -> Shop {
        let ipos = item.get_pos();
        // log::info!("shop item \n{:?}", item.clone());
        match self.location.clone() {
            Location::Settlement(mut settle) => {
                if let Some(shop) = settle.get_shop_from_item_pos((
                    ipos.0 as i64 - self.dist_fo.0,
                    ipos.1 as i64 - self.dist_fo.1,
                )) {
                    shop
                } else {
                    Shop::default()
                }
            }
            _ => todo!(),
        }
    }

    fn get_cur_settle_name(&mut self) -> String {
        let loc = self.location.clone();
        match loc {
            Location::Settlement(mut settle) => settle.get_sname().replace(" ", "_"),
            _ => "oops".to_string(),
        }
    }

    fn game_save(&mut self, filename: String) -> bool {
        let path = format!("src/save/{}", filename);
        //let json = serde_json::to_string(self).expect("failed to save 1");
        let json = r#"{
            "name": "gamestate",
            "works": "yes"
        }"#;
        //let mut file = std::fs::File::create(path).expect("failed to save 2");
        //fs::remove_file(path.clone()).expect("failed to delete previous save");
        let mut sfile = OpenOptions::new()
            .write(true)
            .create(true)
            .open(path)
            .expect("failed to open save file");
        sfile.write_all(json.as_bytes());
        let fn_list_path = "src/save/index.txt";
        let mut nfile = OpenOptions::new()
            .append(true)
            .open(fn_list_path)
            .expect("failed to save fname");
        writeln!(nfile, "{}", filename.clone());
        self.game_mode = GameMode::Play;
        true
    }

    fn save_game(&mut self) -> bool {
        let file = File::open("src/save/index.txt").expect("failed to open savelist");
        let reader = BufReader::new(file);
        let mut savelist = Vec::new();

        for line in reader.lines() {
            let line = line.unwrap_or("ERR".to_string());
            savelist.push(line);
        }
        let save_str = if savelist.len() == 0 {
            "It seems you dont have any logs, would you like to log your progress?".to_string()
        } else {
            savelist.push("new save".to_string());
            "Would you like to overwrite a previous log?".to_string()
        };

        self.gui.reset_cursor();
        loop {
            self.gui.guild_records_draw(
                save_str.clone(),
                savelist.clone(),
                self.map.clone(),
                self.player.clone(),
                self.portals.clone(),
                self.enemies.clone(),
                self.items.clone(),
                self.npcs.clone(),
                loc_shop_items(self.dist_fo, self.location.clone()),
                self.env_inters.clone(),
            );
            if poll(std::time::Duration::from_millis(100)).unwrap() {
                if let Event::Key(event) = read().unwrap() {
                    // log::info!("keykind {:?}", event.kind.clone());
                    let now = Instant::now();
                    if now.duration_since(self.last_event_time) > self.key_debounce_dur {
                        self.last_event_time = now;
                        match event.code {
                            KeyCode::Up => {
                                self.gui.move_cursor("UP");
                            }
                            KeyCode::Down => {
                                self.gui.move_cursor("DN");
                            }
                            KeyCode::Left => {
                                self.gui.move_cursor("LF");
                            }
                            KeyCode::Right => {
                                self.gui.move_cursor("RT");
                            }
                            KeyCode::Char('a') => self.gui.move_cursor("LF"),
                            KeyCode::Char('s') => self.gui.move_cursor("UP"),
                            KeyCode::Char('d') => self.gui.move_cursor("DN"),
                            KeyCode::Char('f') => self.gui.move_cursor("RT"),
                            KeyCode::Enter => match savelist.len() {
                                0 => {
                                    if self.gui.get_ysno() {
                                        let settle_name = self.get_cur_settle_name();
                                        let save_name = format!("save_1_{}.json", settle_name);
                                        if self.game_save(save_name) {
                                            return true;
                                        }
                                    } else {
                                        self.game_mode = GameMode::Play;
                                        return true;
                                    };
                                }
                                _ => {
                                    let choice = self.gui.get_cursor();
                                    if choice.0 == savelist.len() - 1 {
                                        let settle_name = self.get_cur_settle_name();
                                        let save_name = format!("save_1_{}.json", settle_name);
                                        if self.game_save(save_name) {
                                            return true;
                                        }
                                    } else {
                                        let save_name = savelist[choice.0].clone();
                                        let sn_parts: Vec<&str> = save_name.split("_").collect();
                                        let settle_name = self.get_cur_settle_name();
                                        let new_save_name =
                                            format!("save_{}_{}.json", sn_parts[1], settle_name);
                                        if self.game_save(new_save_name) {
                                            return true;
                                        }
                                    };
                                }
                            },
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    fn clinic(&mut self) -> bool {
        let mut paid = false;
        self.gui.reset_cursor();
        loop {
            self.gui.clinic_draw(
                self.map.clone(),
                self.player.clone(),
                self.portals.clone(),
                self.enemies.clone(),
                self.items.clone(),
                self.npcs.clone(),
                loc_shop_items(self.dist_fo, self.location.clone()),
                self.env_inters.clone(),
            );
            if poll(std::time::Duration::from_millis(100)).unwrap() {
                if let Event::Key(event) = read().unwrap() {
                    // log::info!("keykind {:?}", event.kind.clone());
                    let now = Instant::now();
                    if now.duration_since(self.last_event_time) > self.key_debounce_dur {
                        self.last_event_time = now;
                        match event.code {
                            KeyCode::Up => {
                                self.gui.move_cursor("UP");
                            }
                            KeyCode::Down => {
                                self.gui.move_cursor("DN");
                            }
                            KeyCode::Left => {
                                self.gui.move_cursor("LF");
                            }
                            KeyCode::Right => {
                                self.gui.move_cursor("RT");
                            }
                            KeyCode::Char('a') => self.gui.move_cursor("LF"),
                            KeyCode::Char('s') => self.gui.move_cursor("UP"),
                            KeyCode::Char('d') => self.gui.move_cursor("DN"),
                            KeyCode::Char('f') => self.gui.move_cursor("RT"),
                            KeyCode::Enter => {
                                if self.gui.get_ysno() {
                                    if self.player.dec_money(20) {
                                        self.player.heal_player();
                                        paid = true;
                                        break;
                                    }
                                    break;
                                }
                                self.game_mode = GameMode::Play;
                                return true;
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        let resp_string = if paid {
            "Thanks for coming by! Hope you're feeling better!".to_string()
        } else {
            "It seems that you dont have enough money, sorry about that. Medical supplies are scarce down here and we cant do it for free.".to_string()
        };

        self.gui.reset_cursor();
        loop {
            self.gui.clinic_resp_draw(
                resp_string.clone(),
                self.map.clone(),
                self.player.clone(),
                self.portals.clone(),
                self.enemies.clone(),
                self.items.clone(),
                self.npcs.clone(),
                loc_shop_items(self.dist_fo, self.location.clone()),
                self.env_inters.clone(),
            );
            if poll(std::time::Duration::from_millis(100)).unwrap() {
                if let Event::Key(event) = read().unwrap() {
                    // log::info!("keykind {:?}", event.kind.clone());
                    let now = Instant::now();
                    if now.duration_since(self.last_event_time) > self.key_debounce_dur {
                        self.last_event_time = now;
                        match event.code {
                            KeyCode::Up => {
                                self.gui.move_cursor("UP");
                            }
                            KeyCode::Down => {
                                self.gui.move_cursor("DN");
                            }
                            KeyCode::Left => {
                                self.gui.move_cursor("LF");
                            }
                            KeyCode::Right => {
                                self.gui.move_cursor("RT");
                            }
                            KeyCode::Char('a') => self.gui.move_cursor("LF"),
                            KeyCode::Char('s') => self.gui.move_cursor("UP"),
                            KeyCode::Char('d') => self.gui.move_cursor("DN"),
                            KeyCode::Char('f') => self.gui.move_cursor("RT"),
                            KeyCode::Enter => {
                                break;
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
        self.game_mode = GameMode::Play;
        true
    }

    fn guild_post(&mut self) -> bool {
        let local_puzzles = self.puzzles.get_local_puzzles(self.dist_fo);
        let mut ppost_strings = Vec::new();
        let pos = self.dist_fo;
        for (ppos, mut p) in local_puzzles {
            let dx = ppos.0 + pos.0;
            let dy = ppos.1 + pos.1;
            let xdir = dx / dx.abs();
            let ydir = dy / dy.abs();
            let dir_string = match (xdir, ydir) {
                (xdir, ydir) if xdir <= 0 && ydir <= 0 => "North East".to_string(),
                (xdir, ydir) if xdir <= 0 && ydir > 0 => "South East".to_string(),
                (xdir, ydir) if xdir > 0 && ydir <= 0 => "North West".to_string(),
                (xdir, ydir) if xdir > 0 && ydir > 0 => "South West".to_string(),
                _ => "dunno".to_string(),
            };
            let ptype_string = {
                match p.get_ptype() {
                    PuzzleType::Maze => "Maze".to_string(),
                    PuzzleType::Teleport => "Teleport".to_string(),
                    PuzzleType::Inverted => "Inverted".to_string(),
                    _ => "dunno".to_string(),
                }
            };
            let f_str = format!("{}#{}", ptype_string, dir_string);
            ppost_strings.push(f_str.clone());
        }
        self.gui.reset_cursor();
        loop {
            self.gui.guild_post_draw(
                ppost_strings.clone(),
                self.map.clone(),
                self.player.clone(),
                self.portals.clone(),
                self.enemies.clone(),
                self.items.clone(),
                self.npcs.clone(),
                loc_shop_items(self.dist_fo, self.location.clone()),
                self.env_inters.clone(),
            );
            if poll(std::time::Duration::from_millis(100)).unwrap() {
                if let Event::Key(event) = read().unwrap() {
                    // log::info!("keykind {:?}", event.kind.clone());
                    let now = Instant::now();
                    if now.duration_since(self.last_event_time) > self.key_debounce_dur {
                        self.last_event_time = now;
                        match event.code {
                            KeyCode::Up => {
                                self.gui.move_cursor("UP");
                            }
                            KeyCode::Down => {
                                self.gui.move_cursor("DN");
                            }
                            KeyCode::Left => {
                                self.gui.move_cursor("LF");
                            }
                            KeyCode::Right => {
                                self.gui.move_cursor("RT");
                            }
                            KeyCode::Char('a') => self.gui.move_cursor("LF"),
                            KeyCode::Char('s') => self.gui.move_cursor("UP"),
                            KeyCode::Char('d') => self.gui.move_cursor("DN"),
                            KeyCode::Char('f') => self.gui.move_cursor("RT"),
                            KeyCode::Enter => {
                                break;
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
        self.game_mode = GameMode::Play;
        true
    }

    fn church_post(&mut self) -> bool {
        let pos = self.dist_fo;
        let local_settles = self.settles.get_local_settles(pos);
        let mut settles = Vec::new();
        let loc_pos = self.location_pos();
        for (spos, mut s) in local_settles {
            if spos.0 != loc_pos.0 && spos.1 != loc_pos.1 {
                let dx = spos.0 + pos.0;
                let dy = spos.1 + pos.1;
                let xdir = dx / dx.abs();
                let ydir = dy / dy.abs();
                let dir_string = match (xdir, ydir) {
                    (xdir, ydir) if xdir <= 0 && ydir <= 0 => "North East".to_string(),
                    (xdir, ydir) if xdir <= 0 && ydir > 0 => "South East".to_string(),
                    (xdir, ydir) if xdir > 0 && ydir <= 0 => "North West".to_string(),
                    (xdir, ydir) if xdir > 0 && ydir > 0 => "South West".to_string(),
                    _ => "dunno".to_string(),
                };
                let settle_name = s.get_sname();
                let settle_str = format!("{}#{}", settle_name.clone(), dir_string.clone());
                settles.push(settle_str);
            }
        }

        self.gui.reset_cursor();
        loop {
            self.gui.church_post_draw(
                settles.clone(),
                self.map.clone(),
                self.player.clone(),
                self.portals.clone(),
                self.enemies.clone(),
                self.items.clone(),
                self.npcs.clone(),
                loc_shop_items(self.dist_fo, self.location.clone()),
                self.env_inters.clone(),
            );
            if poll(std::time::Duration::from_millis(100)).unwrap() {
                if let Event::Key(event) = read().unwrap() {
                    // log::info!("keykind {:?}", event.kind.clone());
                    let now = Instant::now();
                    if now.duration_since(self.last_event_time) > self.key_debounce_dur {
                        self.last_event_time = now;
                        match event.code {
                            KeyCode::Up => {
                                self.gui.move_cursor("UP");
                            }
                            KeyCode::Down => {
                                self.gui.move_cursor("DN");
                            }
                            KeyCode::Left => {
                                self.gui.move_cursor("LF");
                            }
                            KeyCode::Right => {
                                self.gui.move_cursor("RT");
                            }
                            KeyCode::Char('a') => self.gui.move_cursor("LF"),
                            KeyCode::Char('s') => self.gui.move_cursor("UP"),
                            KeyCode::Char('d') => self.gui.move_cursor("DN"),
                            KeyCode::Char('f') => self.gui.move_cursor("RT"),
                            KeyCode::Enter => {
                                break;
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
        self.game_mode = GameMode::Play;
        true
    }

    fn env_interaction(&mut self, env_inter: EnvInter) -> bool {
        match env_inter {
            EnvInter::Records => self.save_game(),
            EnvInter::Clinic => self.clinic(),
            EnvInter::GuildPost => self.guild_post(),
            EnvInter::ChurchPost => self.church_post(),
            _ => todo!(),
        }
    }

    fn interaction(&mut self) -> bool {
        self.gui.reset_cursor();
        loop {
            self.gui.inter_adj_draw(
                self.map.clone(),
                self.player.clone(),
                self.portals.clone(),
                self.enemies.clone(),
                self.items.clone(),
                self.npcs.clone(),
                loc_shop_items(self.dist_fo, self.location.clone()),
                self.env_inters.clone(),
            );
            if poll(std::time::Duration::from_millis(100)).unwrap() {
                if let Event::Key(event) = read().unwrap() {
                    // log::info!("keykind {:?}", event.kind.clone());
                    let now = Instant::now();
                    if now.duration_since(self.last_event_time) > self.key_debounce_dur {
                        self.last_event_time = now;
                        let res = self.inter_key(event.code);
                        if !res {
                            break;
                        }
                    }
                }
            }
        }

        let intee = self.interactee.clone();
        let res = match intee {
            Interactable::Item(_) => self.item_interaction(),
            Interactable::ShopItem(si) => self.shop_item_interaction(si),
            Interactable::NPC(_) => self.npc_interaction(),
            Interactable::Enemy(e) => {
                self.game_mode = GameMode::Fight(FightSteps::Open);
                self.enemy_encounter(e)
            }
            Interactable::EnvInter(env_inter) => self.env_interaction(env_inter),
            Interactable::Null => false,
            _ => todo!(),
        };
        self.gui.set_info_mode(GUIMode::Normal);
        if !res {
            return false;
        }
        true
    }

    fn check_place_item(&mut self, x: usize, y: usize) -> bool {
        let mut rng = rand::thread_rng();
        let types = vec![
            Items::Rock,
            Items::EdibleRoot,
            Items::Apple,
            Items::MetalScrap,
        ];
        if self.map.cells[y][x] == Cells::Empty
            && !self.in_loc_check((x, y))
            && !self.enemies.contains_key(&(x, y))
            && !self.items.contains_key(&(x, y))
        {
            if let Some(i_type) = types.choose(&mut rng) {
                match i_type {
                    Items::EdibleRoot => {
                        self.items.insert((x, y), Item::new_edible_root(x, y));
                    }
                    Items::Apple => {
                        self.items.insert((x, y), Item::new_apple(x, y));
                    }
                    Items::MetalScrap => {
                        self.items.insert((x, y), Item::new_metal_scrap(x, y));
                    }
                    Items::Rock => {
                        self.items.insert((x, y), Item::new_rock(x, y));
                    }
                    _ => todo!(),
                };
                return true;
            }
        }
        false
    }

    fn repop_items(&mut self) {
        let mut rng = rand::thread_rng();
        let (vx, vy, vw, vh) = self.map.get_viewport();
        //xx
        match (-self.map.gen_x, -self.map.gen_y) {
            (x, y) if x < 0 && y == 0 => {
                for _ in 0..30 {
                    loop {
                        let x = rng.gen_range(10..vx - 5);
                        let y = rng.gen_range(10..MAP_H - 10);
                        let res = self.check_place_item(x, y);
                        if res {
                            break;
                        }
                    }
                }
            }
            (x, y) if x > 0 && y == 0 => {
                for _ in 0..30 {
                    loop {
                        let x = rng.gen_range((vx + vw + 5)..MAP_W - 10);
                        let y = rng.gen_range(10..MAP_H - 10);
                        let res = self.check_place_item(x, y);
                        if res {
                            break;
                        }
                    }
                }
            }
            (x, y) if y < 0 && x == 0 => {
                for _ in 0..30 {
                    loop {
                        let x = rng.gen_range(10..MAP_W - 10);
                        let y = rng.gen_range(10..vy - 5);
                        let res = self.check_place_item(x, y);
                        if res {
                            break;
                        }
                    }
                }
            }
            (x, y) if y > 0 && x == 0 => {
                for _ in 0..30 {
                    loop {
                        let x = rng.gen_range(10..MAP_W - 10);
                        let y = rng.gen_range((vy + vh + 5)..MAP_H - 10);
                        let res = self.check_place_item(x, y);
                        if res {
                            break;
                        }
                    }
                }
            } // asdf
            (x, y) if x > 0 && y > 0 => {
                for _ in 0..15 {
                    loop {
                        let x = rng.gen_range((vx + vw + 5)..MAP_W - 10);
                        let y = rng.gen_range(10..MAP_H - 10);
                        let res = self.check_place_item(x, y);
                        if res {
                            break;
                        }
                    }
                }
                for _ in 0..15 {
                    loop {
                        let x = rng.gen_range(10..MAP_W - 10);
                        let y = rng.gen_range((vy + vh + 5)..MAP_H - 10);
                        let res = self.check_place_item(x, y);
                        if res {
                            break;
                        }
                    }
                }
            }
            (x, y) if x > 0 && y < 0 => {
                for _ in 0..15 {
                    loop {
                        let x = rng.gen_range((vx + vw + 5)..MAP_W - 10);
                        let y = rng.gen_range(10..MAP_H - 10);
                        let res = self.check_place_item(x, y);
                        if res {
                            break;
                        }
                    }
                }
                for _ in 0..15 {
                    loop {
                        let x = rng.gen_range(10..MAP_W - 10);
                        let y = rng.gen_range(10..vy - 5);
                        let res = self.check_place_item(x, y);
                        if res {
                            break;
                        }
                    }
                }
            }
            (x, y) if x < 0 && y > 0 => {
                for _ in 0..15 {
                    loop {
                        let x = rng.gen_range(10..vx - 5);
                        let y = rng.gen_range(10..MAP_H - 10);
                        let res = self.check_place_item(x, y);
                        if res {
                            break;
                        }
                    }
                }
                for _ in 0..15 {
                    loop {
                        let x = rng.gen_range(10..MAP_W - 10);
                        let y = rng.gen_range((vy + vh + 5)..MAP_H - 10);
                        let res = self.check_place_item(x, y);
                        if res {
                            break;
                        }
                    }
                }
            }
            (x, y) if x < 0 && y < 0 => {
                for _ in 0..15 {
                    loop {
                        let x = rng.gen_range(10..vx - 5);
                        let y = rng.gen_range(10..MAP_H - 10);
                        let res = self.check_place_item(x, y);
                        if res {
                            break;
                        }
                    }
                }
                for _ in 0..15 {
                    loop {
                        let x = rng.gen_range(10..MAP_W - 10);
                        let y = rng.gen_range(10..vy - 5);
                        let res = self.check_place_item(x, y);
                        if res {
                            break;
                        }
                    }
                }
            }
            _ => {}
        }
    }

    pub fn start_update_threads(game_state: Arc<Mutex<Self>>) {
        // let game_state = Arc::new(Mutex::new(self));

        let game_clone = Arc::clone(&game_state);
        thread::spawn(move || {
            loop {
                // log::info!("update npc pre");
                {
                    let mut game = game_clone.lock().unwrap();
                    // game.update_npcs();
                    // log::info!("update npc");
                    let step = game.step_group;
                    if game.game_mode == GameMode::Play {
                        game.update_enemies(step);
                        game.update_npcs(step);
                        if step < 15 {
                            game.step_group += 1;
                        } else if step > 30 {
                        } else {
                            game.step_group = 0;
                        }
                    }
                }
                thread::sleep(Duration::from_millis(35));
            }
        });
    }

    pub fn update(&mut self) -> bool {
        let (vw, vh) = self.gui.get_viewport();
        self.map.set_viewport(vh, vw);
        // log::info!("update");
        let res = match self.game_mode {
            GameMode::Play => self.play_update(),
            GameMode::Interact(_) => self.interaction(),
            GameMode::Fight(_) => {
                let Interactable::Enemy(e) = self.interactee.clone() else {
                    todo!()
                };
                self.enemy_encounter(e)
            }
            GameMode::Dead => false,
            _ => todo!(),
        };

        if !res {
            return false;
        }

        if self.items.len() < 50 {
            self.repop_items();
        }

        if self.npcs.len() < 20 {
            self.repop_npcs();
        }

        if self.enemies.len() < 30 {
            self.repop_enemies();
        }

        let ppos = (self.player.x, self.player.y);

        if let Some(e) = self.enemies.get(&(ppos)) {
            self.interactee = Interactable::Enemy(e.clone());
            self.game_mode = GameMode::Fight(FightSteps::Open);
        }

        self.new_loc_check();
        self.compass_check();

        let now = Instant::now();
        if now.duration_since(self.portal_cool) > Duration::from_secs(10) && self.portal_check() {
            self.portal_cool = Instant::now();
        }

        true
    }

    pub fn draw(&mut self) {
        self.location_check();
        let litems = if self.location != Location::Null {
            self.update_location();
            loc_shop_items(self.dist_fo, self.location.clone())
        } else {
            HashMap::new()
        };
        self.map_location();
        let debug_strs = {
            let dist_fo = format!("({}, {})", self.dist_fo.0, self.dist_fo.1);
            let comp = format!("({}, {})", self.comp_head.0, self.comp_head.1);
            //let spos_list = self.settles.get_settle_pos();
            // let spos_list = &self.comp_list;
            let spos_s = self
                .comp_list
                .clone()
                .iter()
                .map(|((x, y), s)| format!("({}, {}): {}", x, y, s))
                .collect::<Vec<String>>()
                .join(", ");
            (dist_fo, spos_s, comp)
        };
        self.gui.draw(
            debug_strs.clone(),
            self.map.clone(),
            self.player.clone(),
            self.portals.clone(),
            self.enemies.clone(),
            self.items.clone(),
            self.npcs.clone(),
            litems,
            self.env_inters.clone(),
        );
    }
}
