use rust_embed::Embed;

#[derive(Embed)]
#[folder = "assets/"]
#[prefix = "prefix/"]
struct Asset;

//dialogue

use rand::seq::SliceRandom;
use rand::Rng;

use crate::npc::Convo;
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

pub enum AssetType {
    Comms(Comms),
    Convos(Convos),
    Ascii(Ascii),
}

pub enum Comms {
    CaveCity,
    CaveEngine,
    CaveGuild,
    CaveCult,
    GuildCity,
    GuildEngine,
    GuildGuild,
    GuildCult,
    ObsidianCity,
    ObsidianEngine,
    ObsidianGuild,
    ObsidianCult,
}

pub enum Convos {
    CaveCity,
    CaveEngine,
    CaveGuild,
    CaveCult,
    GuildCity,
    GuildEngine,
    GuildGuild,
    GuildCult,
    ObsidianCity,
    ObsidianEngine,
    ObsidianGuild,
    ObsidianCult,
}

pub enum Ascii {
    Npcs(Npcs),
    Enemies(Enemies),
    Items(Items),
}

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
    Terminal
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Enemies {
    Golem,
    CrazedExplorer,
    Goblin,
    Slime,
    Snake,
    Spider,
    Bandit,
    Ghoul,
    Bug,
    Null,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Items {
    HealthPotion, // +10 health
    VitalityPotion,
    Antidote,
    LuckPotion,
    AgilityPotion,
    Salve,
    Dowel,
    WoodenBoard,
    IronSword,
    MetalScrap,
    Apple,
    EdibleRoot,
    Guts,
    Rock,
    // weapons
    BronzeClaymore,
    IronClaymore,
    SteelClaymore,

    BronzeLongsword,
    IronLongsword,
    SteelLongsword,

    BronzeGreatsword,
    IronGreatsword,
    SteelGreatsword,

    BronzeShortsword,
    IronShortsword,
    SteelShortsword,

    BasicStaff,
    PineStaff,
    WoodStaff,
    MapleStaff,
    OakStaff,
    BludgeonStaff,
    GemStaff,

    BronzeHeavyAxe,
    IronHeavyAxe,
    SteelHeavyAxe,

    BronzeLightAxe,
    IronLightAxe,
    SteelLightAxe,

    BronzePickAxe,
    IronPickAxe,
    SteelPickAxe,

    BronzePickHammer,
    IronPickHammer,
    SteelPickHammer,

    ShadowAxe,

    BronzeWarAxe,
    IronWarAxe,
    SteelWarAxe,

    LightArmour,
    MediumArmour,
    HeavyArmour,

    SmallWoodShield,
    LargeWoodShield,
    IronShield, // +10 defence
    SteelShield,

    //
    Plants(Plants),
    // wearable
    ShieldingPendant,
    AgilityPendant,
    StrengthPendant,
    // documents
    Scroll,
    Gold,
    Null,
}


// pub struct Dialogue {
//     pub cave_comms: CommDialogue,
//     pub cave_convos: ConvoDialogue,
//     pub guild_comms: CommDialogue,
//     pub guild_convos: ConvoDialogue,
//     pub cult_comms: CommDialogue,
//     pub cult_convos: ConvoDialogue,
//     pub file_paths: HashMap<String, String>,
// }

pub fn get_npc_names()

pub fn get_comms(comm_type: Comms) -> Vec<String> {
    match comm_type {
        Comms::CaveCity => Asset::get("prefix/npcs/cave/")
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

impl Dialogue {
    pub fn new() -> Self {
        let ntypes = ["cave".to_string(), "guild".to_string(), "cult".to_string()];
        let mut comm_dialogue = Vec::new();
        for ntype in &ntypes {
            comm_dialogue.push(load_comms(ntype));
        }

        let mut convo_dialogue = Vec::new();
        for ntype in &ntypes {
            convo_dialogue.push(load_convos(ntype));
        }

        Self {
            cave_comms: comm_dialogue[0].clone(),
            cave_convos: convo_dialogue[0].clone(),
            guild_comms: comm_dialogue[1].clone(),
            guild_convos: convo_dialogue[1].clone(),
            cult_comms: comm_dialogue[2].clone(),
            cult_convos: convo_dialogue[2].clone(),
            file_paths: HashMap::new(),
        }
    }

    pub fn get_cave_comm(&mut self) -> &String {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..3) {
            0 => self
                .cave_comms
                .city
                .choose(&mut rng)
                .unwrap_or(&self.cave_comms.city[0]),
            1 => self
                .cave_comms
                .engine
                .choose(&mut rng)
                .unwrap_or(&self.cave_comms.engine[0]),
            2 => self
                .cave_comms
                .guild
                .choose(&mut rng)
                .unwrap_or(&self.cave_comms.guild[0]),
            3 => self
                .cave_comms
                .cult
                .choose(&mut rng)
                .unwrap_or(&self.cave_comms.cult[0]),
            _ => todo!(),
        }
    }

    pub fn get_guild_comm(&mut self) -> &String {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..3) {
            0 => self
                .guild_comms
                .city
                .choose(&mut rng)
                .unwrap_or(&self.guild_comms.city[0]),
            1 => self
                .guild_comms
                .engine
                .choose(&mut rng)
                .unwrap_or(&self.guild_comms.engine[0]),
            2 => self
                .guild_comms
                .guild
                .choose(&mut rng)
                .unwrap_or(&self.guild_comms.guild[0]),
            3 => self
                .guild_comms
                .cult
                .choose(&mut rng)
                .unwrap_or(&self.guild_comms.cult[0]),
            _ => todo!(),
        }
    }

    pub fn get_cult_comm(&mut self) -> &String {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..3) {
            0 => self
                .cult_comms
                .city
                .choose(&mut rng)
                .unwrap_or(&self.cult_comms.city[0]),
            1 => self
                .cult_comms
                .engine
                .choose(&mut rng)
                .unwrap_or(&self.cult_comms.engine[0]),
            2 => self
                .cult_comms
                .cult
                .choose(&mut rng)
                .unwrap_or(&self.cult_comms.guild[0]),
            3 => self
                .cult_comms
                .cult
                .choose(&mut rng)
                .unwrap_or(&self.cult_comms.cult[0]),
            _ => todo!(),
        }
    }

    pub fn get_cave_convo(&mut self) -> &Convo {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..3) {
            0 => self
                .cave_convos
                .city
                .choose(&mut rng)
                .unwrap_or(&self.cave_convos.city[0]),
            1 => self
                .cave_convos
                .engine
                .choose(&mut rng)
                .unwrap_or(&self.cave_convos.engine[0]),
            2 => self
                .cave_convos
                .guild
                .choose(&mut rng)
                .unwrap_or(&self.cave_convos.guild[0]),
            3 => self
                .cave_convos
                .cult
                .choose(&mut rng)
                .unwrap_or(&self.cave_convos.cult[0]),
            _ => todo!(),
        }
    }

    pub fn get_guild_convo(&mut self) -> &Convo {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..3) {
            0 => self
                .guild_convos
                .city
                .choose(&mut rng)
                .unwrap_or(&self.guild_convos.city[0]),
            1 => self
                .guild_convos
                .engine
                .choose(&mut rng)
                .unwrap_or(&self.guild_convos.engine[0]),
            2 => self
                .guild_convos
                .guild
                .choose(&mut rng)
                .unwrap_or(&self.guild_convos.guild[0]),
            3 => self
                .guild_convos
                .guild
                .choose(&mut rng)
                .unwrap_or(&self.guild_convos.cult[0]),
            _ => todo!(),
        }
    }

    pub fn get_cult_convo(&mut self) -> &Convo {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..3) {
            0 => self
                .cult_convos
                .city
                .choose(&mut rng)
                .unwrap_or(&self.cult_convos.city[0]),
            1 => self
                .cult_convos
                .engine
                .choose(&mut rng)
                .unwrap_or(&self.cult_convos.engine[0]),
            2 => self
                .cult_convos
                .cult
                .choose(&mut rng)
                .unwrap_or(&self.cult_convos.guild[0]),
            3 => self
                .cult_convos
                .cult
                .choose(&mut rng)
                .unwrap_or(&self.cult_convos.cult[0]),
            _ => todo!(),
        }
    }

    pub fn retrieve_new_dialogue(&mut self) {}
}
