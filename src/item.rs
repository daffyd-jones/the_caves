//item
use crate::enums::{Items, InterOpt, ItemOpt};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct Item {
    pub itype: Items,
    pub sname: String,
    pub desc: String,
    pub iopts: HashMap<InterOpt, String>,
    pub equip: bool,
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
            equip: false,
            x: 0,
            y: 0,
            properties: p,
        }
    }
}

impl Item {
    pub fn new(itype: Items, sname: String, desc: String, iopts: HashMap<InterOpt, String>, equip: bool, x: usize, y: usize, properties: HashMap<String, u16>) -> Self {
        Self {itype, sname, desc, iopts, equip, x, y, properties}
    }

    pub fn new_edible_root(x: usize, y: usize) -> Self {
        let mut prop = HashMap::new();
        prop.insert(String::from("health"), 3);
        let mut iopts = HashMap::new();
        iopts.insert(InterOpt::Item(ItemOpt::PickUp), String::from("Pick Up"));
        iopts.insert(InterOpt::Item(ItemOpt::Drp), String::from("Drop"));
        iopts.insert(InterOpt::Item(ItemOpt::Use), String::from("Use"));
        Self {
            itype: Items::EdibleRoot,
            sname: "Edible Root".to_string(),
            desc: "Weird looking root, doesnt look very tasty.".to_string(),
            iopts,
            equip: false,
            x,
            y,
            properties: prop,
        }
    }

    pub fn new_rock(x: usize, y: usize) -> Self {
        let mut prop = HashMap::new();
        prop.insert(String::from("health"), 0);
        let mut iopts = HashMap::new();
        iopts.insert(InterOpt::Item(ItemOpt::PickUp), String::from("Pick Up"));
        iopts.insert(InterOpt::Item(ItemOpt::Drp), String::from("Drop"));
        Self {
            itype: Items::Rock,
            sname: "Rock".to_string(),
            desc: "Its a rock.".to_string(),
            iopts,
            equip: false,
            x,
            y,
            properties: prop,
        }
    }

    pub fn new_bug_bits(x: usize, y: usize) -> Self {
        let mut prop = HashMap::new();
        prop.insert(String::from("health"), 1);
        let mut iopts = HashMap::new();
        iopts.insert(InterOpt::Item(ItemOpt::PickUp), String::from("Pick Up"));
        iopts.insert(InterOpt::Item(ItemOpt::Drp), String::from("Drop"));
        iopts.insert(InterOpt::Item(ItemOpt::Use), String::from("Use"));
        Self {
            itype: Items::BugBits,
            sname: "Bug Parts".to_string(),
            desc: "Parts of a dead bug.".to_string(),
            iopts,
            equip: false,
            x,
            y,
            properties: prop,
        }
    }

    pub fn new_metal_scrap(x: usize, y: usize) -> Self {
        let mut prop = HashMap::new();
        prop.insert(String::from("health"), 0);
        prop.insert(String::from("value"), 1);
        let mut iopts = HashMap::new();
        iopts.insert(InterOpt::Item(ItemOpt::PickUp), String::from("Pick Up"));
        iopts.insert(InterOpt::Item(ItemOpt::Drp), String::from("Drop"));

        Self {
            itype: Items::MetalScrap,
            sname: "Metal Scrap".to_string(),
            desc: "Scrap of metal.".to_string(),
            iopts,
            equip: false,
            x,
            y,
            properties: prop,
        }
    }

    pub fn new_apple(x: usize, y: usize) -> Self {
        let mut prop = HashMap::new();
        prop.insert(String::from("health"), 5);
        prop.insert(String::from("value"), 5);
        let mut iopts = HashMap::new();
        iopts.insert(InterOpt::Item(ItemOpt::PickUp), String::from("Pick Up"));
        iopts.insert(InterOpt::Item(ItemOpt::Drp), String::from("Drop"));
        iopts.insert(InterOpt::Item(ItemOpt::Use), String::from("Use"));

        Self {
            itype: Items::Apple,
            sname: "Apple".to_string(),
            desc: "A slightly bruised apple that as been here for a while.".to_string(),
            iopts,
            equip: false,
            x,
            y,
            properties: prop,
        }
    }

    pub fn new_health_potion(x: usize, y: usize) -> Self {
        let mut prop = HashMap::new();
        prop.insert(String::from("health"), 30);
        prop.insert(String::from("value"), 50);
        let mut iopts = HashMap::new();
        iopts.insert(InterOpt::Item(ItemOpt::PickUp), String::from("Pick Up"));
        iopts.insert(InterOpt::Item(ItemOpt::Drp), String::from("Drop"));
        iopts.insert(InterOpt::Item(ItemOpt::Use), String::from("Use"));
        Self {
            itype: Items::HealthPotion,
            sname: "Health Potion".to_string(),
            desc: "Mixture of curdled liquids. Returns vitality to the body.".to_string(),
            iopts,
            equip: false,
            x,
            y,
            properties: prop,
        }
    }

    pub fn new_salve(x: usize, y: usize) -> Self {
        let mut prop = HashMap::new();
        prop.insert(String::from("health"), 15);
        prop.insert(String::from("value"), 30);
        let mut iopts = HashMap::new();
        iopts.insert(InterOpt::Item(ItemOpt::PickUp), String::from("Pick Up"));
        iopts.insert(InterOpt::Item(ItemOpt::Drp), String::from("Drop"));
        iopts.insert(InterOpt::Item(ItemOpt::Use), String::from("Use"));

        Self {
            itype: Items::HealthPotion,
            sname: "Salve".to_string(),
            desc: "Thick paste for smearing on wounds. It heals better than it smells.".to_string(),
            iopts,
            equip: false,
            x,
            y,
            properties: prop,
        }
    }

    pub fn new_dowel(x: usize, y: usize) -> Self {
        let mut prop = HashMap::new();
        prop.insert(String::from("damage"), 5);
        prop.insert(String::from("value"), 30);
        let mut iopts = HashMap::new();
        iopts.insert(InterOpt::Item(ItemOpt::PickUp), String::from("Pick Up"));
        iopts.insert(InterOpt::Item(ItemOpt::Drp), String::from("Drop"));
        iopts.insert(InterOpt::Item(ItemOpt::Use), String::from("Use"));

        Self {
            itype: Items::Dowel,
            sname: "Dowel".to_string(),
            desc: "Most of a broomstick. Its sharp at one end.".to_string(),
            iopts,
            equip: true,
            x,
            y,
            properties: prop,
        }
    }

    pub fn new_wooden_board(x: usize, y: usize) -> Self {
        let mut prop = HashMap::new();
        prop.insert(String::from("defence"), 5);
        prop.insert(String::from("value"), 30);
        let mut iopts = HashMap::new();
        iopts.insert(InterOpt::Item(ItemOpt::PickUp), String::from("Pick Up"));
        iopts.insert(InterOpt::Item(ItemOpt::Drp), String::from("Drop"));
        iopts.insert(InterOpt::Item(ItemOpt::Use), String::from("Use"));

        Self {
            itype: Items::WoodenBoard,
            sname: "Wooden Board".to_string(),
            desc: "A wooden board with a strap attached to it.".to_string(),
            iopts,
            equip: true,
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

    pub fn get_pos(&mut self) -> (usize, usize) {
        (self.x.clone(), self.y.clone())
    }

    pub fn set_pos(&mut self, pos: (usize, usize)) {
        self.x = pos.0;
        self.y = pos.1;
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
