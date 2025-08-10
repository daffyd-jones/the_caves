//settlement rs
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

mod med_settle;
mod settle_parts;
mod sm_settle;

// med: 300x200 sm: 150x100 | sm: 2(75)x2(25) med: 3(75)x3(25)

fn parse_map(
    s_map: &str,
    mut cells: Vec<Vec<Cells>>,
    shop_type: Shops,
) -> (
    Vec<Vec<Cells>>,
    HashMap<(usize, usize), NPCWrap>,
    HashMap<(usize, usize), Item>,
    HashMap<(usize, usize), Item>,
    HashMap<(usize, usize), EnvInter>,
) {
    // let mut cells: Vec<Vec<Cells>> = Vec::new();
    let mut rng = rand::thread_rng();
    let map_codet = s_map.lines().next().unwrap_or("");
    let map_code: Vec<&str> = map_codet.split("|").collect();
    let npc_types: Vec<&str> = map_code.clone()[0].split(" ").collect();
    let sitem_types: Vec<&str> = map_code.clone()[1].split(" ").collect();
    let item_types: Vec<&str> = map_code.clone()[2].split(" ").collect();

    let data1 = fs::read_to_string("src/npcs/npc_names.json");
    //log::info!("{:?}", &data1);
    let names: Vec<String> = match data1 {
        Ok(content) => serde_json::from_str(&content).unwrap(),
        Err(e) => {
            log::info!("{:?}", e);
            Vec::new()
        }
    };

    let comms = match shop_type {
        Shops::Item => load_comms(&"cave".to_string()),
        Shops::Guild => load_comms(&"guild".to_string()),
        Shops::Church => load_comms(&"cult".to_string()),
        _ => load_comms(&"cave".to_string()),
    };

    let convos = match shop_type {
        Shops::Item => load_convos(&"cave".to_string()),
        Shops::Guild => load_convos(&"guild".to_string()),
        Shops::Church => load_convos(&"cult".to_string()),
        _ => load_convos(&"cave".to_string()),
    };

    let data4 = fs::read_to_string("src/npcs/npc_shops.json");
    //log::info!("{:?}", &data4);
    let shops: ShopData = match data4 {
        Ok(content) => serde_json::from_str(&content).unwrap(),
        Err(e) => {
            log::info!("{:?}", e);
            ShopData {
                shops: Vec::new(),
                guilds: Vec::new(),
                churches: Vec::new(),
            }
        }
    };

    log::info!("shops: {:?}", &shops);

    let data5 = fs::read_to_string("src/npcs/npc_shop_convos.json");
    //log::info!("{:?}", &data5);
    let shop_convos: ShopConvos = match data5 {
        Ok(content) => serde_json::from_str(&content).unwrap(),
        Err(e) => {
            log::info!("{:?}", e);
            ShopConvos {
                shops: Vec::new(),
                guilds: Vec::new(),
                churches: Vec::new(),
            }
        }
    };

    let mut ncount = 0;
    let mut icount = 0;
    let mut sicount = 0;
    //let mut cells = vec![vec![Cells::Null; 150]; 50];
    let mut npcs = HashMap::new();
    let mut items = HashMap::new();
    let mut sitems = HashMap::new();
    let mut env_inters = HashMap::new();
    for (y, line) in s_map.lines().skip(1).enumerate() {
        for (x, ch) in line.chars().enumerate() {
            let cell = match ch {
                '!' => Cells::Seasonal1,
                '$' => Cells::Seasonal2,
                '%' => Cells::Seasonal3,
                '_' => Cells::Empty,
                ',' => Cells::Grass1,
                '\'' => Cells::Grass2,
                '\"' => Cells::Grass3,
                '⚶' => Cells::TallGrass,
                '·' => Cells::Dirt1,
                '.' => Cells::Dirt2,
                ':' => Cells::Dirt3,
                '*' => Cells::Rock,
                '▒' => Cells::Wall,
                '🬤' => Cells::Broken1,
                '🬗' => Cells::Broken2,
                '🬐' => Cells::Broken3,
                '🬑' => Cells::Broken4,
                '🬮' => Cells::Broken5,
                '🬡' => Cells::Broken6,
                ' ' => Cells::Floor,
                '░' => Cells::Floor2,
                '▧' => Cells::Tile1,
                '▨' => Cells::Tile2,
                '▩' => Cells::Tile3,
                '~' => Cells::Water,
                '═' => Cells::MwH,
                '║' => Cells::MwV,
                '╣' => Cells::MwVL,
                '╠' => Cells::MwVR,
                '╩' => Cells::MwHU,
                '╦' => Cells::MwHD,
                '╝' => Cells::MwUL,
                '╚' => Cells::MwUR,
                '╗' => Cells::MwDL,
                '╔' => Cells::MwDR,
                '╬' => Cells::MwCR,
                '─' => Cells::SwH,
                '│' => Cells::SwV,
                '┤' => Cells::SwVL,
                '├' => Cells::SwVR,
                '┴' => Cells::SwHU,
                '┬' => Cells::SwHD,
                '┘' => Cells::SwUL,
                '└' => Cells::SwUR,
                '┐' => Cells::SwDL,
                '┌' => Cells::SwDR,
                '┼' => Cells::SwCR,
                '╭' => Cells::CurUL,
                '╮' => Cells::CurUR,
                '╰' => Cells::CurBL,
                '╯' => Cells::CurBR,
                '╟' => Cells::BsVR,
                '╢' => Cells::BsVL,
                '╤' => Cells::BsHD,
                '╧' => Cells::BsHU,
                '┆' => Cells::BknWV,
                '┄' => Cells::BknWH,
                '≡' => Cells::Cong,
                '°' => Cells::Deg,
                '×' => Cells::Mult,
                '¸' => Cells::Ced,
                '¨' => Cells::Diae,
                '■' => Cells::Blsq,
                '¦' => Cells::VBrk,
                '±' => Cells::PlMin,
                'ø' => Cells::SmZer,
                'Ø' => Cells::BZer,
                '©' => Cells::Cop,
                'Ħ' => Cells::DblBracedGate,
                'ỻ' => Cells::BracedGate,
                'Π' => Cells::Arch,
                'ʭ' => Cells::Bricks,
                'ʬ' => Cells::Crops,
                'ѧ' => Cells::SmallCampfire,
                'Ѧ' => Cells::Campfire,
                'π' => Cells::Table,
                'ṑ' => Cells::Jar,
                '⑁' => Cells::Chair,
                'Һ' => Cells::ChairRight1,
                'Ⴙ' => Cells::ChairRight2,
                'ж' => Cells::Firewood,
                'ঌ' => Cells::FireSmoke,
                '܀' => Cells::FireDiamond,
                'ஃ' => Cells::FireTri,
                'Ŧ' => Cells::Stand1,
                'Ÿ' => Cells::Stand2,
                'Ť' => Cells::Stand3,
                'ƃ' => Cells::StandBL,
                'ƌ' => Cells::StandDL,
                'Ƃ' => Cells::StandBS,
                'Ƌ' => Cells::StandDS,
                '◍' => Cells::CircleVL,
                '⏣' => Cells::CircleHex,
                '⌬' => Cells::CircleC,
                '⌹' => Cells::Drawers,
                '⌸' => Cells::Shelves,
                '⚱' => Cells::Vase,
                '𜲄' => Cells::LadderV,
                '𜲅' => Cells::LadderH,
                '𜲐' => Cells::TickV,
                '𜲑' => Cells::TickH,
                'ቋ' => Cells::Tech1,
                '🝻' => Cells::Tech2,
                '🜟' => Cells::Tech3,
                'ଏ' => Cells::Tech4,
                'Ҧ' => Cells::Tech5,
                'Ҹ' => Cells::Tech6,
                'Ҵ' => Cells::Tech7,
                'ౝ' => Cells::Tech8,
                '𜰔' => Cells::Tech9,
                '𜰓' => Cells::Tech10,
                '𜰉' => Cells::Tech11,
                '𜰊' => Cells::Tech12,
                '⛀' => Cells::Tech13,
                '⛁' => Cells::Tech14,
                '⛂' => Cells::Tech15,
                '⛃' => Cells::Tech16,
                'Ⴉ' => Cells::Tech17,
                'ቖ' => Cells::Relic1,
                '⚗' => Cells::Alembic,
                'ቷ' => Cells::OldWall1,
                'ቿ' => Cells::OldWall2,
                'ቨ' => Cells::OldWall3,
                'ቩ' => Cells::OldWall4,
                'ቭ' => Cells::OldWall5,
                '🀫' => Cells::CardTile1,
                '🀘' => Cells::CardTile2,
                '🀆' => Cells::CardTile3,
                '🀙' => Cells::CardTile4,
                'Ʌ' => Cells::Tent,
                '🁢' => Cells::Bed,
                // '@' => Cells::NPCM,
                '{' => Cells::LBrce,
                '}' => Cells::RBrce,
                '(' => Cells::LParen,
                ')' => Cells::RParen,
                '¤' => Cells::GenCur,
                'o' => Cells::Item,
                'l' => Cells::Log,
                'c' => Cells::Clinic,
                'p' => Cells::GPost,
                's' => Cells::CPost,
                '#' => Cells::Transparent,
                _ => Cells::Empty,
            };
            cells[y][x] = cell;
            if ch == '@' {
                let def_name = "Kevthony".to_string();
                match npc_types[ncount] {
                    "CommNPC" => {
                        //let com_def = vec!["Welcome to the caves!!".to_string(), "Theres a tonne of folk down here, lerger cities as you go into the cave.".to_string()];
                        let rnd_comms = {
                            let mut tvec = Vec::new();
                            for _ in 0..4 {
                                tvec.push(match rng.gen_range(0..3) {
                                    0 => comms
                                        .city
                                        .choose(&mut rng)
                                        .unwrap_or(&comms.city[0])
                                        .clone(),
                                    1 => comms
                                        .engine
                                        .choose(&mut rng)
                                        .unwrap_or(&comms.engine[0])
                                        .clone(),
                                    2 => comms
                                        .guild
                                        .choose(&mut rng)
                                        .unwrap_or(&comms.guild[0])
                                        .clone(),
                                    3 => comms
                                        .cult
                                        .choose(&mut rng)
                                        .unwrap_or(&comms.cult[0])
                                        .clone(),
                                    _ => todo!(),
                                });
                                // let tidx = rng.gen_range(0..comms.len());
                                // tvec.push(comms[tidx].clone());
                            }
                            tvec
                        };
                        let name = names.choose(&mut rng).unwrap_or(&def_name.clone()).clone();
                        let t_comm = new_comm_npc(name.clone(), x, y, rnd_comms.clone());
                        npcs.insert((x, y), NPCWrap::CommNPC(t_comm.clone()));
                    }
                    "ConvNPC" => {
                        let name = names.choose(&mut rng).unwrap_or(&def_name.clone()).clone();
                        //let comms = vec!["Welcome to the caves!!".to_string(), "Theres a tonne of folk down here, lerger cities as you go into the cave.".to_string()];
                        let conv: Convo = match rng.gen_range(0..3) {
                            0 => convos
                                .city
                                .choose(&mut rng)
                                .unwrap_or(&convos.city[0])
                                .clone(),
                            1 => convos
                                .engine
                                .choose(&mut rng)
                                .unwrap_or(&convos.engine[0])
                                .clone(),
                            2 => convos
                                .guild
                                .choose(&mut rng)
                                .unwrap_or(&convos.guild[0])
                                .clone(),
                            3 => convos
                                .cult
                                .choose(&mut rng)
                                .unwrap_or(&convos.cult[0])
                                .clone(),
                            _ => todo!(),
                        };

                        let t_comm = new_conv_npc(name.clone(), x, y, conv.clone());
                        npcs.insert((x, y), NPCWrap::ConvNPC(t_comm.clone()));
                    }
                    "ShopNPC" => {
                        let name = names.choose(&mut rng).unwrap_or(&def_name.clone()).clone();
                        // let s_conv: HashMap<String, String> = shops
                        //     .shops
                        //     .choose(&mut rng)
                        //     .unwrap_or(&shops.shops[0].clone())
                        //     .clone();

                        let s_conv = match shop_type {
                            Shops::Item => shops
                                .shops
                                .choose(&mut rng)
                                .unwrap_or(&shops.shops[0].clone())
                                .clone(),
                            Shops::Guild => shops
                                .guilds
                                .choose(&mut rng)
                                .unwrap_or(&shops.guilds[0].clone())
                                .clone(),
                            Shops::Church => shops
                                .churches
                                .choose(&mut rng)
                                .unwrap_or(&shops.churches[0].clone())
                                .clone(),
                            _ => todo!(),
                        };

                        let convo = match shop_type {
                            Shops::Item => shop_convos
                                .shops
                                .choose(&mut rng)
                                .unwrap_or(&shop_convos.shops[0].clone())
                                .clone(),
                            Shops::Guild => shop_convos
                                .guilds
                                .choose(&mut rng)
                                .unwrap_or(&shop_convos.guilds[0].clone())
                                .clone(),
                            Shops::Church => shop_convos
                                .churches
                                .choose(&mut rng)
                                .unwrap_or(&shop_convos.churches[0].clone())
                                .clone(),
                            _ => todo!(),
                        };

                        let t_shop = new_shop_npc(
                            name.clone(),
                            x,
                            y,
                            s_conv.clone(),
                            convo.clone(),
                            shop_type,
                        );
                        npcs.insert((x, y), NPCWrap::ShopNPC(t_shop.clone()));
                    }
                    _ => todo!(),
                }
                ncount += 1;
            }
            if ch == 'o' {
                match sitem_types[sicount] {
                    "HealthPotion" => {
                        let ti = Item::new_health_potion(x, y);
                        sitems.insert((x, y), ti.clone());
                    }
                    "Salve" => {
                        let ti = Item::new_salve(x, y);
                        sitems.insert((x, y), ti.clone());
                    }
                    "Dowel" => {
                        let ti = Item::new_dowel(x, y);
                        sitems.insert((x, y), ti.clone());
                    }
                    "SmallWoodShield" => {
                        let ti = Item::new_small_wood_shield(x, y);
                        sitems.insert((x, y), ti.clone());
                    }
                    "Apple" => {
                        let ti = Item::new_apple(x, y);
                        sitems.insert((x, y), ti.clone());
                    }
                    "BronzeClaymore" => {
                        let ti = Item::new_bronze_claymore(x, y);
                        sitems.insert((x, y), ti.clone());
                    }
                    "BronzeShortsword" => {
                        let ti = Item::new_bronze_shortsword(x, y);
                        sitems.insert((x, y), ti.clone());
                    }
                    "BronzeLongsword" => {
                        let ti = Item::new_bronze_longsword(x, y);
                        sitems.insert((x, y), ti.clone());
                    }
                    "BronzeLightAxe" => {
                        let ti = Item::new_bronze_light_axe(x, y);
                        sitems.insert((x, y), ti.clone());
                    }
                    "BronzeHeavyAxe" => {
                        let ti = Item::new_bronze_heavy_axe(x, y);
                        sitems.insert((x, y), ti.clone());
                    }
                    "BronzeWarAxe" => {
                        let ti = Item::new_bronze_war_axe(x, y);
                        sitems.insert((x, y), ti.clone());
                    }
                    "BronzePickHammer" => {
                        let ti = Item::new_bronze_pick_hammer(x, y);
                        sitems.insert((x, y), ti.clone());
                    }
                    "WoodStaff" => {
                        let ti = Item::new_wood_staff(x, y);
                        sitems.insert((x, y), ti.clone());
                    }
                    "LightArmour" => {
                        let ti = Item::new_light_armour(x, y);
                        sitems.insert((x, y), ti.clone());
                    }
                    "ShieldingPendant" => {
                        let ti = Item::new_shielding_pendant(x, y);
                        sitems.insert((x, y), ti.clone());
                    }
                    "StrengthPendant" => {
                        let ti = Item::new_strength_pendant(x, y);
                        sitems.insert((x, y), ti.clone());
                    }
                    "AgilityPendant" => {
                        let ti = Item::new_agility_pendant(x, y);
                        sitems.insert((x, y), ti.clone());
                    }
                    _ => {
                        log::info!("itm {:?}", sitem_types[sicount]);
                    }
                }
                sicount += 1;
            }
            if ch == 'O' {
                match item_types[icount] {
                    "HealthPotion" => {
                        let ti = Item::new_health_potion(x, y);
                        items.insert((x, y), ti.clone());
                    }
                    "Salve" => {
                        let ti = Item::new_salve(x, y);
                        items.insert((x, y), ti.clone());
                    }
                    "Dowel" => {
                        let ti = Item::new_dowel(x, y);
                        items.insert((x, y), ti.clone());
                    }
                    "SmallWoodShield" => {
                        let ti = Item::new_small_wood_shield(x, y);
                        items.insert((x, y), ti.clone());
                    }
                    "Apple" => {
                        let ti = Item::new_apple(x, y);
                        items.insert((x, y), ti.clone());
                    }
                    "BronzeClaymore" => {
                        let ti = Item::new_bronze_claymore(x, y);
                        items.insert((x, y), ti.clone());
                    }
                    "BronzeShortsword" => {
                        let ti = Item::new_bronze_shortsword(x, y);
                        items.insert((x, y), ti.clone());
                    }
                    "BronzeLongsword" => {
                        let ti = Item::new_bronze_longsword(x, y);
                        items.insert((x, y), ti.clone());
                    }
                    "BronzeLightAxe" => {
                        let ti = Item::new_bronze_light_axe(x, y);
                        items.insert((x, y), ti.clone());
                    }
                    "BronzeHeavyAxe" => {
                        let ti = Item::new_bronze_heavy_axe(x, y);
                        items.insert((x, y), ti.clone());
                    }
                    "BronzeWarAxe" => {
                        let ti = Item::new_bronze_war_axe(x, y);
                        items.insert((x, y), ti.clone());
                    }
                    "BronzePickHammer" => {
                        let ti = Item::new_bronze_pick_hammer(x, y);
                        items.insert((x, y), ti.clone());
                    }
                    "WoodStaff" => {
                        let ti = Item::new_wood_staff(x, y);
                        items.insert((x, y), ti.clone());
                    }
                    "LightArmour" => {
                        let ti = Item::new_light_armour(x, y);
                        items.insert((x, y), ti.clone());
                    }
                    _ => {
                        log::info!("itm {:?}", item_types[icount]);
                    }
                }
                icount += 1;
            }
            if ch == 'l' {
                env_inters.insert((x, y), EnvInter::Records);
            }
            if ch == 'p' {
                env_inters.insert((x, y), EnvInter::GuildPost);
            }
            if ch == 'c' {
                env_inters.insert((x, y), EnvInter::Clinic);
            }
            if ch == 'C' {
                env_inters.insert((x, y), EnvInter::Construction);
            }
            if ch == 's' {
                env_inters.insert((x, y), EnvInter::ChurchPost);
            }
            if ch == '℧' {
                env_inters.insert((x, y), EnvInter::Cauldron);
            }
            if ch == 'h' {
                env_inters.insert((x, y), EnvInter::Herbalist);
            }
            if ch == 'd' {
                env_inters.insert(
                    (x, y),
                    EnvInter::Door(Door::HLocked(rng.gen_range(0..10) as u8)),
                );
            }
            if ch == 'D' {
                env_inters.insert(
                    (x, y),
                    EnvInter::Door(Door::VLocked(rng.gen_range(0..10) as u8)),
                );
            }
        }
    }
    (cells, npcs, sitems, items, env_inters)
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

fn build_med_settle() -> (
    Vec<Vec<Cells>>,
    HashMap<(usize, usize), NPCWrap>,
    HashMap<(usize, usize), Item>,
    HashMap<(usize, usize), Item>,
    HashMap<(usize, usize), EnvInter>,
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

    let (item_map, item_npcs, item_sitems, item_items, item_env_inter) = parse_map(
        ITEM_SHOPS.choose(&mut rng).unwrap_or(&ITEM_SHOPS[0]),
        vec![vec![Cells::Null; 75]; 25],
        Shops::Item,
    );
    let (guild_map, guild_npcs, guild_sitems, guild_items, guild_env_inter) = parse_map(
        GUILD_SHOPS.choose(&mut rng).unwrap_or(&GUILD_SHOPS[0]),
        vec![vec![Cells::Null; 75]; 25],
        Shops::Guild,
    );
    let (church_map, church_npcs, church_sitems, church_items, church_env_inter) = parse_map(
        CHURCHES.choose(&mut rng).unwrap_or(&CHURCHES[0]),
        vec![vec![Cells::Null; 75]; 25],
        Shops::Church,
    );
    let (anchor_map, anchor_npcs, anchor_sitems, anchor_items, anchor_env_inter) = parse_map(
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
    ) = parse_map(
        RESIDENTIALS.choose(&mut rng).unwrap_or(&RESIDENTIALS[0]),
        vec![vec![Cells::Null; 75]; 25],
        Shops::Null,
    );
    let (clinic_map, clinic_npcs, clinic_sitems, clinic_items, clinic_env_inter) = parse_map(
        CLINICS.choose(&mut rng).unwrap_or(&CLINICS[0]),
        vec![vec![Cells::Null; 75]; 25],
        Shops::Null,
    );
    let (herbalist_map, herbalist_npcs, herbalist_sitems, herbalist_items, herbalist_env_inter) =
        parse_map(
            HERBALISTS.choose(&mut rng).unwrap_or(&HERBALISTS[0]),
            vec![vec![Cells::Null; 75]; 25],
            Shops::Null,
        );
    let (open1_map, open1_npcs, open1_sitems, open1_items, open1_env_inter) = parse_map(
        OPENS.choose(&mut rng).unwrap_or(&OPENS[0]),
        vec![vec![Cells::Null; 75]; 25],
        Shops::Null,
    );
    let (open2_map, open2_npcs, open2_sitems, open2_items, open2_env_inter) = parse_map(
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

fn add_guild_walls(mut map: Vec<Vec<Cells>>, left: bool) -> Vec<Vec<Cells>> {
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

fn build_guild_settle() -> (
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

fn place_guild_parts(
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

const OBSIDIAN_ITEM_STALL_1: &str = r#"ShopNPC|HealthPotion Salve IronLongsword WoodenStaff HealthPotion Salve IronLongsword WoodenStaff HealthPotion Salve IronLongsword WoodenStaff HealthPotion Salve IronLongsword WoodenStaff|Null
_________________│______
_________________│______
___─┬─___─┬─____________
___o│o___o│o_____│______
___o│o___o│o_____│@_____
___─┼─___─┼─_____└──────
___o│o___o│o____________
___o│o___o│o____________
___─┴─___─┴─____________
________________________
________________________
________________________
"#;

const OBSIDIAN_HERBALIST_STALL_1: &str = r#"ShopNPC|HealthPotion Salve HealthPotion Salve HealthPotion Salve HealthPotion Salve HealthPotion Salve HealthPotion Salve HealthPotion Salve HealthPotion Salve|Null
________________________
_⚱__ṑṑ_ṑṑṑ______________
_ṑ⚱________─┬─___─┬─____
_ṑṑ________o│o___o│o____
_ṑ_________o│o___o│o____
___________─┼─___─┼─____
___________o│o___o│o____
──_──┐_____o│o___o│o____
_____│_____─┴─___─┴─____
___@_│__________________
__π⑁_│℧___________ቋ_____
_____│__________________
"#;

const OBSIDIAN_CLINIC_STALL_1: &str = r#"CommNPC|Null|Null
________________________
________________________
__┌───┐__________┌───┐__
__│_🁢______________🁢_│__
__├───┤__________├───┤__
__│_🁢______________🁢_│__
__├───┤__________├───┤__
__│_🁢______________🁢_│__
__└───┘__________└───┘__
_____┌───┐________⌹⌹⌹___
_┌───┘_@_│______________
_│_______│______________
"#;

const OBSIDIAN_CANTEEN_STALL_1: &str = r#"CommNPC|Null|Null
________________________
________________________
____Ⴙπ⑁___π⑁______──┐___
____________________│___
_____π⑁__Ⴙπ_________│___
____________________│___
____Ⴙπ____π⑁________│___
__________________@_│___
____Ⴙπ⑁__Ⴙπ_________│___
________________________
________________________
________________________
"#;

const OBSIDIAN_FILLER_1: &str = r#"Null|Null|Null
________│_______________
_______ቋ│_______________
________________________
___≡≡≡≡≡│_______________
────────┘_______________
________________________
_____________┌─_________
──────______⌹│__________
__⌹⌹⌹_______⌹│🝻_________
_____________│__________
_____________│≡≡________
_____________│__________
"#;

const OBSIDIAN_OFFICE_1: &str = r#"Null|Null|Null
________│_______________
________________________
________│______Һπ│π⑁____
────────┘_____───┼───___
_______________Һπ│π⑁____
________________________
________________________
________________________
____Һπ│π⑁______Һπ│π⑁____
___───┼───____───┼───___
____Һπ│π⑁______Һπ│π⑁____
________________________
"#;

const OBSIDIAN_DORM: &str = r#"Null|Null|Null
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

const OBSIDIAN_WALLS_BOTTOM: &str = r#"
######################################################################################____######################################
######################################################################################____######################################
####_______________________║_______________________║_______________________║________▨_▩__▩_▧________║_______________________####
####_______________________║_______________________║_______________________║________▨___▨__▧________║_______________________####
####_______________________║_______________________║_______________________║________▨_▩__▩_▧________║_______________________####
####_______________________║_______________________║_______________________║________▨__▧___▧________║_______________________####
####_______________________║_______________________║_______________________║________▨_▩__▩_▧________║_______________________####
####_______________________║_______________________║_______________________║________▨___▨__▧________________________________####
####_______________________║_______________________║_______________________║________▨_▩__▩_▧________║_______________________####
####_______________________║_______________________║_______________________║________▨__▧___▧________║_______________________####
####_______________________║_______________________║_______________________║________▨_▩__▩_▧________║_______________________####
####_______________________║_______________________║_______________________║ஃ_______▨___▨__▧_______ஃ║_______________________####
####_______________________║_______________________║_______________________║Ÿ_______▨_▩__▩_▧_______Ÿ║_______________________####
####══════════__═══════════╩═══════════__══════════╩═══════════__══════════╝________▨__▧___▧________╠═══════════════════════####
####________________________________________________________________________________▨_▩__▩_▧________║_______________________####
####________________________________________________________________________________________________║_______________________####
####____________________________🀫_🀫_🀫_🀫_🀫_🀫_🀫_🀫_🀫_🀫_🀫_🀫_🀫_🀫_🀫_🀫__🀫_🀫_🀫_🀫_🀫_🀫_🀫_🀫_🀫_🀫_🀫_🀫_🀫_🀫_🀫_🀫____║_______________________####
####▨_▨_▨_▨_▨_▨_▨_▨_▨_▨_▨_▨_▨___🀫_🀙_________🀆___🀆___🀆___🀆___🀆______🀆___🀆___🀆___🀆___🀆_________🀙_🀫____║_______________________####
####▩___▩___▩___▩___▩___▩___▩___🀫___🀙_____🀆____🀆_🀆_🀆_🀆_🀆_🀆_🀆_🀆____🀆_🀆_🀆_🀆_🀆_🀆_🀆_🀆_🀆__🀆_____🀙___🀫____║_______________________####
__________▧_______▧_______▧_____🀫_____🀙_🀆_____🀆___🀆___🀆___🀆___🀆__🀆___🀆___🀆___🀆___🀆_____🀆_🀙_____🀫____________________________####
______▨_______▨_______▨_________🀫_____🀆_🀙___________________┌──────┐___________________🀙_🀆_____🀫____║_______________________####
####▩___▩___▩___▩___▩___▩___▩___🀫___🀆_____🀙_______╔═════════╧══════╧═════════╗_______🀙_____🀆___🀫____║_______________________####
####▧_▧_▧_▧_▧_▧_▧_▧_▧_▧_▧_▧_▧___🀫_🀆_________🀙_____║__________________________║_____🀙_________🀆_🀫____║_______________________####
####____________________________🀫___🀆_____🀙_______║__________________________║_______🀙_____🀆___🀫____║_______________________####
####____________________________🀫_____🀆_🀙_________║__________________________║_________🀙_🀆_____🀫____║_______________________####
####____________________________🀫_____🀙_🀆_________╚══════════════════════════╝_________🀆_🀙_____🀫____╠═══════════════════════####
####═══════════════════════╗ঌ___🀫___🀙_____🀆__________________________________________🀆_____🀙___🀫___ঌ║_______________________####
####_______________________║Ť___🀫_🀙_________🀆______________________________________🀆_________🀙_🀫___Ť║_______________________####
####_______________________║____🀫___🀙_______🀆______________________________________🀆_______🀙___🀫____║_______________________####
####_______________________║____🀫_____🀙_____🀆_└──────────────┘____└──────────────┘_🀆_____🀙_____🀫____║_______________________####
####_______________________║____🀫_______🀙_🀆__________________________________________🀆_🀙_______🀫____║_______________________####
####_______________________║____🀫_______🀆_🀙___└──────────────┘____└──────────────┘___🀙_🀆_______🀫____________________________####
####____________________________🀫_____🀆_____🀙______________________________________🀙_____🀆_____🀫____║_______________________####
####_______________________║____🀫___🀆_______🀙_└──────────────┘____└──────────────┘_🀙_______🀆___🀫____║_______________________####
####_______________________║____🀫_🀆_________🀙______________________________________🀙_________🀆_🀫____║_______________________####
####_______________________║܀___🀫___🀆_______🀙_└──────────────┘____└──────────────┘_🀙_______🀆___🀫___܀║_______________________####
####_______________________║Ŧ___🀫_____🀆___🀙__________________________________________🀙___🀆_____🀫___Ŧ║_______________________####
####_______________________║____🀫_______🀙_____└──────────────┘____└──────────────┘_____🀙_______🀫____║_______________________####
####_______________________║____🀫_____🀙___🀆_____🀆___🀆___🀆___🀆______🀆___🀆___🀆___🀆_____🀆___🀙_____🀫____╠═══════════════════════####
####_______________________║____🀫___🀙_______🀆__🀆_🀆_🀆_🀆_🀆_🀆_🀆_🀆____🀆_🀆_🀆_🀆_🀆_🀆_🀆_🀆__🀆_______🀙___🀫____║_______________________####
####_______________________║____🀫_🀙___________🀆___🀆___🀆___🀆___🀆__🀆___🀆___🀆___🀆___🀆___________🀙_🀫____║_______________________####
####_______________________║____🀫_🀫_🀫_🀫_🀫_🀫_🀫_🀫_🀫_🀫_🀫_🀫_🀫_🀫_🀫_🀫__🀫_🀫_🀫_🀫_🀫_🀫_🀫_🀫_🀫_🀫_🀫_🀫_🀫_🀫_🀫_🀫____║_______________________####
####_______________________║________________________________________________________________________║_______________________####
####_______________________║_________▩_▧_▨_▧_▨_▧_▨_▧_▨_▧_▨_▧_▨_▧▨_▧_▨_▧_▨_▧_▨_▧_▨_▧_▨_▧_▨_▩_________║_______________________####
####_______________________║_________▨____________________________________________________▧_________________________________####
####_______________________║_________▧____________________________________________________▨_________║_______________________####
####_______________________║_________▨____________________________________________________▧_________║_______________________####
####_______________________║_________▧____________________________________________________▨_________║_______________________####
####_______________________║_______ஃ_▨____________________________________________________▧_ஃ_______║_______________________####
####_______________________║_______Ŧ_▧____________________________________________________▨_Ŧ_______║_______________________####
##############################################################____##############################################################
##############################################################____##############################################################
"#;

const OOBSIDIAN_WALLS_BOTTOM: &str = r#"
######################################################################################____######################################
######################################################################################____######################################
####_______________________║_______________________║_______________________║________▨_▩__▩_▧________║_______________________####
####_______________________║_______________________║_______________________║________▨___▨__▧________║_______________________####
####_______________________║_______________________║_______________________║________▨_▩__▩_▧________║_______________________####
####_______________________║_______________________║_______________________║________▨__▧___▧________║_______________________####
####_______________________║_______________________║_______________________║________▨_▩__▩_▧________║_______________________####
####_______________________║_______________________║_______________________║________▨___▨__▧________________________________####
####_______________________║_______________________║_______________________║________▨_▩__▩_▧________║_______________________####
####_______________________║_______________________║_______________________║________▨__▧___▧________║_______________________####
####_______________________║_______________________║_______________________║________▨_▩__▩_▧________║_______________________####
####_______________________║_______________________║_______________________║________▨___▨__▧________║_______________________####
####_______________________║_______________________║_______________________║________▨_▩__▩_▧________║_______________________####
####══════════__═══════════╩═══════════__══════════╩═══════════__══════════╝________▨______▧________╠═══════════════════════####
####________________________________________________________________________________________________║_______________________####
####________________________________________________________________________________________________║_______________________####
####________________________________________________________________________________________________║_______________________####
####▨_▨_▨_▨_▨_▨_▨_▨_▨_▨_▨_▨_________________________________________________________________________║_______________________####
####▩___▩___▩___▩___▩___▩___________________________________________________________________________║_______________________####
__________▧_______▧_________________________________________________________________________________________________________####
______▨_______▨_______▨_____________________________________┌──────┐________________________________║_______________________####
####▩___▩___▩___▩___▩___▩_________________________╔═════════╧══════╧═════════╗______________________║_______________________####
####▧_▧_▧_▧_▧_▧_▧_▧_▧_▧_▧_▧_______________________║__________________________║______________________║_______________________####
####______________________________________________║__________________________║______________________║_______________________####
####______________________________________________║__________________________║______________________║_______________________####
####______________________________________________╚══════════════════════════╝______________________╠═══════════════════════####
####═══════════════════════╗________________________________________________________________________║_______________________####
####_______________________║________________________________________________________________________║_______________________####
####_______________________║________________________________________________________________________║_______________________####
####_______________________║__________________└──────────────┘____└──────────────┘__________________║_______________________####
####_______________________║________________________________________________________________________║_______________________####
####_______________________║__________________└──────────────┘____└──────────────┘__________________________________________####
####________________________________________________________________________________________________║_______________________####
####_______________________║__________________└──────────────┘____└──────────────┘__________________║_______________________####
####_______________________║________________________________________________________________________║_______________________####
####_______________________║__________________└──────────────┘____└──────────────┘__________________║_______________________####
####_______________________║________________________________________________________________________║_______________________####
####_______________________║__________________└──────────────┘____└──────────────┘__________________║_______________________####
####_______________________║________________________________________________________________________╠═══════════════════════####
####_______________________║________________________________________________________________________║_______________________####
####_______________________║________________________________________________________________________║_______________________####
####_______________________║________________________________________________________________________║_______________________####
####_______________________║________________________________________________________________________║_______________________####
####_______________________║_________▩_▧_▨_▧_▨_▧_▨_▧_▨_▧_▨_▧_▨_▧▨_▧_▨_▧_▨_▧_▨_▧_▨_▧_▨_▧_▨_▩_________║_______________________####
####_______________________║_________▨____________________________________________________▧_________________________________####
####_______________________║_________▧____________________________________________________▨_________║_______________________####
####_______________________║_________▨____________________________________________________▧_________║_______________________####
####_______________________║_________▧____________________________________________________▨_________║_______________________####
####_______________________║_________▨____________________________________________________▧_________║_______________________####
####_______________________║_________▧____________________________________________________▨_________║_______________________####
##############################################################____##############################################################
##############################################################____##############################################################
"#;

fn add_obsidian_walls(mut map: Vec<Vec<Cells>>, left: bool) -> Vec<Vec<Cells>> {
    let walls = match left {
        true => OBSIDIAN_WALLS_BOTTOM,
        false => OBSIDIAN_WALLS_BOTTOM,
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
                '╝' => Cells::MwUL,
                '╚' => Cells::MwUR,
                '╗' => Cells::MwDL,
                '╔' => Cells::MwDR,
                '╤' => Cells::BsHD,
                '╧' => Cells::BsHU,
                '┐' => Cells::SwDL,
                '┌' => Cells::SwDR,
                '┘' => Cells::SwUL,
                '└' => Cells::SwUR,
                '─' => Cells::SwH,
                '▧' => Cells::Tile1,
                '▨' => Cells::Tile2,
                '▩' => Cells::Tile3,
                '#' => Cells::Wall,
                'Һ' => Cells::ChairRight1,
                'Ⴙ' => Cells::ChairRight2,
                'ж' => Cells::Firewood,
                'ঌ' => Cells::FireSmoke,
                '܀' => Cells::FireDiamond,
                'ஃ' => Cells::FireTri,
                'Ŧ' => Cells::Stand1,
                'Ÿ' => Cells::Stand2,
                'Ť' => Cells::Stand3,
                'ƃ' => Cells::StandBL,
                'ƌ' => Cells::StandDL,
                'Ƃ' => Cells::StandBS,
                'Ƌ' => Cells::StandDS,
                '◍' => Cells::CircleVL,
                '⏣' => Cells::CircleHex,
                '⌬' => Cells::CircleC,
                '⌹' => Cells::Drawers,
                '⌸' => Cells::Shelves,
                '⚱' => Cells::Vase,
                '𜲄' => Cells::LadderV,
                '𜲅' => Cells::LadderH,
                '𜲐' => Cells::TickV,
                '𜲑' => Cells::TickH,
                'ቋ' => Cells::Tech1,
                '🝻' => Cells::Tech2,
                '🜟' => Cells::Tech3,
                'ଏ' => Cells::Tech4,
                'Ҧ' => Cells::Tech5,
                'Ҹ' => Cells::Tech6,
                'Ҵ' => Cells::Tech7,
                'ౝ' => Cells::Tech8,
                '𜰔' => Cells::Tech9,
                '𜰓' => Cells::Tech10,
                '𜰉' => Cells::Tech11,
                '𜰊' => Cells::Tech12,
                '⛀' => Cells::Tech13,
                '⛁' => Cells::Tech14,
                '⛂' => Cells::Tech15,
                '⛃' => Cells::Tech16,
                'Ⴉ' => Cells::Tech17,
                'ቖ' => Cells::Relic1,
                'ቷ' => Cells::OldWall1,
                'ቿ' => Cells::OldWall2,
                'ቨ' => Cells::OldWall3,
                'ቩ' => Cells::OldWall4,
                'ቭ' => Cells::OldWall5,
                '🀫' => Cells::CardTile1,
                '🀘' => Cells::CardTile2,
                '🀆' => Cells::CardTile3,
                '🀙' => Cells::CardTile4,
                _ => todo!(),
            };
        }
    }
    map
}

const OBSIDIAN_ITEM_STALLS: [&str; 1] = [OBSIDIAN_ITEM_STALL_1];
const OBSIDIAN_CLINIC_STALLS: [&str; 1] = [OBSIDIAN_CLINIC_STALL_1];
const OBSIDIAN_HERBALIST_STALLS: [&str; 1] = [OBSIDIAN_HERBALIST_STALL_1];
const OBSIDIAN_CANTEEN_STALLS: [&str; 1] = [OBSIDIAN_CANTEEN_STALL_1];
const OBSIDIAN_FILLERS: [&str; 1] = [OBSIDIAN_FILLER_1];
const OBSIDIAN_OFFICES: [&str; 1] = [OBSIDIAN_OFFICE_1];
const OBSIDIAN_DORMS: [&str; 1] = [OBSIDIAN_DORM];

fn build_obsidian_settle() -> (
    Vec<Vec<Cells>>,
    HashMap<(usize, usize), NPCWrap>,
    HashMap<(usize, usize), Item>,
    HashMap<(usize, usize), Item>,
    HashMap<(usize, usize), EnvInter>,
) {
    let mut rng = rand::thread_rng();
    let cells = vec![vec![Cells::Empty; 128]; 52];
    let face_top = &false;
    // let face_top = [true, false].choose(&mut rng).unwrap_or(&true);
    let mut blocks: Vec<u8> = (1..8).collect();
    blocks.shuffle(&mut rng);

    let (item_map, item_npcs, item_sitems, item_items, item_env_inter) = parse_map(
        OBSIDIAN_ITEM_STALLS
            .choose(&mut rng)
            .unwrap_or(&OBSIDIAN_ITEM_STALLS[0]),
        vec![vec![Cells::Null; 24]; 12],
        Shops::Item,
    );
    let (clinic_map, clinic_npcs, clinic_sitems, clinic_items, clinic_env_inter) = parse_map(
        OBSIDIAN_CLINIC_STALLS
            .choose(&mut rng)
            .unwrap_or(&OBSIDIAN_CLINIC_STALLS[0]),
        vec![vec![Cells::Null; 24]; 12],
        Shops::Null,
    );
    let (herbalist_map, herbalist_npcs, herbalist_sitems, herbalist_items, herbalist_env_inter) =
        parse_map(
            OBSIDIAN_HERBALIST_STALLS
                .choose(&mut rng)
                .unwrap_or(&OBSIDIAN_HERBALIST_STALLS[0]),
            vec![vec![Cells::Null; 24]; 12],
            Shops::Item,
        );
    let (canteen_map, canteen_npcs, canteen_sitems, canteen_items, canteen_env_inter) = parse_map(
        OBSIDIAN_CANTEEN_STALLS
            .choose(&mut rng)
            .unwrap_or(&OBSIDIAN_CANTEEN_STALLS[0]),
        vec![vec![Cells::Null; 24]; 12],
        Shops::Null,
    );
    let (filler1_map, filler1_npcs, filler1_sitems, filler1_items, filler1_env_inter) = parse_map(
        OBSIDIAN_FILLERS
            .choose(&mut rng)
            .unwrap_or(&OBSIDIAN_FILLERS[0]),
        vec![vec![Cells::Null; 24]; 12],
        Shops::Null,
    );
    let (filler2_map, filler2_npcs, filler2_sitems, filler2_items, filler2_env_inter) = parse_map(
        OBSIDIAN_FILLERS
            .choose(&mut rng)
            .unwrap_or(&OBSIDIAN_FILLERS[0]),
        vec![vec![Cells::Null; 24]; 12],
        Shops::Null,
    );
    let (office_map, office_npcs, office_sitems, office_items, office_env_inter) = parse_map(
        OBSIDIAN_OFFICES
            .choose(&mut rng)
            .unwrap_or(&OBSIDIAN_OFFICES[0]),
        vec![vec![Cells::Null; 24]; 12],
        Shops::Null,
    );
    let (dorm_map, dorm_npcs, dorm_sitems, dorm_items, dorm_env_inter) = parse_map(
        OBSIDIAN_DORMS
            .choose(&mut rng)
            .unwrap_or(&OBSIDIAN_DORMS[0]),
        vec![vec![Cells::Null; 24]; 24],
        Shops::Null,
    );

    let (b1_map, b1_npcs, b1_sitems, b1_items, b1_env_inter) = place_obsidion_parts(
        vec![vec![Cells::Null; 128]; 52],
        item_map,
        item_npcs,
        item_sitems,
        item_items,
        item_env_inter,
        blocks[0],
        *face_top,
    );
    let (b2_map, b2_npcs, b2_sitems, b2_items, b2_env_inter) = place_obsidion_parts(
        b1_map.clone(),
        clinic_map,
        clinic_npcs,
        clinic_sitems,
        clinic_items,
        clinic_env_inter,
        blocks[1],
        *face_top,
    );
    let (b3_map, b3_npcs, b3_sitems, b3_items, b3_env_inter) = place_obsidion_parts(
        b2_map.clone(),
        herbalist_map,
        herbalist_npcs,
        herbalist_sitems,
        herbalist_items,
        herbalist_env_inter,
        blocks[2],
        *face_top,
    );
    let (b4_map, b4_npcs, b4_sitems, b4_items, b4_env_inter) = place_obsidion_parts(
        b3_map.clone(),
        canteen_map,
        canteen_npcs,
        canteen_sitems,
        canteen_items,
        canteen_env_inter,
        blocks[3],
        *face_top,
    );
    let (b5_map, b5_npcs, b5_sitems, b5_items, b5_env_inter) = place_obsidion_parts(
        b4_map.clone(),
        filler1_map,
        filler1_npcs,
        filler1_sitems,
        filler1_items,
        filler1_env_inter,
        blocks[4],
        *face_top,
    );
    let (b6_map, b6_npcs, b6_sitems, b6_items, b6_env_inter) = place_obsidion_parts(
        b5_map.clone(),
        filler2_map,
        filler2_npcs,
        filler2_sitems,
        filler2_items,
        filler2_env_inter,
        blocks[5],
        *face_top,
    );
    let (b7_map, b7_npcs, b7_sitems, b7_items, b7_env_inter) = place_obsidion_parts(
        b6_map.clone(),
        office_map,
        office_npcs,
        office_sitems,
        office_items,
        office_env_inter,
        blocks[6],
        *face_top,
    );
    let (b8_map, b8_npcs, b8_sitems, b8_items, b8_env_inter) = place_obsidion_parts(
        b7_map.clone(),
        dorm_map,
        dorm_npcs,
        dorm_sitems,
        dorm_items,
        dorm_env_inter,
        8,
        *face_top,
    );

    let final_map = add_obsidian_walls(b8_map, *face_top);

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
    final_sitems.extend(b1_sitems);
    final_sitems.extend(b2_sitems);
    final_sitems.extend(b3_sitems);
    final_sitems.extend(b4_sitems);
    final_sitems.extend(b5_sitems);
    final_sitems.extend(b6_sitems);
    final_sitems.extend(b7_sitems);
    final_sitems.extend(b8_sitems);
    final_items.extend(b1_items);
    final_items.extend(b2_items);
    final_items.extend(b3_items);
    final_items.extend(b4_items);
    final_items.extend(b5_items);
    final_items.extend(b6_items);
    final_items.extend(b7_items);
    final_items.extend(b8_items);
    final_env_inter.extend(b1_env_inter);
    final_env_inter.extend(b2_env_inter);
    final_env_inter.extend(b3_env_inter);
    final_env_inter.extend(b4_env_inter);
    final_env_inter.extend(b5_env_inter);
    final_env_inter.extend(b6_env_inter);
    final_env_inter.extend(b7_env_inter);
    final_env_inter.extend(b8_env_inter);
    (
        final_map,
        final_npcs,
        final_sitems,
        final_items,
        final_env_inter,
    )
}

fn place_obsidion_parts(
    mut map: Vec<Vec<Cells>>,
    part: Vec<Vec<Cells>>,
    npcs: HashMap<(usize, usize), NPCWrap>,
    sitems: HashMap<(usize, usize), Item>,
    items: HashMap<(usize, usize), Item>,
    env_inter: HashMap<(usize, usize), EnvInter>,
    block: u8,
    top: bool,
) -> (
    Vec<Vec<Cells>>,
    HashMap<(usize, usize), NPCWrap>,
    HashMap<(usize, usize), Item>,
    HashMap<(usize, usize), Item>,
    HashMap<(usize, usize), EnvInter>,
) {
    let (sx, sy) = if top {
        match block {
            1 => (4, 2),
            2 => (4, 14),
            3 => (4, 26),
            4 => (4, 38),
            5 => (52, 38),
            6 => (76, 38),
            7 => (100, 38),
            8 => (100, 2),
            _ => {
                log::info!("small parts error");
                (0, 0)
            }
        }
    } else {
        match block {
            1 => (4, 2),
            2 => (28, 2),
            3 => (52, 2),
            4 => (100, 2),
            5 => (100, 14),
            6 => (100, 26),
            7 => (100, 38),
            8 => (4, 26),
            _ => {
                log::info!("guild parts error");
                (0, 0)
            }
        }
    };

    for j in 0..part.len() {
        for i in 0..part[0].len() {
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

fn get_npc_shops(
    mut npcs: HashMap<(usize, usize), NPCWrap>,
    sitems: HashMap<(usize, usize), Item>,
) -> (HashMap<Shops, Shop>, HashMap<(usize, usize), NPCWrap>) {
    let mut s_npcs = HashMap::new();
    for (k, v) in npcs.clone() {
        match v {
            NPCWrap::ShopNPC(_) => {
                s_npcs.insert(k, v);
                //npcs.remove(&k);
            }
            _ => {}
        }
    }
    let mut shops = HashMap::new();

    for (_, n) in s_npcs {
        //let nb = box_npc(n);
        let mut snpc = match n {
            NPCWrap::ShopNPC(shop_npc) => shop_npc,
            _ => todo!(),
        };
        let shop_name = "shop_name".to_string();
        match snpc.get_shop_type() {
            Shops::Item => shops.insert(
                Shops::Item,
                Shop::new_item_shop(
                    snpc.get_sh_conv()[&shop_name].clone(),
                    NPCWrap::ShopNPC(snpc),
                    sitems.clone(),
                ),
            ),
            Shops::Guild => shops.insert(
                Shops::Guild,
                Shop::new_guild(
                    snpc.get_sh_conv()[&shop_name].clone(),
                    NPCWrap::ShopNPC(snpc),
                    HashMap::new(),
                ),
            ),
            Shops::Church => shops.insert(
                Shops::Church,
                Shop::new_church(
                    snpc.get_sh_conv()[&shop_name].clone(),
                    NPCWrap::ShopNPC(snpc),
                    HashMap::new(),
                ),
            ),
            //_ => Some(Shop::default()),
            _ => todo!(),
        };
    }
    (shops, npcs.clone())
}

#[derive(Clone, Debug, PartialEq)]
pub struct Settlement {
    pub stype: Settle,
    pub sname: String,
    pub pos: (i16, i16),
    pub npcs: HashMap<(usize, usize), NPCWrap>,
    pub items: HashMap<(usize, usize), Item>,
    pub npcs_sent: bool,
    pub items_sent: bool,
    pub shops: HashMap<Shops, Shop>,
    pub env_inters: HashMap<(usize, usize), EnvInter>,
    pub map: Vec<Vec<Cells>>,
    pub found: bool,
}

impl Settlement {
    pub fn new(
        stype: Settle,
        sname: String,
        pos: (i16, i16),
        npcs: HashMap<(usize, usize), NPCWrap>,
        items: HashMap<(usize, usize), Item>,
        shops: HashMap<Shops, Shop>,
        env_inters: HashMap<(usize, usize), EnvInter>,
        map: Vec<Vec<Cells>>,
        found: bool,
    ) -> Self {
        Self {
            stype,
            sname,
            pos,
            npcs,
            items,
            npcs_sent: false,
            items_sent: false,
            shops,
            env_inters,
            map,
            found,
        }
    }

    pub fn demo_settle(pos: (i16, i16), npcs: HashMap<(usize, usize), NPCWrap>) -> Self {
        // let (map, mpcs, sitems, items, env_inters) = build_obsidian_settle();
        // let (map, mpcs, sitems, items, env_inters) = build_guild_settle();
        let (map, mut mpcs, sitems, items, env_inters) = build_small_settle(true);
        let (shops, snpcs) = get_npc_shops(mpcs.clone(), sitems);

        Self {
            stype: Settle::Small,
            sname: "Cave Opening".to_string(),
            pos: pos,
            npcs: snpcs,
            items: items,
            npcs_sent: false,
            items_sent: false,
            shops: shops,
            env_inters,
            map: map,
            found: true,
        }
    }

    pub fn new_small_settle(pos: (i16, i16)) -> Self {
        let data1 = fs::read_to_string("src/locations/settle_names.json");
        //log::info!("{:?}", &data1);
        let names: Vec<String> = match data1 {
            Ok(content) => serde_json::from_str(&content).unwrap(),
            Err(e) => {
                log::info!("{:?}", e);
                Vec::new()
            }
        };
        let mut rng = rand::thread_rng();
        let name_oops = "Jadeitite".to_string();
        let name = names.choose(&mut rng).unwrap_or(&name_oops.clone()).clone();
        let (map, mut npcs, sitems, items, env_inters) = build_small_settle(false);
        let (shops, snpcs) = get_npc_shops(npcs.clone(), sitems);
        Self {
            stype: Settle::Small,
            sname: name,
            pos: pos,
            npcs: npcs,
            items: items,
            npcs_sent: false,
            items_sent: false,
            shops: shops,
            env_inters,
            map: map,
            found: false,
        }
    }

    pub fn new_node_settle(pos: (i16, i16), sname: String) -> Self {
        // let (map, npcs, sitems, items, env_inters) = build_med_settle();
        let (map, npcs, sitems, items, env_inters) = build_small_settle(false);
        let (shops, snpcs) = get_npc_shops(npcs.clone(), sitems);
        Self {
            stype: Settle::Med,
            // stype: Settle::Small,
            sname,
            pos,
            npcs: snpcs,
            items,
            npcs_sent: false,
            items_sent: false,
            shops,
            env_inters,
            map,
            found: false,
        }
    }

    pub fn tog_found(&mut self) {
        if !self.found {
            self.found = !self.found;
        }
    }

    pub fn get_all_shop_items(&mut self) -> Option<HashMap<(usize, usize), Item>> {
        let mut asi = HashMap::new();
        for (_, shop) in &self.shops {
            for ((x, y), i) in shop.get_stock() {
                asi.insert((x, y), i.clone());
            }
        }
        if asi.len() == 0 {
            None
        } else {
            Some(asi.clone())
        }
    }

    pub fn get_shop_from_item_pos(&mut self, pos: (i16, i16)) -> Option<Shop> {
        for (sh, s) in &self.shops {
            for ((x, y), i) in s.get_stock() {
                // if x  && y == (pos.1 - self.pos.1) {
                if (x as i16 + self.pos.0) == pos.0 && (y as i16 + self.pos.1) == pos.1 {
                    return Some(s.clone());
                }
            }
        }
        None
    }

    pub fn get_stats(&mut self) -> (String, String) {
        let mut npc_names = Vec::new();
        for (_, n) in self.npcs.clone() {
            let sname = box_npc(n).get_sname();
            npc_names.push(sname);
        }
        let mut shops = Vec::new();
        for (e, s) in self.shops.clone() {
            let s_string = match e {
                Shops::Item => format!("Item: {}", s.sname),
                Shops::Guild => "Guild".to_string(),
                Shops::Church => "Church".to_string(),
                _ => "".to_string(),
            };
            shops.push(s_string);
        }
        (
            self.sname.clone(),
            format!(
                r#"
{}
---

Shops:
{}

Residents:
{}
        "#,
                self.sname.clone(),
                shops.join("\n"),
                npc_names.join("\n")
            ),
        )
    }

    pub fn update_shop(&mut self, mut shop: Shop) {
        let stype = shop.get_sptype();
        self.shops.insert(stype, shop);
    }

    pub fn add_task_env(&mut self, env: EnvInter) {
        let mut rng = rand::thread_rng();
        let map = self.map.clone();
        loop {
            let x = rng.gen_range(0..map[0].len() - 1);
            let y = rng.gen_range(0..map.len() - 1);
            if map[y][x] == Cells::Empty && !self.env_inters.contains_key(&(x, y)) {
                self.env_inters.insert((x, y), env);
                break;
            }
        }
    }

    pub fn get_pos(&mut self) -> (i16, i16) {
        self.pos.clone()
    }

    pub fn set_pos(&mut self, tpos: (i16, i16)) {
        self.pos = tpos;
    }

    pub fn get_npcs(&mut self) -> HashMap<(usize, usize), NPCWrap> {
        self.npcs.clone()
    }

    pub fn get_npcs_sent(&mut self) -> bool {
        self.npcs_sent.clone()
    }

    pub fn tog_npcs_sent(&mut self) {
        self.npcs_sent = !self.npcs_sent;
    }

    pub fn get_items(&mut self) -> HashMap<(usize, usize), Item> {
        self.items.clone()
    }

    pub fn get_items_sent(&mut self) -> bool {
        self.items_sent.clone()
    }

    pub fn tog_items_sent(&mut self) {
        self.items_sent = !self.items_sent;
    }

    pub fn get_shops(&mut self) -> HashMap<Shops, Shop> {
        self.shops.clone()
    }

    pub fn get_map(&mut self) -> Vec<Vec<Cells>> {
        self.map.clone()
    }

    pub fn get_sname(&mut self) -> String {
        self.sname.clone()
    }

    pub fn get_env_inters(&mut self) -> HashMap<(usize, usize), EnvInter> {
        self.env_inters.clone()
    }
}
