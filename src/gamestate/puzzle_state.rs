//puzzle_state
use crate::enums::{Cells, CompMode, GUIMode, GameMode, Interactable, Location, NPCWrap};
use crate::gamestate::GameState;
use crate::gui_utils::GuiArgs;
use crate::map::{MAP_H, MAP_W};
use crate::npc::NPC;
use crate::puzzle::Puzzle;
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
        let dx = (lpos.0 + pos.0).abs();
        let dy = (lpos.1 + pos.1).abs();
        if dx < (MAP_W + MAP_W).try_into().unwrap()
            && dy < (MAP_H + MAP_H).try_into().unwrap()
            && !puzzle.is_prop_pass()
        {
            let ports = puzzle.get_portals();
            for ((ix, iy), (ox, oy)) in ports {
                let i_npos = (
                    (self.dist_fo.0 + ix as i16 + lpos.0) as usize,
                    (self.dist_fo.1 + iy as i16 + lpos.1) as usize,
                );
                let o_npos = (
                    (self.dist_fo.0 + ox as i16 + lpos.0) as usize,
                    (self.dist_fo.1 + oy as i16 + lpos.1) as usize,
                );
                self.portals.insert(i_npos, o_npos);
            }
            puzzle.toggle_ppass();
        }
        Location::Puzzle(puzzle)
    }

    fn portal_shift(&mut self, npos: (usize, usize), ppos: (usize, usize)) {
        //move player
        // let tnpos = {
        //     let map = self.map.cells.clone();
        //     match map {
        //         map if map[npos.1][npos.0 + 1] == Cells::Empty => (npos.0 + 1, npos.1),
        //         map if map[npos.1][npos.0 - 1] == Cells::Empty => (npos.0 - 1, npos.1),
        //         map if map[npos.1 + 1][npos.0] == Cells::Empty => (npos.0, npos.1 + 1),
        //         map if map[npos.1 - 1][npos.0] == Cells::Empty => (npos.0, npos.1 - 1),
        //         _ => (ppos.0, ppos.1 - 1),
        //     }
        // };
        let tnpos = npos;
        log::info!("ppos: {:#?}\nnpos: {:#?}", ppos, npos);
        self.player.set_pos((tnpos.0, tnpos.1));
        //move map
        //move gs: items, npcs, enemies, (anything with shift)
        // let dx = ppos.0 as i16 - tnpos.0 as i16;
        // let dy = ppos.1 as i16 - tnpos.1 as i16;
        // let dx = tnpos.0 as i16 - ppos.0 as i16;
        // let dy = tnpos.1 as i16 - ppos.1 as i16;

        let center = (MAP_W / 2, MAP_H / 2);
        let dcen = (
            (center.0 as i16 - tnpos.0 as i16),
            (center.1 as i16 - tnpos.1 as i16),
        );

        let tdfo = self.dist_fo;
        let ndfo = (tdfo.0 + dcen.0 as i16, tdfo.1 + dcen.1 as i16);
        log::info!("dcen: {:#?}\ndfo: {:#?}\nndfo {:#?}", dcen, tdfo, ndfo);
        self.dist_fo = ndfo;
        // self.translate_state(dcen.0, dcen.1);
        self.map.center_player(tnpos.0, tnpos.1);
    }

    pub fn portal_check(&mut self) -> bool {
        let plyr = self.player.clone();
        let ppos = plyr.get_pos();
        if let Some((x, y)) = self.portals.get(&(ppos.0, ppos.1)) {
            self.portal_shift((*x, *y), ppos);
            return true;
        }
        false
    }
}
