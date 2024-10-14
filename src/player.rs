//player
// mod enums;
use crate::enums::{EncOpt};
use crate::item::{Item};
use std::collections::HashMap;
use rand::{Rng};

#[derive(Clone, Debug)]
pub struct Player {
    pub x: usize,
    pub y: usize,
    pub health: u16,
    pub inventory: Vec<Item>,
    pub attack: u16,
    pub defence: u16,
    pub damage: u16,
    pub money: u16,
    dodge: bool,
    enc_last_turn: (EncOpt, u16),
    enc_opt: HashMap<EncOpt, String>,
    // pub inventory: HashMap<Item, u16>,
}

impl Player {
    pub fn new(x: usize, y: usize) -> Self {
        let inventory: Vec<Item> = Vec::new();
        let mut enc_opt = HashMap::new();
        enc_opt.insert(EncOpt::Attack, "Attack".to_string());
        enc_opt.insert(EncOpt::UseItem, "Use item".to_string());
        enc_opt.insert(EncOpt::Dodge, "Dodge".to_string());
        Self {
            x,
            y,
            health: 100,
            inventory,
            attack: 20,
            defence: 10,
            damage: 10,
            money: 10,
            dodge: false,
            enc_last_turn: (EncOpt::Null, 0),
            enc_opt,
        }
    }

    pub fn set_enc_last_turn(&mut self, turn: (EncOpt, u16)) {
        self.enc_last_turn = turn;
    }

    pub fn get_last_turn(&mut self) -> (EncOpt, u16) {
        self.enc_last_turn.clone()
    }

    pub fn get_enc_turn(&mut self) -> (u16, u16) {
        let mut rng = rand::thread_rng();
        let attack = rng.gen_range((self.attack/3)..self.attack);
        let damage = rng.gen_range((self.damage/3)..self.damage);
        (attack.clone(), damage.clone())
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
        self.defence
    }

    pub fn get_health(&mut self) -> u16 {
        self.health
    }

    pub fn add_to_inv(&mut self, item: Item) {
        self.inventory.push(item);
    }

    pub fn get_inventory(&mut self) -> Vec<Item> {
        self.inventory.clone()
    }

    pub fn rem_inv_item(&mut self, idx: usize) {
        self.inventory.remove(idx);
    }

    pub fn apply_item_effect(&mut self, mut item: Item) {
        let prop = item.get_properties();
        for (stat, effect) in &prop {
            let h = String::from("Health");
            match stat {
                 h => self.health += *effect,
                _ => todo!(),
            }
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
        return false;
    }

}
