//shop.rs
use crate::enums::{Shops, NPCWrap};
use crate::npc::{ShopNPC};
use crate::item::{Item};

#[derive(Clone, Debug, PartialEq)]
pub struct Shop {
    sptype: Shops,
    sname: String,
    npc: NPCWrap,
    stock: Vec<Item>,
}

impl Shop {
    pub fn new_item_shop(sname: String, npc: NPCWrap, stock: Vec<Item>) -> Self {
        Self {
            sptype: Shops::Item,
            sname: sname,
            npc: NPCWrap::Null,
            stock: stock,
        }
    }
}
