use crate::enums::{Cells, Door, EnvInter, NPCWrap, Settle};
use crate::enums::{ShopItem, Shops};
use crate::item::Item;
use crate::npc::ShopNPC;
use crate::settlement::parse_map;
use crate::settlement::settle_parts::*;

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
    sitems: HashMap<(usize, usize), ShopItem>,
    items: HashMap<(usize, usize), Item>,
    env_inter: HashMap<(usize, usize), EnvInter>,
    block: u8,
) -> (
    Vec<Vec<Cells>>,
    HashMap<(usize, usize), NPCWrap>,
    HashMap<(usize, usize), ShopItem>,
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

enum BlockType {
    Item,
    Guild,
    Church,
    Herbalist,
    Clinic,
    Anchor,
    Residential,
    Open,
    Weapons,
    Armour,
    Consignment,
}

pub fn build_med_settle() -> (
    Vec<Vec<Cells>>,
    HashMap<(usize, usize), NPCWrap>,
    HashMap<(usize, usize), ShopItem>,
    HashMap<(usize, usize), Item>,
    HashMap<(usize, usize), EnvInter>,
    HashMap<(usize, usize), ShopNPC>,
) {
    let mut rng = rand::thread_rng();
    let mut blocks: Vec<u8> = (1..10).collect();
    blocks.shuffle(&mut rng);
    let essential = vec![
        BlockType::Item,
        BlockType::Anchor,
        BlockType::Church,
        BlockType::Guild,
        BlockType::Residential,
        BlockType::Clinic,
        BlockType::Herbalist,
        BlockType::Open,
        BlockType::Open,
    ];

    let (item_map, item_npcs, item_sitems, item_items, item_env_inter, item_shop_npcs) = parse_map(
        ITEM_SHOPS.choose(&mut rng).unwrap_or(&ITEM_SHOPS[0]),
        vec![vec![Cells::Null; 75]; 25],
        Shops::Item,
    );
    let (guild_map, guild_npcs, guild_sitems, guild_items, guild_env_inter, guild_shop_npcs) =
        parse_map(
            GUILD_SHOPS.choose(&mut rng).unwrap_or(&GUILD_SHOPS[0]),
            vec![vec![Cells::Null; 75]; 25],
            Shops::Guild,
        );
    let (church_map, church_npcs, church_sitems, church_items, church_env_inter, church_shop_npcs) =
        parse_map(
            CHURCHES.choose(&mut rng).unwrap_or(&CHURCHES[0]),
            vec![vec![Cells::Null; 75]; 25],
            Shops::Church,
        );
    let (anchor_map, anchor_npcs, anchor_sitems, anchor_items, anchor_env_inter, anchor_shop_npcs) =
        parse_map(
            ANCHORS.choose(&mut rng).unwrap_or(&ANCHORS[0]),
            vec![vec![Cells::Null; 75]; 25],
            Shops::Null,
        );
    let (
        residential_map,
        residential_npcs,
        residential_sitems,
        residential_items,
        residential_env_inter,
        residential_shop_npcs,
    ) = parse_map(
        RESIDENTIALS.choose(&mut rng).unwrap_or(&RESIDENTIALS[0]),
        vec![vec![Cells::Null; 75]; 25],
        Shops::Null,
    );
    let (clinic_map, clinic_npcs, clinic_sitems, clinic_items, clinic_env_inter, clinic_shop_npcs) =
        parse_map(
            CLINICS.choose(&mut rng).unwrap_or(&CLINICS[0]),
            vec![vec![Cells::Null; 75]; 25],
            Shops::Null,
        );
    let (
        herbalist_map,
        herbalist_npcs,
        herbalist_sitems,
        herbalist_items,
        herbalist_env_inter,
        herbalist_shop_npcs,
    ) = parse_map(
        HERBALISTS.choose(&mut rng).unwrap_or(&HERBALISTS[0]),
        vec![vec![Cells::Null; 75]; 25],
        Shops::Null,
    );
    let (open1_map, open1_npcs, open1_sitems, open1_items, open1_env_inter, open1_shop_npcs) =
        parse_map(
            OPENS.choose(&mut rng).unwrap_or(&OPENS[0]),
            vec![vec![Cells::Null; 75]; 25],
            Shops::Null,
        );
    let (open2_map, open2_npcs, open2_sitems, open2_items, open2_env_inter, open2_shop_npcs) =
        parse_map(
            OPENS.choose(&mut rng).unwrap_or(&OPENS[0]),
            vec![vec![Cells::Null; 75]; 25],
            Shops::Null,
        );

    let (b1_map, b1_npcs, b1_sitems, b1_items, b1_env_inter) = place_med_parts(
        vec![vec![Cells::Null; 225]; 75],
        item_map,
        item_npcs,
        item_sitems,
        item_items,
        item_env_inter,
        blocks[0],
    );
    let (b2_map, b2_npcs, b2_sitems, b2_items, b2_env_inter) = place_med_parts(
        b1_map.clone(),
        guild_map,
        guild_npcs,
        guild_sitems,
        guild_items,
        guild_env_inter,
        blocks[1],
    );
    let (b3_map, b3_npcs, b3_sitems, b3_items, b3_env_inter) = place_med_parts(
        b2_map.clone(),
        church_map,
        church_npcs,
        church_sitems,
        church_items,
        church_env_inter,
        blocks[2],
    );
    let (b4_map, b4_npcs, b4_sitems, b4_items, b4_env_inter) = place_med_parts(
        b3_map.clone(),
        residential_map,
        residential_npcs,
        residential_sitems,
        residential_items,
        residential_env_inter,
        blocks[3],
    );
    let (b5_map, b5_npcs, b5_sitems, b5_items, b5_env_inter) = place_med_parts(
        b4_map.clone(),
        clinic_map,
        clinic_npcs,
        clinic_sitems,
        clinic_items,
        clinic_env_inter,
        blocks[4],
    );
    let (b6_map, b6_npcs, b6_sitems, b6_items, b6_env_inter) = place_med_parts(
        b5_map.clone(),
        herbalist_map,
        herbalist_npcs,
        herbalist_sitems,
        herbalist_items,
        herbalist_env_inter,
        blocks[5],
    );
    let (b7_map, b7_npcs, b7_sitems, b7_items, b7_env_inter) = place_med_parts(
        b6_map.clone(),
        open1_map,
        open1_npcs,
        open1_sitems,
        open1_items,
        open1_env_inter,
        blocks[6],
    );
    let (b8_map, b8_npcs, b8_sitems, b8_items, b8_env_inter) = place_med_parts(
        b7_map.clone(),
        open2_map,
        open2_npcs,
        open2_sitems,
        open2_items,
        open2_env_inter,
        blocks[7],
    );
    let (final_map, b9_npcs, b9_sitems, b9_items, b9_env_inter) = place_med_parts(
        b8_map.clone(),
        anchor_map,
        anchor_npcs,
        anchor_sitems,
        anchor_items,
        anchor_env_inter,
        blocks[8],
    );
    let mut final_npcs = HashMap::new();
    let mut final_sitems = HashMap::new();
    let mut final_items = HashMap::new();
    let mut final_env_inter = HashMap::new();
    let mut final_shop_npcs = HashMap::new();
    final_npcs.extend(b1_npcs);
    final_npcs.extend(b2_npcs);
    final_npcs.extend(b3_npcs);
    final_npcs.extend(b4_npcs);
    final_npcs.extend(b5_npcs);
    final_npcs.extend(b6_npcs);
    final_npcs.extend(b7_npcs);
    final_npcs.extend(b8_npcs);
    final_npcs.extend(b9_npcs);
    final_sitems.extend(b1_sitems);
    final_sitems.extend(b2_sitems);
    final_sitems.extend(b3_sitems);
    final_sitems.extend(b4_sitems);
    final_sitems.extend(b5_sitems);
    final_sitems.extend(b6_sitems);
    final_sitems.extend(b7_sitems);
    final_sitems.extend(b8_sitems);
    final_sitems.extend(b9_sitems);
    final_items.extend(b1_items);
    final_items.extend(b2_items);
    final_items.extend(b3_items);
    final_items.extend(b4_items);
    final_items.extend(b5_items);
    final_items.extend(b6_items);
    final_items.extend(b7_items);
    final_items.extend(b8_items);
    final_items.extend(b9_items);
    final_env_inter.extend(b1_env_inter);
    final_env_inter.extend(b2_env_inter);
    final_env_inter.extend(b3_env_inter);
    final_env_inter.extend(b4_env_inter);
    final_env_inter.extend(b5_env_inter);
    final_env_inter.extend(b6_env_inter);
    final_env_inter.extend(b7_env_inter);
    final_env_inter.extend(b8_env_inter);
    final_env_inter.extend(b9_env_inter);
    final_shop_npcs.extend(item_shop_npcs);
    final_shop_npcs.extend(guild_shop_npcs);
    final_shop_npcs.extend(church_shop_npcs);
    final_shop_npcs.extend(herbalist_shop_npcs);
    final_shop_npcs.extend(clinic_shop_npcs);
    final_shop_npcs.extend(open1_shop_npcs);
    final_shop_npcs.extend(open2_shop_npcs);
    final_shop_npcs.extend(residential_shop_npcs);
    final_shop_npcs.extend(anchor_shop_npcs);
    (
        final_map,
        final_npcs,
        final_sitems,
        final_items,
        final_env_inter,
        final_shop_npcs,
    )
}
