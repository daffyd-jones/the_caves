//item
use crate::enums::{Equip, InterOpt, ItemEffect, ItemOpt, Items, Plants};
use rand::Rng;
use ratatui::style::Color;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
//#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Item {
    pub itype: Items,
    pub sname: String,
    pub icon: (char, Color),
    pub desc: String,
    pub iopts: HashMap<InterOpt, String>,
    pub equip: bool,
    pub craft: bool,
    pub produces: Items,
    pub equip_type: Equip,
    pub effect: ItemEffect,
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
            icon: (' ', Color::White),
            iopts: h,
            equip: false,
            craft: false,
            produces: Items::Null,
            equip_type: Equip::Null,
            effect: ItemEffect::Null,
            x: 0,
            y: 0,
            properties: p,
        }
    }
}

pub fn rand_hermit_item(x: usize, y: usize) -> Item {
    let mut rng = rand::thread_rng();
    match rng.gen_range(0..8) {
        0 => Item::new_health_potion(x, y),
        1 => Item::new_luck_potion(x, y),
        2 => Item::new_gem_staff(x, y),
        3 => Item::new_shielding_pendant(x, y),
        4 => Item::new_agility_pendant(x, y),
        5 => Item::new_strength_pendant(x, y),
        6 => Item::new_bludgeon_staff(x, y),
        7 => Item::new_antidote(x, y),
        _ => Item::default(),
    }
}

impl Item {
    pub fn new(
        itype: Items,
        sname: String,
        icon: (char, Color),
        desc: String,
        iopts: HashMap<InterOpt, String>,
        equip: bool,
        craft: bool,
        produces: Items,
        equip_type: Equip,
        effect: ItemEffect,
        x: usize,
        y: usize,
        properties: HashMap<String, u16>,
    ) -> Self {
        Self {
            itype,
            sname,
            icon,
            desc,
            iopts,
            equip,
            craft,
            produces,
            equip_type,
            effect,
            x,
            y,
            properties,
        }
    }

    pub fn new_scroll(x: usize, y: usize) -> Self {
        let mut prop = HashMap::new();
        prop.insert(String::from("health"), 3);
        prop.insert(String::from("value"), 2);
        let mut iopts = HashMap::new();
        iopts.insert(InterOpt::Item(ItemOpt::PickUp), String::from("Pick Up"));
        iopts.insert(InterOpt::Item(ItemOpt::Drp), String::from("Drop"));
        iopts.insert(InterOpt::Item(ItemOpt::Use), String::from("Use"));
        Self {
            itype: Items::Scroll,
            sname: "Scroll".to_string(),
            icon: ('S', Color::Yellow),
            desc: "Looks to be an old scroll.".to_string(),
            iopts,
            equip: false,
            craft: false,
            produces: Items::Null,
            equip_type: Equip::Null,
            effect: ItemEffect::Null,
            x,
            y,
            properties: prop,
        }
    }

    pub fn new_edible_root(x: usize, y: usize) -> Self {
        let mut prop = HashMap::new();
        prop.insert(String::from("health"), 3);
        prop.insert(String::from("value"), 2);
        let mut iopts = HashMap::new();
        iopts.insert(InterOpt::Item(ItemOpt::PickUp), String::from("Pick Up"));
        iopts.insert(InterOpt::Item(ItemOpt::Drp), String::from("Drop"));
        iopts.insert(InterOpt::Item(ItemOpt::Use), String::from("Use"));
        Self {
            itype: Items::EdibleRoot,
            sname: "Edible Root".to_string(),
            icon: ('ȝ', Color::Yellow),
            desc: "Weird looking root, doesnt look very tasty.".to_string(),
            iopts,
            equip: false,
            craft: false,
            produces: Items::Null,
            equip_type: Equip::Null,
            effect: ItemEffect::Health,
            x,
            y,
            properties: prop,
        }
    }

    pub fn new_rock(x: usize, y: usize) -> Self {
        let mut prop = HashMap::new();
        prop.insert(String::from("health"), 0);
        prop.insert(String::from("value"), 0);
        let mut iopts = HashMap::new();
        iopts.insert(InterOpt::Item(ItemOpt::PickUp), String::from("Pick Up"));
        iopts.insert(InterOpt::Item(ItemOpt::Drp), String::from("Drop"));
        Self {
            itype: Items::Rock,
            sname: "Rock".to_string(),
            icon: ('o', Color::Yellow),
            desc: "Its a rock.".to_string(),
            iopts,
            equip: false,
            craft: false,
            produces: Items::Null,
            equip_type: Equip::Null,
            effect: ItemEffect::Null,
            x,
            y,
            properties: prop,
        }
    }

    pub fn new_guts(x: usize, y: usize) -> Self {
        let mut prop = HashMap::new();
        prop.insert(String::from("health"), 1);
        prop.insert(String::from("value"), 0);
        let mut iopts = HashMap::new();
        iopts.insert(InterOpt::Item(ItemOpt::PickUp), String::from("Pick Up"));
        iopts.insert(InterOpt::Item(ItemOpt::Drp), String::from("Drop"));
        iopts.insert(InterOpt::Item(ItemOpt::Use), String::from("Use"));
        Self {
            itype: Items::Guts,
            sname: "Guts".to_string(),
            icon: ('ʚ', Color::Yellow),
            desc: "Parts of a dead creature.".to_string(),
            iopts,
            equip: false,
            craft: false,
            produces: Items::Null,
            equip_type: Equip::Null,
            effect: ItemEffect::Health,
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
            icon: ('ϟ', Color::Yellow),
            desc: "Scrap of metal.".to_string(),
            iopts,
            equip: false,
            craft: false,
            produces: Items::Null,
            equip_type: Equip::Null,
            effect: ItemEffect::Null,
            x,
            y,
            properties: prop,
        }
    }

