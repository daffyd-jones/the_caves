//item
use crate::enums::{Items, InterOpt, ItemOpt};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Item {
    pub itype: Items,
    pub sname: String,
    pub desc: String,
    pub iopts: HashMap<InterOpt, String>,
    pub x: usize,
    pub y: usize,
    pub properties: HashMap<String, u16>,
}

impl Default for Item {
    fn default() -> Self {
        let mut h = HashMap::new();
        h.insert(InterOpt::Null, "".to_string());
        let mut p = HashMap::new();
        p.insert("".to_string(), 0);
        Self {
            itype: Items::Null,
            sname: "".to_string(),
            desc: "".to_string(),
            iopts: h,
            x: 0,
            y: 0,
            properties: p,
        }
    }
}

impl Item {
    pub fn new(itype: Items, sname: String, desc: String, iopts: HashMap<InterOpt, String>, x: usize, y: usize, properties: HashMap<String, u16>) -> Self {
        Self {itype, sname, desc, iopts, x, y, properties}
    }

    pub fn new_edible_root(x: usize, y: usize) -> Self {
        let mut prop = HashMap::new();
        prop.insert(String::from("Health"), 5);
        let mut iopts = HashMap::new();
        iopts.insert(InterOpt::Item(ItemOpt::PickUp), String::from("Pick Up"));
        iopts.insert(InterOpt::Item(ItemOpt::Drp), String::from("Drop"));
        iopts.insert(InterOpt::Item(ItemOpt::Use), String::from("Use"));
        Self {
            itype: Items::EdibleRoot,
            sname: "Edible Root".to_string(),
            desc: "Weird looking root, doesnt look very tasty.".to_string(),
            iopts,
            x,
            y,
            properties: prop,
        }
    }

    pub fn new_rock(x: usize, y: usize) -> Self {
        let mut prop = HashMap::new();
        prop.insert(String::from("Health"), 5);
        let mut iopts = HashMap::new();
        iopts.insert(InterOpt::Item(ItemOpt::PickUp), String::from("Pick Up"));
        iopts.insert(InterOpt::Item(ItemOpt::Drp), String::from("Drop"));
        Self {
            itype: Items::Rock,
            sname: "Rock".to_string(),
            desc: "Its a rock.".to_string(),
            iopts,
            x,
            y,
            properties: prop,
        }
    }

    pub fn new_bug_bits(x: usize, y: usize) -> Self {
        let mut prop = HashMap::new();
        prop.insert(String::from("Health"), 1);
        let mut iopts = HashMap::new();
        iopts.insert(InterOpt::Item(ItemOpt::PickUp), String::from("Pick Up"));
        iopts.insert(InterOpt::Item(ItemOpt::Drp), String::from("Drop"));
        iopts.insert(InterOpt::Item(ItemOpt::Use), String::from("Use"));
        Self {
            itype: Items::BugBits,
            sname: "Bug Parts".to_string(),
            desc: "Parts of a dead bug.".to_string(),
            iopts,
            x,
            y,
            properties: prop,
        }
    }

    pub fn get_itype(&mut self) -> Items {
        self.itype.clone()
    }

    pub fn get_sname(&mut self) -> String {
        self.sname.clone()
    }

    pub fn get_properties(&mut self) -> HashMap<String, u16> {
        self.properties.clone()
    }

    pub fn get_desc(&mut self) -> String {
        self.desc.clone()
    }

    pub fn get_iopts(&mut self) -> HashMap<InterOpt, String> {
        self.iopts.clone()
    }
}
