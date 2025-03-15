//enemies
use crate::enums::Cells;
use crate::enums::Enemies;
const PALLETE: &str = "empty: ' . , ' * | wall: ▒ | other ▓ ░ ~ | pipes: ═ ║ ╣ ╠ ╩ ╦ ╗ ╝ ╚ ╔ ╬   ┐ └ ┴ ┬ ├ ─ ┼ ┘ ┌ ┤ │ ≡ ° × ¤ ¸ ¨ · ■ ¦ ± ¡ ø Ø ©";
use crate::enums::Items;
use rand::Rng;
// use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq)]
//#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Enemy {
    pub etype: Enemies,
    pub sname: String,
    pub pos: (usize, usize),
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
    pub fn new(
        etype: Enemies,
        sname: String,
        pos: (usize, usize),
        health: u16,
        attack: u16,
        defence: u16,
        damage: u16,
        drop: Vec<Items>,
    ) -> Self {
        let mut rng = rand::thread_rng();
        let step = rng.gen_range(0..19);
        let step_grp = rng.gen_range(0..15);
        Self {
            etype,
            sname,
            pos,
            steps: step,
            step_grp,
            cell: Cells::Empty,
            health,
            attack,
            defence,
            damage,
            drop,
        }
    }

    pub fn new_bug(
        pos: (usize, usize),
        health: u16,
        attack: u16,
        defence: u16,
        damage: u16,
        drop: Vec<Items>,
    ) -> Self {
        let mut rng = rand::thread_rng();
        let step = rng.gen_range(0..19);
        let step_grp = rng.gen_range(0..15);
        Self {
            etype: Enemies::Bug,
            sname: "Bug".to_string(),
            pos,
            steps: step,
            step_grp,
            cell: Cells::Empty,
            health,
            attack,
            defence,
            damage,
            drop,
        }
    }

    pub fn new_slime(
        pos: (usize, usize),
        health: u16,
        attack: u16,
        defence: u16,
        damage: u16,
        drop: Vec<Items>,
    ) -> Self {
        let mut rng = rand::thread_rng();
        let step = rng.gen_range(0..19);
        let step_grp = rng.gen_range(0..15);
        Self {
            etype: Enemies::Slime,
            sname: "Slime".to_string(),
            pos,
            steps: step,
            step_grp,
            cell: Cells::Empty,
            health,
            attack,
            defence,
            damage,
            drop,
        }
    }

    pub fn new_goblin_man(
        pos: (usize, usize),
        health: u16,
        attack: u16,
        defence: u16,
        damage: u16,
        drop: Vec<Items>,
    ) -> Self {
        let mut rng = rand::thread_rng();
        let step = rng.gen_range(0..19);
        let step_grp = rng.gen_range(0..15);
        Self {
            etype: Enemies::GoblinMan,
            sname: "Goblin Man".to_string(),
            pos,
            steps: step,
            step_grp,
            cell: Cells::Empty,
            health,
            attack,
            defence,
            damage,
            drop,
        }
    }

    pub fn new_crazed_explorer(
        pos: (usize, usize),
        health: u16,
        attack: u16,
        defence: u16,
        damage: u16,
        drop: Vec<Items>,
    ) -> Self {
        let mut rng = rand::thread_rng();
        let step = rng.gen_range(0..19);
        let step_grp = rng.gen_range(0..15);
        Self {
            etype: Enemies::CrazedExplorer,
            sname: "Crazed Explorer".to_string(),
            pos,
            steps: step,
            step_grp,
            cell: Cells::Empty,
            health,
            attack,
            defence,
            damage,
            drop,
        }
    }

    pub fn new_golem(
        pos: (usize, usize),
        health: u16,
        attack: u16,
        defence: u16,
        damage: u16,
        drop: Vec<Items>,
    ) -> Self {
        let mut rng = rand::thread_rng();
        let step = rng.gen_range(0..19);
        let step_grp = rng.gen_range(0..15);
        Self {
            etype: Enemies::Golem,
            sname: "Golem".to_string(),
            pos,
            steps: step,
            step_grp,
            cell: Cells::Empty,
            health,
            attack,
            defence,
            damage,
            drop,
        }
    }

    pub fn get_sname(&mut self) -> String {
        self.sname.clone()
    }

    pub fn mmove(&mut self, dir: &str) {
        match dir {
            "UP" => self.pos.1 -= 1,
            "DN" => self.pos.1 += 1,
            "LF" => self.pos.0 -= 1,
            "RT" => self.pos.0 += 1,
            _ => println!(""),
        }
    }

    pub fn fight_turn(&self) -> (u16, u16) {
        let mut rng = rand::thread_rng();
        let attack = rng.gen_range((self.attack / 4)..self.attack);
        let damage = rng.gen_range((self.damage / 4)..self.damage);
        (attack.clone(), damage.clone())
    }

    pub fn get_defence(&self) -> u16 {
        self.defence.clone()
    }

    pub fn get_pos(&mut self) -> (usize, usize) {
        self.pos
    }

    pub fn set_pos(&mut self, pos: (usize, usize)) {
        self.pos = pos;
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
