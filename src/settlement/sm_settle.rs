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

fn place_small_parts(
    mut map: Vec<Vec<Cells>>,
    part: Vec<Vec<Cells>>,
    npcs: HashMap<(usize, usize), NPCWrap>,
    sitems: HashMap<(usize, usize), Item>,
    items: HashMap<(usize, usize), Item>,
    env_inter: HashMap<(usize, usize), EnvInter>,
    quad: u8,
) -> (
    Vec<Vec<Cells>>,
    HashMap<(usize, usize), NPCWrap>,
    HashMap<(usize, usize), Item>,
    HashMap<(usize, usize), Item>,
    HashMap<(usize, usize), EnvInter>,
) {
    let (sx, sy) = {
        match quad {
            1 => (0, 0),
            2 => (75, 0),
            3 => (0, 25),
            4 => (75, 25),
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

fn build_small_settle(
    is_cave_o: bool,
) -> (
    Vec<Vec<Cells>>,
    HashMap<(usize, usize), NPCWrap>,
    HashMap<(usize, usize), Item>,
    HashMap<(usize, usize), Item>,
    HashMap<(usize, usize), EnvInter>,
) {
    let cells = vec![vec![Cells::Null; 150]; 50];
    let item_cell = vec![vec![Cells::Null; 75]; 25];
    let guild_cell = vec![vec![Cells::Null; 75]; 25];
    let church_cell = vec![vec![Cells::Null; 75]; 25];
    let anchor_cell = vec![vec![Cells::Null; 75]; 25];
    let mut rng = rand::thread_rng();
    let item_shop = if is_cave_o {
        CAVE_O1
    } else {
        ITEM_SHOPS.choose(&mut rng).expect("item parse failed")
    };

    let guild = if is_cave_o {
        CAVE_O3
    } else {
        GUILD_SHOPS.choose(&mut rng).expect("guild parse failed")
    };
    let church = if is_cave_o {
        CAVE_O4
    } else {
        CHURCHES.choose(&mut rng).expect("church parse failed")
    };
    let anchor = if is_cave_o {
        CAVE_O2
    } else {
        ANCHORS.choose(&mut rng).expect("anchor parse failed")
    };

    // let guild = guild_shops.choose(&mut rng).expect("guild parse failed");
    // let church = churches.choose(&mut rng).expect("church parse failed");
    // let anchor = anchors.choose(&mut rng).expect("anchor parse failed");
    let (item_map, item_npcs, item_sitems, item_items, item_env_inter) =
        parse_map(item_shop, item_cell.clone(), Shops::Item);
    let (guild_map, guild_npcs, guild_sitems, guild_items, guild_env_inter) =
        parse_map(guild, guild_cell.clone(), Shops::Guild);
    let (church_map, church_npcs, church_sitems, church_items, church_env_inter) =
        parse_map(church, church_cell.clone(), Shops::Church);
    let (anchor_map, anchor_npcs, anchor_sitems, anchor_items, anchor_env_inter) =
        parse_map(anchor, anchor_cell.clone(), Shops::Null);
    let mut quads: Vec<u8> = vec![1, 2, 3, 4];
    let q1 = if is_cave_o {
        1
    } else {
        quads.choose(&mut rng).expect("rand 1 failed").clone()
    };
    quads.retain(|&x| x != q1);
    //let q1 = quads.choose(&mut rng).expect("rand 1 failed").clone();
    //quads.retain(|&x| x != q1);
    let (q1_map, q1_npcs, q1_sitems, q1_items, q1_env_inter) = place_small_parts(
        cells.clone(),
        item_map,
        item_npcs,
        item_sitems,
        item_items,
        item_env_inter,
        q1,
    );
    let q2 = if is_cave_o {
        3
    } else {
        quads.choose(&mut rng).expect("rand 2 failed").clone()
    };
    quads.retain(|&x| x != q2);

    //let q2 = quads.choose(&mut rng).expect("rand 2 failed").clone();
    //quads.retain(|&x| x != q2);
    let (q2_map, q2_npcs, q2_sitems, q2_items, q2_env_inter) = place_small_parts(
        q1_map.clone(),
        guild_map,
        guild_npcs,
        guild_sitems,
        guild_items,
        guild_env_inter,
        q2,
    );
    let q3 = if is_cave_o {
        4
    } else {
        quads.choose(&mut rng).expect("rand 3 failed").clone()
    };
    quads.retain(|&x| x != q3);
    //let q3 = quads.choose(&mut rng).expect("rand 3 failed").clone();
    //quads.retain(|&x| x != q3);
    let (q3_map, q3_npcs, q3_sitems, q3_items, q3_env_inter) = place_small_parts(
        q2_map.clone(),
        church_map,
        church_npcs,
        church_sitems,
        church_items,
        church_env_inter,
        q3,
    );
    let q4 = if is_cave_o {
        2
    } else {
        quads.choose(&mut rng).expect("rand 4 failed").clone()
    };
    quads.retain(|&x| x != q4);
    //let q4 = quads.choose(&mut rng).expect("rand 4 failed").clone();
    let (final_map, q4_npcs, q4_sitems, q4_items, q4_env_inter) = place_small_parts(
        q3_map.clone(),
        anchor_map,
        anchor_npcs,
        anchor_sitems,
        anchor_items,
        anchor_env_inter,
        q4,
    );
    let mut final_npcs = HashMap::new();
    let mut final_sitems = HashMap::new();
    let mut final_items = HashMap::new();
    let mut final_env_inter = HashMap::new();
    //let mut s_npcs = HashMap::new();
    //s_npcs.insert();
    final_npcs.extend(q1_npcs);
    final_npcs.extend(q2_npcs);
    final_npcs.extend(q3_npcs);
    final_npcs.extend(q4_npcs);
    final_sitems.extend(q1_sitems);
    final_sitems.extend(q2_sitems);
    final_sitems.extend(q3_sitems);
    final_sitems.extend(q4_sitems);
    final_items.extend(q1_items);
    final_items.extend(q2_items);
    final_items.extend(q3_items);
    final_items.extend(q4_items);
    final_env_inter.extend(q1_env_inter);
    final_env_inter.extend(q2_env_inter);
    final_env_inter.extend(q3_env_inter);
    final_env_inter.extend(q4_env_inter);
    //log::info!("{:?}", &final_sitems);
    // log::info!("{:?}", &final_items);
    (
        final_map,
        final_npcs,
        final_sitems,
        final_items,
        final_env_inter,
    )
}
