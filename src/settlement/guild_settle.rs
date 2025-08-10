use crate::dialogue::{load_comms, load_convos, CommDialogue, ConvoDialogue};
use crate::enums::Shops;
use crate::enums::{Cells, Door, EnvInter, NPCWrap, Settle};
use crate::item::Item;
use crate::npc::{new_comm_npc, new_conv_npc, new_shop_npc, Convo, ShopConvos, ShopData};
use crate::npc_utils::box_npc;
use crate::settlement::parse_map;
use crate::shop::Shop;

use rand::prelude::SliceRandom;
use rand::Rng;

//use serde::{Deserialize, Serialize};
//use serde_json::Result;
//use serde_json::Value;
use std::fs;

use std::collections::HashMap;

const GUILD_ITEM_STALL_1: &str = r#"ShopNPC|HealthPotion Salve IronLongsword WoodenStaff HealthPotion Salve IronLongsword WoodenStaff HealthPotion Salve IronLongsword WoodenStaff HealthPotion Salve IronLongsword WoodenStaff|Null
________________________
________________________
___________─┬─___─┬─____
_────┐_____o│o___o│o____
_____│_____o│o___o│o____
_____│_____─┼─___─┼─____
_____│_____o│o___o│o____
___@_│_____o│o___o│o____
_____│_____─┴─___─┴─____
________________________
________________________
________________________
"#;

const GUILD_CLINIC_STALL_1: &str = r#"CommNPC|Null|Null
________________________
________________________
__┌───┐___________┌───__
__│_🁢_____________│_____
__├───┤___________│_@___
__│_🁢_____________└──___
__├───┤_________________
__│_🁢___________________
__└───┘┌─_─┬─_─┬─_─┐____
_______│_🁢_│_🁢_│_🁢_│____
_______└───┴───┴───┘____
________________________
"#;

const GUILD_WEAPON_STALL_1: &str = r#"ShopNPC|IronLongsword WoodenStaff IronLongsword WoodenStaff IronLongsword WoodenStaff IronLongsword WoodenStaff IronLongsword WoodenStaff IronLongsword WoodenStaff IronLongsword WoodenStaff IronLongsword WoodenStaff|Null
________________________
________________________
___________─┬─___─┬─____
_────┐_____o│o___o│o____
_____│_____o│o___o│o____
_____│_____─┼─___─┼─____
_____│_____o│o___o│o____
___@_│_____o│o___o│o____
_____│_____─┴─___─┴─____
________________________
________________________
________________________
"#;

const GUILD_ARMOUR_STALL_1: &str = r#"ShopNPC|IronLongsword WoodenStaff IronLongsword WoodenStaff IronLongsword WoodenStaff IronLongsword WoodenStaff IronLongsword WoodenStaff IronLongsword WoodenStaff IronLongsword WoodenStaff IronLongsword WoodenStaff|Null
________________________
________________________
___________─┬─___─┬─____
_────┐_____o│o___o│o____
_____│_____o│o___o│o____
_____│_____─┼─___─┼─____
_____│_____o│o___o│o____
___@_│_____o│o___o│o____
_____│_____─┴─___─┴─____
________________________
________________________
________________________
"#;

const GUILD_CANTEEN_STALL_1: &str = r#"CommNPC|Null|Null
________________________
________________________
_____π⑁___π⑁______──┐___
____________________│___
_____π⑁___π_________│___
____________________│___
_____π____π⑁________│___
__________________@_│___
_____π⑁___π_________│___
________________________
________________________
________________________
"#;

const GUILD_FILLER_1: &str = r#"Null|Null|Null
________________________
________________________
___≡≡≡__________________
___≡≡≡≡≡________________
____≡≡≡_________________
________________________
________π____π⑁_________
________________________
________π⑁___π__________
________________________
________________________
________________________
"#;

const GUILD_OFFICE_1: &str = r#"Null|Null|Null
________________________
________________________
________________________
________________________
________________________
________________________
________________________
________________________
________________________
________________________
________________________
________________________
________________________
________________________
════════__══════════════
________________________
________________________
________________________
________________________
________________________
________________________
________________________
________________________
________________________
"#;