    pub fn new_apple(x: usize, y: usize) -> Self {
        let mut prop = HashMap::new();
        prop.insert(String::from("health"), 5);
        //prop.insert(String::from("effect"), 5);
        prop.insert(String::from("value"), 5);
        let mut iopts = HashMap::new();
        iopts.insert(InterOpt::Item(ItemOpt::PickUp), String::from("Pick Up"));
        iopts.insert(InterOpt::Item(ItemOpt::Drp), String::from("Drop"));
        iopts.insert(InterOpt::Item(ItemOpt::Use), String::from("Use"));

        Self {
            itype: Items::Apple,
            sname: "Apple".to_string(),
            icon: ('ỏ', Color::Yellow),
            desc: "A slightly bruised apple that as been here for a while.".to_string(),
            iopts,
            equip: false,
            craft: false,
            produces: Items::Null,
            equip_type: Equip::Null,
            effect: ItemEffect::Health,
            x,
            y,
            properties: prop,
        }
    }

    pub fn new_health_potion(x: usize, y: usize) -> Self {
        let mut prop = HashMap::new();
        prop.insert(String::from("health"), 30);
        //prop.insert(String::from("effect"), 30);
        prop.insert(String::from("value"), 50);
        let mut iopts = HashMap::new();
        iopts.insert(InterOpt::Item(ItemOpt::PickUp), String::from("Pick Up"));
        iopts.insert(InterOpt::Item(ItemOpt::Drp), String::from("Drop"));
        iopts.insert(InterOpt::Item(ItemOpt::Use), String::from("Use"));
        Self {
            itype: Items::HealthPotion,
            sname: "Health Potion".to_string(),
            icon: ('ṓ', Color::Green),
            desc: "Mixture of curdled liquids. Returns vitality to the body.".to_string(),
            iopts,
            equip: false,
            craft: false,
            produces: Items::Null,
            equip_type: Equip::Null,
            effect: ItemEffect::Health,
            x,
            y,
            properties: prop,
        }
    }

    pub fn new_antidote(x: usize, y: usize) -> Self {
        let mut prop = HashMap::new();
        // prop.insert(String::from("health"), 30);
        //prop.insert(String::from("effect"), 30);
        prop.insert(String::from("value"), 50);
        let mut iopts = HashMap::new();
        iopts.insert(InterOpt::Item(ItemOpt::PickUp), String::from("Pick Up"));
        iopts.insert(InterOpt::Item(ItemOpt::Drp), String::from("Drop"));
        iopts.insert(InterOpt::Item(ItemOpt::Use), String::from("Use"));
        Self {
            itype: Items::Antidote,
            sname: "Antidote".to_string(),
            icon: ('ṓ', Color::Green),
            desc: "A thick purple liquid. Dispels any poison, toxin or infection.".to_string(),
            iopts,
            equip: false,
            craft: false,
            produces: Items::Null,
            equip_type: Equip::Null,
            effect: ItemEffect::Health,
            x,
            y,
            properties: prop,
        }
    }

    pub fn new_luck_potion(x: usize, y: usize) -> Self {
        let mut prop = HashMap::new();
        prop.insert(String::from("luck"), 3);
        //prop.insert(String::from("effect"), 30);
        prop.insert(String::from("value"), 50);
        let mut iopts = HashMap::new();
        iopts.insert(InterOpt::Item(ItemOpt::PickUp), String::from("Pick Up"));
        iopts.insert(InterOpt::Item(ItemOpt::Drp), String::from("Drop"));
        iopts.insert(InterOpt::Item(ItemOpt::Use), String::from("Use"));
        Self {
            itype: Items::LuckPotion,
            sname: "Luck Potion".to_string(),
            icon: ('ṓ', Color::Green),
            desc: "Green potion with gold flecks. Increases the users luck.".to_string(),
            iopts,
            equip: false,
            craft: false,
            produces: Items::Null,
            equip_type: Equip::Null,
            effect: ItemEffect::Luck,
            x,
            y,
            properties: prop,
        }
    }

    pub fn new_agility_potion(x: usize, y: usize) -> Self {
        let mut prop = HashMap::new();
        prop.insert(String::from("Attack"), 5);
        //prop.insert(String::from("effect"), 30);
        prop.insert(String::from("value"), 50);
        let mut iopts = HashMap::new();
        iopts.insert(InterOpt::Item(ItemOpt::PickUp), String::from("Pick Up"));
        iopts.insert(InterOpt::Item(ItemOpt::Drp), String::from("Drop"));
        iopts.insert(InterOpt::Item(ItemOpt::Use), String::from("Use"));
        Self {
            itype: Items::AgilityPotion,
            sname: "Agility Potion".to_string(),
            icon: ('ṓ', Color::Blue),
            desc:
                "A light green liquid with swirls of blue. Increases ability to land attack hits."
                    .to_string(),
            iopts,
            equip: false,
            craft: false,
            produces: Items::Null,
            equip_type: Equip::Null,
            effect: ItemEffect::Attack,
            x,
            y,
            properties: prop,
        }
    }

    pub fn new_salve(x: usize, y: usize) -> Self {
        let mut prop = HashMap::new();
        prop.insert(String::from("health"), 15);
        //prop.insert(String::from("effect"), 15);
        prop.insert(String::from("value"), 30);
        let mut iopts = HashMap::new();
        iopts.insert(InterOpt::Item(ItemOpt::PickUp), String::from("Pick Up"));
        iopts.insert(InterOpt::Item(ItemOpt::Drp), String::from("Drop"));
        iopts.insert(InterOpt::Item(ItemOpt::Use), String::from("Use"));

        Self {
            itype: Items::Salve,
            sname: "Salve".to_string(),
            icon: ('ṓ', Color::Blue),
            desc: "Thick paste for smearing on wounds. It heals better than it smells.".to_string(),
            iopts,
            equip: false,
            craft: false,
            produces: Items::Null,
            equip_type: Equip::Null,
            effect: ItemEffect::Health,
            x,
            y,
            properties: prop,
        }
    }

    pub fn new_dowel(x: usize, y: usize) -> Self {
        let mut prop = HashMap::new();
        prop.insert(String::from("damage"), 5);
        //prop.insert(String::from("effect"), 5);
        prop.insert(String::from("value"), 10);
        let mut iopts = HashMap::new();
        iopts.insert(InterOpt::Item(ItemOpt::PickUp), String::from("Pick Up"));
        iopts.insert(InterOpt::Item(ItemOpt::Drp), String::from("Drop"));
        iopts.insert(InterOpt::Item(ItemOpt::Equip), String::from("Equip"));

        Self {
            itype: Items::Dowel,
            sname: "Dowel".to_string(),
            icon: ('˩', Color::Red),
            desc: "Most of a broomstick. Its sharp at one end.".to_string(),
            iopts,
            equip: true,
            craft: false,
            produces: Items::Null,
            equip_type: Equip::Weapon,
            effect: ItemEffect::Damage,
            x,
            y,
            properties: prop,
        }
    }

