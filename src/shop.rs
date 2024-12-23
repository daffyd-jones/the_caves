//shop.rs
use crate::enums::{Shops, NPCWrap};
use crate::npc::{ShopNPC};
use crate::item::{Item};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
//#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[derive(Clone, Debug, PartialEq)]
pub struct Shop {
    sptype: Shops,
    sname: String,
    npc: NPCWrap,
    stock: HashMap<(usize, usize), Item>,
    paid: bool,
}

impl Default for Shop {
    fn default() -> Self {
        let npc = ShopNPC::default();
        let swrap = NPCWrap::ShopNPC(npc);
        let stock = HashMap::new();
        Self {
            sptype: Shops::Null,
            sname: "Spoof".to_string(),
            npc: swrap,
            stock: stock,
            paid: true,
        }
    }
}



impl Shop {
    pub fn new_item_shop(sname: String, npc: NPCWrap, stock: HashMap<(usize, usize), Item>) -> Self {
        Self {
            sptype: Shops::Item,
            sname: sname,
            npc: npc,
            stock: stock,
            paid: true,
        }
    }

    pub fn new_guild(sname: String, npc: NPCWrap, stock: HashMap<(usize, usize), Item>) -> Self {
        Self {
            sptype: Shops::Guild,
            sname: sname,
            npc: npc,
            stock: stock,
            paid: true,
        }
    }

    pub fn new_church(sname: String, npc: NPCWrap, stock: HashMap<(usize, usize), Item>) -> Self {
        Self {
            sptype: Shops::Church,
            sname: sname,
            npc: npc,
            stock: stock,
            paid: true,
        }
    }

    pub fn get_sptype(&mut self) -> Shops {
        self.sptype.clone()
    }

    pub fn get_stock(&self) -> HashMap<(usize, usize), Item> {
        self.stock.clone()
    }

    pub fn get_npc(&self) -> NPCWrap {
        self.npc.clone()
    }

    pub fn remove_item(&mut self, pos: (usize, usize)) {
        log::info!("rem item pos\n{:?}", pos.clone());
        log::info!("pre rem stock\n{:?}", self.stock.clone());
        let rem = self.stock.remove(&pos);
        log::info!("shop item rem\n{:?}", rem.clone());
    }

    pub fn set_paid(&mut self, paid: bool) {
        self.paid = paid;
    }

    pub fn get_paid(&mut self) -> bool {
        self.paid.clone()
    }
}
