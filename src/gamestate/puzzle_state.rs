//puzzle_state
use crate::enums::{
    Cells, CompMode, EnvInter, GUIMode, GameMode, Interactable, Location, NPCWrap, PuzzlePiece,
};
use crate::gamestate::GameState;
use crate::gui_utils::GuiArgs;
use crate::map::{MAP_H, MAP_W};
use crate::npc::NPC;
use crate::puzzle::{Puzzle, PuzzleDoor, PuzzleKey};
use crate::settlement::Settlement;

use crate::item::Item;
use std::time::Instant;

use crate::gamestate::in_range;
use crate::gamestate::loc_shop_items;
use crate::npc_utils::box_npc;
use crate::npc_utils::wrap_nbox;
use ratatui::crossterm::event::{poll, read, Event, KeyCode};
use std::collections::HashMap;

impl GameState {
    pub fn update_puzzle(&mut self, mut puzzle: Puzzle) -> Location {
        let lpos = puzzle.get_pos();
        let pos = self.dist_fo;
        let dx = (lpos.0 + pos.0) as usize;
        let dy = (lpos.1 + pos.1) as usize;
        // let dx = (lpos.0 + pos.0).abs();
        // let dy = (lpos.1 + pos.1).abs();
        if dx < MAP_W
            && dy < MAP_H
        // if dx < (MAP_W + MAP_W).try_into().unwrap()
        //     && dy < (MAP_H + MAP_H).try_into().unwrap()
            && !puzzle.is_prop_pass()
        {
            let sitems = puzzle.get_items();
            for ((_x, _y), mut i) in sitems {
                let ipos = i.get_pos();
                if pos == (0, 0) {
                    let npos = (
                        (self.dist_fo.0 + ipos.0 as i16 + lpos.0) as usize,
                        (self.dist_fo.1 + ipos.1 as i16 + lpos.1) as usize,
                    );
                    i.set_pos(npos);
                    self.items.insert(npos, i.clone());
                } else {
                    let npos = (
                        (self.dist_fo.0 + ipos.0 as i16 + lpos.0) as usize,
                        (self.dist_fo.1 + ipos.1 as i16 + lpos.1) as usize,
                    );
                    i.set_pos(npos);
                    self.items.insert(npos, i.clone());
                }
            }
            let tnpcs = puzzle.get_npcs();
            for ((x, y), n) in tnpcs {
                let mut nbox = box_npc(n);
                let nwpos = (
                    (self.dist_fo.0 + x as i16 + lpos.0) as usize,
                    (self.dist_fo.1 + y as i16 + lpos.1) as usize,
                );
                nbox.set_pos(nwpos);
                self.npcs.insert(nwpos, wrap_nbox(nbox));
            }
            log::info!("updating puzzle!!!");
            if let Some(doors) = puzzle.get_doors() {
                for (_, d) in doors {
                    let drs = d.idxs.clone();
                    for (dx, dy) in drs {
                        // log::info!(
                        //     "door: {:#?} |: ({}, {})",
                        //     d,
                        //     (self.dist_fo.0 + dx as i16 + lpos.0) as usize,
                        //     (self.dist_fo.1 + dy as i16 + lpos.1) as usize
                        // );
                        self.puzzle_pieces.insert(
                            (
                                (self.dist_fo.0 + dx as i16 + lpos.0) as usize,
                                (self.dist_fo.1 + dy as i16 + lpos.1) as usize,
                            ),
                            PuzzlePiece::PuzzleDoor(d.clone()),
                        );
                    }
                }
            } else {
                log::info!("No doors!!!");
            }
            if let Some(keys) = puzzle.get_keys() {
                // log::info!("key amt:\n{:#?}", keys);
                for ((x, y), k) in keys {
                    let nwpos = (
                        (self.dist_fo.0 + x as i16 + lpos.0) as usize,
                        (self.dist_fo.1 + y as i16 + lpos.1) as usize,
                    );
                    // log::info!("key: {:#?} |: {:#?}", k, nwpos);
                    self.puzzle_pieces.insert(nwpos, PuzzlePiece::PuzzleKey(k));
                }
            } else {
                log::info!("No keys!!!");
            }

            puzzle.toggle_ppass();
        }
        Location::Puzzle(puzzle)
    }

    pub fn shift_puzzle_pieces(&mut self, dir: &str) {
        let temp_pp = self.puzzle_pieces.clone();
        let mut new_pp = HashMap::new();
        let mw = self.map.cells[0].len();
        let mh = self.map.cells.len();
        for ((x, y), pp) in temp_pp {
            log::info!("pp-[{:#?}]: ({}, {})", pp, x, y);
            match dir {
                "UP" => {
                    if y < mh {
                        new_pp.insert((x, y + 1), pp.clone());
                    }
                }
                "DN" => {
                    if y > 0 {
                        new_pp.insert((x, y - 1), pp.clone());
                    }
                }
                "LF" => {
                    if y < mw {
                        new_pp.insert((x + 1, y), pp.clone());
                    }
                }
                "RT" => {
                    if y > 0 {
                        new_pp.insert((x - 1, y), pp.clone());
                    }
                }
                _ => todo!(),
            }
        }
        self.puzzle_pieces = new_pp;
    }

    pub fn puzzle_door(&mut self, door: PuzzleDoor) -> bool {
        let key = self
            .player
            .puzzle_pieces
            .clone()
            .iter()
            .position(|e| {
                if let PuzzlePiece::PuzzleKey(key) = e {
                    key.set == door.set
                } else {
                    false
                }
            })
            .map(|idx| self.player.puzzle_pieces.remove(idx));
        let drs: Vec<_> = self
            .puzzle_pieces
            .iter()
            .filter_map(|(pos, pp)| match pp {
                PuzzlePiece::PuzzleDoor(dr) if dr.id == door.id => Some(*pos),
                _ => None,
            })
            .collect();
        for pos in drs {
            self.puzzle_pieces.remove(&pos);
        }
        true
    }

    pub fn puzzle_key(&mut self, key: PuzzleKey) -> bool {
        self.player.puzzle_pieces.push(PuzzlePiece::PuzzleKey(key));
        true
    }

    pub fn puzzle_piece_interaction(&mut self, puz_piece: PuzzlePiece) -> bool {
        match puz_piece {
            PuzzlePiece::PuzzleDoor(door) => self.puzzle_door(door),
            PuzzlePiece::PuzzleKey(key) => self.puzzle_key(key),
        }
    }
}
