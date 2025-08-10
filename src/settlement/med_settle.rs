use crate::dialogue::{load_comms, load_convos, CommDialogue, ConvoDialogue};
use crate::enums::Shops;
use crate::enums::{Cells, Door, EnvInter, NPCWrap, Settle};
use crate::item::Item;
use crate::npc::{new_comm_npc, new_conv_npc, new_shop_npc, Convo, ShopConvos, ShopData};
use crate::npc_utils::box_npc;
use crate::shop::Shop;
use rand::prelude::SliceRandom;
use rand::Rng;

//use serde::{Deserialize, Serialize};
//use serde_json::Result;
//use serde_json::Value;
use std::fs;

use std::collections::HashMap;

pub fn place_med_parts(
    mut map: Vec<Vec<Cells>>,
    part: Vec<Vec<Cells>>,
    npcs: HashMap<(usize, usize), NPCWrap>,
    sitems: HashMap<(usize, usize), Item>,
    items: HashMap<(usize, usize), Item>,
    env_inter: HashMap<(usize, usize), EnvInter>,
    block: u8,
) -> (
    Vec<Vec<Cells>>,
    HashMap<(usize, usize), NPCWrap>,
    HashMap<(usize, usize), Item>,
    HashMap<(usize, usize), Item>,
    HashMap<(usize, usize), EnvInter>,
) {
    let (sx, sy) = {
        match block {
            1 => (0, 0),
            2 => (75, 0),
            3 => (150, 0),
            4 => (0, 25),
            5 => (75, 25),
            6 => (150, 25),
            7 => (0, 50),
            8 => (75, 50),
            9 => (150, 50),
            _ => {
                log::info!("small parts error");
                (0, 0)
            }
        }
    };

    for j in 0..part.len() {
        for i in 0..part[0].len() {
            //log::info!("copying map q: {:?} | dir: ({}, {}) | idx: ({}, {}) | char: {:?}", quad, sx, sy, i, j, part[j][i]);
            map[j + &sy][i + &sx] = part[j][i];
        }
    }
    let mut new_npcs = HashMap::new();
    for (npos, npc) in npcs {
        new_npcs.insert(((npos.0 + &sx), (npos.1 + &sy)), npc);
    }
    let mut new_sitems = HashMap::new();
    for (ipos, item) in sitems {
        new_sitems.insert(((ipos.0 + &sx), (ipos.1 + &sy)), item);
    }
    let mut new_items = HashMap::new();
    for (ipos, mut item) in items {
        item.set_pos(((ipos.0 + &sx), (ipos.1 + &sy)));
        new_items.insert(((ipos.0 + &sx), (ipos.1 + &sy)), item);
    }
    let mut new_env_inter = HashMap::new();
    for (epos, env) in env_inter.clone() {
        new_env_inter.insert(((epos.0 + &sx), (epos.1 + &sy)), env);
    }

    (map, new_npcs, new_sitems, new_items, new_env_inter)
}
