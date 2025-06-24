use crate::enums::{EnvInter, TaskEnv};
//settlements
//use crate::enums::{Settle};
use crate::settlement::Settlement;
use crate::tasks::{Task, TaskType};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

//#[derive(Serialize, Deserialize, Debug)]
pub struct Settlements {
    settlements: HashMap<(i16, i16), Settlement>,
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

    pub fn check_location(&self, bpos: (i16, i16), rad: u16) -> Option<Settlement> {
        for (spos, s) in &self.settlements {
            let xx = (spos.0 - bpos.0 * -1) as i32;
            let yy = (spos.1 - bpos.1 * -1) as i32;
            let hyp = ((xx.pow(2) + yy.pow(2)) as f64).sqrt() as u64;
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

    pub fn spawn_new_settlement(&mut self, cpos: (i16, i16)) {
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

    pub fn spawn_node_settlement(&mut self, pos: (i16, i16), name: String) {
        self.settlements
            .insert(pos, Settlement::new_node_settle(pos, name));
    }

    pub fn get_settle_pos(&mut self) -> Vec<(i16, i16)> {
        self.settlements.clone().into_keys().collect()
    }

    pub fn get_compass_pos(&mut self) -> HashMap<(i16, i16), String> {
        let mut tvec = HashMap::new();
        for (pos, mut s) in self.settlements.clone() {
            // if s.found {
            //     tvec.insert(pos, s.get_sname());
            // }

            tvec.insert(pos, s.get_sname());
        }
        tvec.clone()
    }

    pub fn get_local_settles(&mut self, pos: (i16, i16)) -> HashMap<(i16, i16), Settlement> {
        let mut local_settles = HashMap::new();
        for (spos, s) in &self.settlements {
            let xx = (spos.0 - -pos.0) as i32;
            let yy = (spos.1 - -pos.1) as i32;
            let hyp = ((xx.pow(2) + yy.pow(2)) as f64).sqrt() as u16;
            if hyp <= 4000 {
                local_settles.insert(spos.clone(), s.clone());
            }
        }
        local_settles.clone()
    }

    pub fn set_task_content(&mut self, task: Task) {
        match task.ttype {
            TaskType::RetrieveItem => self.set_retrieve_item_content(task),
            TaskType::PassItem => self.set_pass_item_content(task),
            TaskType::PassMessage => self.set_pass_msg_content(task),
            _ => {}
        }
    }

    fn set_retrieve_item_content(&mut self, task: Task) {
        let settle_loc = task.start_loc;
        let mut settle = self.settlements.get(&settle_loc).unwrap().clone();
        settle.add_task_env(EnvInter::TaskEnv(TaskEnv::BoardStartEntity));
        self.settlements.insert(settle_loc, settle);
    }

    fn set_pass_item_content(&self, task: Task) {
        todo!()
    }

    fn set_pass_msg_content(&self, task: Task) {
        todo!()
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