    pub fn new_bronze_claymore(x: usize, y: usize) -> Self {
        let mut prop = HashMap::new();
        prop.insert(String::from("damage"), 15);
        prop.insert(String::from("attack"), 15);
        //prop.insert(String::from("effect"), 5);
        prop.insert(String::from("value"), 60);
        let mut iopts = HashMap::new();
        iopts.insert(InterOpt::Item(ItemOpt::PickUp), String::from("Pick Up"));
        iopts.insert(InterOpt::Item(ItemOpt::Drp), String::from("Drop"));
        iopts.insert(InterOpt::Item(ItemOpt::Equip), String::from("Equip"));

        Self {
            itype: Items::BronzeClaymore,
            sname: "Bronze Claymore".to_string(),
            icon: ('Ṫ', Color::Yellow),
            desc: "A bronze double edged sword".to_string(),
            iopts,
            equip: true,
            craft: false,
            produces: Items::Null,
            equip_type: Equip::Weapon,
            effect: ItemEffect::Damage,
            x,
            y,
            properties: prop,
        }
    }

    pub fn new_iron_claymore(x: usize, y: usize) -> Self {
        let mut prop = HashMap::new();
        prop.insert(String::from("damage"), 20);
        prop.insert(String::from("attack"), 20);
        //prop.insert(String::from("effect"), 5);
        prop.insert(String::from("value"), 85);
        let mut iopts = HashMap::new();
        iopts.insert(InterOpt::Item(ItemOpt::PickUp), String::from("Pick Up"));
        iopts.insert(InterOpt::Item(ItemOpt::Drp), String::from("Drop"));
        iopts.insert(InterOpt::Item(ItemOpt::Equip), String::from("Equip"));

        Self {
            itype: Items::IronClaymore,
            sname: "Iron Claymore".to_string(),
            icon: ('Ṫ', Color::Gray),
            desc: "An iron double edged sword".to_string(),
            iopts,
            equip: true,
            craft: false,
            produces: Items::Null,
            equip_type: Equip::Weapon,
            effect: ItemEffect::Damage,
            x,
            y,
            properties: prop,
        }
    }

    pub fn new_steel_claymore(x: usize, y: usize) -> Self {
        let mut prop = HashMap::new();
        prop.insert(String::from("damage"), 25);
        prop.insert(String::from("attack"), 25);
        //prop.insert(String::from("effect"), 5);
        prop.insert(String::from("value"), 110);
        let mut iopts = HashMap::new();
        iopts.insert(InterOpt::Item(ItemOpt::PickUp), String::from("Pick Up"));
        iopts.insert(InterOpt::Item(ItemOpt::Drp), String::from("Drop"));
        iopts.insert(InterOpt::Item(ItemOpt::Equip), String::from("Equip"));

        Self {
            itype: Items::SteelClaymore,
            sname: "Steel Claymore".to_string(),
            icon: ('Ṫ', Color::White),
            desc: "A steel double edged sword".to_string(),
            iopts,
            equip: true,
            craft: false,
            produces: Items::Null,
            equip_type: Equip::Weapon,
            effect: ItemEffect::Damage,
            x,
            y,
            properties: prop,
        }
    }

    pub fn new_bronze_longsword(x: usize, y: usize) -> Self {
        let mut prop = HashMap::new();
        prop.insert(String::from("damage"), 12);
        prop.insert(String::from("attack"), 12);
        //prop.insert(String::from("effect"), 5);
        prop.insert(String::from("value"), 50);
        let mut iopts = HashMap::new();
        iopts.insert(InterOpt::Item(ItemOpt::PickUp), String::from("Pick Up"));
        iopts.insert(InterOpt::Item(ItemOpt::Drp), String::from("Drop"));
        iopts.insert(InterOpt::Item(ItemOpt::Equip), String::from("Equip"));

        Self {
            itype: Items::BronzeLongsword,
            sname: "Bronze Longsword".to_string(),
            icon: ('†', Color::Yellow),
            desc: "A bronze longsword".to_string(),
            iopts,
            equip: true,
            craft: false,
            produces: Items::Null,
            equip_type: Equip::Weapon,
            effect: ItemEffect::Damage,
            x,
            y,
            properties: prop,
        }
    }

    pub fn new_iron_longsword(x: usize, y: usize) -> Self {
        let mut prop = HashMap::new();
        prop.insert(String::from("damage"), 17);
        prop.insert(String::from("attack"), 17);
        //prop.insert(String::from("effect"), 5);
        prop.insert(String::from("value"), 75);
        let mut iopts = HashMap::new();
        iopts.insert(InterOpt::Item(ItemOpt::PickUp), String::from("Pick Up"));
        iopts.insert(InterOpt::Item(ItemOpt::Drp), String::from("Drop"));
        iopts.insert(InterOpt::Item(ItemOpt::Equip), String::from("Equip"));

        Self {
            itype: Items::IronLongsword,
            sname: "Iron Longsword".to_string(),
            icon: ('†', Color::Gray),
            desc: "An iron longsword".to_string(),
            iopts,
            equip: true,
            craft: false,
            produces: Items::Null,
            equip_type: Equip::Weapon,
            effect: ItemEffect::Damage,
            x,
            y,
            properties: prop,
        }
    }

    pub fn new_bronze_greatsword(x: usize, y: usize) -> Self {
        let mut prop = HashMap::new();
        prop.insert(String::from("damage"), 17);
        prop.insert(String::from("attack"), 17);
        //prop.insert(String::from("effect"), 5);
        prop.insert(String::from("value"), 50);
        let mut iopts = HashMap::new();
        iopts.insert(InterOpt::Item(ItemOpt::PickUp), String::from("Pick Up"));
        iopts.insert(InterOpt::Item(ItemOpt::Drp), String::from("Drop"));
        iopts.insert(InterOpt::Item(ItemOpt::Equip), String::from("Equip"));

        Self {
            itype: Items::BronzeGreatsword,
            sname: "Bronze Greatsword".to_string(),
            icon: ('ϯ', Color::Yellow),
            desc: "A bronze greatword".to_string(),
            iopts,
            equip: true,
            craft: false,
            produces: Items::Null,
            equip_type: Equip::Weapon,
            effect: ItemEffect::Damage,
            x,
            y,
            properties: prop,
        }
    }

