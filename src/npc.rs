//npc
use crate::enums::{NPCs, PuzzleType};
use rand::{Rng};
use std::collections::HashMap;
// use serde_json::Value;
use serde::{Deserialize, Serialize};
use rand::prelude::SliceRandom;


pub fn new_comm_npc(sname: String, x: usize, y: usize, comms: Vec<String>) -> CommNPC {
    let mut rng = rand::thread_rng();
    let step = rng.gen_range(0..19);
    let step_grp = rng.gen_range(0..15);
    CommNPC {
        base: BaseNPC {
            ntype: NPCs::CommNPC,
            sname: sname,
            steps: step,
            step_grp: step_grp,
            x: x,
            y: y,
        },
        comms: comms,
    }
}

pub fn new_conv_npc(sname: String, x: usize, y: usize, conv: Convo) -> ConvNPC {
    let mut rng = rand::thread_rng();
    let step = rng.gen_range(0..19);
    let step_grp = rng.gen_range(0..15);
    ConvNPC {
        base: BaseNPC {
            ntype: NPCs::ConvNPC,
            sname: sname,
            steps: step,
            step_grp: step_grp,
            x: x,
            y: y,
        },
        conv: conv,
    }
}

pub fn new_spawn_npc(
    sname: String, x: usize, y: usize, 
    conv: Convo, comms: Vec<String>, 
    ptype: PuzzleType
    ) -> SpawnNPC {
    let mut rng = rand::thread_rng();
    let step = rng.gen_range(0..19);
    let step_grp = rng.gen_range(0..15);
    SpawnNPC {
        base: BaseNPC {
            ntype: NPCs::SpawnNPC,
            sname: sname,
            steps: step,
            step_grp: step_grp,
            x: x,
            y: y,
        },
        conv: conv,
        comms: comms,
        spawned: false,
        ptype: ptype 
    }
}

pub fn new_shop_npc(sname: String, x: usize, y: usize, sh_conv: HashMap<String, String>) -> ShopNPC {
    //let mut rng = rand::thread_rng();
    //let step = rng.gen_range(0..19);
    let step = 50;
    //let step_grp = rng.gen_range(0..15);
    let step_grp = 100;
    // let sh_conv = HashMap::new();
    ShopNPC {
        base: BaseNPC {
            ntype: NPCs::ShopNPC,
            sname: sname,
            steps: step,
            step_grp: step_grp,
            x: x,
            y: y,
        },
        sh_conv: sh_conv,
    }
}

