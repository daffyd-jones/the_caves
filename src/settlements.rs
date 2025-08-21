use crate::enums::{EnvInter, Settle, TaskEnv};
//settlements
//use crate::enums::{Settle};
use crate::settlement::Settlement;
use crate::tasks::{Task, TaskType};
use rand::seq::SliceRandom;
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
        // let (xb, yb) = (300 - 64, 200 - 26);
        let (xb, yb) = (300 - 76, 200 - 26);
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
        let mut rng = rand::thread_rng();
        self.settlements.insert(
            pos,
            // match [Settle::Small]
            match [Settle::Small, Settle::Med, Settle::Guild, Settle::Obsidian]
                .choose(&mut rng)
                .unwrap()
            {
                Settle::Small => Settlement::new_node_small_settle(pos, name),
                Settle::Med => Settlement::new_node_med_settle(pos, name),
                Settle::Guild => Settlement::new_node_guild_settle(pos, name),
                Settle::Obsidian => Settlement::new_node_obsidian_settle(pos, name),
                _ => Settlement::new_node_small_settle(pos, name),
            },
        );
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
            if hyp <= 2000 {
                local_settles.insert(spos.clone(), s.clone());
            }
        }
        local_settles.clone()
    }

    pub fn set_task_content(&mut self, task: Task) {
        match task {
            Task::BoardItemWanted { receiver_loc, .. } => {
                self.set_board_item_wanted_content(receiver_loc)
            }
            // Task::BoardPassItem => self.set_pass_item_content(task),
            // Task::BoardPassMessage => self.set_pass_msg_content(task),
            _ => {}
        }
    }

    fn set_board_item_wanted_content(&mut self, loc: (i16, i16)) {
        let mut settle = self.settlements.get(&loc).unwrap().clone();
        settle.add_task_env(EnvInter::TaskEnv(TaskEnv::BoardGoalEntity));
        self.settlements.insert(loc, settle);
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
