use crate::enums::{Cells, NPCWrap, NPCs, PuzzleType};
use crate::gamestate::GameState;
use crate::map::{MAP_H, MAP_W};
use crate::npc::{new_comm_npc, new_conv_npc, new_spawn_npc, new_trade_npc, ConOpt, Convo, Stage};
use crate::puzzle::Puzzle;

use crate::npc_utils::box_npc;
use crate::npc_utils::npc_move;
use crate::npc_utils::wrap_nbox;
use rand::prelude::SliceRandom;
use rand::Rng;
use std::collections::HashMap;

impl GameState {
    pub fn update_npcs(&mut self, step: u8) {
        let mut n_temp = self.npcs.clone();
        let mut new_n = HashMap::new();
        let mh = self.map.cells.len();
        let mw = self.map.cells[0].len();
        for ((x, y), n) in &mut n_temp {
            let mut nbox = box_npc(n.clone());
            if nbox.get_step_grp() != step || *x < 200 || *x > 400 || *y < 180 || *y > 225 {
                new_n.insert((*x, *y), wrap_nbox(nbox));
            } else {
                let (pos, nnpc) = npc_move(nbox, self.map.cells.clone(), mw, mh, *x, *y);
                let bwrp = wrap_nbox(nnpc);
                new_n.insert(pos, bwrp);
            }
        }
        self.npcs = new_n;
    }

    pub fn shift_npcs(&mut self, dir: &str) {
        let temp_n = self.npcs.clone();
        let mut new_n = HashMap::new();
        let mw = self.map.cells[0].len();
        let mh = self.map.cells.len();
        for ((x, y), n) in temp_n {
            let mut nbox = box_npc(n);
            match dir {
                "UP" => {
                    if y < mh - 10 {
                        nbox.mmove("DN");
                        let npc_w = wrap_nbox(nbox);
                        new_n.insert((x, y + 1), npc_w.clone());
                    }
                }
                "DN" => {
                    if y > 10 {
                        nbox.mmove("UP");
                        let npc_w = wrap_nbox(nbox);
                        new_n.insert((x, y - 1), npc_w.clone());
                    }
                }
                "LF" => {
                    if x < mw - 10 {
                        nbox.mmove("RT");
                        let npc_w = wrap_nbox(nbox);
                        new_n.insert((x + 1, y), npc_w.clone());
                    }
                }
                "RT" => {
                    if x > 10 {
                        nbox.mmove("LF");
                        let npc_w = wrap_nbox(nbox);
                        new_n.insert((x - 1, y), npc_w.clone());
                    }
                }
                _ => todo!(),
            };
        }
        self.npcs = new_n;
    }

    fn build_maze_npc(&self, mut puzzle: (i64, Puzzle)) -> (String, Convo, Vec<String>) {
        let mut rng = rand::thread_rng();
        let name = self
            .npc_names
            .choose(&mut rng)
            .unwrap_or(&"Kevthony".to_string())
            .clone();
        let rnd_comms = {
            let mut tvec = Vec::new();
            for _ in 0..4 {
                let tidx = rng.gen_range(0..self.npc_spcomms.len());
                tvec.push(self.npc_spcomms[tidx].clone());
            }
            tvec
        };
        let convos = self.npc_spconvos.get("maze").unwrap();
        let mut conv: Convo = convos
            .choose(&mut rng)
            .unwrap_or(&convos[0].clone())
            .clone();
        let ppos = puzzle.1.get_pos();
        let dx = ppos.0 - -self.dist_fo.0;
        let dy = ppos.1 - -self.dist_fo.1;
        let dir = match (dx, dy) {
            (x, y) if x < 0 && y < 0 => "North West",
            (x, y) if x < 0 && y > 0 => "South West",
            (x, y) if x > 0 && y < 0 => "North East",
            (x, y) if x > 0 && y > 0 => "South East",
            (x, y) if x < 0 && y == 0 => "West",
            (x, y) if x > 0 && y == 0 => "East",
            (x, y) if x == 0 && y < 0 => "North",
            (x, y) if x == 0 && y > 0 => "South",
            _ => "",
        };
        let dis = puzzle.0;
        let text = format!("It was about {dis} paces {dir}");

        let stage = Stage {
            text,
            opts: vec![
                ConOpt {
                    text: "Thank's Ill look for it. Good luck!".to_string(),
                    next: "e".to_string(),
                },
                ConOpt {
                    text: "What did it look like?".to_string(),
                    next: "desc".to_string(),
                },
            ],
        };
        conv.stages.insert("where".to_string(), stage);

        (name, conv, rnd_comms)
    }

