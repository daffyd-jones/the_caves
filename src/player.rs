//player
// mod enums;
use crate::enums::{EncOpt, EnvInter, Equip, ItemEffect, PuzzlePiece};
use crate::item::Item;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const INVENTORY_MAX: usize = 75;

#[derive(Clone, Debug)]
//#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Player {
    pub x: usize,
    pub y: usize,
    pub health: u16,
    pub inventory: Vec<Item>,
    pub equipped: HashMap<Equip, Item>,
    pub attack: u16,
    pub defence: u16,
    pub damage: u16,
    pub money: u16,
    dodge: bool,
    enc_last_turn: (EncOpt, u16),
    enc_opt: HashMap<EncOpt, String>,
    pub puzzle_pieces: Vec<PuzzlePiece>,
}

impl Player {
    pub fn new(x: usize, y: usize) -> Self {
        let inventory: Vec<Item> = Vec::new();
        let puzzle_pieces: Vec<PuzzlePiece> = Vec::new();
        let equipped: HashMap<Equip, Item> = HashMap::new();
        let mut enc_opt = HashMap::new();
        enc_opt.insert(EncOpt::Attack, "Attack".to_string());
        enc_opt.insert(EncOpt::UseItem, "Use item".to_string());
        enc_opt.insert(EncOpt::Dodge, "Dodge".to_string());
        Self {
            x,
            y,
            health: 100,
            inventory,
            equipped,
            attack: 20,
            defence: 10,
            damage: 10,
            money: 100,
            dodge: false,
            enc_last_turn: (EncOpt::Null, 0),
            enc_opt,
            puzzle_pieces,
        }
    }

    pub fn get_pos(self) -> (usize, usize) {
        (self.x, self.y)
    }

    pub fn set_pos(&mut self, pos: (usize, usize)) {
        self.x = pos.0;
        self.y = pos.1;
    }

    pub fn set_enc_last_turn(&mut self, turn: (EncOpt, u16)) {
        self.enc_last_turn = turn;
    }

    pub fn get_last_turn(&mut self) -> (EncOpt, u16) {
        self.enc_last_turn
    }

    pub fn get_enc_turn(&mut self) -> (u16, u16) {
        let mut rng = rand::thread_rng();
        let attack = rng.gen_range((self.attack / 2)..self.attack);
        let damage = rng.gen_range((self.damage / 2)..self.damage);
        let attack_added = {
            let mut att_acc = attack;
            for (_k, v) in &self.equipped {
                if let Some(val) = v.properties.get("attack") {
                    att_acc += val;
                };
            }
            att_acc
        };
        let damage_added = {
            let mut dam_acc = damage;
            for (_k, v) in &self.equipped {
                if let Some(val) = v.properties.get("damage") {
                    dam_acc += val;
                };
            }
            dam_acc
        };
        (attack_added, damage_added)
    }

    pub fn toggle_dodge(&mut self) {
        self.dodge = !self.dodge;
    }

    pub fn get_dodge(&mut self) -> bool {
        self.dodge
    }

    pub fn get_enc_opt(&mut self) -> HashMap<EncOpt, String> {
        self.enc_opt.clone()
    }

    pub fn apply_attack(&mut self, amt: u16) {
        if self.health > amt {
            self.health -= amt;
        } else {
            self.health = 0;
        }
    }

    pub fn get_defence(&mut self) -> u16 {
        let mut def_acc = self.defence;
        for (_k, v) in &self.equipped {
            if let Some(val) = v.properties.get("defence") {
                def_acc += val;
            };
        }
        def_acc
    }

    pub fn get_health(&mut self) -> u16 {
        self.health
    }

    pub fn heal_player(&mut self) {
        self.health = 100;
    }

    pub fn add_to_inv(&mut self, item: Item) -> bool {
        if item.effect == ItemEffect::Gold {
            return self.inc_money(item.properties["value"]);
        }
        if self.inventory.len() < 75 {
            self.inventory.push(item);
            return true;
        }
        false
    }

    pub fn get_inventory(&mut self) -> Vec<Item> {
        self.inventory.clone()
    }

    pub fn get_equipped(&self) -> HashMap<Equip, Item> {
        self.equipped.clone()
    }

    pub fn rem_inv_item(&mut self, idx: usize) {
        self.inventory.remove(idx);
    }

    pub fn inv_full(&mut self) -> bool {
        if self.inventory.len() < INVENTORY_MAX {
            return false;
        }
        true
    }

    pub fn add_equip(&mut self, mut item: Item) -> Option<Item> {
        let etype = item.get_equip_type();
        self.equipped.insert(etype, item)
    }

    pub fn apply_item_effect(&mut self, mut item: Item) {
        let prop = item.get_properties();
        let effect = item.get_effect();
        match effect {
            ItemEffect::Health => {
                let amt = prop.get("health").unwrap();
                if self.health + amt > 100 {
                    self.health = 100;
                } else {
                    self.health += amt;
                }
            }
            _ => todo!(),
        }
    }

    pub fn pos(&mut self) -> (usize, usize) {
        (self.x, self.y)
    }

    pub fn move_up(&mut self) {
        self.y -= 1;
    }

    pub fn move_down(&mut self) {
        self.y += 1;
    }

    pub fn move_left(&mut self) {
        self.x -= 1;
    }

    pub fn move_right(&mut self) {
        self.x += 1;
    }

    pub fn dec_money(&mut self, amt: u16) -> bool {
        if amt <= self.money {
            self.money -= amt;
            return true;
        }
        false
    }

    pub fn inc_money(&mut self, amt: u16) -> bool {
        self.money += amt;
        true
    }
}
