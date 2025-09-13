use ratatui::text::ToLine;
use rust_embed::Embed;

#[derive(Embed)]
#[folder = "assets/"]
#[prefix = "prefix/"]
struct Asset;

//dialogue

use rand::seq::SliceRandom;
use rand::Rng;

use crate::enums::{Enemies, EnvInter, Items, Plants};
use crate::npc::{Convo, ShopConvos, ShopData};
use std::collections::HashMap;
use std::fs;

#[derive(Clone)]
pub struct CommDialogue {
    pub city: Vec<String>,
    pub engine: Vec<String>,
    pub guild: Vec<String>,
    pub cult: Vec<String>,
}

#[derive(Clone)]
pub struct ConvoDialogue {
    pub city: Vec<Convo>,
    pub engine: Vec<Convo>,
    pub guild: Vec<Convo>,
    pub cult: Vec<Convo>,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum AssetType {
    Comms(Comms),
    Convos(Convos),
    Ascii(Ascii),
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Comms {
    CaveCity,
    CaveEngine,
    CaveGuild,
    CaveObsidians,
    GuildCity,
    GuildEngine,
    GuildGuild,
    GuildObsidians,
    ObsidianCity,
    ObsidianEngine,
    ObsidianGuild,
    ObsidianObsidians,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Convos {
    CaveCity,
    CaveEngine,
    CaveGuild,
    CaveObsidians,
    GuildCity,
    GuildEngine,
    GuildGuild,
    GuildObsidians,
    ObsidianCity,
    ObsidianEngine,
    ObsidianGuild,
    ObsidianObsidians,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Ascii {
    Npcs(Npcs),
    Enemies(Enemies),
    Items(Items),
    EnvInter(EnvInter),
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Npcs {
    Settler,
    GuildMember,
    GuildHead,
    ObsidianMember,
    ObsidianSteward,
    ShopKeeper,
    Herbalist,
    Clinic,
    WeaponSmith,
    Armorer,
    Terminal,
}

pub fn get_npc_name() -> String {
    let mut rng = rand::thread_rng();
    let name_asset = Asset::get("prefix/npcs/npc_names.json").unwrap();
    let names = std::str::from_utf8(name_asset.data.as_ref());
    let npc_names: Vec<String> = match names {
        Ok(content) => serde_json::from_str(content).unwrap(),
        Err(e) => {
            log::info!("{:?}", e);
            Vec::new()
        }
    };
    npc_names.choose(&mut rng).unwrap().clone()
}

pub fn get_comms(comm_type: Comms) -> Vec<String> {
    let comm_asset = match comm_type {
        Comms::CaveCity => Asset::get("prefix/npcs/cave/comms_city.json").unwrap(),
        Comms::CaveEngine => Asset::get("prefix/npcs/cave/comms_engine.json").unwrap(),
        Comms::CaveGuild => Asset::get("prefix/npcs/cave/comms_guild.json").unwrap(),
        Comms::CaveObsidians => Asset::get("prefix/npcs/cave/comms_cult.json").unwrap(),
        Comms::GuildCity => Asset::get("prefix/npcs/guild/comms_city.json").unwrap(),
        Comms::GuildEngine => Asset::get("prefix/npcs/guild/comms_engine.json").unwrap(),
        Comms::GuildGuild => Asset::get("prefix/npcs/guild/comms_guild.json").unwrap(),
        Comms::GuildObsidians => Asset::get("prefix/npcs/guild/comms_cult.json").unwrap(),
        Comms::ObsidianCity => Asset::get("prefix/npcs/cult/comms_city.json").unwrap(),
        Comms::ObsidianEngine => Asset::get("prefix/npcs/cult/comms_engine.json").unwrap(),
        Comms::ObsidianGuild => Asset::get("prefix/npcs/cult/comms_guild.json").unwrap(),
        Comms::ObsidianObsidians => Asset::get("prefix/npcs/cult/comms_cult.json").unwrap(),
    };
    let comms_str = std::str::from_utf8(comm_asset.data.as_ref());
    let comms: Vec<String> = match comms_str {
        Ok(content) => serde_json::from_str(content).unwrap(),
        Err(e) => {
            log::info!("{:?}", e);
            Vec::new()
        }
    };
    comms
}

pub fn get_convos(convo_type: Convos) -> Convo {
    let convo_asset = match convo_type {
        Convos::CaveCity => Asset::get("prefix/npcs/cave/convos_city.json").unwrap(),
        Convos::CaveEngine => Asset::get("prefix/npcs/cave/convos_engine.json").unwrap(),
        Convos::CaveGuild => Asset::get("prefix/npcs/cave/convos_guild.json").unwrap(),
        Convos::CaveObsidians => Asset::get("prefix/npcs/cave/convos_cult.json").unwrap(),
        Convos::GuildCity => Asset::get("prefix/npcs/guild/convos_city.json").unwrap(),
        Convos::GuildEngine => Asset::get("prefix/npcs/guild/convos_engine.json").unwrap(),
        Convos::GuildGuild => Asset::get("prefix/npcs/guild/convos_guild.json").unwrap(),
        Convos::GuildObsidians => Asset::get("prefix/npcs/guild/convos_cult.json").unwrap(),
        Convos::ObsidianCity => Asset::get("prefix/npcs/cult/convos_city.json").unwrap(),
        Convos::ObsidianEngine => Asset::get("prefix/npcs/cult/convos_engine.json").unwrap(),
        Convos::ObsidianGuild => Asset::get("prefix/npcs/cult/convos_guild.json").unwrap(),
        Convos::ObsidianObsidians => Asset::get("prefix/npcs/cult/convos_cult.json").unwrap(),
    };
    let convo_str = std::str::from_utf8(convo_asset.data.as_ref());
    let convos: Vec<Convo> = match convo_str {
        Ok(content) => serde_json::from_str(content).unwrap(),
        Err(e) => {
            log::info!("{:?}", e);
            Vec::new()
        }
    };
    let mut rng = rand::thread_rng();
    convos.choose(&mut rng).unwrap().clone()
}

pub fn get_shops() -> ShopData {
    let shop_asset = Asset::get("prefix/npcs/npc_shops.json").unwrap();
    let shop_str = std::str::from_utf8(shop_asset.data.as_ref());
    let shops: ShopData = match shop_str {
        Ok(content) => serde_json::from_str(content).unwrap(),
        Err(e) => {
            log::info!("{:?}", e);
            ShopData {
                shops: Vec::new(),
                guilds: Vec::new(),
                churches: Vec::new(),
            }
        }
    };
    shops
}

pub fn get_shop_convos() -> ShopConvos {
    let shop_asset = Asset::get("prefix/npcs/npc_shop_convos.json").unwrap();
    let shop_str = std::str::from_utf8(shop_asset.data.as_ref());
    let shops: ShopConvos = match shop_str {
        Ok(content) => serde_json::from_str(content).unwrap(),
        Err(e) => {
            log::info!("{:?}", e);
            ShopConvos {
                shops: Vec::new(),
                guilds: Vec::new(),
                churches: Vec::new(),
            }
        }
    };
    shops
}

fn npc_ascii(npc: Npcs) -> String {
    match npc {
        Npcs::Settler => "settler".to_string(),
        Npcs::GuildMember => "guild-member".to_string(),
        Npcs::GuildHead => "guild-head".to_string(),
        Npcs::ObsidianMember => "obsidian-member".to_string(),
        Npcs::ObsidianSteward => "obsidian-steward".to_string(),
        Npcs::ShopKeeper => "shop-keeper".to_string(),
        Npcs::Herbalist => "herbalist".to_string(),
        Npcs::Clinic => "clinic".to_string(),
        Npcs::WeaponSmith => "weapon-smith".to_string(),
        Npcs::Armorer => "armorer".to_string(),
        Npcs::Terminal => "terminal".to_string(),
    }
}

fn enemy_ascii(enemy: Enemies) -> String {
    match enemy {
        Enemies::Spider => "spider".to_string(),
        Enemies::Golem => "golem".to_string(),
        Enemies::CrazedExplorer => "explorer".to_string(),
        Enemies::Goblin => "goblin".to_string(),
        Enemies::Slime => "slime".to_string(),
        Enemies::Snake => "snake".to_string(),
        Enemies::Bandit => "bandit".to_string(),
        Enemies::Ghoul => "ghoul".to_string(),
        Enemies::Bug => "bug".to_string(),
        Enemies::Null => "null".to_string(),
    }
}

fn item_ascii(item: Items) -> String {
    match item {
        Items::HealthPotion => "health-potion".to_string(),
        Items::VitalityPotion => "vitality-potion".to_string(),
        Items::Antidote => "antidote".to_string(),
        Items::LuckPotion => "luck-potion".to_string(),
        Items::AgilityPotion => "agility-potion".to_string(),
        Items::Salve => "salve".to_string(),
        Items::Dowel => "dowel".to_string(),
        Items::WoodenBoard => "wooden-board".to_string(),
        Items::IronSword => "iron-sword".to_string(),
        Items::MetalScrap => "metal-scrap".to_string(),
        Items::Apple => "apple".to_string(),
        Items::EdibleRoot => "edible-root".to_string(),
        Items::Guts => "guts".to_string(),
        Items::Rock => "rock".to_string(),
        Items::BronzeClaymore => "bronze-claymore".to_string(),
        Items::IronClaymore => "iron-claymore".to_string(),
        Items::SteelClaymore => "steel-claymore".to_string(),
        Items::BronzeLongsword => "bronze-longsword".to_string(),
        Items::IronLongsword => "iron-longsword".to_string(),
        Items::SteelLongsword => "steel-longsword".to_string(),
        Items::BronzeGreatsword => "bronze-greatsword".to_string(),
        Items::IronGreatsword => "iron-greatsword".to_string(),
        Items::SteelGreatsword => "steel-greatsword".to_string(),
        Items::BronzeShortsword => "bronze-shortsword".to_string(),
        Items::IronShortsword => "iron-shortsword".to_string(),
        Items::SteelShortsword => "steel-shortsword".to_string(),
        Items::BasicStaff => "basic-staff".to_string(),
        Items::PineStaff => "pine-staff".to_string(),
        Items::WoodStaff => "wood-staff".to_string(),
        Items::MapleStaff => "maple-staff".to_string(),
        Items::OakStaff => "oak-staff".to_string(),
        Items::BludgeonStaff => "bludgeon-staff".to_string(),
        Items::GemStaff => "gem-staff".to_string(),
        Items::BronzeHeavyAxe => "bronze-heavy-axe".to_string(),
        Items::IronHeavyAxe => "iron-heavy-axe".to_string(),
        Items::SteelHeavyAxe => "steel-heavy-axe".to_string(),
        Items::BronzeLightAxe => "bronze-light-axe".to_string(),
        Items::IronLightAxe => "iron-light-axe".to_string(),
        Items::SteelLightAxe => "steel-light-axe".to_string(),
        Items::BronzePickAxe => "bronze-pick-axe".to_string(),
        Items::IronPickAxe => "iron-pick-axe".to_string(),
        Items::SteelPickAxe => "steel-pick-axe".to_string(),
        Items::BronzePickHammer => "bronze-pick-hammer".to_string(),
        Items::IronPickHammer => "iron-pick-hammer".to_string(),
        Items::SteelPickHammer => "steel-pick-hammer".to_string(),
        Items::ShadowAxe => "shadow-axe".to_string(),
        Items::BronzeWarAxe => "bronze-war-axe".to_string(),
        Items::IronWarAxe => "iron-war-axe".to_string(),
        Items::SteelWarAxe => "steel-war-axe".to_string(),
        Items::LightArmour => "light-armour".to_string(),
        Items::MediumArmour => "med-armour".to_string(),
        Items::HeavyArmour => "heavy-armour".to_string(),
        Items::SmallWoodShield => "small-wood-shield".to_string(),
        Items::LargeWoodShield => "large-wood-shield".to_string(),
        Items::IronShield => "iron-shield".to_string(),
        Items::SteelShield => "steel-shield".to_string(),
        Items::Plants(plants) => match plants {
            Plants::Moss => "moss".to_string(),
            Plants::LuminousMushroom => "luminus-mushroom".to_string(),
            Plants::LichenousGrowth => "lichenous-growth".to_string(),
            Plants::VineBulb => "vine-bulb".to_string(),
            Plants::VioletShadow => "violet-shadow".to_string(),
            Plants::LampenFlower => "lampen-flower".to_string(),
            Plants::LuckyClover => "lucky-clover".to_string(),
            Plants::Shroom => "shroom".to_string(),
            Plants::Null => "null".to_string(),
        },
        Items::ShieldingPendant => "shielding-pendant".to_string(),
        Items::AgilityPendant => "agility-pendant".to_string(),
        Items::StrengthPendant => "strength-pendant".to_string(),
        Items::Scroll => "scroll".to_string(),
        Items::Gold => "gold".to_string(),
        Items::Null => "null".to_string(),
    }
}

pub fn env_inter_ascii(env: EnvInter) -> String {
    match env {
        EnvInter::Records => "records".to_string(),
        EnvInter::Clinic => "clinic".to_string(),
        EnvInter::GuildPost => "guild-post".to_string(),
        EnvInter::ChurchPost => "church-post".to_string(),
        EnvInter::Construction => "construction".to_string(),
        EnvInter::Cauldron => "cauldron".to_string(),
        EnvInter::Task(_task_type) => "task".to_string(),
        EnvInter::Door(door) => "door".to_string(),
        EnvInter::ShopNPC(shops) => match shops {
            crate::enums::Shops::Item => "shop-keeper".to_string(),
            crate::enums::Shops::Guild => "guild-head".to_string(),
            crate::enums::Shops::Church => "obsidian-stweard".to_string(),
            crate::enums::Shops::Clinic => "clinic".to_string(),
            crate::enums::Shops::Herbalist => "herbalist".to_string(),
            crate::enums::Shops::Weapon => "weaponsmith".to_string(),
            crate::enums::Shops::Armor => "armourer".to_string(),
            crate::enums::Shops::Consignment => "shop-keeper".to_string(),
            crate::enums::Shops::Null => "".to_string(),
        },
        EnvInter::Herbalist => "herbalist".to_string(),
        EnvInter::Hermit => "hermit".to_string(),
        EnvInter::TaskEnv(task_env) => "settler".to_string(),
        EnvInter::WoodenHatch => "wooden-hatch".to_string(),
        EnvInter::Null => "".to_string(),
    }
}

pub fn get_ascii(ascii: Ascii) -> String {
    let ascii_asset = match ascii {
        Ascii::Npcs(_) => Asset::get("prefix/ascii/npc_asciis.json").unwrap(),
        Ascii::Enemies(_) => Asset::get("prefix/ascii/enemy_asciis.json").unwrap(),
        Ascii::Items(_) => Asset::get("prefix/ascii/item_asciis.json").unwrap(),
        Ascii::EnvInter(_) => Asset::get("prefix/ascii/env_inter_asciis.json").unwrap(),
    };
    let ascii_str = std::str::from_utf8(ascii_asset.data.as_ref());
    let asciis: HashMap<String, String> = match ascii_str {
        Ok(content) => serde_json::from_str(&content).unwrap(),
        Err(e) => {
            log::info!("{:?}", e);
            HashMap::new()
        }
    };
    match ascii {
        Ascii::Npcs(npc) => asciis
            .get(&npc_ascii(npc))
            .unwrap_or_else(|| asciis.get(&npc_ascii(Npcs::Settler)).unwrap())
            .clone(),
        Ascii::Enemies(enemy) => asciis
            .get(&enemy_ascii(enemy))
            .unwrap_or_else(|| asciis.get(&enemy_ascii(Enemies::Spider)).unwrap())
            .clone(),
        Ascii::Items(item) => asciis
            .get(&item_ascii(item))
            .unwrap_or_else(|| asciis.get(&item_ascii(Items::Apple)).unwrap())
            .clone(),
        Ascii::EnvInter(env_inter) => asciis
            .get(&env_inter_ascii(env_inter))
            .unwrap_or_else(|| asciis.get(&env_inter_ascii(EnvInter::Records)).unwrap())
            .clone(),
    }
}

pub fn load_comms(ntype: &String) -> CommDialogue {
    let city_path = format!("src/npcs/{}/comms_city.json", ntype);
    let engine_path = format!("src/npcs/{}/comms_engine.json", ntype);
    let guild_path = format!("src/npcs/{}/comms_guild.json", ntype);
    let cult_path = format!("src/npcs/{}/comms_cult.json", ntype);
    let data1 = fs::read_to_string(city_path);
    // println!("{:?}", data1);
    let city: Vec<String> = match data1 {
        Ok(content) => serde_json::from_str(&content).unwrap(),
        Err(e) => {
            log::info!("{:?}", e);
            Vec::new()
        }
    };

    let data2 = fs::read_to_string(engine_path);
    // println!("{:?}", data2);
    let engine: Vec<String> = match data2 {
        Ok(content) => serde_json::from_str(&content).unwrap(),
        Err(e) => {
            log::info!("{:?}", e);
            Vec::new()
        }
    };

    let data3 = fs::read_to_string(guild_path);
    // println!("{:?}", data3);
    let guild: Vec<String> = match data3 {
        Ok(content) => serde_json::from_str(&content).unwrap(),
        Err(e) => {
            log::info!("{:?}", e);
            Vec::new()
        }
    };

    let data4 = fs::read_to_string(cult_path);
    // println!("{:?}", data4);
    let cult: Vec<String> = match data4 {
        Ok(content) => serde_json::from_str(&content).unwrap(),
        Err(e) => {
            log::info!("{:?}", e);
            Vec::new()
        }
    };
    CommDialogue {
        city,
        engine,
        guild,
        cult,
    }
}

pub fn load_convos(ntype: &String) -> ConvoDialogue {
    let city_path = format!("src/npcs/{}/convos_city.json", ntype);
    let engine_path = format!("src/npcs/{}/convos_engine.json", ntype);
    let guild_path = format!("src/npcs/{}/convos_guild.json", ntype);
    let cult_path = format!("src/npcs/{}/convos_cult.json", ntype);
    let data1 = fs::read_to_string(city_path);
    // print!("{:?}", data1);
    let city: Vec<Convo> = match data1 {
        Ok(content) => serde_json::from_str(&content).unwrap(),
        Err(e) => {
            log::info!("{:?}", e);
            Vec::new()
        }
    };

    let data2 = fs::read_to_string(engine_path);
    // print!("{:?}", data2);
    let engine: Vec<Convo> = match data2 {
        Ok(content) => serde_json::from_str(&content).unwrap(),
        Err(e) => {
            log::info!("{:?}", e);
            Vec::new()
        }
    };

    let data3 = fs::read_to_string(guild_path);
    // print!("{:?}", data3);
    let guild: Vec<Convo> = match data3 {
        Ok(content) => serde_json::from_str(&content).unwrap(),
        Err(e) => {
            log::info!("{:?}", e);
            Vec::new()
        }
    };

    let data4 = fs::read_to_string(cult_path);
    // print!("{:?}", data4);
    let cult: Vec<Convo> = match data4 {
        Ok(content) => serde_json::from_str(&content).unwrap(),
        Err(e) => {
            log::info!("{:?}", e);
            Vec::new()
        }
    };
    ConvoDialogue {
        city,
        engine,
        guild,
        cult,
    }
}

// impl Dialogue {
//     pub fn new() -> Self {
//         let ntypes = ["cave".to_string(), "guild".to_string(), "cult".to_string()];
//         let mut comm_dialogue = Vec::new();
//         for ntype in &ntypes {
//             comm_dialogue.push(load_comms(ntype));
//         }

//         let mut convo_dialogue = Vec::new();
//         for ntype in &ntypes {
//             convo_dialogue.push(load_convos(ntype));
//         }

//         Self {
//             cave_comms: comm_dialogue[0].clone(),
//             cave_convos: convo_dialogue[0].clone(),
//             guild_comms: comm_dialogue[1].clone(),
//             guild_convos: convo_dialogue[1].clone(),
//             cult_comms: comm_dialogue[2].clone(),
//             cult_convos: convo_dialogue[2].clone(),
//             file_paths: HashMap::new(),
//         }
//     }

//     pub fn get_cave_comm(&mut self) -> &String {
//         let mut rng = rand::thread_rng();
//         match rng.gen_range(0..3) {
//             0 => self
//                 .cave_comms
//                 .city
//                 .choose(&mut rng)
//                 .unwrap_or(&self.cave_comms.city[0]),
//             1 => self
//                 .cave_comms
//                 .engine
//                 .choose(&mut rng)
//                 .unwrap_or(&self.cave_comms.engine[0]),
//             2 => self
//                 .cave_comms
//                 .guild
//                 .choose(&mut rng)
//                 .unwrap_or(&self.cave_comms.guild[0]),
//             3 => self
//                 .cave_comms
//                 .cult
//                 .choose(&mut rng)
//                 .unwrap_or(&self.cave_comms.cult[0]),
//             _ => todo!(),
//         }
//     }

//     pub fn get_guild_comm(&mut self) -> &String {
//         let mut rng = rand::thread_rng();
//         match rng.gen_range(0..3) {
//             0 => self
//                 .guild_comms
//                 .city
//                 .choose(&mut rng)
//                 .unwrap_or(&self.guild_comms.city[0]),
//             1 => self
//                 .guild_comms
//                 .engine
//                 .choose(&mut rng)
//                 .unwrap_or(&self.guild_comms.engine[0]),
//             2 => self
//                 .guild_comms
//                 .guild
//                 .choose(&mut rng)
//                 .unwrap_or(&self.guild_comms.guild[0]),
//             3 => self
//                 .guild_comms
//                 .cult
//                 .choose(&mut rng)
//                 .unwrap_or(&self.guild_comms.cult[0]),
//             _ => todo!(),
//         }
//     }

//     pub fn get_cult_comm(&mut self) -> &String {
//         let mut rng = rand::thread_rng();
//         match rng.gen_range(0..3) {
//             0 => self
//                 .cult_comms
//                 .city
//                 .choose(&mut rng)
//                 .unwrap_or(&self.cult_comms.city[0]),
//             1 => self
//                 .cult_comms
//                 .engine
//                 .choose(&mut rng)
//                 .unwrap_or(&self.cult_comms.engine[0]),
//             2 => self
//                 .cult_comms
//                 .cult
//                 .choose(&mut rng)
//                 .unwrap_or(&self.cult_comms.guild[0]),
//             3 => self
//                 .cult_comms
//                 .cult
//                 .choose(&mut rng)
//                 .unwrap_or(&self.cult_comms.cult[0]),
//             _ => todo!(),
//         }
//     }

//     pub fn get_cave_convo(&mut self) -> &Convo {
//         let mut rng = rand::thread_rng();
//         match rng.gen_range(0..3) {
//             0 => self
//                 .cave_convos
//                 .city
//                 .choose(&mut rng)
//                 .unwrap_or(&self.cave_convos.city[0]),
//             1 => self
//                 .cave_convos
//                 .engine
//                 .choose(&mut rng)
//                 .unwrap_or(&self.cave_convos.engine[0]),
//             2 => self
//                 .cave_convos
//                 .guild
//                 .choose(&mut rng)
//                 .unwrap_or(&self.cave_convos.guild[0]),
//             3 => self
//                 .cave_convos
//                 .cult
//                 .choose(&mut rng)
//                 .unwrap_or(&self.cave_convos.cult[0]),
//             _ => todo!(),
//         }
//     }

//     pub fn get_guild_convo(&mut self) -> &Convo {
//         let mut rng = rand::thread_rng();
//         match rng.gen_range(0..3) {
//             0 => self
//                 .guild_convos
//                 .city
//                 .choose(&mut rng)
//                 .unwrap_or(&self.guild_convos.city[0]),
//             1 => self
//                 .guild_convos
//                 .engine
//                 .choose(&mut rng)
//                 .unwrap_or(&self.guild_convos.engine[0]),
//             2 => self
//                 .guild_convos
//                 .guild
//                 .choose(&mut rng)
//                 .unwrap_or(&self.guild_convos.guild[0]),
//             3 => self
//                 .guild_convos
//                 .guild
//                 .choose(&mut rng)
//                 .unwrap_or(&self.guild_convos.cult[0]),
//             _ => todo!(),
//         }
//     }

//     pub fn get_cult_convo(&mut self) -> &Convo {
//         let mut rng = rand::thread_rng();
//         match rng.gen_range(0..3) {
//             0 => self
//                 .cult_convos
//                 .city
//                 .choose(&mut rng)
//                 .unwrap_or(&self.cult_convos.city[0]),
//             1 => self
//                 .cult_convos
//                 .engine
//                 .choose(&mut rng)
//                 .unwrap_or(&self.cult_convos.engine[0]),
//             2 => self
//                 .cult_convos
//                 .cult
//                 .choose(&mut rng)
//                 .unwrap_or(&self.cult_convos.guild[0]),
//             3 => self
//                 .cult_convos
//                 .cult
//                 .choose(&mut rng)
//                 .unwrap_or(&self.cult_convos.cult[0]),
//             _ => todo!(),
//         }
//     }

//     pub fn retrieve_new_dialogue(&mut self) {}
// }
