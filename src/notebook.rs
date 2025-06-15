//notebook

use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::collections::HashMap;
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

//#[derive(Serialize, Deserialize)]
pub struct Notebook {
    settles: HashMap<String, String>,
    convos: Vec<String>,
    knowledge: HashMap<String, String>,
    tasks: HashMap<String, String>,
}

impl Notebook {
    pub fn new() -> Result<Self> {
        Ok(Self {
            settles: HashMap::new(),
            convos: Vec::new(),
            knowledge: HashMap::new(),
            tasks: HashMap::new(),
        })
    }

    pub fn enter_settles(&mut self, sname: String, snote: String) {
        self.settles.insert(sname, snote);
    }

    pub fn enter_convo(&mut self, snote: &str) {
        // let fsnote = format!("{}", snote);
        self.convos.push(snote.to_string().clone());
    }

    pub fn enter_knowledge(&mut self, sname: String, snote: String) {
        self.knowledge.insert(sname, snote);
    }

    pub fn enter_tasks(&mut self, sname: String, snote: String) {
        self.tasks.insert(sname, snote);
    }

    pub fn get_notes(
        &self,
    ) -> (
        HashMap<String, String>,
        Vec<String>,
        HashMap<String, String>,
        HashMap<String, String>,
    ) {
        (
            self.settles.clone(),
            self.convos.clone(),
            self.knowledge.clone(),
            self.tasks.clone(),
        )
    }
}
