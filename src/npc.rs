//npc
use crate::enums::{NPCs, PuzzleType, Shops};
use crate::item::Item;
use rand::Rng;
use std::collections::HashMap;
use std::default;
// use serde_json::Value;
use rand::prelude::SliceRandom;
use serde::{Deserialize, Serialize};

pub fn new_comm_npc(sname: String, x: usize, y: usize, comms: Vec<String>) -> CommNPC {
    let mut rng = rand::thread_rng();
    let step = rng.gen_range(0..19);
    let step_grp = rng.gen_range(0..15);
    CommNPC {
        base: BaseNPC {
            ntype: NPCs::CommNPC,
            sname,
            steps: step,
            step_grp,
            x,
            y,
        },
        comms,
    }
}

pub fn new_conv_npc(sname: String, x: usize, y: usize, conv: Convo) -> ConvNPC {
    let mut rng = rand::thread_rng();
    let step = rng.gen_range(0..19);
    let step_grp = rng.gen_range(0..15);
    ConvNPC {
        base: BaseNPC {
            ntype: NPCs::ConvNPC,
            sname,
            steps: step,
            step_grp,
            x,
            y,
        },
        conv,
    }
}

pub fn new_spawn_npc(
    sname: String,
    x: usize,
    y: usize,
    conv: Convo,
    comms: Vec<String>,
    ptype: PuzzleType,
) -> SpawnNPC {
    let mut rng = rand::thread_rng();
    let step = rng.gen_range(0..19);
    let step_grp = rng.gen_range(0..15);
    SpawnNPC {
        base: BaseNPC {
            ntype: NPCs::SpawnNPC,
            sname,
            steps: step,
            step_grp,
            x,
            y,
        },
        conv,
        comms,
        spawned: false,
        ptype,
    }
}

pub fn new_shop_npc(
    sname: String,
    sh_conv: HashMap<String, String>,
    convo: Convo,
    shop_type: Shops,
) -> ShopNPC {
    ShopNPC {
        sname,
        shop_type,
        sh_conv,
        convo,
    }
}

pub fn new_trade_npc(
    sname: String,
    x: usize,
    y: usize,
    items: Vec<Item>,
    sh_conv: HashMap<String, String>,
) -> TradeNPC {
    let mut rng = rand::thread_rng();
    let step = rng.gen_range(0..19);
    let step_grp = rng.gen_range(0..15);
    TradeNPC {
        base: BaseNPC {
            ntype: NPCs::TradeNPC,
            sname,
            steps: step,
            step_grp,
            x,
            y,
        },
        items,
        sh_conv,
    }
}

//--
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Convo {
    pub id: String,
    #[serde(flatten)]
    pub stages: HashMap<String, Stage>,
}

impl Default for Convo {
    fn default() -> Self {
        Self {
            id: "0".to_string(),
            stages: HashMap::new(),
        }
    }
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

fn default_convo() -> Convo {
    let stages = HashMap::new();
    Convo {
        id: "default".to_string(),
        stages,
    }
}
//--

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ShopData {
    pub shops: Vec<HashMap<String, String>>,
    pub guilds: Vec<HashMap<String, String>>,
    pub churches: Vec<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ShopConvos {
    pub shops: Vec<Convo>,
    pub guilds: Vec<Convo>,
    pub churches: Vec<Convo>,
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

    pub fn as_trade_npc(&self) -> Option<&TradeNPC> {
        self.as_any().downcast_ref::<TradeNPC>()
    }
}

//--
//#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
        self.ntype
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
        self.steps
    }

    fn get_step_grp(&mut self) -> u8 {
        self.step_grp
    }

    fn mmove(&mut self, dir: &str) {
        match dir {
            "UP" => self.y -= 1,
            "DN" => self.y += 1,
            "LF" => self.x -= 1,
            "RT" => self.x += 1,
            _ => println!(""),
        }
    }
}

impl BaseNPC {
    pub fn new() -> Self {
        Self {
            ntype: NPCs::Null,
            sname: "".to_string(),
            steps: 0,
            step_grp: 0,
            x: 0,
            y: 0,
        }
    }
}

//#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
        self.base.ntype
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
        self.base.steps
    }

    fn inc_steps(&mut self) {
        self.base.steps += 1;
    }

    fn get_step_grp(&mut self) -> u8 {
        self.base.step_grp
    }

    fn mmove(&mut self, dir: &str) {
        match dir {
            "UP" => self.base.y -= 1,
            "DN" => self.base.y += 1,
            "LF" => self.base.x -= 1,
            "RT" => self.base.x += 1,
            _ => println!(""),
        }
    }
}

