//gamestate
use crate::enums::{Cells, Enemies, Items, NPCs, NPCWrap, ItemOpt, GUIMode, InterSteps, InterOpt, GameMode, FightSteps, Interactable, EncOpt, Location};
use crate::map::{Map, MAP_W, MAP_H};
use crate::player::Player;
use crate::enemy::{Enemy};
use crate::npc::{NPC, BaseNPC, CommNPC, Convo, NQuest, new_comm_npc, new_conv_npc, new_quest_npc};
use crate::lsystems::LSystems;
use crate::gui::GUI;
use crate::settlements::Settlements;
use crate::settlement::Settlement;
// use crate::gui_man_draw::GUI;
use crate::item::Item;
use crate::notebook::Notebook;

use ratatui::crossterm::event::{read, Event, KeyCode, KeyEvent, poll};
use ratatui::crossterm::terminal;
use ratatui::crossterm::event::KeyEventKind::{Press, Release};
use std::io::stdout;
use rand::Rng;
use rand::prelude::SliceRandom;
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
// use ratatui::prelude::Line;
// use ratatui::widgets::{Block, Borders, Paragraph, Wrap, Padding};
// use ratatui::layout::{Layout, Constraint, Direction, Margin};
// use ratatui::style::{Color, Style};
// use ratatui::text::{Text, Span};
// use ratatui::widgets::Row;
// use ratatui::widgets::Table;
// use ratatui::widgets::Cell;

use std::time::{Duration, Instant};
use std::thread;
use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};
use serde_json::Result;
use serde_json::Value;
use std::fs;

use std::collections::HashMap;
use std::collections::HashSet;

fn place_enemies(mut map: Vec<Vec<Cells>>) -> HashMap<(usize, usize), Enemy> {
    let mut enemies = HashMap::new();
    let mut rng = rand::thread_rng();
    let etype = Enemies::Bug;
    let m_h = map.len() - 1;
    let m_w = map[0].len() - 1;
    for _ in 0..100 {
        loop {
            let x = rng.gen_range(10..m_w-10);
            let y = rng.gen_range(10..m_h-10);
            if map[y][x] == Cells::Empty {
                let mut temp_vec = Vec::new();
                temp_vec.push(Items::BugBits);
                let e_temp = Enemy::new(etype, "Bug".to_string(), x, y, 20, 15, 5, 5, temp_vec);
                enemies.insert((x, y), e_temp);
                break;
            }
        }
    }
    enemies
}