    pub fn new_bronze_shortsword(x: usize, y: usize) -> Self {
        let mut prop = HashMap::new();
        prop.insert(String::from("damage"), 7);
        prop.insert(String::from("attack"), 7);
        //prop.insert(String::from("effect"), 5);
        prop.insert(String::from("value"), 40);
        let mut iopts = HashMap::new();
        iopts.insert(InterOpt::Item(ItemOpt::PickUp), String::from("Pick Up"));
        iopts.insert(InterOpt::Item(ItemOpt::Drp), String::from("Drop"));
        iopts.insert(InterOpt::Item(ItemOpt::Equip), String::from("Equip"));

        Self {
            itype: Items::BronzeShortsword,
            sname: "Bronze Shortsword".to_string(),
            icon: ('Ϯ', Color::Yellow),
            desc: "A bronze shortsword".to_string(),
            iopts,
            equip: true,
            craft: false,
            produces: Items::Null,
            equip_type: Equip::Weapon,
            effect: ItemEffect::Damage,
            x,
            y,
            properties: prop,
        }
    }

    pub fn new_basic_staff(x: usize, y: usize) -> Self {
        let mut prop = HashMap::new();
        prop.insert(String::from("damage"), 7);
        prop.insert(String::from("attack"), 7);
        //prop.insert(String::from("effect"), 5);
        prop.insert(String::from("value"), 40);
        let mut iopts = HashMap::new();
        iopts.insert(InterOpt::Item(ItemOpt::PickUp), String::from("Pick Up"));
        iopts.insert(InterOpt::Item(ItemOpt::Drp), String::from("Drop"));
        iopts.insert(InterOpt::Item(ItemOpt::Equip), String::from("Equip"));

        Self {
            itype: Items::BasicStaff,
            sname: "Basic Staff".to_string(),
            icon: ('ɭ', Color::Red),
            desc: "A basic wooden staff".to_string(),
            iopts,
            equip: true,
            craft: false,
            produces: Items::Null,
            equip_type: Equip::Weapon,
            effect: ItemEffect::Damage,
            x,
            y,
            properties: prop,
        }
    }

    pub fn new_wood_staff(x: usize, y: usize) -> Self {
        let mut prop = HashMap::new();
        prop.insert(String::from("damage"), 15);
        prop.insert(String::from("attack"), 15);
        //prop.insert(String::from("effect"), 5);
        prop.insert(String::from("value"), 50);
        let mut iopts = HashMap::new();
        iopts.insert(InterOpt::Item(ItemOpt::PickUp), String::from("Pick Up"));
        iopts.insert(InterOpt::Item(ItemOpt::Drp), String::from("Drop"));
        iopts.insert(InterOpt::Item(ItemOpt::Equip), String::from("Equip"));

        Self {
            itype: Items::WoodStaff,
            sname: "Wood Staff".to_string(),
            icon: ('ſ', Color::Red),
            desc: "A solid wood staff sith a knot at the top.".to_string(),
            iopts,
            equip: true,
            craft: false,
            produces: Items::Null,
            equip_type: Equip::Weapon,
            effect: ItemEffect::Damage,
            x,
            y,
            properties: prop,
        }
    }

    pub fn new_bludgeon_staff(x: usize, y: usize) -> Self {
        let mut prop = HashMap::new();
        prop.insert(String::from("damage"), 20);
        prop.insert(String::from("attack"), 20);
        //prop.insert(String::from("effect"), 5);
        prop.insert(String::from("value"), 80);
        let mut iopts = HashMap::new();
        iopts.insert(InterOpt::Item(ItemOpt::PickUp), String::from("Pick Up"));
        iopts.insert(InterOpt::Item(ItemOpt::Drp), String::from("Drop"));
        iopts.insert(InterOpt::Item(ItemOpt::Equip), String::from("Equip"));

        Self {
            itype: Items::BludgeonStaff,
            sname: "Bludgeon Staff".to_string(),
            icon: ('ƪ', Color::Red),
            desc: "A staff that has a knob at the end for hitting.".to_string(),
            iopts,
            equip: true,
            craft: false,
            produces: Items::Null,
            equip_type: Equip::Weapon,
            effect: ItemEffect::Damage,
            x,
            y,
            properties: prop,
        }
    }

    pub fn new_gem_staff(x: usize, y: usize) -> Self {
        let mut prop = HashMap::new();
        prop.insert(String::from("damage"), 25);
        prop.insert(String::from("attack"), 25);
        //prop.insert(String::from("effect"), 5);
        prop.insert(String::from("value"), 120);
        let mut iopts = HashMap::new();
        iopts.insert(InterOpt::Item(ItemOpt::PickUp), String::from("Pick Up"));
        iopts.insert(InterOpt::Item(ItemOpt::Drp), String::from("Drop"));
        iopts.insert(InterOpt::Item(ItemOpt::Equip), String::from("Equip"));

        Self {
            itype: Items::GemStaff,
            sname: "Gem Staff".to_string(),
            icon: ('ẛ', Color::Red),
            desc: "A staff with a gem at the top.".to_string(),
            iopts,
            equip: true,
            craft: false,
            produces: Items::Null,
            equip_type: Equip::Weapon,
            effect: ItemEffect::Damage,
            x,
            y,
            properties: prop,
        }
    }