impl CommNPC {
    pub fn get_comm(&mut self) -> String {
        let mut rng = rand::thread_rng();
        if let Some(comm) = self.comms.choose(&mut rng) {
            comm.to_string()
        } else {
            "".to_string()
        }
    }
}

//#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
        self.base.ntype
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
        self.base.steps
    }

    fn inc_steps(&mut self) {
        self.base.steps += 1;
    }

    fn get_step_grp(&mut self) -> u8 {
        self.base.step_grp
    }

    fn mmove(&mut self, dir: &str) {
        match dir {
            "UP" => self.base.y -= 1,
            "DN" => self.base.y += 1,
            "LF" => self.base.x -= 1,
            "RT" => self.base.x += 1,
            _ => println!(""),
        }
    }
}

impl ConvNPC {
    pub fn get_conv(&mut self) -> Convo {
        self.conv.clone()
    }
}

//#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[derive(Clone, Debug, PartialEq)]
pub struct ShopNPC {
    pub sname: String,
    pub shop_type: Shops,
    pub sh_conv: HashMap<String, String>,
    pub convo: Convo,
}

impl Default for ShopNPC {
    fn default() -> Self {
        let sh_conv = HashMap::new();
        let convo = default_convo();
        Self {
            sname: "Tracy".to_string(),
            shop_type: Shops::Null,
            sh_conv,
            convo,
        }
    }
}

//#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
        self.base.ntype
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
        self.base.steps
    }

    fn inc_steps(&mut self) {
        self.base.steps += 1;
    }

    fn get_step_grp(&mut self) -> u8 {
        self.base.step_grp
    }

    fn mmove(&mut self, dir: &str) {
        match dir {
            "UP" => self.base.y -= 1,
            "DN" => self.base.y += 1,
            "LF" => self.base.x -= 1,
            "RT" => self.base.x += 1,
            _ => println!(""),
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
        } else {
            "".to_string()
        }
    }

    pub fn is_spawned(&mut self) -> bool {
        self.spawned
    }

    pub fn toggle_spawned(&mut self) {
        self.spawned = !self.spawned;
    }

    pub fn get_ptype(&mut self) -> PuzzleType {
        self.ptype.clone()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TradeNPC {
    base: BaseNPC,
    items: Vec<Item>,
    sh_conv: HashMap<String, String>,
}

impl NPC for TradeNPC {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn get_ntype(&mut self) -> NPCs {
        self.base.ntype
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
        self.base.steps
    }

    fn inc_steps(&mut self) {
        self.base.steps += 1;
    }

    fn get_step_grp(&mut self) -> u8 {
        self.base.step_grp
    }

    fn mmove(&mut self, dir: &str) {
        match dir {
            "UP" => self.base.y -= 1,
            "DN" => self.base.y += 1,
            "LF" => self.base.x -= 1,
            "RT" => self.base.x += 1,
            _ => println!(""),
        }
    }
}

impl TradeNPC {
    pub fn get_items(&mut self) -> Vec<Item> {
        self.items.clone()
    }

    pub fn remove_item(&mut self, idx: usize) {
        self.items.remove(idx);
    }

    pub fn get_sh_conv(&mut self) -> HashMap<String, String> {
        self.sh_conv.clone()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TaskNPC {
    base: BaseNPC,
    reward: Item,
    convo: Convo,
    comms: Vec<String>,
}

impl NPC for TaskNPC {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn get_ntype(&mut self) -> NPCs {
        self.base.ntype
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
        self.base.steps
    }

    fn inc_steps(&mut self) {
        self.base.steps += 1;
    }

    fn get_step_grp(&mut self) -> u8 {
        self.base.step_grp
    }

    fn mmove(&mut self, dir: &str) {
        match dir {
            "UP" => self.base.y -= 1,
            "DN" => self.base.y += 1,
            "LF" => self.base.x -= 1,
            "RT" => self.base.x += 1,
            _ => println!(""),
        }
    }
}

impl TaskNPC {
    pub fn get_reward(&mut self) -> Item {
        self.reward.clone()
    }

    pub fn get_convo(&mut self) -> Convo {
        self.convo.clone()
    }

    pub fn get_comms(&mut self) -> Vec<String> {
        self.comms.clone()
    }
}
