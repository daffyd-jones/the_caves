//shop.rs
use crate::enums::{Shops, NPCWrap};
use crate::npc::{ShopNPC};
use crate::item::{Item};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct Shop {
    sptype: Shops,
    sname: String,
    npc: NPCWrap,
    stock: HashMap<(usize, usize), Item>,
}

impl Shop {
    pub fn new_item_shop(sname: String, npc: NPCWrap, stock: HashMap<(usize, usize), Item>) -> Self {
        Self {
            sptype: Shops::Item,
            sname: sname,
            npc: NPCWrap::Null,
            stock: stock,
        }
    }

    pub fn get_stock(&self) -> HashMap<(usize, usize), Item> {
        self.stock.clone()
    }
}