    pub fn new_bronze_heavy_axe(x: usize, y: usize) -> Self {
        let mut prop = HashMap::new();
        prop.insert(String::from("damage"), 15);
        prop.insert(String::from("attack"), 15);
        //prop.insert(String::from("effect"), 5);
        prop.insert(String::from("value"), 60);
        let mut iopts = HashMap::new();
        iopts.insert(InterOpt::Item(ItemOpt::PickUp), String::from("Pick Up"));
        iopts.insert(InterOpt::Item(ItemOpt::Drp), String::from("Drop"));
        iopts.insert(InterOpt::Item(ItemOpt::Equip), String::from("Equip"));

        Self {
            itype: Items::BronzeHeavyAxe,
            sname: "Bronze Heavy Axe".to_string(),
            icon: ('Ͳ', Color::Yellow),
            desc: "A bronze heavy axe".to_string(),
            iopts,
            equip: true,
            craft: false,
            produces: Items::Null,
            equip_type: Equip::Weapon,
            effect: ItemEffect::Damage,
            x,
            y,
            properties: prop,
        }
    }

    pub fn new_bronze_light_axe(x: usize, y: usize) -> Self {
        let mut prop = HashMap::new();
        prop.insert(String::from("damage"), 15);
        prop.insert(String::from("attack"), 15);
        //prop.insert(String::from("effect"), 5);
        prop.insert(String::from("value"), 50);
        let mut iopts = HashMap::new();
        iopts.insert(InterOpt::Item(ItemOpt::PickUp), String::from("Pick Up"));
        iopts.insert(InterOpt::Item(ItemOpt::Drp), String::from("Drop"));
        iopts.insert(InterOpt::Item(ItemOpt::Equip), String::from("Equip"));

        Self {
            itype: Items::BronzeLightAxe,
            sname: "Bronze Light Axe".to_string(),
            icon: ('ͳ', Color::Yellow),
            desc: "A bronze light axe".to_string(),
            iopts,
            equip: true,
            craft: false,
            produces: Items::Null,
            equip_type: Equip::Weapon,
            effect: ItemEffect::Damage,
            x,
            y,
            properties: prop,
        }
    }

    pub fn new_bronze_pick_axe(x: usize, y: usize) -> Self {
        let mut prop = HashMap::new();
        prop.insert(String::from("damage"), 15);
        prop.insert(String::from("attack"), 15);
        //prop.insert(String::from("effect"), 5);
        prop.insert(String::from("value"), 55);
        let mut iopts = HashMap::new();
        iopts.insert(InterOpt::Item(ItemOpt::PickUp), String::from("Pick Up"));
        iopts.insert(InterOpt::Item(ItemOpt::Drp), String::from("Drop"));
        iopts.insert(InterOpt::Item(ItemOpt::Equip), String::from("Equip"));

        Self {
            itype: Items::BronzePickAxe,
            sname: "Bronze PickAxe".to_string(),
            icon: ('ፐ', Color::Yellow),
            desc: "A bronze pick axe".to_string(),
            iopts,
            equip: true,
            craft: false,
            produces: Items::Null,
            equip_type: Equip::Weapon,
            effect: ItemEffect::Damage,
            x,
            y,
            properties: prop,
        }
    }

    pub fn new_bronze_pick_hammer(x: usize, y: usize) -> Self {
        let mut prop = HashMap::new();
        prop.insert(String::from("damage"), 17);
        prop.insert(String::from("attack"), 17);
        //prop.insert(String::from("effect"), 5);
        prop.insert(String::from("value"), 55);
        let mut iopts = HashMap::new();
        iopts.insert(InterOpt::Item(ItemOpt::PickUp), String::from("Pick Up"));
        iopts.insert(InterOpt::Item(ItemOpt::Drp), String::from("Drop"));
        iopts.insert(InterOpt::Item(ItemOpt::Equip), String::from("Equip"));

        Self {
            itype: Items::BronzePickHammer,
            sname: "Bronze Pick Hammer".to_string(),
            icon: ('Ƭ', Color::Yellow),
            desc: "A bronze pick hammer".to_string(),
            iopts,
            equip: true,
            craft: false,
            produces: Items::Null,
            equip_type: Equip::Weapon,
            effect: ItemEffect::Damage,
            x,
            y,
            properties: prop,
        }
    }

    pub fn new_bronze_shadow_axe(x: usize, y: usize) -> Self {
        let mut prop = HashMap::new();
        prop.insert(String::from("damage"), 30);
        prop.insert(String::from("attack"), 30);
        //prop.insert(String::from("effect"), 5);
        prop.insert(String::from("value"), 200);
        let mut iopts = HashMap::new();
        iopts.insert(InterOpt::Item(ItemOpt::PickUp), String::from("Pick Up"));
        iopts.insert(InterOpt::Item(ItemOpt::Drp), String::from("Drop"));
        iopts.insert(InterOpt::Item(ItemOpt::Equip), String::from("Equip"));

        Self {
            itype: Items::ShadowAxe,
            sname: "Shadow Axe".to_string(),
            icon: ('ፕ', Color::Yellow),
            desc: "Shadow Axe".to_string(),
            iopts,
            equip: true,
            craft: false,
            produces: Items::Null,
            equip_type: Equip::Weapon,
            effect: ItemEffect::Damage,
            x,
            y,
            properties: prop,
        }
    }

    pub fn new_bronze_war_axe(x: usize, y: usize) -> Self {
        let mut prop = HashMap::new();
        prop.insert(String::from("damage"), 50);
        prop.insert(String::from("attack"), 50);
        //prop.insert(String::from("effect"), 5);
        prop.insert(String::from("value"), 300);
        let mut iopts = HashMap::new();
        iopts.insert(InterOpt::Item(ItemOpt::PickUp), String::from("Pick Up"));
        iopts.insert(InterOpt::Item(ItemOpt::Drp), String::from("Drop"));
        iopts.insert(InterOpt::Item(ItemOpt::Equip), String::from("Equip"));

        Self {
            itype: Items::BronzeWarAxe,
            sname: "Bronze War Axe".to_string(),
            icon: ('ቸ', Color::Yellow),
            desc: "A bronze war axe".to_string(),
            iopts,
            equip: true,
            craft: false,
            produces: Items::Null,
            equip_type: Equip::Weapon,
            effect: ItemEffect::Damage,
            x,
            y,
            properties: prop,
        }
    }

    //BronzeLongsword BronzeLightAxe Salve Salve Dowel WoodenBoard BronzePickHammer BronzeShortsword Apple|Apple