    pub fn check_place_npcs(&mut self, x: usize, y: usize) -> bool {
        let mut rng = rand::thread_rng();
        let types = {
            let rnd = rng.gen_range(0..30);
            if rnd == 0 {
                vec![NPCs::CommNPC, NPCs::ConvNPC, NPCs::SpawnNPC]
            } else {
                vec![NPCs::CommNPC, NPCs::ConvNPC, NPCs::TradeNPC, NPCs::SpawnNPC]
            }
        };
        if self.map.cells[y][x] == Cells::Empty
            && !self.in_loc_check((x, y))
            && !self.enemies.contains_key(&(x, y))
            && !self.items.contains_key(&(x, y))
            && !self.npcs.contains_key(&(x, y))
        {
            if let Some(i_type) = types.choose(&mut rng) {
                let def_name = "Kevthony".to_string();
                let npc = match i_type {
                    NPCs::CommNPC => {
                        let rnd_comms = {
                            let mut tvec = Vec::new();
                            for _ in 0..4 {
                                let tidx = rng.gen_range(0..self.npc_comms.len());
                                tvec.push(self.npc_comms[tidx].clone());
                            }
                            tvec
                        };
                        let name = self
                            .npc_names
                            .choose(&mut rng)
                            .unwrap_or(&def_name.clone())
                            .clone();
                        NPCWrap::CommNPC(new_comm_npc(name.to_string(), x, y, rnd_comms))
                    }
                    NPCs::ConvNPC => {
                        let conv: Convo = self
                            .npc_convos
                            .choose(&mut rng)
                            .unwrap_or(&self.npc_convos[0].clone())
                            .clone();
                        let name = self
                            .npc_names
                            .choose(&mut rng)
                            .unwrap_or(&def_name.clone())
                            .clone();
                        NPCWrap::ConvNPC(new_conv_npc(name.to_string(), x, y, conv))
                    }
                    NPCs::SpawnNPC => {
                        let mut puzzle = self.puzzles.nearest_puzzle(self.dist_fo);
                        let ptype = puzzle.1.get_ptype();
                        let parts = match ptype {
                            PuzzleType::Maze => self.build_maze_npc(puzzle),
                            // PuzzleType::Teleport => self.build_teleport_npc(puzzle),
                            // PuzzleType::Inverted => self.build_inverted_npc(puzzle),
                            _ => todo!(),
                        };
                        NPCWrap::SpawnNPC(new_spawn_npc(parts.0, x, y, parts.1, parts.2, ptype))
                    }
                    NPCs::TradeNPC => {
                        let name = self
                            .npc_names
                            .choose(&mut rng)
                            .unwrap_or(&def_name.clone())
                            .clone();
                        let conv = self
                            .npc_trade
                            .choose(&mut rng)
                            .unwrap_or(&self.npc_trade[0].clone())
                            .clone();
                        let items = self.pop_trade_items();
                        NPCWrap::TradeNPC(new_trade_npc(name.to_string(), x, y, items, conv))
                    }
                    _ => todo!(),
                };
                self.npcs.insert((x, y), npc);
                return true;
            }
        }
        false
    }