const GUILD_DORM_1: &str = r#"Null|Null|Null
________________________
________________________
________┌───┬───┐_______
________│_🁢_│_🁢_│_______
____┌───┴─_─┴─_─┴───┐___
____│_🁢___________🁢_│___
____├───┤_______├───┤___
____│_🁢___________🁢_│___
____├───┤_______├───┤___
____│_🁢___________🁢_│___
____└───┘_______└───┘___
________________________
________________________
___┌───┐_______┌───┐____
___│_🁢___________🁢_│____
___├───┤_______├───┤____
___│_🁢___________🁢_│____
___├───┤_______├───┤____
___│_🁢___________🁢_│____
___└───┬─_─┬─_─┬───┘____
_______│_🁢_│_🁢_│________
_______└───┴───┘________
________________________
________________________
"#;

const GUILD_WALLS_LEFT: &str = r#"
################################################################################################################################
################################################################################################################################
####_______________________║________________________║______________________║_______________________║________________________####
####_______________________║________________________║______________________║_______________________║________________________####
####_______________________║________________________║______________________║_______________________║________________________####
####_______________________║________________________║______________________║_______________________║________________________####
####_______________________║________________________║______________________║_______________________║________________________####
####_______________________║________________________║______________________║_______________________║________________________####
####_______________________║________________________║______________________║_______________________║________________________####
####_______________________║________________________║______________________║_______________________║________________________####
####_______________________║________________________║______________________║_______________________║________________________####
####__________▩____________║________________________║______________________║_______________________║________________________####
####_______________________║________________________║______________________║_______________________║________________________####
####_______________________║________________________║______________________║_______________________║________________________####
####═══════════__══════════╩═══════════__═══════════╩══════════__══════════╩═══════════__══════════╣________________________####
####_______________________________________________________________________________________________║________________________####
####______________________________________________________________________________________________ঌ║________________________####
####________▧_▨▧_▨__________________▧_▨▧_▨___________________▧_▨▧_▨__________________▧_▨▧_▨_______Ť║________________________####
####ঌ________▩__▩____________________▩__▩_____________________▩__▩____________________▩__▩_________║________________________####
####Ť_______▨_▧▨_▧________####______▨_▧▨_▧________####_______▨_▧▨_▧_______####_______▨_▧▨_▧________║________________________####
####______________________####____________________####ঌ___________________####______________________________________________####
####______________________####____________________####Ť___________________####______________________________________________####
####_______________________________________________________________________________________________║________________________####
####▩_▩_▩_▩_▩_▩_▩_▩_▩_▩_▩_▩_▩_▩_▩_▩_▩_▩_▩_▩_▩_▩_▩_▩_▩_▩_▩_▩_▩_▩_▩_▩_▩_▩_▩_▩_▩_▩_▩_▩_▩_▩_▩_▩_▩_▩_▩_▩║________________________####
####_▧_____▧_____▧_____▧_____▧_____▧_____▧_____▧_____▧_____▧_____▧_____▧_____▧_____▧_____▧_____▧___║________________________####
________▨_____▨_____▨_____▨_____▨_____▨_____▨_____▨_____▨_____▨_____▨_____▨_____▨_____▨_____▨_____▨╠════════════════════════####
_____▨_____▨_____▨_____▨_____▨_____▨_____▨_____▨_____▨_____▨_____▨_____▨_____▨_____▨_____▨_____▨___║________________________####
####____▧_____▧_____▧_____▧_____▧_____▧_____▧_____▧_____▧_____▧_____▧_____▧_____▧_____▧_____▧_____▧║________________________####
####▩_▩_▩_▩_▩_▩_▩_▩_▩_▩_▩_▩_▩_▩_▩_▩_▩_▩_▩_▩_▩_▩_▩_▩_▩_▩_▩_▩_▩_▩_▩_▩_▩_▩_▩_▩_▩_▩_▩_▩_▩_▩_▩_▩_▩_▩_▩_▩║________________________####
####_______________________________________________________________________________________________║________________________####
####______________________####____________________####ঌ___________________####_____________________║________________________####
####ঌ_____________________####____________________####Ť___________________####______________________________________________####
####Ť_______▧_▨▧_▨________####______▧_▨▧_▨________####_______▧_▨▧_▨_______####_______▧_▨▧_▨_________________________________####
####_________▩__▩____________________▩__▩_____________________▩__▩____________________▩__▩_________║________________________####
####________▨_▧▨_▧__________________▨_▧▨_▧___________________▨_▧▨_▧__________________▨_▧▨_▧_______ঌ║________________________####
####______________________________________________________________________________________________Ť║________________________####
####_______________________________________________________________________________________________║________________________####
####═══════════__═══════════╦══════════__══════════╦════════════__══════════╦══════════__══════════╣________________________####
####________________________║______________________║________________________║______________________║________________________####
####________________________║______________________║________________________║______________________║________________________####
####________________________║______________________║________________________║______________________║________________________####
####________________________║______________________║________________________║______________________║________________________####
####________________________║______________________║________________________║______________________║________________________####
####________________________║______________________║________________________║______________________║________________________####
####________________________║______________________║________________________║______________________║________________________####
####________________________║______________________║________________________║______________________║________________________####
####________________________║______________________║________________________║______________________║________________________####
####________________________║______________________║________________________║______________________║________________________####
####________________________║______________________║________________________║______________________║________________________####
####________________________║______________________║________________________║______________________║________________________####
################################################################################################################################
################################################################################################################################
"#;

