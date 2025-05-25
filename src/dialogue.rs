//dialogue

// dialogue categories
//
// # plot arcs
// + lost city
// + search for engine
// + guild
// + cult
// + restoration
//
// # npc types
// + comms
// + convo
// + shop
// + spawn
//
// # dialogue groups
// + lost city
//  - comms
//   - cave
//   - guild
//   - cult
//  - convo
//   - cave
//   - guild
//   - cult
// + search for engine
//  - comms
//   - cave
//   - guild
//   - cult
//  - convo
//   - cave
//   - guild
//   - cult
// + guild restoration
//  - comms
//   - cave
//   - guild
//   - cult
//  - convo
//   - cave
//   - guild
//   - cult
// + cult rituals
//  - comms
//   - cave
//   - guild
//   - cult
//  - convo
//   - cave
//   - guild
//   - cult
//
// # by npc groups
// + cave
//  - comms
//   - lost city
//   - search for engine
//   - guild
//   - cult
//  - convo
//   - lost city
//   - search for engine
//   - guild
//   - cult
// + guild
//  - comms
//   - lost city
//   - search for engine
//   - cult
//  - convo
//   - lost city
//   - search for engine
//   - cult
// + cult
//  - comms
//   - lost city
//   - search for engine
//   - guild
//  - convo
//   - lost city
//   - search for engine
//   - guild
//
// + Hermit
// + Herbalist
//
//

use crate::npc::Convo;
use std::collections::HashMap;
use std::fs;

struct CommDialogue {
    city: Vec<String>,
    engine: Vec<String>,
    guild: Vec<String>,
    cult: Vec<String>,
}

struct ConvoDialogue {
    city: Vec<Convo>,
    engine: Vec<Convo>,
    guild: Vec<Convo>,
    cult: Vec<Convo>,
}

struct Dialogue {
    cave_comms: CommDialogue,
    cave_convos: ConvoDialogue,
    guild_comms: CommDialogue,
    guild_convos: ConvoDialogue,
    cult_comms: CommDialogue,
    cult_convos: ConvoDialogue,
    file_paths: HashMap<String, String>,
}

impl Dialogue {
    // pub fn new() -> Self {
    //     let data1 = fs::read_to_string("src/npcs/cave_comms_city.json");
    //     let names: Vec<String> = match data1 {
    //         Ok(content) => serde_json::from_str(&content).unwrap(),
    //         Err(e) => {
    //             log::info!("{:?}", e);
    //             Vec::new()
    //         }
    //     };

    //     let data2 = fs::read_to_string("src/npcs/cave_comms_engine.json");
    //     let names: Vec<Convo> = match data2 {
    //         Ok(content) => serde_json::from_str(&content).unwrap(),
    //         Err(e) => {
    //             log::info!("{:?}", e);
    //             Vec::new()
    //         }
    //     };
    //     Self {}
    // }

    pub fn retrieve_cave_dialogue(&mut self) {}
}
