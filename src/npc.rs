//npc
use crate::enums::{NPCs, Items};
use rand::{Rng};
use std::collections::HashMap;
use serde_json::Value;
use rand::prelude::SliceRandom;


pub fn new_comm_npc(sname: String, x: usize, y: usize, comms: Vec<String>) -> CommNPC {
    CommNPC {
        base: BaseNPC {
            ntype: NPCs::CommNPC,
            sname: sname,
            x: x,
            y: y,
        },
        comms: comms,
    }
}

pub fn new_conv_npc(sname: String, x: usize, y: usize, conv: HashMap<String, Value>) -> ConvNPC {
    ConvNPC {
        base: BaseNPC {
            ntype: NPCs::CommNPC,
            sname: sname,
            x: x,
            y: y,
        },
        conv: conv,
    }
}

pub fn new_quest_npc(sname: String, x: usize, y: usize, quest: HashMap<String, Value>) -> QuestNPC {
    QuestNPC {
        base: BaseNPC {
            ntype: NPCs::QuestNPC,
            sname: sname,
            x: x,
            y: y,
        },
        quest: quest,
    }
}

//--
pub trait NPC {
    fn get_ntype(&mut self) -> NPCs;
    fn get_sname(&mut self) -> String;
    fn mmove(&mut self, dir: &str);
}


//--
#[derive(Clone, Debug)]
pub struct BaseNPC {
    ntype: NPCs,
    sname: String,
    x: usize,
    y: usize,
}

impl NPC for BaseNPC {
    fn get_ntype(&mut self) -> NPCs {
        self.ntype.clone()
    }

    fn get_sname(&mut self) -> String {
        self.sname.clone()
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

#[derive(Clone, Debug)]
struct CommNPC {
    base: BaseNPC,
    comms: Vec<String>,
}

impl NPC for CommNPC {
    fn get_ntype(&mut self) -> NPCs {
        self.base.ntype.clone()
    }

    fn get_sname(&mut self) -> String {
        self.base.sname.clone()
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
        if let Some(comm) = self.comms.choose(& rng) {
            comm.to_string()
        }
    }
}


#[derive(Clone, Debug)]
struct ConvNPC {
    base: BaseNPC,
    conv: HashMap<String, Value>,
}

impl NPC for ConvNPC {
    fn get_ntype(&mut self) -> NPCs {
        self.base.ntype.clone()
    }

    fn get_sname(&mut self) -> String {
        self.base.sname.clone()
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
    pub fn get_conv(&mut self) -> HashMap<String, Value> {
        self.conv.clone()
    }
}

#[derive(Clone, Debug)]
struct QuestNPC {
    base: BaseNPC,
    quest: HashMap<String, Value>,
}

impl NPC for QuestNPC {
    fn get_ntype(&mut self) -> NPCs {
        self.base.ntype.clone()
    }

    fn get_sname(&mut self) -> String {
        self.base.sname.clone()
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

impl QuestNPC {
    pub fn get_quest(&mut self) -> HashMap<String, Value> {
        self.quest.clone()
    }
}


