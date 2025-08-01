use rand::Rng;

use crate::enums::{Cells, CompMode, Location};
use crate::gamestate::in_range;
use crate::gamestate::GameState;
use std::collections::HashMap;

impl GameState {
    pub fn map_location(&mut self) {
        if self.location != Location::Null {
            let (lpos, lmap) = match self.location.clone() {
                Location::Settlement(settle) => {
                    let p = settle.pos;
                    let m = settle.map;
                    (p, m)
                }
                Location::Puzzle(puzzle) => {
                    let p = puzzle.pos;
                    let m = puzzle.map;
                    (p, m)
                }
                Location::Feature(feature) => {
                    let p = feature.pos;
                    let m = if feature.hermit {
                        feature.hermit_map
                    } else {
                        feature.map
                    };
                    (p, m)
                }
                _ => todo!(),
            };
            let mut map_vec = self.map.cells.clone();
            let pos = self.dist_fo;
            for (i, row) in lmap.iter().enumerate() {
                for (j, &cell) in row.iter().enumerate() {
                    if cell != Cells::Transparent {
                        let main_i = (pos.1 + i as i16 + lpos.1) as usize;
                        let main_j = (pos.0 + j as i16 + lpos.0) as usize;
                        if main_i < map_vec.len() && main_j < map_vec[0].len() {
                            map_vec[main_i][main_j] = cell;
                        }
                    }
                }
            }
            self.map.cells = map_vec.clone()
        }
    }

    pub fn compass_check(&mut self) {
        let spos_list = self.settles.get_compass_pos();
        if spos_list.len() > self.comp_list.len() {
            self.comp_list = spos_list.clone();
        }
        if self.comp_mode == CompMode::Location {
            return;
        }
        let dfo = self.dist_fo;
        let mut distances = HashMap::new();
        let mut d_min = 0;
        for ((x, y), _) in spos_list {
            let (dx, dy) = ((x - -dfo.0) as i32, (y - -dfo.1) as i32);
            let hyp = ((dx.pow(2) + dy.pow(2)) as f64).sqrt() as u16;
            if hyp < d_min || d_min == 0 {
                d_min = hyp;
            }
            // d_min = hyp;
            distances.insert(hyp, (x, y));
        }
        self.comp_head = distances[&d_min];
        let comp_names = self.sort_comp_list();
        self.gui.set_comp_list(comp_names);
    }

    pub fn new_loc_check(&mut self) {
        let cpos = self.dist_fo;
        let chyp = ((cpos.0.pow(2) + cpos.1.pow(2)) as f64).sqrt() as u16;
        if chyp > 800 {
            let ks = chyp / 800;
            if ks > self.depth.into() {
                self.settles.spawn_new_settlement(cpos);
                self.depth += 1;
            }
        }
    }

    pub fn update_location(&mut self) {
        let location = self.location.clone();
        self.location = match location {
            Location::Settlement(settle) => self.update_settlement(settle),
            Location::Puzzle(puzzle) => self.update_puzzle(puzzle),
            Location::Feature(feature) => self.update_feature(feature),
            _ => todo!(),
        };
    }

    pub fn location_pos(&mut self) -> (i16, i16) {
        let loc = self.location.clone();
        match loc {
            Location::Settlement(mut settle) => settle.get_pos(),
            Location::Puzzle(mut puzz) => puzz.get_pos(),
            Location::Feature(feat) => feat.pos,
            _ => (0, 0),
        }
    }

    pub fn location_check(&mut self) {
        let mut rng = rand::thread_rng();
        if self.location == Location::Null {
            if let Some(mut feature) = self.features.check_location(self.dist_fo, self.loc_rad / 2)
            {
                if rng.gen_range(0..1) == 0 {
                    feature.place_hermit();
                    feature.place_hermit_parts();
                    feature.hermit = true;
                }
                self.location = Location::Feature(feature);
            }
            if let Some(mut settlement) = self.settles.check_location(self.dist_fo, self.loc_rad) {
                self.notebook
                    .enter_settles(settlement.get_sname(), settlement.get_stats().1);
                self.location = Location::Settlement(settlement);
            };
            if let Some(puzzle) = self.puzzles.check_location(self.dist_fo, self.loc_rad) {
                self.location = Location::Puzzle(puzzle);
            };
        } else {
            match &mut self.location {
                Location::Settlement(ref mut settle) => {
                    let lpos = settle.get_pos();
                    if !in_range(lpos, (-self.dist_fo.0, -self.dist_fo.1), self.loc_rad) {
                        settle.tog_npcs_sent();
                        settle.tog_found();
                        self.settles.update_settlement(settle.clone());
                        self.location = Location::Null;
                    }
                }
                Location::Puzzle(ref mut puzzle) => {
                    let lpos = puzzle.get_pos();
                    if !in_range(lpos, (-self.dist_fo.0, -self.dist_fo.1), self.loc_rad) {
                        self.puzzles.update_puzzle(puzzle.clone());
                        self.location = Location::Null;
                    }
                }
                Location::Feature(ref mut feature) => {
                    if !in_range(
                        feature.pos,
                        (-self.dist_fo.0, -self.dist_fo.1),
                        self.loc_rad,
                    ) {
                        feature.cont_sent = false;
                        feature.hermit = false;
                        self.features.update_feature(feature.clone());
                    }
                }
                _ => todo!(),
            }
        }
    }

    pub fn in_loc_check(&mut self, pos: (usize, usize)) -> bool {
        let loc = self.location.clone();
        let dpos = self.dist_fo;
        match loc {
            Location::Null => false,
            Location::Settlement(mut settle) => {
                let lpos = settle.get_pos();
                let (xx, yy) = ((lpos.0 + dpos.0) as usize, (lpos.1 + dpos.1) as usize);
                pos.0 >= xx && pos.0 <= xx + 150 && pos.1 >= yy && pos.1 <= yy + 50
            }
            Location::Puzzle(mut puzzle) => {
                let lpos = puzzle.get_pos();
                let (xx, yy) = ((lpos.0 + dpos.0) as usize, (lpos.1 + dpos.1) as usize);
                pos.0 >= xx && pos.0 <= xx + 300 && pos.1 >= yy && pos.1 <= yy + 200
            }
            Location::Feature(puzzle) => {
                let lpos = puzzle.pos;
                let (xx, yy) = ((lpos.0 + dpos.0) as usize, (lpos.1 + dpos.1) as usize);
                pos.0 >= xx && pos.0 <= xx + 300 && pos.1 >= yy && pos.1 <= yy + 200
            }
            _ => false,
        }
    }
}
