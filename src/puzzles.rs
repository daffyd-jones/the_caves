//puzzles.rs
//
//use crate::enums{};
use crate::puzzle::{Puzzle};
use std::collections::HashMap;



pub struct Puzzles {
    puzzles: HashMap<(i64, i64), Puzzle>,
}

impl Puzzles {
    pub fn new() -> Self {
        let puzzles = HashMap::new();
        Self {
            puzzles,
        }
    }

    pub fn demo_self() -> Self {
        let pos = (-150, -700);
        let puzzle = Puzzle::new_maze(pos);
        let mut puzzles = HashMap::new();
        puzzles.insert(pos, puzzle);
        Self {
            puzzles,
        }
    }

    pub fn check_location(&self, bpos: (i64, i64), rad: u16) -> Option<Puzzle> {
        for (ppos, p) in &self.puzzles {
            let xx = ppos.0 - bpos.0*-1;
            let yy = ppos.1 - bpos.1*-1;
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


}