fn place_npcs(mut map: Vec<Vec<Cells>>) -> HashMap<(usize, usize), NPCWrap> {
    let data1 = fs::read_to_string("src/npcs/npc_names.json");
    log::info!("{:?}", &data1);
    let names: Vec<String> = match data1 {
        Ok(content) => serde_json::from_str(&content).unwrap(),
        Err(e) => {
            log::info!("{:?}", e);
            Vec::new()
        },
    };

    let data2 = fs::read_to_string("src/npcs/npc_comms.json");
    log::info!("{:?}", &data2);
    let comms: Vec<String> = match data2 {
        Ok(content) => serde_json::from_str(&content).unwrap(),
        Err(e) => {
            log::info!("{:?}", e);
            Vec::new()
        },
    };

    let data3 = fs::read_to_string("src/npcs/npc_convos.json");
    log::info!("{:?}", &data3);
    let convos: Vec<Convo> = match data3 {
        Ok(content) => serde_json::from_str(&content).unwrap(),
        Err(e) => {
            log::info!("{:?}", e);
            Vec::new()
        },
    };

    let data4 = fs::read_to_string("src/npcs/npc_quests.json");
    log::info!("{:?}", &data4);
    let quests: HashMap<String, NQuest> = match data4 {
        Ok(content) => serde_json::from_str(&content).unwrap(),
        Err(e) => {
            log::info!("{:?}", e);
            HashMap::new()
        },
    };
    let mut npcs = HashMap::new();
    let mut rng = rand::thread_rng();
    let types = vec![NPCs::CommNPC];
    // let types = vec![NPCs::CommNPC, NPCs::ConvNPC, NPCs::QuestNPC];
    let m_h = map.len() - 1;
    let m_w = map[0].len() - 1;
    for _ in 0..50 {
        loop {
            let x = rng.gen_range(10..m_w-10);
            let y = rng.gen_range(10..m_h-10);
            if map[y][x] == Cells::Empty {
                if let Some(i_type) = types.choose(&mut rng){
                    let npc = match i_type {
                        NPCs::CommNPC => {
                            let sname = &names[0];
                            let comm: Vec<String> = comms.clone();
                            NPCWrap::CommNPC(new_comm_npc(sname.to_string(), x, y, comm))
                        },
                        NPCs::ConvNPC => {
                            let sname = &names[0];
                            let conv: Convo = convos[0].clone();
                            NPCWrap::ConvNPC(new_conv_npc(sname.to_string(), x, y, conv))
                        },
                        NPCs::QuestNPC => {
                            let sname = &names[0];
                            let quest: NQuest = quests["get_quest"].clone();
                            NPCWrap::QuestNPC(new_quest_npc(sname.to_string(), x, y, quest))
                        },
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

fn init_items(mut map: Vec<Vec<Cells>>, enemies: HashMap<(usize, usize), Enemy>) -> HashMap<(usize, usize), Item> {
    let mut items = HashMap::new();
    let mut rng = rand::thread_rng();
    let types = vec![Items::Rock, Items::EdibleRoot];
    let m_h = map.len() - 1;
    let m_w = map[0].len() - 1;
    for _ in 0..200 {
        loop {
            let x = rng.gen_range(10..m_w-10);
            let y = rng.gen_range(10..m_h-10);
            if map[y][x] == Cells::Empty && !enemies.contains_key(&(x, y)) {
                if let Some(i_type) = types.choose(&mut rng){
                    let itm = match i_type {
                        Items::EdibleRoot => {
                            Item::new_edible_root(x, y)
                        },
                        Items::Rock => {
                            Item::new_rock(x, y)
                        },
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
        "UP" => {
            let map_coll = cells[pos.1 - 1][pos.0] == Cells::Wall;
            // let item_coll = self.items.contains_key(&(pos.0, pos.1 - 1));
            map_coll //|| item_coll
        },
        "DN" => {
            let map_coll = cells[pos.1 + 1][pos.0] == Cells::Wall;
            // let item_coll = self.items.contains_key(&(pos.0, pos.1 + 1));
            map_coll //|| item_coll
        },
        "LF" => {
            let map_coll = cells[pos.1][pos.0 - 1] == Cells::Wall;
            // let item_coll = self.items.contains_key(&(pos.0 - 1, pos.1));
            map_coll //|| item_coll
        },
        "RT" => {
            let map_coll = cells[pos.1][pos.0 + 1] == Cells::Wall;
            // let item_coll = self.items.contains_key(&(pos.0 + 1, pos.1));
            map_coll //|| item_coll
        },
        _ => false
    }
}

fn npc_move(mut npc: Box<dyn NPC>, map: Vec<Vec<Cells>>, mw: usize, mh: usize, x: usize, y: usize) -> ((usize, usize), Box<dyn NPC>) {
    let mut rng = rand::thread_rng();
    let dch = rng.gen_range(0..20);
    if dch % 4 == 0 {
        npc.set_steps(dch);
    }
    let pos = if npc.get_steps() < 5 {
        npc.inc_steps();
        // if y == 0 {(x, y)} else {
        if y <= 10 || n_collision("UP", npc.get_pos().clone(), map.clone()) {(x, y)} else {
            npc.mmove("UP");
            (x, y - 1)
        }
    } else if npc.get_steps() >= 5 && npc.get_steps() < 10 {
        npc.inc_steps();
        // if x == 0 {(x, y)} else {
        if x <= 10 || n_collision("LF", npc.get_pos().clone(), map.clone()) {(x, y)} else {
            npc.mmove("LF");
            (x - 1, y)
        }
    } else if npc.get_steps() >= 10 && npc.get_steps() < 15 {
        npc.inc_steps();
        // if y >= mh-5 {(x, y)} else {
        if y >= mh-10 || n_collision("DN", npc.get_pos().clone(), map.clone()) {(x, y)} else {
            npc.mmove("DN");
            (x, y + 1)
        }
    } else if npc.get_steps() >= 15 && npc.get_steps() < 20 {
        npc.inc_steps();
        // if x >= mw-5 {(x, y)} else {
        if x >= mw-10 || n_collision("RT", npc.get_pos().clone(), map.clone()) {(x, y)} else {
            npc.mmove("RT");
            (x + 1, y)
        }
    } else if npc.get_steps() == 20 {
        npc.set_steps(0);
        (x, y)
    } else {(x, y)};
    (pos, npc)
    // (pos, Box::new(npc))
}

fn box_npc(npc: NPCWrap) -> Box<dyn NPC> {
    match npc {
        NPCWrap::CommNPC(comm_npc) => Box::new(comm_npc),
        NPCWrap::ConvNPC(conv_npc) => Box::new(conv_npc),
        NPCWrap::QuestNPC(quest_npc) => Box::new(quest_npc),
        _ => todo!(),
    }
}

fn wrap_nbox(mut nbox: Box<dyn NPC>) -> NPCWrap {
    match nbox.get_ntype() {
        NPCs::CommNPC => {
            if let Some(comm_npc) = nbox.as_comm_npc() {
                NPCWrap::CommNPC(comm_npc.clone())
            } else {NPCWrap::BaseNPC(BaseNPC::new())}
        },
        NPCs::ConvNPC => {
            if let Some(conv_npc) = nbox.as_conv_npc() {
                NPCWrap::ConvNPC(conv_npc.clone())
            } else {NPCWrap::BaseNPC(BaseNPC::new())}
        },
        NPCs::QuestNPC => {
            if let Some(quest_npc) = nbox.as_quest_npc() {
                NPCWrap::QuestNPC(quest_npc.clone())
            } else {NPCWrap::BaseNPC(BaseNPC::new())}
        },
        _ => todo!(),
    }
}



pub struct GameState {
    game_mode: GameMode,
    notebook: Notebook,
    gui: GUI,
    map: Map,
    settles: Settlements,
    player: Player,
    dist_fo: (i64, i64),
    loc_rad: u16,
    level: u32,
    l_systems: LSystems,
    l_rate: u64,
    enemies: HashMap<(usize, usize), Enemy>,
    step_group: u8,
    items: HashMap<(usize, usize), Item>,
    // item_drop: Vec<((usize, usize), Item)>,
    npcs: HashMap<(usize, usize), NPCWrap>,
    key_debounce_dur: Duration,
    last_event_time: Instant,
    interactee: Interactable,
    location: Location,
    loc_map: Option<Vec<Vec<Cells>>>,
    enc: EncOpt,
}

impl GameState {
    pub fn new() -> Arc<Mutex<Self>> {
        let gui = GUI::new();
        let mut map = Map::new();
        let x = map.px.clone();
        let y = map.py.clone();
        let player = Player::new(x, y);
        let mut l_systems = LSystems::new();
        let enemies = place_enemies(map.cells.clone());
        let items = init_items(map.cells.clone(), enemies.clone());
        let npcs = place_npcs(map.cells.clone());
        let notebook = Notebook::new().unwrap();
        let l_rate = 100 as u64;

        let settles = Settlements::demo_self();

        Arc::new(Mutex::new(GameState {
            game_mode: GameMode::Play,
            notebook,
            gui,
            map,
            settles,
            player,
            dist_fo: (0, 0),
            loc_rad: 500,
            level: 0,
            l_systems,
            l_rate,
            enemies,
            step_group: 0,
            items,
            // item_drop,
            npcs,
            key_debounce_dur: Duration::from_millis(60),
            last_event_time: Instant::now(),
            interactee: Interactable::Null,
            location: Location::Null,
            loc_map: None,
            enc: EncOpt::Null,
        }))

        // Self {
        //     game_mode: GameMode::Play,
        //     notebook,
        //     gui,
        //     map,
        //     player,
        //     dist_fo: (0, 0, 0, 0),
        //     level: 0,
        //     l_systems,
        //     l_rate,
        //     enemies,
        //     enemy_rate: 0,
        //     items,
        //     // item_drop,
        //     npcs,
        //     key_debounce_dur: Duration::from_millis(80),
        //     last_event_time: Instant::now(),
        //     interactee: Interactable::Null,
        //     enc: EncOpt::Null,
        // }
    }


    fn collision(&mut self, dir: &str) -> bool {
        match dir {
            "UP" => {
                let map_coll = self.map.cells[self.player.y - 1][self.player.x] == Cells::Wall;
                let item_coll = self.items.contains_key(&(self.player.x, self.player.y - 1));
                map_coll || item_coll
            },
            "DN" => {
                let map_coll = self.map.cells[self.player.y + 1][self.player.x] == Cells::Wall;
                let item_coll = self.items.contains_key(&(self.player.x, self.player.y + 1));
                map_coll || item_coll
            },
            "LF" => {
                let map_coll = self.map.cells[self.player.y][self.player.x - 1] == Cells::Wall;
                let item_coll = self.items.contains_key(&(self.player.x - 1, self.player.y));
                map_coll || item_coll
            },
            "RT" => {
                let map_coll = self.map.cells[self.player.y][self.player.x + 1] == Cells::Wall;
                let item_coll = self.items.contains_key(&(self.player.x + 1, self.player.y));
                map_coll || item_coll
            },
            _ => false
        }
    }

    fn e_collision(&mut self, dir: &str, entity: Enemy) -> bool {
        match dir {
            "UP" => {
                let map_coll = self.map.cells[entity.y - 1][entity.x] == Cells::Wall;
                let item_coll = self.items.contains_key(&(entity.x, entity.y - 1));
                map_coll || item_coll
            },
            "DN" => {
                let map_coll = self.map.cells[entity.y + 1][entity.x] == Cells::Wall;
                let item_coll = self.items.contains_key(&(entity.x, entity.y + 1));
                map_coll || item_coll
            },
            "LF" => {
                let map_coll = self.map.cells[entity.y][entity.x - 1] == Cells::Wall;
                let item_coll = self.items.contains_key(&(entity.x - 1, entity.y));
                map_coll || item_coll
            },
            "RT" => {
                let map_coll = self.map.cells[entity.y][entity.x + 1] == Cells::Wall;
                let item_coll = self.items.contains_key(&(entity.x + 1, entity.y));
                map_coll || item_coll
            },
            _ => false
        }
    }

    // fn n_collision(&mut self, dir: &str, entity: &dyn NPC) -> bool {
    //     match dir {
    //         "UP" => {
    //             let map_coll = self.map.cells[entity.y - 1][entity.x] == Cells::Wall;
    //             let item_coll = self.items.contains_key(&(entity.x, entity.y - 1));
    //             map_coll || item_coll
    //         },
    //         "DN" => {
    //             let map_coll = self.map.cells[entity.y + 1][entity.x] == Cells::Wall;
    //             let item_coll = self.items.contains_key(&(entity.x, entity.y + 1));
    //             map_coll || item_coll
    //         },
    //         "LF" => {
    //             let map_coll = self.map.cells[entity.y][entity.x - 1] == Cells::Wall;
    //             let item_coll = self.items.contains_key(&(entity.x - 1, entity.y));
    //             map_coll || item_coll
    //         },
    //         "RT" => {
    //             let map_coll = self.map.cells[entity.y][entity.x + 1] == Cells::Wall;
    //             let item_coll = self.items.contains_key(&(entity.x + 1, entity.y));
    //             map_coll || item_coll
    //         },
    //         _ => false
    //     }
    // }

    fn update_enemies(&mut self, step: u8) {
        let mut e_temp = self.enemies.clone();
        let mut new_e = HashMap::new();
        let mh = self.map.cells.len();
        let mw = self.map.cells[0].len();
        for ((x, y), mut e) in &mut e_temp {
            let mut rng = rand::thread_rng();
            let dch = rng.gen_range(0..20);
            if dch % 4 == 0 {
                e.steps = dch;
            }
            // log::info!("esteps: {}, eCx: {}, ey: {}", e.steps.clone(), x.clone(), y.clone());
            // e.update();
            let (xx, yy) = if e.get_step_grp() != step {
                (*x, *y)
            } else if e.steps < 5 {
                e.steps += 1;
                if *y == 0 || self.e_collision("UP", e.clone()) {(*x, *y)} else {
                    e.mmove("UP");
                    (*x, y - 1)
                }
            } else if e.steps >= 5 && e.steps < 10 {
                e.steps += 1;
                if *x == 0 || self.e_collision("LF", e.clone()) {(*x, *y)} else {
                    e.mmove("LF");
                    (x - 1, *y)
                }
            } else if e.steps >= 10 && e.steps < 15 {
                e.steps += 1;
                if *y >= mh-5 || self.e_collision("DN", e.clone()) {(*x, *y)} else {
                    e.mmove("DN");
                    (*x, y + 1)
                }
            } else if e.steps >= 15 && e.steps < 20 {
                e.steps += 1;
                if *x >= mw-5 || self.e_collision("RT", e.clone()) {(*x, *y)} else {
                    e.mmove("RT");
                    (x + 1, *y)
                }
            } else if e.steps == 20 {
                e.steps = 0;
                (*x, *y)
            } else {(*x, *y)};
            new_e.insert((xx, yy), e.clone());
        }
        // self.enemies = new_e.into_iter().map(|(k, v)| (k, v.clone())).collect();
        self.enemies = new_e;
    }

    // fn npc_move(&mut self, npc: dyn NPC, mw: usize, mh: usize, x: usize, y: usize) -> ((usize, usize), dyn NPC) {
    //     let pos = if npc.get_steps() < 5 {
    //         npc.inc_steps();
    //         if y == 0 {(x, y)} else {
    //         // if y == 0 || self.e_collision("UP", npc.clone()) {(x, y)} else {
    //             npc.mmove("UP");
    //             (x, y - 1)
    //         }
    //     } else if npc.get_steps() >= 5 && npc.get_steps() < 10 {
    //         npc.inc_steps();
    //         if x == 0 {(x, y)} else {
    //         // if x == 0 || self.e_collision("LF", npc.clone()) {(x, y)} else {
    //             npc.mmove("LF");
    //             (x - 1, y)
    //         }
    //     } else if npc.get_steps() >= 10 && npc.get_steps() < 15 {
    //         npc.inc_steps();
    //         if y >= mh-5 {(x, y)} else {
    //         // if y >= mh-5 || self.e_collision("DN", npc.clone()) {(x, y)} else {
    //             npc.mmove("DN");
    //             (x, y + 1)
    //         }
    //     } else if npc.get_steps() >= 15 && npc.get_steps() < 20 {
    //         npc.inc_steps();
    //         if x >= mw-5 {(x, y)} else {
    //         // if x >= mw-5 || self.e_collision("RT", npc.clone()) {(x, y)} else {
    //             npc.mmove("RT");
    //             (x + 1, y)
    //         }
    //     } else if npc.get_steps() == 20 {
    //         npc.set_steps(0);
    //         (x, y)
    //     } else {(x, y)};
    //     (pos, npc)
    // }

    fn update_npcs(&mut self, step: u8) {
        let mut n_temp = self.npcs.clone();
        let mut new_n = HashMap::new();
        let mh = self.map.cells.len();
        let mw = self.map.cells[0].len();
        for ((x, y), mut n) in &mut n_temp {
            // log::info!("esteps: {}, eCx: {}, ey: {}", e.steps.clone(), x.clone(), y.clone());
            match n {
                NPCWrap::CommNPC(npc) => {
                    let mut npc_t = npc.clone();
                    let mut npc_b = Box::new(npc_t);
                    if npc_b.get_step_grp() != step {
                        new_n.insert((*x, *y), NPCWrap::CommNPC(npc.clone()));
                    } else {
                        let (pos, mut nnpc) = npc_move(npc_b, self.map.cells.clone(), mw, mh, *x, *y);
                        match nnpc.get_ntype() {
                            NPCs::CommNPC => {
                                if let Some(comm_npc) = nnpc.as_comm_npc() {
                                    new_n.insert(pos, NPCWrap::CommNPC(comm_npc.clone()));
                                }
                            },
                            _ => todo!(),
                        }
                    }
                },
                _ => todo!(),
            }
        }
        self.npcs = new_n;
    }

    fn shift_enemies(&mut self, dir: &str) {
        let temp_e = self.enemies.clone();
        let mut new_e = HashMap::new();
        let mw = self.map.cells[0].len();
        let mh = self.map.cells.len();
        for ((x, y), mut e) in temp_e {
            match dir {
                "UP" => if y < mh - 5 {
                    e.y+=1;
                    new_e.insert((x, y+1), e.clone());
                    // log::info!("new key {:?}", (x, y+1));
                    // log::info!("new en {:?}", e);

                },
                "DN" => if y > 5 {
                    e.y-=1;
                    new_e.insert((x, y-1), e.clone());
                    // log::info!("new key {:?}", (x, y+1));
                    // log::info!("new en {:?}", e);
                },
                "LF" => if x < mw - 5 {
                    e.x+=1;
                    new_e.insert((x+1, y), e.clone());
                    // log::info!("new key {:?}", (x, y+1));
                    // log::info!("new en {:?}", e);
                },
                "RT" => if x > 5 {
                    e.x-=1;
                    new_e.insert((x-1, y), e.clone());
                    // log::info!("new key {:?}", (x, y+1));
                    // log::info!("new en {:?}", e);
                },
                _ => todo!(),
            };
        }
        self.enemies = new_e;
    }

    fn shift_items(&mut self, dir: &str) {
        let temp_i = self.items.clone();
        let mut new_i = HashMap::new();
        let mw = self.map.cells[0].len();
        let mh = self.map.cells.len();
        for ((x, y), mut i) in temp_i {
            match dir {
                "UP" => if y < mh {
                    i.y+=1;
                    new_i.insert((x, y+1), i.clone());
                    // log::info!("new key {:?}", (x, y+1));
                    // log::info!("new en {:?}", e);

                },
                "DN" => if y > 0 {
                    i.y-=1;
                    new_i.insert((x, y-1), i.clone());
                    // log::info!("new key {:?}", (x, y+1));
                    // log::info!("new en {:?}", e);
                },
                "LF" => if x < mw {
                    i.x+=1;
                    new_i.insert((x+1, y), i.clone());
                    // log::info!("new key {:?}", (x, y+1));
                    // log::info!("new en {:?}", e);
                },
                "RT" => if x > 0 {
                    i.x-=1;
                    new_i.insert((x-1, y), i.clone());
                    // log::info!("new key {:?}", (x, y+1));
                    // log::info!("new en {:?}", e);
                },
                _ => todo!(),
            };
        }
        self.items = new_i;
    }

    fn shift_npcs(&mut self, dir: &str) {
        let temp_n = self.npcs.clone();
        let mut new_n = HashMap::new();
        let mw = self.map.cells[0].len();
        let mh = self.map.cells.len();
        for ((x, y), mut n) in temp_n {
            let mut nbox = box_npc(n);
            match dir {
                "UP" => if y < mh-10 {
                    // n.y+=1;
                    nbox.mmove("DN");
                    let npc_w = wrap_nbox(nbox);
                    new_n.insert((x, y+1), npc_w.clone());
                },
                "DN" => if y > 10 {
                    // n.y-=1;
                    nbox.mmove("UP");
                    let npc_w = wrap_nbox(nbox);
                    new_n.insert((x, y-1), npc_w.clone());
                },
                "LF" => if x < mw-10 {
                    // n.x+=1;
                    nbox.mmove("RT");
                    let npc_w = wrap_nbox(nbox);
                    new_n.insert((x+1, y), npc_w.clone());
                },
                "RT" => if x > 10 {
                    // n.x-=1;
                    nbox.mmove("LF");
                    let npc_w = wrap_nbox(nbox);
                    new_n.insert((x-1, y), npc_w.clone());
                },
                _ => todo!(),
            };
        }
        self.npcs = new_n;
    }

    fn enemy_turn(&mut self, e: Enemy) -> u16 {
        let (mut atk, mut dmg) = e.fight_turn();
        let pdef = self.player.get_defence();
        let dodge = self.player.get_dodge();
        if atk > pdef {
            if dodge {
                self.player.toggle_dodge();
                dmg /= 2;
            }
            self.player.apply_attack(dmg);
            return dmg;
        }
        0
    }

    fn enemy_drop(&mut self, mut e: Enemy) {
        let mut drps = e.get_drop();
        let i = drps.pop();
        let (x, y) = e.get_pos();
        let itm = match i {
            Some(Items::BugBits) => Item::new_bug_bits(x.clone(), y.clone()),
            _ => todo!(),
        };
        self.items.insert((x, y), itm.clone());
    }

    fn enemy_encounter(&mut self, mut e: Enemy) {
        //you are in fight
        let fst = format!("You are being attacked by a {}", e.get_sname());
        self.gui.reset_cursor();
        loop {
            self.gui.encounter_show_content(fst.clone(), self.map.clone(), self.map_location(), self.player.clone(), self.enemies.clone(), self.items.clone(), self.npcs.clone());
            if poll(std::time::Duration::from_millis(100)).unwrap() {
                if let Event::Key(event) = read().unwrap() {
                    // log::info!("keykind {:?}", event.kind.clone());
                    let now = Instant::now();
                    if now.duration_since(self.last_event_time) > self.key_debounce_dur {
                        self.last_event_time = now;
                        let res = self.enc_key(event.code);
                        if !res {
                            break
                        }
                    }
                }
            }
        }
        //fight start
        let mut pstart = true;
        self.game_mode = GameMode::Fight(FightSteps::Player);
        let mut fight = true;
        let mut win = None;
        while fight {
            let Interactable::Enemy(mut enemy) = self.interactee.clone() else {todo!()};
            e = enemy.clone();
            if !pstart {
                let enatk = "Enemy is attacking.".to_string();
                loop {
                    self.gui.encounter_show_content(enatk.clone(), self.map.clone(), self.map_location(), self.player.clone(), self.enemies.clone(), self.items.clone(), self.npcs.clone());
                    if poll(std::time::Duration::from_millis(100)).unwrap() {
                        if let Event::Key(event) = read().unwrap() {
                            // log::info!("keykind {:?}", event.kind.clone());
                            let now = Instant::now();
                            if now.duration_since(self.last_event_time) > self.key_debounce_dur {
                                self.last_event_time = now;
                                let res = self.enc_key(event.code);
                                if !res {
                                    break
                                }
                            }
                        }
                    }
                }
                //enemy turn
                let turn = self.enemy_turn(e.clone());
                let trn_res = if turn == 0 {
                    "The enemy attempted an attack, but missed.".to_string()
                } else {
                    let fmts = format!("The enemy atacked you for {}hp.", turn.clone());
                    fmts
                };
                self.gui.reset_cursor();
                loop {
                    self.gui.encounter_show_content(trn_res.clone(), self.map.clone(), self.map_location(), self.player.clone(), self.enemies.clone(), self.items.clone(), self.npcs.clone());
                    if poll(std::time::Duration::from_millis(100)).unwrap() {
                        if let Event::Key(event) = read().unwrap() {
                            // log::info!("keykind {:?}", event.kind.clone());
                            let now = Instant::now();
                            if now.duration_since(self.last_event_time) > self.key_debounce_dur {
                                self.last_event_time = now;
                                let res = self.enc_key(event.code);
                                if !res {
                                    break
                                }
                            }
                        }
                    }
                }
                if self.player.get_health() == 0 {
                    win = Some(false);
                    self.game_mode = GameMode::Fight(FightSteps::Null);
                    break;
                }
                self.game_mode = GameMode::Fight(FightSteps::Player);
            }
            if pstart {
                pstart = false;
            }
            //player turn
            //-player choice
            let popt = self.player.get_enc_opt();
            self.gui.reset_cursor();
            loop {
                self.gui.encounter_user_options(popt.clone(), self.map.clone(), self.map_location(), self.player.clone(), self.enemies.clone(), self.items.clone(), self.npcs.clone());
                if poll(std::time::Duration::from_millis(100)).unwrap() {
                    if let Event::Key(event) = read().unwrap() {
                        // log::info!("keykind {:?}", event.kind.clone());
                        let now = Instant::now();
                        if now.duration_since(self.last_event_time) > self.key_debounce_dur {
                            self.last_event_time = now;
                            let res = self.enc_key(event.code);
                            if !res {
                                break
                            }
                        }
                    }
                }
            }
            let lturn = self.player.get_last_turn();
            self.player.set_enc_last_turn((EncOpt::Null, 0));
            let mut itm = false;
            let trn_res = match lturn {
                (EncOpt::Dodge, _) => {
                    "You dodged in an attempt to evade attack.".to_string()
                },
                (EncOpt::Attack, 0) => {
                    "You attempted an attack, but missed.".to_string()
                },
                (EncOpt::Attack, _) => {
                    let ehp = if e.health > lturn.1 {(e.health - lturn.clone().1)} else {0};
                    let fmts = format!("You successfully attacked the {} for {}hp. They have an hp of: {}", e.clone().get_sname(), lturn.clone().1, ehp);
                    fmts
                },
                (EncOpt::UseItem, _) => {
                    itm = true;
                    "".to_string()
                },
                _ => "OOPS!".to_string(),

            };
            self.gui.reset_cursor();
            loop {
                if itm {break;}
                self.gui.encounter_show_content(trn_res.clone(), self.map.clone(), self.map_location(), self.player.clone(), self.enemies.clone(), self.items.clone(), self.npcs.clone());
                if poll(std::time::Duration::from_millis(100)).unwrap() {
                    if let Event::Key(event) = read().unwrap() {
                        // log::info!("keykind {:?}", event.kind.clone());
                        let now = Instant::now();
                        if now.duration_since(self.last_event_time) > self.key_debounce_dur {
                            self.last_event_time = now;
                            let res = self.enc_key(event.code);
                            if !res {
                                break
                            }
                        }
                    }
                }
            }
            let Interactable::Enemy(mut enemy) = self.interactee.clone() else {todo!()};
            e = enemy.clone();
            if e.health == 0 {
                win = Some(true);
                let epos = e.get_pos();
                self.enemies.remove(&epos);
                self.game_mode = GameMode::Fight(FightSteps::Null);
                break;
            }
            self.game_mode = GameMode::Fight(FightSteps::Enemy);
            //round end
        }
        //fight over
        let win_msg = if win.unwrap() {
            self.enemy_drop(e.clone());
            format!("You defeated the {}!", e.get_sname())
        } else {
            format!("You were killed by the {}! You are dead", e.get_sname())
        };
        self.gui.reset_cursor();
        loop {
            self.gui.encounter_show_content(win_msg.clone(), self.map.clone(), self.map_location(), self.player.clone(), self.enemies.clone(), self.items.clone(), self.npcs.clone());
            if poll(std::time::Duration::from_millis(100)).unwrap() {
                if let Event::Key(event) = read().unwrap() {
                    // log::info!("keykind {:?}", event.kind.clone());
                    let now = Instant::now();
                    if now.duration_since(self.last_event_time) > self.key_debounce_dur {
                        self.last_event_time = now;
                        let res = self.inter_key(event.code);
                        if !res {
                            break
                        }
                    }
                }
            }
        }
    }

    fn start_interact(&mut self) {
        let (px, py) = self.player.pos();
        let adj = vec![(px, (py as isize - 1) as usize), (px, py + 1), ((px as isize - 1) as usize, py), (px + 1, py)];
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
        }
        if adj_inter.len() > 0 {
            self.game_mode = GameMode::Interact(InterSteps::AdjOpt);
            self.gui.set_info_mode(GUIMode::Interact);
            self.gui.set_interactable(adj_inter);
        }
    }

    fn get_interactee(&mut self, pos: (usize, usize)) -> Option<Interactable> {
        if let Some(item) = self.items.get(&pos) {
            Some(Interactable::Item(item.clone()))
        } else if let Some(enemy) = self.enemies.get(&pos) {
            Some(Interactable::Enemy(enemy.clone()))
        } else if let Some(npc) = self.npcs.get(&pos) {
            Some(Interactable::NPC(npc.clone()))
        } else {
            Some(Interactable::Null)
        }
    }

    fn use_inv_item(&mut self){
        let (idx, item) = self.gui.get_inv_opt();
        //gui, using item
        self.player.apply_item_effect(item.clone());
        self.player.rem_inv_item(idx);
        self.gui.set_inventory(self.player.get_inventory());
        self.gui.reset_cursor();
        match self.game_mode {
            GameMode::Play => {
                loop {
                    self.gui.item_used_draw(self.map.clone(), self.map_location(), self.player.clone(), self.enemies.clone(), self.items.clone(), self.npcs.clone());
                    if poll(std::time::Duration::from_millis(100)).unwrap() {
                        if let Event::Key(event) = read().unwrap() {
                            // log::info!("keykind {:?}", event.kind.clone());
                            let now = Instant::now();
                            if now.duration_since(self.last_event_time) > self.key_debounce_dur {
                                self.last_event_time = now;
                                match event.code {
                                    KeyCode::Enter => {
                                        break;
                                    },
                                    _ => {},
                                }
                            }
                        }
                    }
                }
            },
            GameMode::Fight(_) => {
                let itstr = format!("You used the {}", item.clone().get_sname());
                loop {
                    self.gui.encounter_show_content(itstr.clone(), self.map.clone(), self.map_location(), self.player.clone(), self.enemies.clone(), self.items.clone(), self.npcs.clone());
                    if poll(std::time::Duration::from_millis(100)).unwrap() {
                        if let Event::Key(event) = read().unwrap() {
                            // log::info!("keykind {:?}", event.kind.clone());
                            let now = Instant::now();
                            if now.duration_since(self.last_event_time) > self.key_debounce_dur {
                                self.last_event_time = now;
                                match event.code {
                                    KeyCode::Enter => {
                                        break;
                                    },
                                    _ => {},
                                }
                            }
                        }
                    }
                }
            },
            _ => {},
        }
    }

    fn play_key(&mut self, code: KeyCode) -> bool {
        match code {
            KeyCode::Up => {
                if self.collision("UP") {} else {
                    if self.player.y - 1 <= self.map.viewport_y + (self.map.viewport_height/7) {
                        self.shift_enemies("UP");
                        self.shift_items("UP");
                        self.shift_npcs("UP");
                        self.map.shift("UP");
                        self.dist_fo.1 -= 1;
                        self.gui.set_dist_fo(self.dist_fo);
                    } else {
                        self.player.y -= 1;
                    }
                }
            },
            KeyCode::Down => {
                if self.collision("DN") {} else {
                    if self.player.y + 1 >= (self.map.viewport_height + self.map.viewport_y) - (self.map.viewport_height/7) {
                        self.shift_enemies("DN");
                        self.shift_items("DN");
                        self.shift_npcs("DN");
                        self.map.shift("DN");
                        self.dist_fo.1 += 1;
                        self.gui.set_dist_fo(self.dist_fo);
                    } else {
                        self.player.y += 1;
                    }
                }
            },
            KeyCode::Left => {
                if self.collision("LF") {} else {
                    if self.player.x - 1 <= self.map.viewport_x + (self.map.viewport_width/7) {
                        self.shift_enemies("LF");
                        self.shift_items("LF");
                        self.shift_npcs("LF");
                        self.map.shift("LF");
                        self.dist_fo.0 -= 1;
                        self.gui.set_dist_fo(self.dist_fo);
                    } else {
                        self.player.x -= 1;
                    }
                }
            },
            KeyCode::Right => {
                if self.collision("RT") {} else {
                    if self.player.x + 1 >= (self.map.viewport_width + self.map.viewport_x) - (self.map.viewport_width/7) {
                        self.shift_enemies("RT");
                        self.shift_items("RT");
                        self.shift_npcs("RT");
                        self.map.shift("RT");
                        self.dist_fo.0 += 1;
                        self.gui.set_dist_fo(self.dist_fo);
                    } else {
                        self.player.x += 1;
                    }
                }
            },
            KeyCode::Char('p') => self.gui.set_info_mode(GUIMode::Bug),
            KeyCode::Char('o') => self.gui.set_info_mode(GUIMode::Normal),
            KeyCode::Char('q') => {
                self.gui.set_info_mode(GUIMode::Inventory);
                self.gui.set_inventory(self.player.get_inventory());
                self.gui.reset_cursor();
            },
            KeyCode::Char('w') => {
                self.gui.set_info_mode(GUIMode::Map);
                self.gui.set_dist_fo(self.dist_fo);
            },
            KeyCode::Char('e') => self.gui.set_info_mode(GUIMode::Normal),
            KeyCode::Char('r') => {
                self.gui.set_info_mode(GUIMode::Notes);
                self.gui.set_notes(self.notebook.get_active_notes());
            },
            KeyCode::Char('a') => self.gui.move_cursor("LF"),
            KeyCode::Char('s') => self.gui.move_cursor("UP"),
            KeyCode::Char('d') => self.gui.move_cursor("DN"),
            KeyCode::Char('f') => self.gui.move_cursor("RT"),
            KeyCode::Char(' ') => self.start_interact(),
            KeyCode::Enter => {
                let gmode = self.gui.get_mode();
                match gmode {
                    GUIMode::Normal => {},
                    GUIMode::Inventory => {
                        self.use_inv_item();
                    },
                    GUIMode::Map => {},
                    GUIMode::Notes => {
                        self.gui.menu_lvl("DN");
                    },
                    _ => {},
                }
            },
            KeyCode::Backspace => {
                let gmode = self.gui.get_mode();
                match gmode {
                    GUIMode::Normal => {},
                    GUIMode::Inventory => {},
                    GUIMode::Map => {},
                    GUIMode::Notes => {
                        self.gui.menu_lvl("UP");
                    },
                    _ => {},
                }
            },
            KeyCode::Esc => return false,
            _ => {},
        }
        true
    }

    // fn drop_key(&mut self, code: KeyCode) -> bool {
    //     match code {
    //         KeyCode::Up => {
    //             self.gui.move_cursor("UP");
    //         },
    //         KeyCode::Down => {
    //             self.gui.move_cursor("DN");
    //         },
    //         KeyCode::Left => {
    //             self.gui.move_cursor("LF");
    //         },
    //         KeyCode::Right => {
    //             self.gui.move_cursor("RT");
    //         },
    //         KeyCode::Char('p') => self.gui.set_info_mode(GUIMode::Bug),
    //         KeyCode::Char('o') => self.gui.set_info_mode(GUIMode::Normal),
    //         KeyCode::Char('z') => {
    //             self.gui.set_info_mode(GUIMode::Normal);
    //             self.game_mode = GameMode::Play;
    //         },
    //         KeyCode::Char('a') => self.gui.move_cursor("LF"),
    //         KeyCode::Char('s') => self.gui.move_cursor("UP"),
    //         KeyCode::Char('d') => self.gui.move_cursor("DN"),
    //         KeyCode::Char('f') => self.gui.move_cursor("RT"),
    //         KeyCode::Enter => {
    //             match self.game_mode {
    //                 GameMode::Interact(InterSteps::AdjOpt) => {
    //                     self.select_adj();
    //                     self.game_mode = GameMode::Interact(InterSteps::IntOpt);
    //                 },
    //                 GameMode::Interact(InterSteps::IntOpt) => {
    //                     self.select_opt();
    //                     self.game_mode = GameMode::Interact(InterSteps::Feedback);
    //                 },
    //                 GameMode::Interact(InterSteps::Feedback) => {
    //                     // self.select_adj();
    //                     self.game_mode = GameMode::Play;
    //                 },
    //                 _ => self.game_mode = GameMode::Play,
    //             }
    //
    //             return false;
    //         },
    //         KeyCode::Esc => return false,
    //         _ => {},
    //     }
    //     true
    // }

    // fn drop_interact(&mut self) {
    //     self.gui.reset_cursor();
    //     loop {
    //         self.gui.inter_adj_draw(self.map.clone(), self.player.clone(), self.enemies.clone(), self.items.clone());
    //         if poll(std::time::Duration::from_millis(100)).unwrap() {
    //             if let Event::Key(event) = read().unwrap() {
    //                 // log::info!("keykind {:?}", event.kind.clone());
    //                 let now = Instant::now();
    //                 if now.duration_since(self.last_event_time) > self.key_debounce_dur {
    //                     self.last_event_time = now;
    //                     let res = self.inter_key(event.code);
    //                     if !res {
    //                         break
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }

    fn select_adj(&mut self) {
        let (pos, st) = self.gui.get_interactee();
        let Some(intee) = self.get_interactee(pos) else {todo!()};
        self.interactee = intee.clone();
        match intee {
            Interactable::Item(item) => {
                // self.item_opt(item.clone());
                self.gui.set_inter_opt(item.iopts);
            },
            Interactable::Enemy(enemy) => {},
            Interactable::NPC(npc) => {},
            Interactable::Null => {},
            _ => todo!(),
        }
    }

    fn pickup_item(&mut self, item: Item) {
        self.player.add_to_inv(item.clone());
        if let Some(itm) = self.items.remove(&(item.x, item.y)) {
        } else {}
    }

    fn select_opt(&mut self) {
        let (opt, _) = self.gui.get_iopt();
        match opt {
            InterOpt::Item(item_opt) => {
                match item_opt {
                    ItemOpt::PickUp => {
                        let Interactable::Item(item) = self.interactee.clone() else {todo!()};
                        self.pickup_item(item);
                    },
                    ItemOpt::Drp => {},
                    ItemOpt::Use => {

                    },
                    _ => todo!(),
                }
            },
            _ => todo!(),
        }
    }

    fn player_attack(&mut self) {
        let (atk, dmg) = self.player.get_enc_turn();
        let Interactable::Enemy(mut enemy) = self.interactee.clone() else {todo!()};
        let endef = enemy.get_defence();
        if atk > endef {
            enemy.apply_attack(dmg.clone());
            self.player.set_enc_last_turn((EncOpt::Attack, dmg));
            self.interactee = Interactable::Enemy(enemy.clone());
        } else {
            self.player.set_enc_last_turn((EncOpt::Attack, 0));
        }

    }

    fn enc_use_item(&mut self) {
        // let inventory = self.player.get_inventory();
        self.gui.set_inventory(self.player.get_inventory());
        self.gui.reset_cursor();
        loop {
            self.gui.encounter_pick_item(self.map.clone(), self.map_location(), self.player.clone(), self.enemies.clone(), self.items.clone(), self.npcs.clone());
            if poll(std::time::Duration::from_millis(100)).unwrap() {
                if let Event::Key(event) = read().unwrap() {
                    // log::info!("keykind {:?}", event.kind.clone());
                    let now = Instant::now();
                    if now.duration_since(self.last_event_time) > self.key_debounce_dur {
                        self.last_event_time = now;
                            match event.code {
                                KeyCode::Up => self.gui.move_cursor("UP"),
                                KeyCode::Down => self.gui.move_cursor("DN"),
                                KeyCode::Left => self.gui.move_cursor("LF"),
                                KeyCode::Right => self.gui.move_cursor("RT"),
                                KeyCode::Char('a') => self.gui.move_cursor("LF"),
                                KeyCode::Char('s') => self.gui.move_cursor("UP"),
                                KeyCode::Char('d') => self.gui.move_cursor("DN"),
                                KeyCode::Char('f') => self.gui.move_cursor("RT"),
                                KeyCode::Enter => {
                                    self.use_inv_item();
                                    self.gui.reset_enc_opt();
                                    self.enc = EncOpt::Null;
                                    break;
                                },
                                _ => {},
                            }
                    }
                }
            }
        }
    }

    fn enc_option(&mut self) {
        let opt = self.enc.clone();
        match opt {
            EncOpt::Attack => {
                self.player_attack();
            },
            EncOpt::UseItem => {
                self.enc_use_item();
                self.player.set_enc_last_turn((EncOpt::UseItem, 0));
            },
            EncOpt::Dodge => {
                self.player.toggle_dodge();
                self.player.set_enc_last_turn((EncOpt::Dodge, 0));
            },
            _ => {},
        }
    }

    fn enc_key(&mut self, code: KeyCode) -> bool {
        match code {
            KeyCode::Up => {
                self.gui.move_cursor("UP");
            },
            KeyCode::Down => {
                self.gui.move_cursor("DN");
            },
            KeyCode::Left => {
                self.gui.move_cursor("LF");
            },
            KeyCode::Right => {
                self.gui.move_cursor("RT");
            },
            KeyCode::Char('p') => self.gui.set_info_mode(GUIMode::Bug),
            KeyCode::Char('o') => self.gui.set_info_mode(GUIMode::Normal),
            KeyCode::Char('z') => {
                self.gui.set_info_mode(GUIMode::Normal);
                self.game_mode = GameMode::Play;
            },
            KeyCode::Char('a') => self.gui.move_cursor("LF"),
            KeyCode::Char('s') => self.gui.move_cursor("UP"),
            KeyCode::Char('d') => self.gui.move_cursor("DN"),
            KeyCode::Char('f') => self.gui.move_cursor("RT"),
            KeyCode::Enter => {
                match self.game_mode {
                    GameMode::Fight(FightSteps::Open) => {
                        // self.select_adj();
                        // self.game_mode = GameMode::Fight(FightSteps::);
                    },
                    GameMode::Fight(FightSteps::Enemy) => {
                        // self.select_opt();
                        // self.game_mode = GameMode::Fight(FightSteps::Player);
                    },
                    GameMode::Fight(FightSteps::Player) => {
                        // self.select_opt();
                        // self.game_mode = GameMode::Fight(FightSteps::Enemy);
                        // let opt = self.enc.clone();
                        let opt = self.gui.get_enc_opt();
                        self.enc = opt.0.clone();
                        log::info!("opt1 {:?}", opt.clone());
                        match opt.0 {
                            _ => self.enc_option(),
                            // _ => {},
                        }

                    },
                    GameMode::Fight(FightSteps::Message) => {
                        // self.select_adj();
                        // self.game_mode = GameMode::Play;
                    },
                    _ => {},
                }

                return false;
            },
            KeyCode::Esc => {
                self.game_mode = GameMode::Play;
                return false;
            },
            _ => {},
        }
        true
    }

    fn comm_key(&mut self, code: KeyCode) -> bool {
        match code {
            KeyCode::Up => {
                self.gui.move_cursor("UP");
            },
            KeyCode::Down => {
                self.gui.move_cursor("DN");
            },
            KeyCode::Left => {
                self.gui.move_cursor("LF");
            },
            KeyCode::Right => {
                self.gui.move_cursor("RT");
            },
            KeyCode::Char('p') => self.gui.set_info_mode(GUIMode::Bug),
            KeyCode::Char('o') => self.gui.set_info_mode(GUIMode::Normal),
            KeyCode::Char('z') => {
                self.gui.set_info_mode(GUIMode::Normal);
                self.game_mode = GameMode::Play;
            },
            KeyCode::Char('a') => self.gui.move_cursor("LF"),
            KeyCode::Char('s') => self.gui.move_cursor("UP"),
            KeyCode::Char('d') => self.gui.move_cursor("DN"),
            KeyCode::Char('f') => self.gui.move_cursor("RT"),
            KeyCode::Enter => {
                self.game_mode = GameMode::Play;
                self.gui.set_info_mode(GUIMode::Normal);
                // match self.game_mode {
                //     GameMode::Interact(InterSteps::AdjOpt) => {
                //         self.select_adj();
                //         self.game_mode = GameMode::Interact(InterSteps::IntOpt);
                //     },
                //     GameMode::Interact(InterSteps::IntOpt) => {
                //         self.select_opt();
                //         self.game_mode = GameMode::Interact(InterSteps::Feedback);
                //     },
                //     GameMode::Interact(InterSteps::Feedback) => {
                //         // self.select_adj();
                //         self.game_mode = GameMode::Play;
                //     },
                //     _ => self.game_mode = GameMode::Play,
                // }

                return false;
            },
            // KeyCode::Esc => return false,
            _ => {},
        }
        true
    }

    fn inter_key(&mut self, code: KeyCode) -> bool {
        match code {
            KeyCode::Up => {
                self.gui.move_cursor("UP");
            },
            KeyCode::Down => {
                self.gui.move_cursor("DN");
            },
            KeyCode::Left => {
                self.gui.move_cursor("LF");
            },
            KeyCode::Right => {
                self.gui.move_cursor("RT");
            },
            KeyCode::Char('p') => self.gui.set_info_mode(GUIMode::Bug),
            KeyCode::Char('o') => self.gui.set_info_mode(GUIMode::Normal),
            KeyCode::Char('z') => {
                self.gui.set_info_mode(GUIMode::Normal);
                self.game_mode = GameMode::Play;
            },
            KeyCode::Char('a') => self.gui.move_cursor("LF"),
            KeyCode::Char('s') => self.gui.move_cursor("UP"),
            KeyCode::Char('d') => self.gui.move_cursor("DN"),
            KeyCode::Char('f') => self.gui.move_cursor("RT"),
            KeyCode::Enter => {
                match self.game_mode {
                    GameMode::Interact(InterSteps::AdjOpt) => {
                        self.select_adj();
                        self.game_mode = GameMode::Interact(InterSteps::IntOpt);
                    },
                    GameMode::Interact(InterSteps::IntOpt) => {
                        self.select_opt();
                        self.game_mode = GameMode::Interact(InterSteps::Feedback);
                    },
                    GameMode::Interact(InterSteps::Feedback) => {
                        // self.select_adj();
                        self.game_mode = GameMode::Play;
                    },
                    _ => self.game_mode = GameMode::Play,
                }

                return false;
            },
            KeyCode::Esc => return false,
            _ => {},
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
                    return self.play_key(event.code);
                } else {true}
            } else {true}
        } else {true}
    }

    fn item_interaction(&mut self) -> bool {
        self.gui.reset_cursor();
        loop {
            self.gui.inter_opt_draw(self.map.clone(), self.map_location(), self.player.clone(), self.enemies.clone(), self.items.clone(), self.npcs.clone());
            if poll(std::time::Duration::from_millis(100)).unwrap() {
                if let Event::Key(event) = read().unwrap() {
                    // log::info!("keykind {:?}", event.kind.clone());
                    let now = Instant::now();
                    if now.duration_since(self.last_event_time) > self.key_debounce_dur {
                        self.last_event_time = now;
                        let res = self.inter_key(event.code);
                        if !res {
                            break
                        }
                    }
                }
            }
        }
        self.gui.reset_cursor();
        loop {
            self.gui.inter_res_draw(self.map.clone(), self.map_location(), self.player.clone(), self.enemies.clone(), self.items.clone(), self.npcs.clone());
            if poll(std::time::Duration::from_millis(100)).unwrap() {
                if let Event::Key(event) = read().unwrap() {
                    // log::info!("keykind {:?}", event.kind.clone());
                    let now = Instant::now();
                    if now.duration_since(self.last_event_time) > self.key_debounce_dur {
                        self.last_event_time = now;
                        let res = self.inter_key(event.code);
                        if !res {
                            break
                        }
                    }
                }
            }
        }
        self.gui.reset_cursor();
        // self.gui.set_info_mode(GUIMode::Normal);
        true
    }

    fn npc_comm_inter(&mut self, mut npc: CommNPC) -> bool {
        self.gui.reset_cursor();
        let comms = format!("{}#{}", npc.get_sname(), npc.get_comm());
        loop {
            self.gui.npc_comm_draw(comms.clone(), self.map.clone(), self.map_location(), self.player.clone(), self.enemies.clone(), self.items.clone(), self.npcs.clone());
            if poll(std::time::Duration::from_millis(100)).unwrap() {
                if let Event::Key(event) = read().unwrap() {
                    // log::info!("keykind {:?}", event.kind.clone());
                    let now = Instant::now();
                    if now.duration_since(self.last_event_time) > self.key_debounce_dur {
                        self.last_event_time = now;
                        let res = self.comm_key(event.code);
                        if !res {
                            break
                        }
                    }
                }
            }
        }
        true
    }

    fn npc_interaction(&mut self) -> bool {
        let npc = self.interactee.clone();
        match npc {
            Interactable::NPC(NPCWrap::CommNPC(comm_npc)) => self.npc_comm_inter(comm_npc),
            _ => todo!(),
        }

    }

    fn interaction(&mut self) -> bool {
        self.gui.reset_cursor();
        loop {
            self.gui.inter_adj_draw(self.map.clone(), self.map_location(), self.player.clone(), self.enemies.clone(), self.items.clone(), self.npcs.clone());
            if poll(std::time::Duration::from_millis(100)).unwrap() {
                if let Event::Key(event) = read().unwrap() {
                    // log::info!("keykind {:?}", event.kind.clone());
                    let now = Instant::now();
                    if now.duration_since(self.last_event_time) > self.key_debounce_dur {
                        self.last_event_time = now;
                        let res = self.inter_key(event.code);
                        if !res {
                            break
                        }
                    }
                }
            }
        }

        let intee = self.interactee.clone();
        let res = match intee {
            Interactable::Item(_) => self.item_interaction(),
            Interactable::NPC(_) => self.npc_interaction(),
            Interactable::Enemy(e) => {
                self.game_mode = GameMode::Fight(FightSteps::Open);
                self.enemy_encounter(e);
                true
            },
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
        let types = vec![Items::Rock, Items::EdibleRoot, Items::Apple, Items::MetalScrap];
        if self.map.cells[y][x] == Cells::Empty && !self.enemies.contains_key(&(x, y)) && !self.items.contains_key(&(x, y)) {
            if let Some(i_type) = types.choose(&mut rng){
                let mut prop = HashMap::new();
                let mut itype = String::new();
                let mut desc = String::new();
                let mut iopts = HashMap::new();
                match i_type {
                    Items::EdibleRoot => {
                        prop.insert(String::from("Health"), 5);
                        itype = String::from("Edible Root");
                        desc = String::from("Weird looking root, doesnt look very tasty.");
                        iopts.insert(InterOpt::Item(ItemOpt::PickUp), String::from("Pick Up"));
                        iopts.insert(InterOpt::Item(ItemOpt::Drp), String::from("Drop"));
                        iopts.insert(InterOpt::Item(ItemOpt::Use), String::from("Use"));
                    },
                    Items::Apple => {
                        prop.insert(String::from("Health"), 10);
                        prop.insert(String::from("Value"), 5);
                        itype = String::from("Apple");
                        desc = String::from("An slightly bruised apple that as been here for a while.");
                        iopts.insert(InterOpt::Item(ItemOpt::PickUp), String::from("Pick Up"));
                        iopts.insert(InterOpt::Item(ItemOpt::Drp), String::from("Drop"));
                        iopts.insert(InterOpt::Item(ItemOpt::Use), String::from("Use"));
                    },
                    Items::MetalScrap => {
                        prop.insert(String::from("Health"), 0);
                        prop.insert(String::from("Value"), 1);
                        itype = String::from("Metal Scrap");
                        desc = String::from("Scrap of metal");
                        iopts.insert(InterOpt::Item(ItemOpt::PickUp), String::from("Pick Up"));
                        iopts.insert(InterOpt::Item(ItemOpt::Drp), String::from("Drop"));
                    },
                    Items::Rock => {
                        prop.insert(String::from("Health"), 0);
                        itype = String::from("Rock");
                        desc = String::from("Its a rock.");
                        iopts.insert(InterOpt::Item(ItemOpt::PickUp), String::from("Pick Up"));
                        iopts.insert(InterOpt::Item(ItemOpt::Drp), String::from("Drop"));
                    },
                    _ => todo!(),
                };
                let i_temp = Item::new(*i_type, itype, desc, iopts, x, y, prop);
                self.items.insert((x, y), i_temp);
                return true;
            }
        }
        false
    }

    fn repop_items(&mut self) {
        let mut rng = rand::thread_rng();
        let (vx, vy, vw, vh) = self.map.get_viewport();
        //xx
        match (self.map.gen_x, self.map.gen_y) {
            (x, y) if x < 0 && y == 0 => {
                for _ in 0..50 {
                    loop {
                        let x = rng.gen_range(10..vx-5);
                        let y = rng.gen_range(10..MAP_H-10);
                        let res = self.check_place_item(x, y);
                        if res {break;}
                    }
                }
            },
            (x, y) if x > 0 && y == 0 => {
                for _ in 0..50 {
                    loop {
                        let x = rng.gen_range((vx + vw + 5)..MAP_W-10);
                        let y = rng.gen_range(10..MAP_H-10);
                        let res = self.check_place_item(x, y);
                        if res {break;}
                    }
                }
            },
            (x, y) if y < 0 && x == 0 => {
                for _ in 0..50 {
                    loop {
                        let x = rng.gen_range(10..MAP_W-10);
                        let y = rng.gen_range(10..vy-5);
                        let res = self.check_place_item(x, y);
                        if res {break;}
                    }
                }
            },
            (x, y) if y > 0 && x == 0 => {
                for _ in 0..50 {
                    loop {
                        let x = rng.gen_range(10..MAP_W-10);
                        let y = rng.gen_range((vy + vh + 5)..MAP_H-10);
                        let res = self.check_place_item(x, y);
                        if res {break;}
                    }
                }
            }, // asdf
            (x, y) if x > 0 && y > 0 => {
                for _ in 0..25 {
                    loop {
                        let x = rng.gen_range((vx + vw + 5)..MAP_W-10);
                        let y = rng.gen_range(10..MAP_H-10);
                        let res = self.check_place_item(x, y);
                        if res {break;}
                    }
                }
                for _ in 0..25 {
                    loop {
                        let x = rng.gen_range(10..MAP_W-10);
                        let y = rng.gen_range((vy + vh + 5)..MAP_H-10);
                        let res = self.check_place_item(x, y);
                        if res {break;}
                    }
                }
            },
            (x, y) if x > 0 && y < 0 => {
                for _ in 0..25 {
                    loop {
                        let x = rng.gen_range((vx + vw + 5)..MAP_W-10);
                        let y = rng.gen_range(10..MAP_H-10);
                        let res = self.check_place_item(x, y);
                        if res {break;}
                    }
                }
                for _ in 0..25 {
                    loop {
                        let x = rng.gen_range(10..MAP_W-10);
                        let y = rng.gen_range(10..vy-5);
                        let res = self.check_place_item(x, y);
                        if res {break;}
                    }
                }
            },
            (x, y) if x < 0 && y > 0 => {
                for _ in 0..25 {
                    loop {
                        let x = rng.gen_range(10..vx-5);
                        let y = rng.gen_range(10..MAP_H-10);
                        let res = self.check_place_item(x, y);
                        if res {break;}
                    }
                }
                for _ in 0..25 {
                    loop {
                        let x = rng.gen_range(10..MAP_W-10);
                        let y = rng.gen_range((vy + vh + 5)..MAP_H-10);
                        let res = self.check_place_item(x, y);
                        if res {break;}
                    }
                }
            },
            (x, y) if x < 0 && y < 0 => {
                for _ in 0..25 {
                    loop {
                        let x = rng.gen_range(10..vx-5);
                        let y = rng.gen_range(10..MAP_H-10);
                        let res = self.check_place_item(x, y);
                        if res {break;}
                    }
                }
                for _ in 0..25 {
                    loop {
                        let x = rng.gen_range(10..MAP_W-10);
                        let y = rng.gen_range(10..vy-5);
                        let res = self.check_place_item(x, y);
                        if res {break;}
                    }
                }
            },
            _ => {},
        }
    }

    fn check_place_npcs(&mut self, x: usize, y: usize) -> bool {
        let data1 = fs::read_to_string("src/npcs/npc_names.json");
        log::info!("{:?}", &data1);
        let names: Vec<String> = match data1 {
            Ok(content) => serde_json::from_str(&content).unwrap(),
            Err(e) => {
                log::info!("{:?}", e);
                Vec::new()
            },
        };
        let data2 = fs::read_to_string("src/npcs/npc_comms.json");
        log::info!("{:?}", &data2);
        let comms: Vec<String> = match data2 {
            Ok(content) => serde_json::from_str(&content).unwrap(),
            Err(e) => {
                log::info!("{:?}", e);
                Vec::new()
            },
        };
        let data3 = fs::read_to_string("src/npcs/npc_convos.json");
        log::info!("{:?}", &data3);
        let convos: Vec<Convo> = match data3 {
            Ok(content) => serde_json::from_str(&content).unwrap(),
            Err(e) => {
                log::info!("{:?}", e);
                Vec::new()
            },
        };
        let data4 = fs::read_to_string("src/npcs/npc_quests.json");
        log::info!("{:?}", &data4);
        let quests: HashMap<String, NQuest> = match data4 {
            Ok(content) => serde_json::from_str(&content).unwrap(),
            Err(e) => {
                log::info!("{:?}", e);
                HashMap::new()
            },
        };
        // let mut npcs = HashMap::new();
        let mut rng = rand::thread_rng();
        let types = vec![NPCs::CommNPC];
        // let types = vec![NPCs::CommNPC, NPCs::ConvNPC, NPCs::QuestNPC];
        if self.map.cells[y][x] == Cells::Empty && !self.enemies.contains_key(&(x, y)) && !self.items.contains_key(&(x, y)) && !self.npcs.contains_key(&(x, y)) {
            if let Some(i_type) = types.choose(&mut rng){
                let npc = match i_type {
                    NPCs::CommNPC => {
                        let sname = &names[0];
                        let comm: Vec<String> = comms.clone();
                        NPCWrap::CommNPC(new_comm_npc(sname.to_string(), x, y, comm))
                    },
                    NPCs::ConvNPC => {
                        let sname = &names[0];
                        let conv: Convo = convos[0].clone();
                        NPCWrap::ConvNPC(new_conv_npc(sname.to_string(), x, y, conv))
                    },
                    NPCs::QuestNPC => {
                        let sname = &names[0];
                        let quest: NQuest = quests["get_quest"].clone();
                        NPCWrap::QuestNPC(new_quest_npc(sname.to_string(), x, y, quest))
                    },
                    _ => todo!(),
                };
                self.npcs.insert((x, y), npc);
                return true;
            }
        }
        false
    }

    fn repop_npcs(&mut self) {
        let mut rng = rand::thread_rng();
        let (vx, vy, vw, vh) = self.map.get_viewport();
        //xx
        match (self.map.gen_x, self.map.gen_y) {
            (x, y) if x < 0 && y == 0 => {
                for _ in 0..20 {
                    loop {
                        let x = rng.gen_range(10..vx-5);
                        let y = rng.gen_range(10..MAP_H-10);
                        let res = self.check_place_npcs(x, y);
                        if res {break;}
                    }
                }
            },
            (x, y) if x > 0 && y == 0 => {
                for _ in 0..20 {
                    loop {
                        let x = rng.gen_range((vx + vw + 5)..MAP_W-10);
                        let y = rng.gen_range(10..MAP_H-10);
                        let res = self.check_place_npcs(x, y);
                        if res {break;}
                    }
                }
            },
            (x, y) if y < 0 && x == 0 => {
                for _ in 0..20 {
                    loop {
                        let x = rng.gen_range(10..MAP_W-10);
                        let y = rng.gen_range(10..vy-5);
                        let res = self.check_place_npcs(x, y);
                        if res {break;}
                    }
                }
            },
            (x, y) if y > 0 && x == 0 => {
                for _ in 0..20 {
                    loop {
                        let x = rng.gen_range(10..MAP_W-10);
                        let y = rng.gen_range((vy + vh + 5)..MAP_H-10);
                        let res = self.check_place_npcs(x, y);
                        if res {break;}
                    }
                }
            }, // asdf
            (x, y) if x > 0 && y > 0 => {
                for _ in 0..10 {
                    loop {
                        let x = rng.gen_range((vx + vw + 5)..MAP_W-10);
                        let y = rng.gen_range(10..MAP_H-10);
                        let res = self.check_place_npcs(x, y);
                        if res {break;}
                    }
                }
                for _ in 0..10 {
                    loop {
                        let x = rng.gen_range(10..MAP_W-10);
                        let y = rng.gen_range((vy + vh + 5)..MAP_H-10);
                        let res = self.check_place_npcs(x, y);
                        if res {break;}
                    }
                }
            },
            (x, y) if x > 0 && y < 0 => {
                for _ in 0..10 {
                    loop {
                        let x = rng.gen_range((vx + vw + 5)..MAP_W-10);
                        let y = rng.gen_range(10..MAP_H-10);
                        let res = self.check_place_npcs(x, y);
                        if res {break;}
                    }
                }
                for _ in 0..10 {
                    loop {
                        let x = rng.gen_range(10..MAP_W-10);
                        let y = rng.gen_range(10..vy-5);
                        let res = self.check_place_npcs(x, y);
                        if res {break;}
                    }
                }
            },
            (x, y) if x < 0 && y > 0 => {
                for _ in 0..10 {
                    loop {
                        let x = rng.gen_range(10..vx-5);
                        let y = rng.gen_range(10..MAP_H-10);
                        let res = self.check_place_npcs(x, y);
                        if res {break;}
                    }
                }
                for _ in 0..10 {
                    loop {
                        let x = rng.gen_range(10..MAP_W-10);
                        let y = rng.gen_range((vy + vh + 5)..MAP_H-10);
                        let res = self.check_place_npcs(x, y);
                        if res {break;}
                    }
                }
            },
            (x, y) if x < 0 && y < 0 => {
                for _ in 0..10 {
                    loop {
                        let x = rng.gen_range(10..vx-5);
                        let y = rng.gen_range(10..MAP_H-10);
                        let res = self.check_place_npcs(x, y);
                        if res {break;}
                    }
                }
                for _ in 0..10 {
                    loop {
                        let x = rng.gen_range(10..MAP_W-10);
                        let y = rng.gen_range(10..vy-5);
                        let res = self.check_place_npcs(x, y);
                        if res {break;}
                    }
                }
            },
            _ => {},
        }
    }

    pub fn start_update_threads(game_state: Arc<Mutex<Self>>) {
        // let game_state = Arc::new(Mutex::new(self));

        let game_clone = Arc::clone(&game_state);
        thread::spawn(move || {
            loop {
                log::info!("update npc pre");
                {
                    let mut game = game_clone.lock().unwrap();
                    // game.update_npcs();
                    log::info!("update npc");
                    let step = game.step_group;
                    game.update_enemies(step.clone());
                    game.update_npcs(step.clone());
                    if step < 15 {
                        game.step_group += 1;
                    } else {
                        game.step_group = 0;
                    }
                }
                thread::sleep(Duration::from_millis(15));
            }
        });
    }

    fn location_check(&mut self) {
        if let Some(settlement) = self.settles.check_location(self.dist_fo.clone(), self.loc_rad.clone()) {
            self.location = Location::Settlement(settlement);
        };
    }

    fn update_settlement(&self, settle: Settlement) {
        // let map = settle.get_map();
        // let pos = settle.get_pos();
    }

    fn update_location(&self) {
        let location = self.location.clone();
        match location {
            Location::Settlement(settle) => self.update_settlement(settle),
            _ => todo!(),
        }
    }

    pub fn update(&mut self) -> bool {
        let (vw, vh) = self.gui.get_viewport();
        self.map.set_viewport(vh, vw);
        log::info!("update");
        let res = match self.game_mode {
            GameMode::Play => self.play_update(),
            GameMode::Interact(_) => self.interaction(),
            GameMode::Fight(_) => {
                let Interactable::Enemy(e) = self.interactee.clone() else {todo!()};
                self.enemy_encounter(e);
                true
            },
            _ => todo!(),
        };

        if !res {
            return false;
        }

        if self.items.len() < 150 {
            self.repop_items();
        }

        if self.npcs.len() < 30 {
            self.repop_npcs();
        }

        let ppos = (self.player.x, self.player.y);

        if let Some(e) = self.enemies.get(&(ppos)) {
            self.interactee = Interactable::Enemy(e.clone());
            self.game_mode = GameMode::Fight(FightSteps::Open);
        }

        self.location_check();
        if self.location != Location::Null {
            self.update_location();
        }

        true
    }

    fn map_location(&self) -> Vec<Vec<Cells>> {
        if self.location != Location::Null {
            let (lpos, lmap) = match self.location.clone() {
                Location::Settlement(mut settle) => {
                    let p = settle.get_pos();
                    let m = settle.get_map();
                    (p, m)
                },
                _ => todo!(),
            };
            let mut map_vec = self.map.cells.clone();
            let pos = self.dist_fo;
            for (i, row) in lmap.iter().enumerate() {
                for (j, &cell) in row.iter().enumerate() {
                    let main_i = (pos.1 + i as i64 + lpos.1) as usize;
                    let main_j = (pos.0 + j as i64 + lpos.0) as usize;
                    if main_i < map_vec.len() && main_j < map_vec[0].len() {
                        map_vec[main_i][main_j] = cell;
                    }
                }
            }
            // log::info!("map\n{:?}", map_vec.clone());
            map_vec.clone()
        } else {self.map.cells.clone()}
    }


    pub fn draw(&mut self) {
        self.gui.draw(self.map.clone(), self.map_location(), self.player.clone(), self.enemies.clone(), self.items.clone(), self.npcs.clone());
    }
}