    pub fn new_small_wood_shield(x: usize, y: usize) -> Self {
        let mut prop = HashMap::new();
        prop.insert(String::from("defence"), 5);
        //prop.insert(String::from("effect"), 5);
        prop.insert(String::from("value"), 30);
        let mut iopts = HashMap::new();
        iopts.insert(InterOpt::Item(ItemOpt::PickUp), String::from("Pick Up"));
        iopts.insert(InterOpt::Item(ItemOpt::Drp), String::from("Drop"));
        iopts.insert(InterOpt::Item(ItemOpt::Equip), String::from("Equip"));

        Self {
            itype: Items::SmallWoodShield,
            sname: "Small Wood Shield".to_string(),
            icon: ('ѳ', Color::Red),
            desc: "A small wooden shield that provides some defence.".to_string(),
            iopts,
            equip: true,
            craft: false,
            produces: Items::Null,
            equip_type: Equip::Shield,
            effect: ItemEffect::Defence,
            x,
            y,
            properties: prop,
        }
    }

    pub fn new_large_wood_shield(x: usize, y: usize) -> Self {
        let mut prop = HashMap::new();
        prop.insert(String::from("defence"), 10);
        //prop.insert(String::from("effect"), 5);
        prop.insert(String::from("value"), 50);
        let mut iopts = HashMap::new();
        iopts.insert(InterOpt::Item(ItemOpt::PickUp), String::from("Pick Up"));
        iopts.insert(InterOpt::Item(ItemOpt::Drp), String::from("Drop"));
        iopts.insert(InterOpt::Item(ItemOpt::Equip), String::from("Equip"));

        Self {
            itype: Items::LargeWoodShield,
            sname: "Large Wood Shield".to_string(),
            icon: ('θ', Color::Red),
            desc: "A large wooden shield that protects a bit from attacks.".to_string(),
            iopts,
            equip: true,
            craft: false,
            produces: Items::Null,
            equip_type: Equip::Shield,
            effect: ItemEffect::Defence,
            x,
            y,
            properties: prop,
        }
    }

    pub fn new_iron_shield(x: usize, y: usize) -> Self {
        let mut prop = HashMap::new();
        prop.insert(String::from("defence"), 15);
        //prop.insert(String::from("effect"), 5);
        prop.insert(String::from("value"), 70);
        let mut iopts = HashMap::new();
        iopts.insert(InterOpt::Item(ItemOpt::PickUp), String::from("Pick Up"));
        iopts.insert(InterOpt::Item(ItemOpt::Drp), String::from("Drop"));
        iopts.insert(InterOpt::Item(ItemOpt::Equip), String::from("Equip"));

        Self {
            itype: Items::IronShield,
            sname: "Iron Shield".to_string(),
            icon: ('Θ', Color::Red),
            desc: "A iron shield that protects from attacks.".to_string(),
            iopts,
            equip: true,
            craft: false,
            produces: Items::Null,
            equip_type: Equip::Shield,
            effect: ItemEffect::Defence,
            x,
            y,
            properties: prop,
        }
    }

    pub fn new_steel_shield(x: usize, y: usize) -> Self {
        let mut prop = HashMap::new();
        prop.insert(String::from("defence"), 20);
        //prop.insert(String::from("effect"), 5);
        prop.insert(String::from("value"), 90);
        let mut iopts = HashMap::new();
        iopts.insert(InterOpt::Item(ItemOpt::PickUp), String::from("Pick Up"));
        iopts.insert(InterOpt::Item(ItemOpt::Drp), String::from("Drop"));
        iopts.insert(InterOpt::Item(ItemOpt::Equip), String::from("Equip"));

        Self {
            itype: Items::SteelShield,
            sname: "Steel Shield".to_string(),
            icon: ('ʘ', Color::Red),
            desc: "A steel shield that protects from attacks.".to_string(),
            iopts,
            equip: true,
            craft: false,
            produces: Items::Null,
            equip_type: Equip::Shield,
            effect: ItemEffect::Defence,
            x,
            y,
            properties: prop,
        }
    }

    pub fn new_light_armour(x: usize, y: usize) -> Self {
        let mut prop = HashMap::new();
        prop.insert(String::from("defence"), 10);
        //prop.insert(String::from("effect"), 5);
        prop.insert(String::from("value"), 30);
        let mut iopts = HashMap::new();
        iopts.insert(InterOpt::Item(ItemOpt::PickUp), String::from("Pick Up"));
        iopts.insert(InterOpt::Item(ItemOpt::Drp), String::from("Drop"));
        iopts.insert(InterOpt::Item(ItemOpt::Equip), String::from("Equip"));

        Self {
            itype: Items::LightArmour,
            sname: "Light Armour".to_string(),
            icon: ('ዣ', Color::Gray),
            desc: "Light armour that provides some defence.".to_string(),
            iopts,
            equip: true,
            craft: false,
            produces: Items::Null,
            equip_type: Equip::Armour,
            effect: ItemEffect::Defence,
            x,
            y,
            properties: prop,
        }
    }

    pub fn new_medium_armour(x: usize, y: usize) -> Self {
        let mut prop = HashMap::new();
        prop.insert(String::from("defence"), 20);
        //prop.insert(String::from("effect"), 5);
        prop.insert(String::from("value"), 60);
        let mut iopts = HashMap::new();
        iopts.insert(InterOpt::Item(ItemOpt::PickUp), String::from("Pick Up"));
        iopts.insert(InterOpt::Item(ItemOpt::Drp), String::from("Drop"));
        iopts.insert(InterOpt::Item(ItemOpt::Equip), String::from("Equip"));

        Self {
            itype: Items::MediumArmour,
            sname: "Medium Armour".to_string(),
            icon: ('ዠ', Color::Gray),
            desc: "Medium armour that provides reasonable defence.".to_string(),
            iopts,
            equip: true,
            craft: false,
            produces: Items::Null,
            equip_type: Equip::Armour,
            effect: ItemEffect::Defence,
            x,
            y,
            properties: prop,
        }
    }

