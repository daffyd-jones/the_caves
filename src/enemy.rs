//enemies
use crate::enums::Enemies;
use crate::enums::Items;
use crate::enums::Cells;
use rand::{Rng};

#[derive(Clone, Debug, PartialEq)]
pub struct Enemy {
    pub etype: Enemies,
    pub sname: String,
    pub x: usize,
    pub y: usize,
    pub steps: u8,
    step_grp: u8,
    pub cell: Cells,
    pub health: u16,
    pub attack: u16,
    pub defence: u16,
    pub damage: u16,
    pub drop: Vec<Items>,
}

impl Enemy {
    pub fn new(etype: Enemies, sname: String, x: usize, y: usize, health: u16,
        attack: u16, defence: u16, damage: u16, drop: Vec<Items>) -> Self {
        let mut rng = rand::thread_rng();
        let step = rng.gen_range(0..19);
        let step_grp = rng.gen_range(0..15);
        Self {etype, sname, x, y, steps: step, step_grp: step_grp, cell: Cells::Empty, health, attack, defence, damage, drop}
    }

    pub fn new_bug(x: usize, y: usize, health: u16, attack: u16, defence: u16, damage: u16, drop: Vec<Items>) -> Self {
        let mut rng = rand::thread_rng();
        let step = rng.gen_range(0..19);
        let step_grp = rng.gen_range(0..15);
        Self {
            etype: Enemies::Bug,
            sname: "Bug".to_string(),
            x: x,
            y: y,
            steps: step,
            step_grp: step_grp,
            cell: Cells::Empty,
            health: health,
            attack: attack,
            defence: defence,
            damage: damage,
            drop: drop,
        }    
    }

    pub fn new_slime(x: usize, y: usize, health: u16, attack: u16, defence: u16, damage: u16, drop: Vec<Items>) -> Self {
        let mut rng = rand::thread_rng();
        let step = rng.gen_range(0..19);
        let step_grp = rng.gen_range(0..15);
        Self {
            etype: Enemies::Slime,
            sname: "Slime".to_string(),
            x: x,
            y: y,
            steps: step,
            step_grp: step_grp,
            cell: Cells::Empty,
            health: health,
            attack: attack,
            defence: defence,
            damage: damage,
            drop: drop,
        }
    }

    pub fn new_goblin_man(x: usize, y: usize, health: u16, attack: u16, defence: u16, damage: u16, drop: Vec<Items>) -> Self {
        let mut rng = rand::thread_rng();
        let step = rng.gen_range(0..19);
        let step_grp = rng.gen_range(0..15);
        Self {
            etype: Enemies::GoblinMan,
            sname: "Goblin Man".to_string(),
            x: x,
            y: y,
            steps: step,
            step_grp: step_grp,
            cell: Cells::Empty,
            health: health,
            attack: attack,
            defence: defence,
            damage: damage,
            drop: drop,
        }
    }

    pub fn new_crazed_explorer(x: usize, y: usize, health: u16, attack: u16, defence: u16, damage: u16, drop: Vec<Items>) -> Self {
        let mut rng = rand::thread_rng();
        let step = rng.gen_range(0..19);
        let step_grp = rng.gen_range(0..15);
        Self {
            etype: Enemies::CrazedExplorer,
            sname: "Crazed Explorer".to_string(),
            x: x,
            y: y,
            steps: step,
            step_grp: step_grp,
            cell: Cells::Empty,
            health: health,
            attack: attack,
            defence: defence,
            damage: damage,
            drop: drop,
        }
    }

    pub fn new_golem(x: usize, y: usize, health: u16, attack: u16, defence: u16, damage: u16, drop: Vec<Items>) -> Self {
        let mut rng = rand::thread_rng();
        let step = rng.gen_range(0..19);
        let step_grp = rng.gen_range(0..15);
        Self {
            etype: Enemies::Golem,
            sname: "Golem".to_string(),
            x: x,
            y: y,
            steps: step,
            step_grp: step_grp,
            cell: Cells::Empty,
            health: health,
            attack: attack,
            defence: defence,
            damage: damage,
            drop: drop,
        }
    }

    pub fn get_sname(&mut self) -> String {
        self.sname.clone()
    }

    pub fn mmove(&mut self, dir: &str) {
        match dir {
            "UP" => self.y -= 1,
            "DN" => self.y += 1,
            "LF" => self.x -= 1,
            "RT" => self.x += 1,
            _ => println!("")
        }
    }

    pub fn fight_turn(&self) -> (u16, u16) {
        let mut rng = rand::thread_rng();
        let attack = rng.gen_range((self.attack/4)..self.attack);
        let damage = rng.gen_range((self.damage/4)..self.damage);
        (attack.clone(), damage.clone())
    }

    pub fn get_defence(&self) -> u16 {
        self.defence.clone()
    }

    pub fn get_pos(&mut self) -> (usize, usize) {
        (self.x, self.y)
    }

    pub fn apply_attack(&mut self, amt: u16) {
        if self.health > amt {
            self.health -= amt;
        } else {
            self.health = 0;
        }
    }

    pub fn get_drop(&mut self) -> Vec<Items> {
        self.drop.clone()
    }

    pub fn get_step_grp(&self) -> u8 {
       self.step_grp.clone()
    }

}





