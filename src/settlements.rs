//settlements
use crate::enums::{Settle};
use crate::settlement::{Settlement};
use std::collections::HashMap;

pub struct Settlements {
    settlements: HashMap<(i64, i64), Settlement>,
}

impl Settlements {
    pub fn new() -> Self {
        let settlements = HashMap::new();
        Self {settlements}
    }

    pub fn demo_self() -> Self {
        let xb = 300 - 75;
        let yb = 200 - 25;
        let mut settlements = HashMap::new();
        let npcs = HashMap::new();
        let demo_settle = Settlement::demo_settle((xb, yb), npcs);
        settlements.insert((xb, yb), demo_settle);
        Self {settlements}
    }

    pub fn check_location(&self, bpos: (i64, i64), rad: u16) -> Option<Settlement> {
        for (spos, s) in &self.settlements {
            let xx = spos.0 - bpos.0;
            let yy = spos.1 - bpos.1;
            let hyp = ((xx.pow(2) + yy.pow(2)) as f64).sqrt() as i64;
            if hyp <= rad.into() {
                return Some(s.clone());
            }
        }
        return None;
    }

    pub fn update_settlement(&mut self, mut settle: Settlement) {
        let spos = settle.get_pos();
        self.settlements.insert(spos, settle);
    }

}


//character pos_fo
//
//
//
//
//
//
//
//
//
//