pub fn add_guild_walls(mut map: Vec<Vec<Cells>>, left: bool) -> Vec<Vec<Cells>> {
    let walls = match left {
        true => GUILD_WALLS_LEFT,
        // false => GUILD_WALLS_RIGHT,
        _ => todo!(),
    };

    for (y, line) in walls.lines().skip(1).enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '_' {
                continue;
            };
            map[y][x] = match ch {
                '═' => Cells::MwH,
                '║' => Cells::MwV,
                '╣' => Cells::MwVL,
                '╠' => Cells::MwVR,
                '╩' => Cells::MwHU,
                '╦' => Cells::MwHD,
                '▧' => Cells::Tile1,
                '▨' => Cells::Tile2,
                '▩' => Cells::Tile3,
                '#' => Cells::Wall,
                _ => todo!(),
            };
        }
    }
    map
}

const GUILD_ITEM_STALLS: [&str; 1] = [GUILD_ITEM_STALL_1];
const GUILD_CLINIC_STALLS: [&str; 1] = [GUILD_CLINIC_STALL_1];
const GUILD_WEAPONS_STALLS: [&str; 1] = [GUILD_WEAPON_STALL_1];
const GUILD_ARMOUR_STALLS: [&str; 1] = [GUILD_ARMOUR_STALL_1];
const GUILD_CANTEEN_STALLS: [&str; 1] = [GUILD_CANTEEN_STALL_1];
const GUILD_FILLERS: [&str; 1] = [GUILD_FILLER_1];
const GUILD_OFFICES: [&str; 1] = [GUILD_OFFICE_1];
const GUILD_DORMS: [&str; 1] = [GUILD_DORM_1];