    pub fn repop_npcs(&mut self) {
        let mut rng = rand::thread_rng();
        let (vx, vy, vw, vh) = self.map.get_viewport();
        //xx
        match (-self.map.gen_x, -self.map.gen_y) {
            (x, y) if x < 0 && y == 0 => {
                for _ in 0..20 {
                    loop {
                        let x = rng.gen_range(10..vx - 5);
                        let y = rng.gen_range(10..MAP_H - 10);
                        let res = self.check_place_npcs(x, y);
                        if res {
                            break;
                        }
                    }
                }
            }
            (x, y) if x > 0 && y == 0 => {
                for _ in 0..20 {
                    loop {
                        let x = rng.gen_range((vx + vw + 5)..MAP_W - 10);
                        let y = rng.gen_range(10..MAP_H - 10);
                        let res = self.check_place_npcs(x, y);
                        if res {
                            break;
                        }
                    }
                }
            }
            (x, y) if y < 0 && x == 0 => {
                for _ in 0..20 {
                    loop {
                        let x = rng.gen_range(10..MAP_W - 10);
                        let y = rng.gen_range(10..vy - 5);
                        let res = self.check_place_npcs(x, y);
                        if res {
                            break;
                        }
                    }
                }
            }
            (x, y) if y > 0 && x == 0 => {
                for _ in 0..20 {
                    loop {
                        let x = rng.gen_range(10..MAP_W - 10);
                        let y = rng.gen_range((vy + vh + 5)..MAP_H - 10);
                        let res = self.check_place_npcs(x, y);
                        if res {
                            break;
                        }
                    }
                }
            } // asdf
            (x, y) if x > 0 && y > 0 => {
                for _ in 0..10 {
                    loop {
                        let x = rng.gen_range((vx + vw + 5)..MAP_W - 10);
                        let y = rng.gen_range(10..MAP_H - 10);
                        let res = self.check_place_npcs(x, y);
                        if res {
                            break;
                        }
                    }
                }
                for _ in 0..10 {
                    loop {
                        let x = rng.gen_range(10..MAP_W - 10);
                        let y = rng.gen_range((vy + vh + 5)..MAP_H - 10);
                        let res = self.check_place_npcs(x, y);
                        if res {
                            break;
                        }
                    }
                }
            }
            (x, y) if x > 0 && y < 0 => {
                for _ in 0..10 {
                    loop {
                        let x = rng.gen_range((vx + vw + 5)..MAP_W - 10);
                        let y = rng.gen_range(10..MAP_H - 10);
                        let res = self.check_place_npcs(x, y);
                        if res {
                            break;
                        }
                    }
                }
                for _ in 0..10 {
                    loop {
                        let x = rng.gen_range(10..MAP_W - 10);
                        let y = rng.gen_range(10..vy - 5);
                        let res = self.check_place_npcs(x, y);
                        if res {
                            break;
                        }
                    }
                }
            }
            (x, y) if x < 0 && y > 0 => {
                for _ in 0..10 {
                    loop {
                        let x = rng.gen_range(10..vx - 5);
                        let y = rng.gen_range(10..MAP_H - 10);
                        let res = self.check_place_npcs(x, y);
                        if res {
                            break;
                        }
                    }
                }
                for _ in 0..10 {
                    loop {
                        let x = rng.gen_range(10..MAP_W - 10);
                        let y = rng.gen_range((vy + vh + 5)..MAP_H - 10);
                        let res = self.check_place_npcs(x, y);
                        if res {
                            break;
                        }
                    }
                }
            }
            (x, y) if x < 0 && y < 0 => {
                for _ in 0..10 {
                    loop {
                        let x = rng.gen_range(10..vx - 5);
                        let y = rng.gen_range(10..MAP_H - 10);
                        let res = self.check_place_npcs(x, y);
                        if res {
                            break;
                        }
                    }
                }
                for _ in 0..10 {
                    loop {
                        let x = rng.gen_range(10..MAP_W - 10);
                        let y = rng.gen_range(10..vy - 5);
                        let res = self.check_place_npcs(x, y);
                        if res {
                            break;
                        }
                    }
                }
            }
            _ => {}
        }
    }
}
