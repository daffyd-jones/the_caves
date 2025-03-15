//puzzles.rs
//
//use crate::enums{};
use crate::enums::PuzzleType;
use crate::puzzle::Puzzle;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

//#[derive(Serialize, Deserialize, Debug)]
pub struct Puzzles {
    puzzles: HashMap<(i64, i64), Puzzle>,
}

impl Puzzles {
    pub fn new() -> Self {
        let puzzles = HashMap::new();
        Self { puzzles }
    }

    pub fn demo_self() -> Self {
        let pos = (-100, -500);
        let puzzle = Puzzle::new_maze(pos);
        // let puzzle = Puzzle::new_teleport(pos);
        log::info!("\nPuzzleFound: {:?}", puzzle);
        let mut puzzles = HashMap::new();
        puzzles.insert(pos, puzzle);
        Self { puzzles }
    }

    pub fn spawn_new_puzzle(&mut self, pos: (i64, i64), ptype: PuzzleType) -> PuzzleType {
        let new_settle_pos = {
            let mut rng = rand::thread_rng();
            let cxabs = pos.0.abs();
            let cyabs = pos.1.abs();
            let nx = rng.gen_range((cxabs + 300)..(cxabs + 800));
            let ny = rng.gen_range((cyabs + 200)..(cyabs + 600));
            let xdir = pos.0 / cxabs;
            let ydir = pos.1 / cyabs;
            (nx * xdir * -1, ny * ydir * -1)
        };
        let puzzle = {
            match &ptype {
                PuzzleType::Maze => Puzzle::new_maze(pos),
                PuzzleType::Teleport => Puzzle::new_maze(pos),
                PuzzleType::Inverted => Puzzle::new_maze(pos),
            }
        };
        self.puzzles.insert(pos, puzzle.clone());
        ptype
    }

    pub fn check_location(&self, bpos: (i64, i64), rad: u16) -> Option<Puzzle> {
        for (ppos, p) in &self.puzzles {
            let xx = ppos.0 - bpos.0 * -1;
            let yy = ppos.1 - bpos.1 * -1;
            let hyp = ((xx.pow(2) + yy.pow(2)) as f64).sqrt() as i64;
            if hyp <= rad.into() {
                return Some(p.clone());
            }
        }
        return None;
    }

    pub fn update_puzzle(&mut self, mut puzzle: Puzzle) {
        let ppos = puzzle.get_pos();
        self.puzzles.insert(ppos, puzzle);
    }

    pub fn get_local_puzzles(&mut self, pos: (i64, i64)) -> HashMap<(i64, i64), Puzzle> {
        let mut local_ps = HashMap::new();
        for (ppos, p) in &self.puzzles {
            let xx = ppos.0 - pos.0 * -1;
            let yy = ppos.1 - pos.1 * -1;
            let hyp = ((xx.pow(2) + yy.pow(2)) as f64).sqrt() as i64;
            if hyp <= 2000.into() {
                local_ps.insert(ppos.clone(), p.clone());
            }
        }
        local_ps.clone()
    }
}