//--
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Convo {
    pub id: String,
    #[serde(flatten)]
    pub stages: HashMap<String, Stage>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Stage {
    pub text: String,
    pub opts: Vec<ConOpt>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ConOpt {
    pub text: String,
    pub next: String,
}
//--

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ShopData {
    pub item_shops: Vec<HashMap<String, String>>,
    pub churches: Vec<HashMap<String, String>>,
}



//--
pub trait NPC {
    fn as_any(&self) -> &dyn std::any::Any;
    fn get_ntype(&mut self) -> NPCs;
    fn get_sname(&mut self) -> String;
    fn get_pos(&mut self) -> (usize, usize);
    fn set_pos(&mut self, pos: (usize, usize));
    fn set_steps(&mut self, steps: u8);
    fn inc_steps(&mut self);
    fn get_steps(&mut self) -> u8;
    fn get_step_grp(&mut self) -> u8;
    fn mmove(&mut self, dir: &str);
}

impl dyn NPC {
    pub fn as_comm_npc(&self) -> Option<&CommNPC> {
        self.as_any().downcast_ref::<CommNPC>()
    }

    pub fn as_conv_npc(&self) -> Option<&ConvNPC> {
        self.as_any().downcast_ref::<ConvNPC>()
    }

    pub fn as_shop_npc(&self) -> Option<&ShopNPC> {
        self.as_any().downcast_ref::<ShopNPC>()
    }

    pub fn as_spawn_npc(&self) -> Option<&SpawnNPC> {
        self.as_any().downcast_ref::<SpawnNPC>()
    }
}


//--
#[derive(Clone, Debug, PartialEq)]
pub struct BaseNPC {
    ntype: NPCs,
    sname: String,
    steps: u8,
    step_grp: u8,
    x: usize,
    y: usize,
}

impl NPC for BaseNPC {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn get_ntype(&mut self) -> NPCs {
        self.ntype.clone()
    }

    fn get_sname(&mut self) -> String {
        self.sname.clone()
    }

    fn get_pos(&mut self) -> (usize, usize) {
        (self.x, self.y)
    }

    fn set_pos(&mut self, pos: (usize, usize)) {
        self.x = pos.0;
        self.y = pos.1;
    }

    fn set_steps(&mut self, steps: u8) {
        self.steps = steps;
    }

    fn inc_steps(&mut self) {
        self.steps += 1;
    }

    fn get_steps(&mut self) -> u8 {
        self.steps.clone()
    }

    fn get_step_grp(&mut self) -> u8 {
        self.step_grp.clone()
    }

    fn mmove(&mut self, dir: &str) {
        match dir {
            "UP" => self.y -= 1,
            "DN" => self.y += 1,
            "LF" => self.x -= 1,
            "RT" => self.x += 1,
            _ => println!("")
        }
    }
}

impl BaseNPC {
    pub fn new() -> Self {
        Self {ntype: NPCs::Null, sname: "".to_string(), steps: 0, step_grp: 0, x: 0, y: 0}
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CommNPC {
    base: BaseNPC,
    comms: Vec<String>,
}

impl NPC for CommNPC {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn get_ntype(&mut self) -> NPCs {
        self.base.ntype.clone()
    }

    fn get_sname(&mut self) -> String {
        self.base.sname.clone()
    }

    fn get_pos(&mut self) -> (usize, usize) {
        (self.base.x, self.base.y)
    }

    fn set_pos(&mut self, pos: (usize, usize)) {
        self.base.x = pos.0;
        self.base.y = pos.1;
    }

    fn set_steps(&mut self, steps: u8) {
        self.base.steps = steps;
    }

    fn get_steps(&mut self) -> u8 {
        self.base.steps.clone()
    }

    fn inc_steps(&mut self) {
        self.base.steps += 1;
    }

    fn get_step_grp(&mut self) -> u8 {
        self.base.step_grp.clone()
    }

    fn mmove(&mut self, dir: &str) {
        match dir {
            "UP" => self.base.y -= 1,
            "DN" => self.base.y += 1,
            "LF" => self.base.x -= 1,
            "RT" => self.base.x += 1,
            _ => println!("")
        }
    }
}

impl CommNPC {
    pub fn get_comm(&mut self) -> String {
        let mut rng = rand::thread_rng();
        if let Some(comm) = self.comms.choose(&mut rng) {
            comm.to_string()
        } else {"".to_string()}
    }
}


#[derive(Clone, Debug, PartialEq)]
pub struct ConvNPC {
    base: BaseNPC,
    conv: Convo,
}

impl NPC for ConvNPC {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn get_ntype(&mut self) -> NPCs {
        self.base.ntype.clone()
    }

    fn get_sname(&mut self) -> String {
        self.base.sname.clone()
    }

    fn get_pos(&mut self) -> (usize, usize) {
        (self.base.x, self.base.y)
    }

    fn set_pos(&mut self, pos: (usize, usize)) {
        self.base.x = pos.0;
        self.base.y = pos.1;
    }

    fn set_steps(&mut self, steps: u8) {
        self.base.steps = steps;
    }

    fn get_steps(&mut self) -> u8 {
        self.base.steps.clone()
    }

    fn inc_steps(&mut self) {
        self.base.steps += 1;
    }

    fn get_step_grp(&mut self) -> u8 {
        self.base.step_grp.clone()
    }

    fn mmove(&mut self, dir: &str) {
        match dir {
            "UP" => self.base.y -= 1,
            "DN" => self.base.y += 1,
            "LF" => self.base.x -= 1,
            "RT" => self.base.x += 1,
            _ => println!("")
        }
    }
}

impl ConvNPC {
    pub fn get_conv(&mut self) -> Convo {
        self.conv.clone()
    }
}

//#[derive(Clone, Debug, PartialEq)]
//pub struct QuestNPC {
//    base: BaseNPC,
//    quest: NQuest,
//}
//
//impl NPC for QuestNPC {
//    fn as_any(&self) -> &dyn std::any::Any {
//        self
//    }
//
//    fn get_ntype(&mut self) -> NPCs {
//        self.base.ntype.clone()
//    }
//
//    fn get_sname(&mut self) -> String {
//        self.base.sname.clone()
//    }
//
//    fn get_pos(&mut self) -> (usize, usize) {
//        (self.base.x, self.base.y)
//    }
//
//    fn set_pos(&mut self, pos: (usize, usize)) {
//        self.base.x = pos.0;
//        self.base.y = pos.1;
//    }
//
//    fn set_steps(&mut self, steps: u8) {
//        self.base.steps = steps;
//    }
//
//    fn get_steps(&mut self) -> u8 {
//        self.base.steps.clone()
//    }
//
//    fn inc_steps(&mut self) {
//        self.base.steps += 1;
//    }
//
//    fn get_step_grp(&mut self) -> u8 {
//        self.base.step_grp.clone()
//    }
//
//    fn mmove(&mut self, dir: &str) {
//        match dir {
//            "UP" => self.base.y -= 1,
//            "DN" => self.base.y += 1,
//            "LF" => self.base.x -= 1,
//            "RT" => self.base.x += 1,
//            _ => println!("")
//        }
//    }
//}
//
//impl QuestNPC {
//    pub fn get_quest(&mut self) -> NQuest {
//        self.quest.clone()
//    }
//}


#[derive(Clone, Debug, PartialEq)]
pub struct ShopNPC {
    base: BaseNPC,
    sh_conv: HashMap<String, String>,
}

impl NPC for ShopNPC {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn get_ntype(&mut self) -> NPCs {
        self.base.ntype.clone()
    }

    fn get_sname(&mut self) -> String {
        self.base.sname.clone()
    }

    fn get_pos(&mut self) -> (usize, usize) {
        (self.base.x, self.base.y)
    }

    fn set_pos(&mut self, pos: (usize, usize)) {
        self.base.x = pos.0;
        self.base.y = pos.1;
    }

    fn set_steps(&mut self, steps: u8) {
        self.base.steps = steps;
    }

    fn get_steps(&mut self) -> u8 {
        self.base.steps.clone()
    }

    fn inc_steps(&mut self) {
        self.base.steps += 1;
    }

    fn get_step_grp(&mut self) -> u8 {
        self.base.step_grp.clone()
    }

    fn mmove(&mut self, dir: &str) {
        match dir {
            "UP" => self.base.y -= 1,
            "DN" => self.base.y += 1,
            "LF" => self.base.x -= 1,
            "RT" => self.base.x += 1,
            _ => println!("")
        }
    }
}

impl ShopNPC {
    pub fn get_sh_conv(&mut self) -> HashMap<String, String> {
        self.sh_conv.clone()
    }
}


#[derive(Clone, Debug, PartialEq)]
pub struct SpawnNPC {
    base: BaseNPC,
    conv: Convo,
    comms: Vec<String>,
    spawned: bool,
    ptype: PuzzleType,
}

impl NPC for SpawnNPC {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn get_ntype(&mut self) -> NPCs {
        self.base.ntype.clone()
    }

    fn get_sname(&mut self) -> String {
        self.base.sname.clone()
    }

    fn get_pos(&mut self) -> (usize, usize) {
        (self.base.x, self.base.y)
    }

    fn set_pos(&mut self, pos: (usize, usize)) {
        self.base.x = pos.0;
        self.base.y = pos.1;
    }

    fn set_steps(&mut self, steps: u8) {
        self.base.steps = steps;
    }

    fn get_steps(&mut self) -> u8 {
        self.base.steps.clone()
    }

    fn inc_steps(&mut self) {
        self.base.steps += 1;
    }

    fn get_step_grp(&mut self) -> u8 {
        self.base.step_grp.clone()
    }

    fn mmove(&mut self, dir: &str) {
        match dir {
            "UP" => self.base.y -= 1,
            "DN" => self.base.y += 1,
            "LF" => self.base.x -= 1,
            "RT" => self.base.x += 1,
            _ => println!("")
        }
    }
}

impl SpawnNPC {
    pub fn get_conv(&mut self) -> Convo {
        self.conv.clone()
    }

    pub fn get_comm(&mut self) -> String {
        let mut rng = rand::thread_rng();
        if let Some(comm) = self.comms.choose(&mut rng) {
            comm.to_string()
        } else {"".to_string()}
    }

    pub fn is_spawned(&mut self) -> bool {
        self.spawned.clone()
    }

    pub fn toggle_spawned(&mut self) {
        self.spawned = !self.spawned;
    }

    pub fn get_ptype(&mut self) -> PuzzleType {
        self.ptype.clone()
    }
}
