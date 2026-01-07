use crate::assets::{
    get_ascii, get_comm, get_convo, get_npc_name, get_shop_convos, get_shops, Comms, Convos,
};
use crate::enums::{Cells, Door, EnvInter, NPCWrap};
use crate::item::Item;
use crate::npc::{new_comm_npc, new_conv_npc, Convo};
use rand::prelude::SliceRandom;
use rand::Rng;
use std::collections::HashMap;
use std::fs;

pub fn parse_map(
    s_map: &str,
    mut cells: Vec<Vec<Cells>>,
) -> (
    Vec<Vec<Cells>>,
    HashMap<(usize, usize), NPCWrap>,
    HashMap<(usize, usize), Item>,
    HashMap<(usize, usize), EnvInter>,
) {
    // let mut cells: Vec<Vec<Cells>> = Vec::new();
    let mut rng = rand::thread_rng();
    let map_codet = s_map.lines().next().unwrap_or("");
    let map_code: Vec<&str> = map_codet.split("|").collect();
    let npc_types: Vec<&str> = map_code.clone()[0].split(" ").collect();
    let item_types: Vec<&str> = map_code.clone()[2].split(" ").collect();

    let comms = [
        Comms::CaveCity,
        Comms::CaveEngine,
        Comms::CaveGuild,
        Comms::CaveObsidians,
    ];

    let convos = [
        Convos::CaveCity,
        Convos::CaveEngine,
        Convos::CaveGuild,
        Convos::CaveObsidians,
    ];

    let mut ncount = 0;
    let mut icount = 0;
    //let mut cells = vec![vec![Cells::Null; 150]; 50];
    let mut npcs = HashMap::new();
    let mut items = HashMap::new();
    let mut env_inters = HashMap::new();
    for (y, line) in s_map.lines().skip(1).enumerate() {
        for (x, ch) in line.chars().enumerate() {
            let cell = match ch {
                '_' => Cells::Empty,
                '#' => Cells::Transparent,
                ',' => Cells::Grass1,
                '⚶' => Cells::TallGrass,
                '\'' => Cells::Grass2,
                '\"' => Cells::Grass3,
                '·' => Cells::Dirt1,
                '.' => Cells::Dirt2,
                ':' => Cells::Dirt3,
                '*' => Cells::Rock,
                '▒' => Cells::Wall,
                '▓' => Cells::Wall2,
                '█' => Cells::Wall3,
                '░' => Cells::Wall4,
                'ඉ' => Cells::Roots,
                '🬤' => Cells::Broken1,
                '🬗' => Cells::Broken2,
                '🬐' => Cells::Broken3,
                '🬑' => Cells::Broken4,
                '🬮' => Cells::Broken5,
                '🬡' => Cells::Broken6,
                ' ' => Cells::Floor,
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
                '╟' => Cells::BsVR,
                '╢' => Cells::BsVL,
                '╤' => Cells::BsHD,
                '╧' => Cells::BsHU,
                '╭' => Cells::CurUL,
                '╮' => Cells::CurUR,
                '╰' => Cells::CurBL,
                '╯' => Cells::CurBR,
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
                'Ħ' => Cells::DblBracedGate, //-------
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
                // '' => Cells::,
                'Ʌ' => Cells::Tent,
                '🁢' => Cells::Bed,
                '&' => Cells::Bush,
                'ᘉ' => Cells::Bramble1,
                'ᘈ' => Cells::Bramble2,
                'ᘍ' => Cells::Bramble3,
                'ᘊ' => Cells::Bramble4,
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
                _ => Cells::Empty,
            };
            cells[y][x] = cell;
            if ch == '@' {
                let def_name = "Kevthony".to_string();
                match npc_types[ncount] {
                    "CommNPC" => {
                        let rnd_comms = {
                            let mut tvec = Vec::new();
                            for comm in &comms {
                                tvec.push(get_comm(*comm));
                            }
                            tvec
                        };
                        let name = get_npc_name();
                        let t_comm = new_comm_npc(name.clone(), x, y, rnd_comms.clone());
                        npcs.insert((x, y), NPCWrap::CommNPC(t_comm.clone()));
                    }
                    "ConvNPC" => {
                        let name = get_npc_name();
                        let conv = get_convo(*convos.choose(&mut rng).unwrap());
                        let t_comm = new_conv_npc(name.clone(), x, y, conv.clone());
                        npcs.insert((x, y), NPCWrap::ConvNPC(t_comm.clone()));
                    }
                    _ => todo!(),
                }
                ncount += 1;
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
    (cells, npcs, items, env_inters)
}

pub fn tile_to_chars(tile: &str) -> Vec<Vec<char>> {
    tile.trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

pub fn ch_to_enum(ch: char) -> Cells {
    match ch {
        '_' => Cells::Empty,
        '#' => Cells::Transparent,
        ',' => Cells::Grass1,
        '⚶' => Cells::TallGrass,
        '\'' => Cells::Grass2,
        '\"' => Cells::Grass3,
        '·' => Cells::Dirt1,
        '.' => Cells::Dirt2,
        ':' => Cells::Dirt3,
        '*' => Cells::Rock,
        '▒' => Cells::Wall,
        '▓' => Cells::Wall2,
        '█' => Cells::Wall3,
        '░' => Cells::Wall4,
        'ඉ' => Cells::Roots,
        '🬤' => Cells::Broken1,
        '🬗' => Cells::Broken2,
        '🬐' => Cells::Broken3,
        '🬑' => Cells::Broken4,
        '🬮' => Cells::Broken5,
        '🬡' => Cells::Broken6,
        ' ' => Cells::Floor,
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
        '╟' => Cells::BsVR,
        '╢' => Cells::BsVL,
        '╤' => Cells::BsHD,
        '╧' => Cells::BsHU,
        '╭' => Cells::CurUL,
        '╮' => Cells::CurUR,
        '╰' => Cells::CurBL,
        '╯' => Cells::CurBR,
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
        'Ħ' => Cells::DblBracedGate, //-------
        'ỻ' => Cells::BracedGate,
        'Π' => Cells::Arch,
        'ʭ' => Cells::Bricks,
        'ʬ' => Cells::Crops,
        'ѧ' => Cells::SmallCampfire,
        'Ѧ' => Cells::Campfire,
        'π' => Cells::Table,
        'ж' => Cells::Firewood,
        'ঌ' => Cells::FireSmoke,
        'Ʌ' => Cells::Tent,
        '🁢' => Cells::Bed,
        '&' => Cells::Bush,
        'ᘉ' => Cells::Bramble1,
        'ᘈ' => Cells::Bramble2,
        'ᘍ' => Cells::Bramble3,
        'ᘊ' => Cells::Bramble4,
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
        _ => Cells::Empty,
    }
}


pub fn item_parse(itm_str: &str, x: usize, y: usize) -> Item {
    match itm_str {
        "Book" => Item::new_book(
                x,
                y,
                "Book Titleet libero dictum, tempus mi vehicula, facilisis elit.".to_string(),
                "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Donec eget dolor sapien. Sed at odio congue, placerat purus vitae, malesuada erat. Aliquam rutrum ipsum non enim ornare elementum. Nulla ac magna magna. Etiam mollis, enim id ullamcorper luctus, quam mi accumsan augue, in mollis orci lectus at nisl. Nulla id massa velit. Nam pulvinar, felis ut sodales dictum, nunc risus ornare augue, sit amet rutrum odio sem iaculis justo. Cras nunc lorem, iaculis ac ante ac, tristique egestas nisl. Phasellus et libero dictum, tempus mi vehicula, facilisis elit.

    Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Nunc vitae sollicitudin risus. Donec iaculis, augue eget maximus rutrum, lorem risus malesuada leo, eu rutrum ex diam et purus. Ut luctus volutpat lacus ut sollicitudin. Maecenas vel rutrum sapien, in aliquam enim. Aenean justo leo, ullamcorper nec sodales sit amet, congue sed tellus. Curabitur lacinia enim sit amet mi semper sodales. Cras placerat, odio in dapibus pellentesque, risus eros suscipit metus, id finibus nisl mauris sed nisi. Nunc pharetra ipsum interdum fringilla tempor. Donec elementum magna ex, eget commodo purus sodales vel. Mauris mollis eros nec egestas tempor. Nunc convallis accumsan porta. Nulla facilisis quam ut lectus bibendum sagittis. Suspendisse congue, lacus id tincidunt fermentum, ex arcu efficitur lectus, faucibus mollis nulla orci non lectus. Aliquam auctor sapien a finibus vulputate.

    Aliquam erat volutpat. Curabitur lorem metus, hendrerit non leo id, laoreet ornare tortor. Pellentesque tincidunt nibh arcu, id fringilla odio vehicula vel. Praesent eleifend luctus purus a tincidunt. Sed eget leo id justo ultrices dictum. Cras vitae dictum turpis, nec efficitur est. Donec id nisl vitae lorem suscipit interdum. Sed at turpis nec sapien laoreet eleifend at quis ipsum. Sed tristique erat a consectetur porta.

    In a pharetra felis. Ut metus est, rhoncus ut vehicula quis, consequat in nibh. Maecenas cursus neque id dui ornare, a pellentesque massa placerat. Maecenas consequat ante vel enim tempor, sed euismod sapien eleifend. Nulla pulvinar odio eu gravida lobortis. Nullam fringilla nisi lectus, in sagittis quam hendrerit at. Pellentesque in nunc sodales, porttitor dolor id, dictum ante. Curabitur posuere orci ac metus dapibus, vel sodales erat ultricies.

    Praesent eu ligula porttitor, pellentesque sem eget, vulputate urna. Pellentesque dignissim volutpat auctor. Vestibulum dictum risus vitae est porta volutpat. Vivamus ac euismod neque, vel sagittis lorem. Maecenas scelerisque at neque ut aliquam. Nunc vel nisl leo. Phasellus non pharetra odio. Nam luctus risus eros, quis vestibulum lorem ullamcorper ut. Maecenas efficitur dolor eget odio elementum, nec imperdiet massa placerat. Nam nulla urna, varius eget pharetra nec, accumsan in nisi. Donec aliquam nisi ut diam dignissim facilisis. Donec finibus mi condimentum vestibulum venenatis. Fusce malesuada at augue nec luctus. Duis id leo diam. Praesent eu iaculis enim.".to_string(),
            ),
        "HealthPotion" => Item::new_health_potion(x, y),
        "AgilityPotion" => Item::new_agility_potion(x, y),
        "StrengthPotion" => Item::new_strength_potion(x, y),
        "Salve" => Item::new_salve(x, y),
        "Dowel" => Item::new_dowel(x, y),
        "Apple" => Item::new_apple(x, y),

        "BronzeClaymore" => Item::new_bronze_claymore(x, y),
        "BronzeShortsword" => Item::new_bronze_shortsword(x, y),
        "BronzeLongsword" => Item::new_bronze_longsword(x, y),
        "BronzeLightAxe" => Item::new_bronze_light_axe(x, y),
        "BronzeHeavyAxe" => Item::new_bronze_heavy_axe(x, y),
        "BronzeWarAxe" => Item::new_bronze_war_axe(x, y),
        "BronzePickHammer" => Item::new_bronze_pick_hammer(x, y),
        "BronzePickAxe" => Item::new_bronze_pick_axe(x, y),

        "IronClaymore" => Item::new_iron_claymore(x, y),
        "IronShortsword" => Item::new_iron_shortsword(x, y),
        "IronLongsword" => Item::new_iron_longsword(x, y),
        "IronLightAxe" => Item::new_iron_light_axe(x, y),
        "IronHeavyAxe" => Item::new_iron_heavy_axe(x, y),
        // "IronWarAxe" => Item::new_iron_war_axe(x, y),
        // "IronPickHammer" => Item::new_iron_pick_hammer(x, y),
        // "IronPickAxe" => Item::new_iron_pick_axe(x, y),

        "SteelClaymore" => Item::new_steel_claymore(x, y),
        "SteelShortsword" => Item::new_steel_shortsword(x, y),
        "SteelLongsword" => Item::new_steel_longsword(x, y),
        "SteelLightAxe" => Item::new_steel_light_axe(x, y),
        "SteelHeavyAxe" => Item::new_steel_heavy_axe(x, y),
        // "SteelWarAxe" => Item::new_steel_war_axe(x, y),
        // "SteelPickHammer" => Item::new_steel_pick_hammer(x, y),
        // "SteelPickAxe" => Item::new_steel_pick_axe(x, y),

        "WoodStaff" => Item::new_wood_staff(x, y),
        "BludgeonStaff" => Item::new_bludgeon_staff(x, y),
        "GemStaff" => Item::new_gem_staff(x, y),
        "LightArmour" => Item::new_light_armour(x, y),
        "MediumArmour" => Item::new_medium_armour(x, y),
        "HeavyArmour" => Item::new_heavy_armour(x, y),
        "ShieldingPendant" => Item::new_shielding_pendant(x, y),
        "StrengthPendant" => Item::new_strength_pendant(x, y),
        "AgilityPendant" => Item::new_agility_pendant(x, y),
        "SmallWoodShield" => Item::new_small_wood_shield(x, y),
        "LargeWoodShield" => Item::new_large_wood_shield(x, y),
        "IronShield" => Item::new_iron_shield(x, y),
        "SteelShield" => Item::new_steel_shield(x, y),
        item if item.contains("Gold") => {
            let gsplit: Vec<&str> = item.split(":").collect();
            Item::new_gold(x, y, gsplit[1].parse().unwrap())
        },
        _ => Item::new_gold(x, y, 0)
    
    }
}
