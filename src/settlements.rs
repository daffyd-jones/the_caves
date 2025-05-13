//settlements
//use crate::enums::{Settle};
use crate::settlement::Settlement;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

//#[derive(Serialize, Deserialize, Debug)]
pub struct Settlements {
    settlements: HashMap<(i64, i64), Settlement>,
}

impl Settlements {
    pub fn new() -> Self {
        let settlements = HashMap::new();
        Self { settlements }
    }

    pub fn demo_self() -> Self {
        let xb = 300 - 76;
        let yb = 200 - 26;
        //let xb = -50;
        //let yb = -50;
        let mut settlements = HashMap::new();
        let npcs = HashMap::new();
        let demo_settle = Settlement::demo_settle((xb, yb), npcs);
        settlements.insert((xb, yb), demo_settle);
        Self { settlements }
    }

    pub fn check_location(&self, bpos: (i64, i64), rad: u16) -> Option<Settlement> {
        for (spos, s) in &self.settlements {
            let xx = spos.0 - bpos.0 * -1;
            let yy = spos.1 - bpos.1 * -1;
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

    pub fn spawn_new_settlement(&mut self, cpos: (i64, i64)) {
        let new_settle_pos = {
            let mut rng = rand::thread_rng();
            let cxabs = cpos.0.abs();
            let cyabs = cpos.1.abs();
            let nx = rng.gen_range((cxabs + 300)..(cxabs + 500));
            let ny = rng.gen_range((cyabs + 200)..(cyabs + 400));
            let xdir = cpos.0 / cxabs;
            let ydir = cpos.1 / cyabs;
            (nx * xdir * -1, ny * ydir * -1)
        };
        let mut rng = rand::thread_rng();
        let stype = rng.gen_range(0..1);
        let settlement = if stype == 0 {
            Settlement::new_small_settle(new_settle_pos.clone())
        } else {
            let npcs = HashMap::new();
            Settlement::demo_settle(new_settle_pos.clone(), npcs)
        };
        self.settlements.insert(new_settle_pos, settlement.clone());
    }

    pub fn spawn_node_settlement(&mut self, pos: (i64, i64), name: String) {
        self.settlements
            .insert(pos, Settlement::new_node_settle(pos, name));
    }

    pub fn get_settle_pos(&mut self) -> Vec<(i64, i64)> {
        self.settlements.clone().into_keys().collect()
    }

    pub fn get_compass_pos(&mut self) -> HashMap<(i64, i64), String> {
        let mut tvec = HashMap::new();
        for (pos, mut s) in self.settlements.clone() {
            if s.found {
                tvec.insert(pos, s.get_sname());
            }
        }
        tvec.clone()
    }

    pub fn get_local_settles(&mut self, pos: (i64, i64)) -> HashMap<(i64, i64), Settlement> {
        let mut local_settles = HashMap::new();
        for (spos, s) in &self.settlements {
            let xx = spos.0 - pos.0 * -1;
            let yy = spos.1 - pos.1 * -1;
            let hyp = ((xx.pow(2) + yy.pow(2)) as f64).sqrt() as i64;
            if hyp <= 5000.into() {
                local_settles.insert(spos.clone(), s.clone());
            }
        }
        local_settles.clone()
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
