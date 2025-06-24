//puzzles.rs
//
//use crate::enums{};
use crate::enums::PuzzleType;
use crate::puzzle::Puzzle;
use rand::{seq::SliceRandom, Rng};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

//#[derive(Serialize, Deserialize, Debug)]
pub struct Puzzles {
    puzzles: HashMap<(i16, i16), Puzzle>,
}

impl Puzzles {
    pub fn new() -> Self {
        let puzzles = HashMap::new();
        Self { puzzles }
    }

    pub fn demo_self() -> Self {
        // let pos = (100, 0);
        let pos = (-100, -500);
        let puzzle = Puzzle::new_ruin(pos);
        // let puzzle = Puzzle::new_teleport(pos);
        // log::info!("\nPuzzleFound: {:?}", puzzle);
        let mut puzzles = HashMap::new();
        puzzles.insert(pos, puzzle);
        Self { puzzles }
    }

    pub fn spawn_new_puzzle(&mut self, pos: (i16, i16), ptype: PuzzleType) -> PuzzleType {
        // let new_settle_pos = {
        //     let mut rng = rand::thread_rng();
        //     let cxabs = pos.0.abs();
        //     let cyabs = pos.1.abs();
        //     let nx = rng.gen_range((cxabs + 300)..(cxabs + 800));
        //     let ny = rng.gen_range((cyabs + 200)..(cyabs + 600));
        //     let xdir = pos.0 / cxabs;
        //     let ydir = pos.1 / cyabs;
        //     (nx * xdir * -1, ny * ydir * -1)
        // };
        let puzzle = {
            match &ptype {
                PuzzleType::Maze => Puzzle::new_maze(pos),
                PuzzleType::Ruin => Puzzle::new_ruin(pos),
                PuzzleType::Teleport => Puzzle::new_ruin(pos),
                PuzzleType::Inverted => Puzzle::new_maze(pos),
            }
        };
        self.puzzles.insert(pos, puzzle.clone());
        ptype
    }

    pub fn nearest_puzzle(&self, pos: (i16, i16)) -> (i16, Puzzle) {
        let keys: Vec<(i16, i16)> = self.puzzles.clone().into_keys().collect();
        let mut mpos = (1000, (0, 0));
        for ppos in keys {
            let xx = (ppos.0 - -pos.0).abs() as u32;
            let yy = (ppos.1 - -pos.1).abs() as u32;
            let hyp = ((xx.pow(2) + yy.pow(2)) as f64).sqrt() as i16;
            if hyp < mpos.0 {
                mpos = (hyp, (ppos));
            }
        }
        (mpos.0, self.puzzles.get(&mpos.1).unwrap().clone())
    }

    pub fn spawn_node_puzzle(&mut self, pos: (i16, i16)) {
        // let mut rng = rand::thread_rng();
        // let choice = *[PuzzleType::Maze, PuzzleType::Ruin]
        //     .choose(&mut rng)
        //     .unwrap_or(&PuzzleType::Maze);

        let choice = PuzzleType::Ruin;
        let puzzle = match choice {
            PuzzleType::Maze => Puzzle::new_maze(pos),
            PuzzleType::Ruin => Puzzle::new_ruin(pos),
            PuzzleType::Teleport => Puzzle::new_ruin(pos),
            PuzzleType::Inverted => Puzzle::new_maze(pos),
        };
        self.puzzles.insert(pos, puzzle.clone());
        // self.puzzles.insert(pos, Puzzle::new_ruin(pos));
    }

    pub fn check_location(&self, bpos: (i16, i16), rad: u16) -> Option<Puzzle> {
        for (ppos, p) in &self.puzzles {
            let xx = (ppos.0 - -bpos.0) as i32;
            let yy = (ppos.1 - -bpos.1) as i32;
            let hyp = ((xx.pow(2) + yy.pow(2)) as f64).sqrt() as u16;
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

    pub fn get_local_puzzles(&mut self, pos: (i16, i16)) -> HashMap<(i16, i16), Puzzle> {
        let mut local_ps = HashMap::new();
        for (ppos, p) in &self.puzzles {
            let xx = (ppos.0 - -pos.0) as i32;
            let yy = (ppos.1 - -pos.1) as i32;
            let hyp = ((xx.pow(2) + yy.pow(2)) as f64).sqrt() as u16;
            if hyp <= 2000 {
                local_ps.insert(ppos.clone(), p.clone());
            }
        }
        local_ps.clone()
    }

    pub fn puzzle_check(&mut self, pos: (i16, i16)) -> bool {
        let dir = (pos.0 / pos.0.abs(), pos.1 / pos.1.abs());
        let space = {
            match dir {
                (x, y) if x >= 0 && y >= 0 => (((pos.0 + 800), (pos.1 + 800)), pos),
                (x, y) if x < 0 && y >= 0 => (((pos.0 - 800), (pos.1 + 800)), pos),
                (x, y) if x >= 0 && y < 0 => (((pos.0 + 800), (pos.1 - 800)), pos),
                (x, y) if x < 0 && y < 0 => (((pos.0 - 800), (pos.1 - 800)), pos),
                _ => todo!(),
            }
        };
        for (k, _) in self.puzzles.clone() {
            let xrange: Vec<i16> = {
                let mut xa = space.0 .0;
                let mut xb = space.1 .0;
                if xa > xb {
                    std::mem::swap(&mut xa, &mut xb);
                }
                (xa..xb).collect()
            };
            let yrange: Vec<i16> = {
                let mut ya = space.0 .0;
                let mut yb = space.1 .0;
                if ya > yb {
                    std::mem::swap(&mut ya, &mut yb);
                }
                (ya..yb).collect()
            };

            if xrange.contains(&k.0) && yrange.contains(&k.1) {
                return false;
            };
        }
        true
    }
}