pub fn build_guild_settle() -> (
    Vec<Vec<Cells>>,
    HashMap<(usize, usize), NPCWrap>,
    HashMap<(usize, usize), Item>,
    HashMap<(usize, usize), Item>,
    HashMap<(usize, usize), EnvInter>,
) {
    let mut rng = rand::thread_rng();
    let cells = vec![vec![Cells::Empty; 128]; 52];
    let face_left = true;
    // let face_left = [true, false].choose(&mut rng).unwrap_or(&true);
    let mut blocks: Vec<u8> = (1..9).collect();
    blocks.shuffle(&mut rng);

    let (item_map, item_npcs, item_sitems, item_items, item_env_inter) = parse_map(
        GUILD_ITEM_STALLS
            .choose(&mut rng)
            .unwrap_or(&GUILD_ITEM_STALLS[0]),
        vec![vec![Cells::Null; 24]; 12],
        Shops::Item,
    );

    let (clinic_map, clinic_npcs, clinic_sitems, clinic_items, clinic_env_inter) = parse_map(
        GUILD_CLINIC_STALLS
            .choose(&mut rng)
            .unwrap_or(&GUILD_CLINIC_STALLS[0]),
        vec![vec![Cells::Null; 24]; 12],
        Shops::Null,
    );

    let (weapons_map, weapons_npcs, weapons_sitems, weapons_items, weapons_env_inter) = parse_map(
        GUILD_WEAPONS_STALLS
            .choose(&mut rng)
            .unwrap_or(&GUILD_WEAPONS_STALLS[0]),
        vec![vec![Cells::Null; 24]; 12],
        Shops::Item,
    );

    let (armour_map, armour_npcs, armour_sitems, armour_items, armour_env_inter) = parse_map(
        GUILD_ARMOUR_STALLS
            .choose(&mut rng)
            .unwrap_or(&GUILD_ARMOUR_STALLS[0]),
        vec![vec![Cells::Null; 24]; 12],
        Shops::Item,
    );

    let (canteen1_map, canteen1_npcs, canteen1_sitems, canteen1_items, canteen1_env_inter) =
        parse_map(
            GUILD_CANTEEN_STALLS
                .choose(&mut rng)
                .unwrap_or(&GUILD_CANTEEN_STALLS[0]),
            vec![vec![Cells::Null; 24]; 12],
            Shops::Null,
        );

    let (filler1_map, filler1_npcs, filler1_sitems, filler1_items, filler1_env_inter) = parse_map(
        GUILD_FILLERS.choose(&mut rng).unwrap_or(&GUILD_FILLERS[0]),
        vec![vec![Cells::Null; 24]; 12],
        Shops::Null,
    );

    let (canteen2_map, canteen2_npcs, canteen2_sitems, canteen2_items, canteen2_env_inter) =
        parse_map(
            GUILD_CANTEEN_STALLS
                .choose(&mut rng)
                .unwrap_or(&GUILD_CANTEEN_STALLS[0]),
            vec![vec![Cells::Null; 24]; 12],
            Shops::Null,
        );

    let (filler2_map, filler2_npcs, filler2_sitems, filler2_items, filler2_env_inter) = parse_map(
        GUILD_FILLERS.choose(&mut rng).unwrap_or(&GUILD_FILLERS[0]),
        vec![vec![Cells::Null; 24]; 12],
        Shops::Null,
    );

    let (office_map, office_npcs, office_sitems, office_items, office_env_inter) = parse_map(
        GUILD_OFFICES.choose(&mut rng).unwrap_or(&GUILD_OFFICES[0]),
        vec![vec![Cells::Null; 24]; 24],
        Shops::Null,
    );

    let (dorm_map, dorm_npcs, dorm_sitems, dorm_items, dorm_env_inter) = parse_map(
        GUILD_DORMS.choose(&mut rng).unwrap_or(&GUILD_DORMS[0]),
        vec![vec![Cells::Null; 24]; 24],
        Shops::Null,
    );

    let (b1_map, b1_npcs, b1_sitems, b1_items, b1_env_inter) = place_guild_parts(
        vec![vec![Cells::Null; 128]; 52],
        item_map,
        item_npcs,
        item_sitems,
        item_items,
        item_env_inter,
        blocks[0],
        face_left,
        // *face_left,
    );

    let (b2_map, b2_npcs, b2_sitems, b2_items, b2_env_inter) = place_guild_parts(
        b1_map.clone(),
        clinic_map,
        clinic_npcs,
        clinic_sitems,
        clinic_items,
        clinic_env_inter,
        blocks[1],
        face_left,
        // *face_left,
    );

    let (b3_map, b3_npcs, b3_sitems, b3_items, b3_env_inter) = place_guild_parts(
        b2_map.clone(),
        weapons_map,
        weapons_npcs,
        weapons_sitems,
        weapons_items,
        weapons_env_inter,
        blocks[2],
        face_left,
        // *face_left,
    );

    let (b4_map, b4_npcs, b4_sitems, b4_items, b4_env_inter) = place_guild_parts(
        b3_map.clone(),
        armour_map,
        armour_npcs,
        armour_sitems,
        armour_items,
        armour_env_inter,
        blocks[3],
        face_left,
        // *face_left,
    );

    let (b5_map, b5_npcs, b5_sitems, b5_items, b5_env_inter) = place_guild_parts(
        b4_map.clone(),
        canteen1_map,
        canteen1_npcs,
        canteen1_sitems,
        canteen1_items,
        canteen1_env_inter,
        blocks[4],
        face_left,
        // *face_left,
    );

    let (b6_map, b6_npcs, b6_sitems, b6_items, b6_env_inter) = place_guild_parts(
        b5_map.clone(),
        filler1_map,
        filler1_npcs,
        filler1_sitems,
        filler1_items,
        filler1_env_inter,
        blocks[5],
        face_left,
        // *face_left,
    );

    let (b7_map, b7_npcs, b7_sitems, b7_items, b7_env_inter) = place_guild_parts(
        b6_map.clone(),
        canteen2_map,
        canteen2_npcs,
        canteen2_sitems,
        canteen2_items,
        canteen2_env_inter,
        blocks[6],
        face_left,
        // *face_left,
    );

    let (b8_map, b8_npcs, b8_sitems, b8_items, b8_env_inter) = place_guild_parts(
        b7_map.clone(),
        filler2_map,
        filler2_npcs,
        filler2_sitems,
        filler2_items,
        filler2_env_inter,
        blocks[7],
        face_left,
        // *face_left,
    );

    let off_pos = if rng.gen() { (9, 10) } else { (10, 9) };

    let (b9_map, b9_npcs, b9_sitems, b9_items, b9_env_inter) = place_guild_parts(
        b8_map.clone(),
        office_map,
        office_npcs,
        office_sitems,
        office_items,
        office_env_inter,
        off_pos.0,
        face_left,
        // *face_left,
    );

    let (b10_map, b10_npcs, b10_sitems, b10_items, b10_env_inter) = place_guild_parts(
        b9_map.clone(),
        dorm_map,
        dorm_npcs,
        dorm_sitems,
        dorm_items,
        dorm_env_inter,
        off_pos.1,
        face_left,
        // *face_left,
    );

    let final_map = add_guild_walls(b10_map, face_left);

    let mut final_npcs = HashMap::new();
    let mut final_sitems = HashMap::new();
    let mut final_items = HashMap::new();
    let mut final_env_inter = HashMap::new();
    final_npcs.extend(b1_npcs);
    final_npcs.extend(b2_npcs);
    final_npcs.extend(b3_npcs);
    final_npcs.extend(b4_npcs);
    final_npcs.extend(b5_npcs);
    final_npcs.extend(b6_npcs);
    final_npcs.extend(b7_npcs);
    final_npcs.extend(b8_npcs);
    final_npcs.extend(b9_npcs);
    final_npcs.extend(b10_npcs);
    final_sitems.extend(b1_sitems);
    final_sitems.extend(b2_sitems);
    final_sitems.extend(b3_sitems);
    final_sitems.extend(b4_sitems);
    final_sitems.extend(b5_sitems);
    final_sitems.extend(b6_sitems);
    final_sitems.extend(b7_sitems);
    final_sitems.extend(b8_sitems);
    final_sitems.extend(b9_sitems);
    final_sitems.extend(b10_sitems);
    final_items.extend(b1_items);
    final_items.extend(b2_items);
    final_items.extend(b3_items);
    final_items.extend(b4_items);
    final_items.extend(b5_items);
    final_items.extend(b6_items);
    final_items.extend(b7_items);
    final_items.extend(b8_items);
    final_items.extend(b9_items);
    final_items.extend(b10_items);
    final_env_inter.extend(b1_env_inter);
    final_env_inter.extend(b2_env_inter);
    final_env_inter.extend(b3_env_inter);
    final_env_inter.extend(b4_env_inter);
    final_env_inter.extend(b5_env_inter);
    final_env_inter.extend(b6_env_inter);
    final_env_inter.extend(b7_env_inter);
    final_env_inter.extend(b8_env_inter);
    final_env_inter.extend(b9_env_inter);
    final_env_inter.extend(b10_env_inter);
    (
        final_map,
        final_npcs,
        final_sitems,
        final_items,
        final_env_inter,
    )
}

