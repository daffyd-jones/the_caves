//notebook

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::Result;
use std::fs;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Quest {
    pub active: bool,
    pub name: String,
    pub stages: HashMap<String, Stage>,
}

impl Default for Quest {
    fn default() -> Self {
        Self {
            active: false,
            name: "".to_string(),
            stages: HashMap::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Stage {
    pub active: bool,
    pub text: String,
}

impl Default for Stage {
    fn default() -> Self {
        Self {
            active: false,
            text: "".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Place {
    pub active: bool,
    pub name: String,
    pub text: String,
}

impl Default for Place {
    fn default() -> Self {
        Self {
            active: false,
            name: "".to_string(),
            text: "".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Person {
    pub active: bool,
    pub name: String,
    pub desc: String,
}

impl Default for Person {
    fn default() -> Self {
        Self {
            active: false,
            name: "".to_string(),
            desc: "".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Lore {
    pub active: bool,
    pub name: String,
    pub desc: String,
}

impl Default for Lore {
    fn default() -> Self {
        Self {
            active: false,
            name: "".to_string(),
            desc: "".to_string(),
        }
    }
}

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct GameData {
//     quests: HashMap<String, Quest>,
//     places: HashMap<String, Place>,
//     people: HashMap<String, Person>,
//     lore: HashMap<String, Lore>,
// }




pub struct Notebook {
    quests: HashMap<String, Quest>,
    places: HashMap<String, Place>,
    people: HashMap<String, Person>,
    lore: HashMap<String, Lore>,
}

impl Notebook {
    pub fn new() -> Result<Self> {
        let data1 = fs::read_to_string("src/notebook/quests.json");
        log::info!("{:?}", &data1);
        let quests = match data1 {
            Ok(content) => serde_json::from_str(&content)?,
            Err(e) => {
                log::info!("{:?}", e);
                HashMap::new()
            },
        };

        // let quests: HashMap<String, Quest> = serde_json::from_str(&data1)?;
        let data2 = fs::read_to_string("src/notebook/places.json");
        log::info!("{:?}", &data2);
        let places = match data2 {
            Ok(content) => serde_json::from_str(&content)?,
            Err(e) => {
                log::info!("{:?}", e);
                HashMap::new()
            },
        };

        // let places: HashMap<String, Place> = serde_json::from_str(&data2)?;
        let data3 = fs::read_to_string("src/notebook/people.json");
        log::info!("{:?}", &data3);
        let people = match data3 {
            Ok(content) => serde_json::from_str(&content)?,
            Err(e) => {
                log::info!("{:?}", e);
                HashMap::new()
            },
        };
        // let people: HashMap<String, Person> = serde_json::from_str(&data3)?;
        let data4 = fs::read_to_string("src/notebook/lore.json");
        log::info!("{:?}", &data4);
        let lore = match data4 {
            Ok(content) => serde_json::from_str(&content)?,
            Err(e) => {
                log::info!("{:?}", e);
                HashMap::new()
            },
        };
        // let lore: HashMap<String, Lore> = serde_json::from_str(&data4)?;
        log::info!("\n{:?}\n{:?}\n{:?}\n{:?}", quests.clone(), places.clone(), people.clone(), lore.clone());
        // println!("{:?}\n{:?}\n{:?}\n{:?}", quests.clone(), places.clone(), people.clone(), lore.clone());


        Ok(Self { quests, places, people, lore })
        // Self {
        //     quests,
        //     places,
        //     people,
        //     lore
        // }
    }

    // pub fn get_active_quests(&self) -> Vec<String, Quest> {
    //     let mut temp = Vec::new();
    //     for (s, q) in &self.quests {
    //         if q.active {
    //             temp.push(q.clone());
    //         }
    //     }
    //     temp
    // }

    pub fn get_active_notes(&self) -> (Vec<Quest>, Vec<Place>, Vec<Person>,  Vec<Lore>) {
        let mut qtemp = Vec::new();
        let mut pltemp = Vec::new();
        let mut ptemp = Vec::new();
        let mut ltemp = Vec::new();

        for (s, q) in &self.quests {
            if q.active {
                qtemp.push(q.clone());
            }
        }
        for (s, p) in &self.places {
            if p.active {
                pltemp.push(p.clone());
            }
        }
        for (s, p) in &self.people {
            if p.active {
                ptemp.push(p.clone());
            }
        }
        for (s, l) in &self.lore {
            if l.active {
                ltemp.push(l.clone());
            }
        }

        (qtemp, pltemp, ptemp, ltemp)
    }

}
