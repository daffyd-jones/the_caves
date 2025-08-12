//shop.rs
use crate::enums::{NPCWrap, ShopItem, Shops};
use crate::item::Item;
use crate::npc::ShopNPC;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct Shop {
    pub sptype: Shops,
    pub sname: String,
    pub npc: ShopNPC,
    pub stock: HashMap<(usize, usize), ShopItem>,
    pub paid: bool,
}

impl Default for Shop {
    fn default() -> Self {
        let npc = ShopNPC::default();
        Self {
            sptype: Shops::Null,
            sname: "Spoof".to_string(),
            npc,
            stock: HashMap::new(),
            paid: true,
        }
    }
}

impl Shop {
    pub fn new_item_shop(
        sname: String,
        stock: HashMap<(usize, usize), ShopItem>,
        npc: ShopNPC,
    ) -> Self {
        Self {
            sptype: Shops::Item,
            sname,
            npc,
            stock,
            paid: true,
        }
    }

    pub fn new_guild(
        sname: String,
        stock: HashMap<(usize, usize), ShopItem>,
        npc: ShopNPC,
    ) -> Self {
        Self {
            sptype: Shops::Guild,
            sname,
            npc,
            stock,
            paid: true,
        }
    }

    pub fn new_church(
        sname: String,
        stock: HashMap<(usize, usize), ShopItem>,
        npc: ShopNPC,
    ) -> Self {
        Self {
            sptype: Shops::Church,
            sname,
            npc,
            stock,
            paid: true,
        }
    }

    pub fn new_clinic(
        sname: String,
        stock: HashMap<(usize, usize), ShopItem>,
        npc: ShopNPC,
    ) -> Self {
        Self {
            sptype: Shops::Clinic,
            sname,
            npc,
            stock,
            paid: true,
        }
    }

    pub fn new_herbalist(
        sname: String,
        stock: HashMap<(usize, usize), ShopItem>,
        npc: ShopNPC,
    ) -> Self {
        Self {
            sptype: Shops::Herbalist,
            sname,
            npc,
            stock,
            paid: true,
        }
    }

    pub fn get_sptype(&mut self) -> Shops {
        self.sptype.clone()
    }

    pub fn get_npc(&self) -> ShopNPC {
        self.npc.clone()
    }

    // pub fn remove_item(&mut self, pos: (usize, usize)) {
    //     log::info!("rem item pos\n{:?}", pos.clone());
    //     log::info!("pre rem stock\n{:?}", self.stock.clone());
    //     let rem = self.stock.remove(&pos);
    //     log::info!("shop item rem\n{:?}", rem.clone());
    // }

    pub fn set_paid(&mut self, paid: bool) {
        self.paid = paid;
    }

    pub fn get_paid(&mut self) -> bool {
        self.paid.clone()
    }
}