    pub fn new_heavy_armour(x: usize, y: usize) -> Self {
        let mut prop = HashMap::new();
        prop.insert(String::from("defence"), 30);
        //prop.insert(String::from("effect"), 5);
        prop.insert(String::from("value"), 90);
        let mut iopts = HashMap::new();
        iopts.insert(InterOpt::Item(ItemOpt::PickUp), String::from("Pick Up"));
        iopts.insert(InterOpt::Item(ItemOpt::Drp), String::from("Drop"));
        iopts.insert(InterOpt::Item(ItemOpt::Equip), String::from("Equip"));

        Self {
            itype: Items::HeavyArmour,
            sname: "Heavy Armour".to_string(),
            icon: ('ዥ', Color::Gray),
            desc: "Heavy armour that provides good defence.".to_string(),
            iopts,
            equip: true,
            craft: false,
            produces: Items::Null,
            equip_type: Equip::Armour,
            effect: ItemEffect::Defence,
            x,
            y,
            properties: prop,
        }
    }

    // wearable
    pub fn new_shielding_pendant(x: usize, y: usize) -> Self {
        let mut prop = HashMap::new();
        prop.insert(String::from("defence"), 30);
        //prop.insert(String::from("effect"), 5);
        prop.insert(String::from("value"), 30);
        let mut iopts = HashMap::new();
        iopts.insert(InterOpt::Item(ItemOpt::PickUp), String::from("Pick Up"));
        iopts.insert(InterOpt::Item(ItemOpt::Drp), String::from("Drop"));
        iopts.insert(InterOpt::Item(ItemOpt::Equip), String::from("Equip"));

        Self {
            itype: Items::ShieldingPendant,
            sname: "Shielding Pendant".to_string(),
            icon: ('ȣ', Color::LightYellow),
            desc: "A pendant that emits a protective aura around the user.".to_string(),
            iopts,
            equip: true,
            craft: false,
            produces: Items::Null,
            equip_type: Equip::Wearing,
            effect: ItemEffect::Defence,
            x,
            y,
            properties: prop,
        }
    }

    pub fn new_agility_pendant(x: usize, y: usize) -> Self {
        let mut prop = HashMap::new();
        prop.insert(String::from("attack"), 30);
        //prop.insert(String::from("effect"), 5);
        prop.insert(String::from("value"), 30);
        let mut iopts = HashMap::new();
        iopts.insert(InterOpt::Item(ItemOpt::PickUp), String::from("Pick Up"));
        iopts.insert(InterOpt::Item(ItemOpt::Drp), String::from("Drop"));
        iopts.insert(InterOpt::Item(ItemOpt::Equip), String::from("Equip"));

        Self {
            itype: Items::AgilityPendant,
            sname: "Agility Pendant".to_string(),
            icon: ('ȣ', Color::LightYellow),
            desc: "A pendant that gives the user faster attacks.".to_string(),
            iopts,
            equip: true,
            craft: false,
            produces: Items::Null,
            equip_type: Equip::Wearing,
            effect: ItemEffect::Attack,
            x,
            y,
            properties: prop,
        }
    }

    pub fn new_strength_pendant(x: usize, y: usize) -> Self {
        let mut prop = HashMap::new();
        prop.insert(String::from("damage"), 30);
        //prop.insert(String::from("effect"), 5);
        prop.insert(String::from("value"), 30);
        let mut iopts = HashMap::new();
        iopts.insert(InterOpt::Item(ItemOpt::PickUp), String::from("Pick Up"));
        iopts.insert(InterOpt::Item(ItemOpt::Drp), String::from("Drop"));
        iopts.insert(InterOpt::Item(ItemOpt::Equip), String::from("Equip"));

        Self {
            itype: Items::StrengthPendant,
            sname: "Strength Pendant".to_string(),
            icon: ('ȣ', Color::LightYellow),
            desc: "A pendant that gives the user stronger attacks.".to_string(),
            iopts,
            equip: true,
            craft: false,
            produces: Items::Null,
            equip_type: Equip::Wearing,
            effect: ItemEffect::Damage,
            x,
            y,
            properties: prop,
        }
    }

    ////////////////////////// Plants

    pub fn new_luminous_mushroom(x: usize, y: usize) -> Self {
        let mut prop = HashMap::new();
        prop.insert(String::from("required"), 5);
        prop.insert(String::from("value"), 8);
        let mut iopts = HashMap::new();
        iopts.insert(InterOpt::Item(ItemOpt::PickUp), String::from("Pick Up"));
        iopts.insert(InterOpt::Item(ItemOpt::Drp), String::from("Drop"));

        Self {
            itype: Items::Plants(Plants::LuminousMushroom),
            sname: "Luminous Mushroom".to_string(),
            icon: ('ϙ', Color::LightBlue),
            desc: "A mushroom that gives off a light glow.".to_string(),
            iopts,
            equip: false,
            craft: true,
            produces: Items::VitalityPotion,
            equip_type: Equip::Null,
            effect: ItemEffect::Null,
            x,
            y,
            properties: prop,
        }
    }

    pub fn new_violet_shadow(x: usize, y: usize) -> Self {
        let mut prop = HashMap::new();
        prop.insert(String::from("required"), 5);
        prop.insert(String::from("value"), 8);
        let mut iopts = HashMap::new();
        iopts.insert(InterOpt::Item(ItemOpt::PickUp), String::from("Pick Up"));
        iopts.insert(InterOpt::Item(ItemOpt::Drp), String::from("Drop"));

        Self {
            itype: Items::Plants(Plants::VioletShadow),
            sname: "Violet Shadow".to_string(),
            icon: ('⚵', Color::LightMagenta),
            desc: "A wispy purple flower that grows in dark corners.".to_string(),
            iopts,
            equip: false,
            craft: true,
            produces: Items::Null,
            equip_type: Equip::Null,
            effect: ItemEffect::Null,
            x,
            y,
            properties: prop,
        }
    }

    // pub fn new_plant(x: usize, y: usize) -> Self {
    //     let mut prop = HashMap::new();
    //     prop.insert(String::from("required"), 5);
    //     prop.insert(String::from("value"), 8);
    //     let mut iopts = HashMap::new();
    //     iopts.insert(InterOpt::Item(ItemOpt::PickUp), String::from("Pick Up"));
    //     iopts.insert(InterOpt::Item(ItemOpt::Drp), String::from("Drop"));