pub fn place_guild_parts(
    mut map: Vec<Vec<Cells>>,
    part: Vec<Vec<Cells>>,
    npcs: HashMap<(usize, usize), NPCWrap>,
    sitems: HashMap<(usize, usize), Item>,
    items: HashMap<(usize, usize), Item>,
    env_inter: HashMap<(usize, usize), EnvInter>,
    block: u8,
    left: bool,
) -> (
    Vec<Vec<Cells>>,
    HashMap<(usize, usize), NPCWrap>,
    HashMap<(usize, usize), Item>,
    HashMap<(usize, usize), Item>,
    HashMap<(usize, usize), EnvInter>,
) {
    let (sx, sy) = if left {
        match block {
            1 => (4, 2),
            2 => (28, 2),
            3 => (52, 2),
            4 => (76, 2),
            5 => (4, 38),
            6 => (28, 38),
            7 => (52, 38),
            8 => (76, 38),
            9 => (100, 2),
            10 => (100, 26),
            _ => {
                log::info!("small parts error");
                (0, 0)
            }
        }
    } else {
        match block {
            1 => (28, 2),
            2 => (52, 2),
            3 => (76, 2),
            4 => (100, 2),
            5 => (28, 38),
            6 => (52, 38),
            7 => (76, 38),
            8 => (100, 26),
            9 => (4, 2),
            10 => (4, 26),
            _ => {
                log::info!("guild parts error");
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
