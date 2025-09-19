//settlement rs
use crate::assets::{
    get_ascii, get_comm, get_convo, get_npc_name, get_shop_convos, get_shops, Comms, Convos,
};
use crate::dialogue::{load_comms, load_convos, CommDialogue, ConvoDialogue};
use crate::enums::{Cells, Door, EnvInter, NPCWrap, Settle, TaskEnv};
use crate::enums::{ShopItem, Shops};
use crate::item::Item;
use crate::npc::{new_comm_npc, new_conv_npc, new_shop_npc, Convo, ShopConvos, ShopData, ShopNPC};
use crate::npc_utils::box_npc;
use crate::settlement::guild_settle::build_guild_settle;
use crate::settlement::med_settle::build_med_settle;
use crate::settlement::obsidian_settle::build_obsidian_settle;
use crate::settlement::sm_settle::build_small_settle;
use crate::shop::Shop;

use rand::prelude::SliceRandom;
use rand::Rng;

//use serde::{Deserialize, Serialize};
//use serde_json::Result;
//use serde_json::Value;
use std::fs;

use std::collections::HashMap;

mod guild_settle;
mod med_settle;
mod obsidian_settle;
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
    HashMap<(usize, usize), ShopItem>,
    HashMap<(usize, usize), Item>,
    HashMap<(usize, usize), EnvInter>,
    HashMap<(usize, usize), ShopNPC>,
) {
    // let mut cells: Vec<Vec<Cells>> = Vec::new();
    let mut rng = rand::thread_rng();
    let map_codet = s_map.lines().next().unwrap_or("");
    let map_code: Vec<&str> = map_codet.split("|").collect();
    let npc_types: Vec<&str> = map_code.clone()[0].split(" ").collect();
    let sitem_types: Vec<&str> = map_code.clone()[1].split(" ").collect();
    let item_types: Vec<&str> = map_code.clone()[2].split(" ").collect();

    // let data1 = fs::read_to_string("src/npcs/npc_names.json");
    // //log::info!("{:?}", &data1);
    // let names: Vec<String> = match data1 {
    //     Ok(content) => serde_json::from_str(&content).unwrap(),
    //     Err(e) => {
    //         log::info!("{:?}", e);
    //         Vec::new()
    //     }
    // };

    // let names = get_npc_name();

    let comms = match shop_type {
        Shops::Item => [
            Comms::CaveCity,
            Comms::CaveEngine,
            Comms::CaveGuild,
            Comms::CaveObsidians,
        ],
        Shops::Guild => [
            Comms::GuildCity,
            Comms::GuildEngine,
            Comms::GuildGuild,
            Comms::GuildObsidians,
        ],
        Shops::Church => [
            Comms::ObsidianCity,
            Comms::ObsidianEngine,
            Comms::ObsidianGuild,
            Comms::ObsidianObsidians,
        ],
        _ => [
            Comms::CaveCity,
            Comms::CaveEngine,
            Comms::CaveGuild,
            Comms::CaveObsidians,
        ],
    };

    let convos = match shop_type {
        Shops::Item => [
            Convos::CaveCity,
            Convos::CaveEngine,
            Convos::CaveGuild,
            Convos::CaveObsidians,
        ],
        Shops::Guild => [
            Convos::GuildCity,
            Convos::GuildEngine,
            Convos::GuildGuild,
            Convos::GuildObsidians,
        ],
        Shops::Church => [
            Convos::ObsidianCity,
            Convos::ObsidianEngine,
            Convos::ObsidianGuild,
            Convos::ObsidianObsidians,
        ],
        _ => [
            Convos::CaveCity,
            Convos::CaveEngine,
            Convos::CaveGuild,
            Convos::CaveObsidians,
        ],
    };

    // let comms = match shop_type {
    //     Shops::Item => load_comms(&"cave".to_string()),
    //     Shops::Guild => load_comms(&"guild".to_string()),
    //     Shops::Church => load_comms(&"cult".to_string()),
    //     _ => load_comms(&"cave".to_string()),
    // };

    // let convos = match shop_type {
    //     Shops::Item => load_convos(&"cave".to_string()),
    //     Shops::Guild => load_convos(&"guild".to_string()),
    //     Shops::Church => load_convos(&"cult".to_string()),
    //     _ => load_convos(&"cave".to_string()),
    // };

    // let data4 = fs::read_to_string("src/npcs/npc_shops.json");
    // let shops: ShopData = match data4 {
    //     Ok(content) => serde_json::from_str(&content).unwrap(),
    //     Err(e) => {
    //         log::info!("{:?}", e);
    //         ShopData {
    //             shops: Vec::new(),
    //             guilds: Vec::new(),
    //             churches: Vec::new(),
    //         }
    //     }
    // };

    let shops = get_shops();
    let shop_convos = get_shop_convos();

    // let data5 = fs::read_to_string("src/npcs/npc_shop_convos.json");
    // //log::info!("{:?}", &data5);
    // let shop_convos: ShopConvos = match data5 {
    //     Ok(content) => serde_json::from_str(&content).unwrap(),
    //     Err(e) => {
    //         log::info!("{:?}", e);
    //         ShopConvos {
    //             shops: Vec::new(),
    //             guilds: Vec::new(),
    //             churches: Vec::new(),
    //         }
    //     }
    // };

    let mut ncount = 0;
    let mut icount = 0;
    let mut sicount = 0;
    //let mut cells = vec![vec![Cells::Null; 150]; 50];
    let mut npcs = HashMap::new();
    let mut items = HashMap::new();
    let mut sitems = HashMap::new();
    let mut env_inters = HashMap::new();
    let mut shop_npcs = HashMap::new();
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
                // '⛀' => Cells::Tech13,
                // '⛁' => Cells::Tech14,
                // '⛂' => Cells::Tech15,
                // '⛃' => Cells::Tech16,
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
                        // let rnd_comms = {
                        //     let mut tvec = Vec::new();
                        //     for _ in 0..4 {
                        //         tvec.push(match rng.gen_range(0..3) {
                        //             0 => comms
                        //                 .city
                        //                 .choose(&mut rng)
                        //                 .unwrap_or(&comms.city[0])
                        //                 .clone(),
                        //             1 => comms
                        //                 .engine
                        //                 .choose(&mut rng)
                        //                 .unwrap_or(&comms.engine[0])
                        //                 .clone(),
                        //             2 => comms
                        //                 .guild
                        //                 .choose(&mut rng)
                        //                 .unwrap_or(&comms.guild[0])
                        //                 .clone(),
                        //             3 => comms
                        //                 .cult
                        //                 .choose(&mut rng)
                        //                 .unwrap_or(&comms.cult[0])
                        //                 .clone(),
                        //             _ => todo!(),
                        //         });
                        //         // let tidx = rng.gen_range(0..comms.len());
                        //         // tvec.push(comms[tidx].clone());
                        //     }
                        //     tvec
                        // };
                        let rnd_comms = {
                            let mut tvec = Vec::new();
                            for comm in &comms {
                                tvec.push(get_comm(*comm));
                            }
                            tvec
                        };
                        // let name = names.choose(&mut rng).unwrap_or(&def_name.clone()).clone();
                        let name = get_npc_name();
                        let t_comm = new_comm_npc(name.clone(), x, y, rnd_comms.clone());
                        npcs.insert((x, y), NPCWrap::CommNPC(t_comm.clone()));
                    }
                    "ConvNPC" => {
                        let name = get_npc_name();
                        // let name = names.choose(&mut rng).unwrap_or(&def_name.clone()).clone();
                        //let comms = vec!["Welcome to the caves!!".to_string(), "Theres a tonne of folk down here, lerger cities as you go into the cave.".to_string()];
                        // let conv: Convo = match rng.gen_range(0..3) {
                        //     0 => convos
                        //         .city
                        //         .choose(&mut rng)
                        //         .unwrap_or(&convos.city[0])
                        //         .clone(),
                        //     1 => convos
                        //         .engine
                        //         .choose(&mut rng)
                        //         .unwrap_or(&convos.engine[0])
                        //         .clone(),
                        //     2 => convos
                        //         .guild
                        //         .choose(&mut rng)
                        //         .unwrap_or(&convos.guild[0])
                        //         .clone(),
                        //     3 => convos
                        //         .cult
                        //         .choose(&mut rng)
                        //         .unwrap_or(&convos.cult[0])
                        //         .clone(),
                        //     _ => todo!(),
                        // };
                        let conv = get_convo(*convos.choose(&mut rng).unwrap());

                        let t_comm = new_conv_npc(name.clone(), x, y, conv.clone());
                        npcs.insert((x, y), NPCWrap::ConvNPC(t_comm.clone()));
                    }
                    "ShopNPC" => {
                        let name = get_npc_name();
                        // let name = names.choose(&mut rng).unwrap_or(&def_name.clone()).clone();
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
                            Shops::Clinic => shops
                                .shops
                                .choose(&mut rng)
                                .unwrap_or(&shops.shops[0].clone())
                                .clone(),
                            Shops::Herbalist => shops
                                .shops
                                .choose(&mut rng)
                                .unwrap_or(&shops.shops[0].clone())
                                .clone(),
                            Shops::Weapon => shops
                                .shops
                                .choose(&mut rng)
                                .unwrap_or(&shops.shops[0].clone())
                                .clone(),
                            Shops::Armor => shops
                                .shops
                                .choose(&mut rng)
                                .unwrap_or(&shops.shops[0].clone())
                                .clone(),
                            Shops::Consignment => shops
                                .shops
                                .choose(&mut rng)
                                .unwrap_or(&shops.shops[0].clone())
                                .clone(),
                            _ => shops
                                .shops
                                .choose(&mut rng)
                                .unwrap_or(&shops.shops[0].clone())
                                .clone(),
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
                            Shops::Clinic => shop_convos
                                .shops
                                .choose(&mut rng)
                                .unwrap_or(&shop_convos.shops[0].clone())
                                .clone(),
                            Shops::Herbalist => shop_convos
                                .shops
                                .choose(&mut rng)
                                .unwrap_or(&shop_convos.shops[0].clone())
                                .clone(),
                            Shops::Weapon => shop_convos
                                .shops
                                .choose(&mut rng)
                                .unwrap_or(&shop_convos.shops[0].clone())
                                .clone(),
                            Shops::Armor => shop_convos
                                .shops
                                .choose(&mut rng)
                                .unwrap_or(&shop_convos.shops[0].clone())
                                .clone(),
                            Shops::Consignment => shop_convos
                                .shops
                                .choose(&mut rng)
                                .unwrap_or(&shop_convos.shops[0].clone())
                                .clone(),
                            _ => shop_convos
                                .shops
                                .choose(&mut rng)
                                .unwrap_or(&shop_convos.shops[0].clone())
                                .clone(),
                        };
                        shop_npcs.insert(
                            (x, y),
                            new_shop_npc(name.clone(), s_conv.clone(), convo.clone(), shop_type),
                        );
                        env_inters.insert((x, y), EnvInter::ShopNPC(shop_type));
                    }
                    _ => todo!(),
                }
                ncount += 1;
            }
            if ch == 'o' {
                let sitm = match sitem_types[sicount] {
                    "HealthPotion" => Item::new_health_potion(x, y),
                    "Salve" => Item::new_salve(x, y),
                    "Dowel" => Item::new_dowel(x, y),
                    "SmallWoodShield" => Item::new_small_wood_shield(x, y),
                    "Apple" => Item::new_apple(x, y),
                    "BronzeClaymore" => Item::new_bronze_claymore(x, y),
                    "BronzeShortsword" => Item::new_bronze_shortsword(x, y),
                    "BronzeLongsword" => Item::new_bronze_longsword(x, y),
                    "BronzeLightAxe" => Item::new_bronze_light_axe(x, y),
                    "BronzeHeavyAxe" => Item::new_bronze_heavy_axe(x, y),
                    "BronzeWarAxe" => Item::new_bronze_war_axe(x, y),
                    "BronzePickHammer" => Item::new_bronze_pick_hammer(x, y),
                    "WoodStaff" => Item::new_wood_staff(x, y),
                    "LightArmour" => Item::new_light_armour(x, y),
                    "ShieldingPendant" => Item::new_shielding_pendant(x, y),
                    "StrengthPendant" => Item::new_strength_pendant(x, y),
                    "AgilityPendant" => Item::new_agility_pendant(x, y),
                    _ => Item::new_agility_pendant(x, y),
                };
                sitems.insert(
                    (x, y),
                    match shop_type {
                        Shops::Item => ShopItem::Item(sitm),
                        Shops::Herbalist => ShopItem::Herbalist(sitm),
                        Shops::Weapon => ShopItem::Weapon(sitm),
                        Shops::Armor => ShopItem::Armor(sitm),
                        Shops::Consignment => ShopItem::Consignment(sitm),
                        _ => ShopItem::Null,
                    },
                );
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
                    item if item.contains("Gold") => {
                        let gsplit: Vec<&str> = item.split(":").collect();
                        items.insert((x, y), Item::new_gold(x, y, gsplit[1].parse().unwrap()));
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
            if ch == 'X' {
                env_inters.insert((x, y), EnvInter::TaskEnv(crate::enums::TaskEnv::Null));
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
    (cells, npcs, sitems, items, env_inters, shop_npcs)
}

fn get_settle_shops(
    npcs: HashMap<(usize, usize), ShopNPC>,
    sitems: HashMap<(usize, usize), ShopItem>,
) -> HashMap<Shops, Shop> {
    let mut shops = HashMap::new();
    for (_, n) in npcs {
        let shop_name = "shop_name".to_string();
        match n.shop_type {
            Shops::Item => shops.insert(
                Shops::Item,
                Shop::new_item_shop(
                    n.sh_conv[&shop_name].clone(),
                    sitems
                        .clone()
                        .into_iter()
                        .filter(|(_k, v)| match *v {
                            ShopItem::Item(_) => true,
                            _ => false,
                        })
                        .collect(),
                    n,
                ),
            ),
            Shops::Herbalist => shops.insert(
                Shops::Herbalist,
                Shop::new_item_shop(
                    n.sh_conv[&shop_name].clone(),
                    sitems
                        .clone()
                        .into_iter()
                        .filter(|(_k, v)| match *v {
                            ShopItem::Herbalist(_) => true,
                            _ => false,
                        })
                        .collect(),
                    n,
                ),
            ),
            Shops::Guild => shops.insert(
                Shops::Guild,
                Shop::new_guild(n.sh_conv[&shop_name].clone(), HashMap::new(), n),
            ),
            Shops::Church => shops.insert(
                Shops::Church,
                Shop::new_church(n.sh_conv[&shop_name].clone(), HashMap::new(), n),
            ),
            Shops::Weapon => shops.insert(
                Shops::Weapon,
                Shop::new_item_shop(
                    n.sh_conv[&shop_name].clone(),
                    sitems
                        .clone()
                        .into_iter()
                        .filter(|(_k, v)| match *v {
                            ShopItem::Item(_) => true,
                            _ => false,
                        })
                        .collect(),
                    n,
                ),
            ),
            Shops::Armor => shops.insert(
                Shops::Item,
                Shop::new_item_shop(
                    n.sh_conv[&shop_name].clone(),
                    sitems
                        .clone()
                        .into_iter()
                        .filter(|(_k, v)| match *v {
                            ShopItem::Item(_) => true,
                            _ => false,
                        })
                        .collect(),
                    n,
                ),
            ),
            //_ => Some(Shop::default()),
            _ => todo!(),
        };
    }
    shops
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
        let (map, mpcs, sitems, items, env_inters, shop_npcs) = build_small_settle(true);
        let shops = get_settle_shops(shop_npcs, sitems);

        Self {
            stype: Settle::Small,
            sname: "Cave Opening".to_string(),
            pos,
            npcs: mpcs,
            items,
            npcs_sent: false,
            items_sent: false,
            shops,
            env_inters,
            map,
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
        let (map, npcs, sitems, items, env_inters, shop_npcs) = build_small_settle(false);
        let shops = get_settle_shops(shop_npcs, sitems);
        Self {
            stype: Settle::Small,
            sname: name,
            pos,
            npcs,
            items,
            npcs_sent: false,
            items_sent: false,
            shops,
            env_inters,
            map,
            found: false,
        }
    }

    pub fn new_node_small_settle(pos: (i16, i16), sname: String) -> Self {
        // let (map, npcs, sitems, items, env_inters) = build_med_settle();
        let (map, npcs, sitems, items, env_inters, shop_npc) = build_small_settle(false);
        let shops = get_settle_shops(shop_npc, sitems);
        Self {
            stype: Settle::Small,
            sname,
            pos,
            npcs,
            items,
            npcs_sent: false,
            items_sent: false,
            shops,
            env_inters,
            map,
            found: false,
        }
    }

    pub fn new_node_med_settle(pos: (i16, i16), sname: String) -> Self {
        let (map, npcs, sitems, items, env_inters, shop_npcs) = build_med_settle();
        let shops = get_settle_shops(shop_npcs, sitems);
        Self {
            stype: Settle::Med,
            sname,
            pos,
            npcs,
            items,
            npcs_sent: false,
            items_sent: false,
            shops,
            env_inters,
            map,
            found: false,
        }
    }

    pub fn new_node_guild_settle(pos: (i16, i16), sname: String) -> Self {
        let (map, npcs, sitems, items, env_inters, shop_npcs) = build_guild_settle();
        let shops = get_settle_shops(shop_npcs, sitems);
        Self {
            stype: Settle::Guild,
            sname,
            pos,
            npcs,
            items,
            npcs_sent: false,
            items_sent: false,
            shops,
            env_inters,
            map,
            found: false,
        }
    }

    pub fn new_node_obsidian_settle(pos: (i16, i16), sname: String) -> Self {
        let (map, npcs, sitems, items, env_inters, shop_npcs) = build_obsidian_settle();
        let shops = get_settle_shops(shop_npcs, sitems);
        Self {
            stype: Settle::Obsidian,
            sname,
            pos,
            npcs,
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

    pub fn get_all_shop_items(&mut self) -> Option<HashMap<(usize, usize), ShopItem>> {
        let mut asi = HashMap::new();
        for (_, shop) in &self.shops {
            shop.stock.clone().into_iter().for_each(|((x, y), i)| {
                asi.insert((x, y), i.clone());
            });
        }
        if asi.len() == 0 {
            None
        } else {
            Some(asi.clone())
        }
    }

    pub fn get_shop_from_item_pos(&mut self, pos: (i16, i16)) -> Option<Shop> {
        for (_, s) in &self.shops {
            for ((x, y), _) in &s.stock {
                if (*x as i16 + self.pos.0) == pos.0 && (*y as i16 + self.pos.1) == pos.1 {
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

    pub fn add_task_env(&mut self, env: EnvInter) -> (usize, usize) {
        // let envs = self.env_inters.clone();
        let pos = {
            let tenvs: HashMap<(usize, usize), EnvInter> = self
                .env_inters
                .clone()
                .into_iter()
                .filter(|&(p, e)| e == EnvInter::TaskEnv(TaskEnv::Null))
                .collect();
            tenvs.into_keys().collect::<Vec<(usize, usize)>>()[0]
        };
        self.env_inters.insert(pos, env);
        pos
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