    //     Self {
    //         itype: Items::Plants(Plants::VioletShadow),
    //         sname: "Violet Shadow".to_string(),
    //         icon: ('ϙ', Color::LightBlue),
    //         desc: "A wispy purple flower that grows in dark corners.".to_string(),
    //         iopts,
    //         equip: false,
    //         craft: true,
    //         produces: Items::Null,
    //         equip_type: Equip::Null,
    //         effect: ItemEffect::Null,
    //         x,
    //         y,
    //         properties: prop,
    //     }
    // }

    pub fn new_lampen_flower(x: usize, y: usize) -> Self {
        let mut prop = HashMap::new();
        prop.insert(String::from("required"), 5);
        prop.insert(String::from("value"), 8);
        let mut iopts = HashMap::new();
        iopts.insert(InterOpt::Item(ItemOpt::PickUp), String::from("Pick Up"));
        iopts.insert(InterOpt::Item(ItemOpt::Drp), String::from("Drop"));

        Self {
            itype: Items::Plants(Plants::LampenFlower),
            sname: "Lampen Flower".to_string(),
            icon: ('⚵', Color::LightCyan),
            // icon: ('ϙ⚵', Color::LightBlue),
            desc: "A flower that grows in small groups.".to_string(),
            iopts,
            equip: false,
            craft: true,
            produces: Items::HealthPotion,
            equip_type: Equip::Null,
            effect: ItemEffect::Null,
            x,
            y,
            properties: prop,
        }
    }

    pub fn new_moss(x: usize, y: usize) -> Self {
        let mut prop = HashMap::new();
        prop.insert(String::from("required"), 10);
        prop.insert(String::from("value"), 5);
        let mut iopts = HashMap::new();
        iopts.insert(InterOpt::Item(ItemOpt::PickUp), String::from("Pick Up"));
        iopts.insert(InterOpt::Item(ItemOpt::Drp), String::from("Drop"));

        Self {
            itype: Items::Plants(Plants::Moss),
            sname: "Moss".to_string(),
            icon: ('⁂', Color::Green),
            desc: "Soft green clump of moss.".to_string(),
            iopts,
            equip: false,
            craft: true,
            produces: Items::Salve,
            equip_type: Equip::Null,
            effect: ItemEffect::Null,
            x,
            y,
            properties: prop,
        }
    }

    pub fn new_lichenous_growth(x: usize, y: usize) -> Self {
        let mut prop = HashMap::new();
        prop.insert(String::from("required"), 8);
        prop.insert(String::from("value"), 8);
        let mut iopts = HashMap::new();
        iopts.insert(InterOpt::Item(ItemOpt::PickUp), String::from("Pick Up"));
        iopts.insert(InterOpt::Item(ItemOpt::Drp), String::from("Drop"));

        Self {
            itype: Items::Plants(Plants::LichenousGrowth),
            sname: "Lichenous Growth".to_string(),
            icon: ('ც', Color::LightMagenta),
            // icon: ('₴', Color::LightBlue),
            desc: "A spongey growth that grows in the dark.".to_string(),
            iopts,
            equip: false,
            craft: true,
            produces: Items::Antidote,
            equip_type: Equip::Null,
            effect: ItemEffect::Null,
            x,
            y,
            properties: prop,
        }
    }

    pub fn new_lucky_clover(x: usize, y: usize) -> Self {
        let mut prop = HashMap::new();
        prop.insert(String::from("required"), 10);
        prop.insert(String::from("value"), 20);
        let mut iopts = HashMap::new();
        iopts.insert(InterOpt::Item(ItemOpt::PickUp), String::from("Pick Up"));
        iopts.insert(InterOpt::Item(ItemOpt::Drp), String::from("Drop"));

        Self {
            itype: Items::Plants(Plants::LuckyClover),
            sname: "Lucky Clover".to_string(),
            icon: ('⌘', Color::Green),
            desc: "A small clover with four pedals.".to_string(),
            iopts,
            equip: false,
            craft: true,
            produces: Items::LuckPotion,
            equip_type: Equip::Null,
            effect: ItemEffect::Null,
            x,
            y,
            properties: prop,
        }
    }

    pub fn new_shroom(x: usize, y: usize) -> Self {
        let mut prop = HashMap::new();
        prop.insert(String::from("required"), 5);
        prop.insert(String::from("value"), 30);
        let mut iopts = HashMap::new();
        iopts.insert(InterOpt::Item(ItemOpt::PickUp), String::from("Pick Up"));
        iopts.insert(InterOpt::Item(ItemOpt::Drp), String::from("Drop"));

        Self {
            itype: Items::Plants(Plants::Shroom),
            sname: "Shroom".to_string(),
            icon: ('ϙ', Color::Magenta),
            // icon: ('ƍ', Color::Magenta),
            desc: "A weird looking mushroom.".to_string(),
            iopts,
            equip: false,
            craft: false,
            produces: Items::Null,
            equip_type: Equip::Null,
            effect: ItemEffect::Null,
            x,
            y,
            properties: prop,
        }
    }

    pub fn new_vine_bulb(x: usize, y: usize) -> Self {
        let mut prop = HashMap::new();
        prop.insert(String::from("required"), 5);
        prop.insert(String::from("value"), 10);
        let mut iopts = HashMap::new();
        iopts.insert(InterOpt::Item(ItemOpt::PickUp), String::from("Pick Up"));
        iopts.insert(InterOpt::Item(ItemOpt::Drp), String::from("Drop"));

        Self {
            itype: Items::Plants(Plants::VineBulb),
            sname: "Vine Bulb".to_string(),
            icon: ('౸', Color::LightRed),
            desc: "The flower bulb of a vine that covers the walls in patches.".to_string(),
            iopts,
            equip: false,
            craft: true,
            produces: Items::AgilityPotion,
            equip_type: Equip::Null,
            effect: ItemEffect::Null,
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

    pub fn is_equip(&mut self) -> bool {
        self.equip.clone()
    }

    pub fn get_equip_type(&mut self) -> Equip {
        self.equip_type.clone()
    }

    pub fn get_effect(&mut self) -> ItemEffect {
        self.effect.clone()
    }

    pub fn get_desc(&mut self) -> String {
        self.desc.clone()
    }

    pub fn get_iopts(&mut self) -> HashMap<InterOpt, String> {
        self.iopts.clone()
    }
}
