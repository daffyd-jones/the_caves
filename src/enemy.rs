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
            health,
            attack,
            defence,
            damage,
            drop,
        }
    }

    // snakes ȥ ɀ ʑ ζ
    // slimes ǚ Ǚ
    // spiders ẅ Ẅ
    // bandit Ồ
    // goblin ớ Ớ
    // ghoul ή

    pub fn new_bug(pos: (usize, usize), lvl: u16) -> Self {
        let mut rng = rand::thread_rng();
        let steps = rng.gen_range(0..19);
        let step_grp = rng.gen_range(0..15);
        let health = 20 * lvl;
        let attack = 10 * lvl;
        let defence = 10 * lvl;
        let damage = 5 * lvl;
        let drop = vec![Items::Guts];
        Self {
            etype: Enemies::Bug,
            sname: "Bug".to_string(),
            pos,
            steps,
            step_grp,
            health,
            attack,
            defence,
            damage,
            drop,
        }
    }

    pub fn new_slime(pos: (usize, usize), lvl: u16) -> Self {
        let mut rng = rand::thread_rng();
        let steps = rng.gen_range(0..19);
        let step_grp = rng.gen_range(0..15);
        let health = 25 * lvl;
        let attack = 12 * lvl;
        let defence = 12 * lvl;
        let damage = 7 * lvl;
        let drop = vec![Items::Guts];
        Self {
            etype: Enemies::Slime,
            sname: "Slime".to_string(),
            pos,
            steps,
            step_grp,
            health,
            attack,
            defence,
            damage,
            drop,
        }
    }

    pub fn new_snake(pos: (usize, usize), lvl: u16) -> Self {
        let mut rng = rand::thread_rng();
        let steps = rng.gen_range(0..19);
        let step_grp = rng.gen_range(0..15);
        let health = 30 * lvl;
        let attack = 15 * lvl;
        let defence = 15 * lvl;
        let damage = 10 * lvl;
        let drop = vec![Items::Guts];
        Self {
            etype: Enemies::Snake,
            sname: "Snake".to_string(),
            pos,
            steps,
            step_grp,
            health,
            attack,
            defence,
            damage,
            drop,
        }
    }

    pub fn new_spider(pos: (usize, usize), lvl: u16) -> Self {
        let mut rng = rand::thread_rng();
        let steps = rng.gen_range(0..19);
        let step_grp = rng.gen_range(0..15);
        let health = 30 * lvl;
        let attack = 17 * lvl;
        let defence = 17 * lvl;
        let damage = 12 * lvl;
        let drop = vec![Items::Guts];
        Self {
            etype: Enemies::Spider,
            sname: "Spider".to_string(),
            pos,
            steps,
            step_grp,
            health,
            attack,
            defence,
            damage,
            drop,
        }
    }

    pub fn new_goblin(pos: (usize, usize), lvl: u16) -> Self {
        let mut rng = rand::thread_rng();
        let steps = rng.gen_range(0..19);
        let step_grp = rng.gen_range(0..15);
        let health = 40 * lvl;
        let attack = 20 * lvl;
        let defence = 20 * lvl;
        let damage = 15 * lvl;
        let drop = vec![Items::Guts];
        Self {
            etype: Enemies::Goblin,
            sname: "Goblin".to_string(),
            pos,
            steps,
            step_grp,
            health,
            attack,
            defence,
            damage,
            drop,
        }
    }

    pub fn new_bandit(pos: (usize, usize), lvl: u16) -> Self {
        let mut rng = rand::thread_rng();
        let steps = rng.gen_range(0..19);
        let step_grp = rng.gen_range(0..15);
        let health = 50 * lvl;
        let attack = 20 * lvl;
        let defence = 20 * lvl;
        let damage = 17 * lvl;
        let drop = vec![Items::Guts];
        Self {
            etype: Enemies::Bandit,
            sname: "Bandit".to_string(),
            pos,
            steps,
            step_grp,
            health,
            attack,
            defence,
            damage,
            drop,
        }
    }

    pub fn new_ghoul(pos: (usize, usize), lvl: u16) -> Self {
        let mut rng = rand::thread_rng();
        let steps = rng.gen_range(0..19);
        let step_grp = rng.gen_range(0..15);
        let health = 60 * lvl;
        let attack = 30 * lvl;
        let defence = 30 * lvl;
        let damage = 20 * lvl;
        let drop = vec![Items::Guts];
        Self {
            etype: Enemies::Ghoul,
            sname: "Ghoul".to_string(),
            pos,
            steps,
            step_grp,
            health,
            attack,
            defence,
            damage,
            drop,
        }
    }

    pub fn new_crazed_explorer(pos: (usize, usize), lvl: u16) -> Self {
        let mut rng = rand::thread_rng();
        let steps = rng.gen_range(0..19);
        let step_grp = rng.gen_range(0..15);
        let health = 30 * lvl;
        let attack = 17 * lvl;
        let defence = 17 * lvl;
        let damage = 12 * lvl;
        let drop = vec![Items::Guts];
        Self {
            etype: Enemies::CrazedExplorer,
            sname: "Crazed Explorer".to_string(),
            pos,
            steps,
            step_grp,
            health,
            attack,
            defence,
            damage,
            drop,
        }
    }

    pub fn new_golem(pos: (usize, usize), lvl: u16) -> Self {
        let mut rng = rand::thread_rng();
        let steps = rng.gen_range(0..19);
        let step_grp = rng.gen_range(0..15);
        let health = 80 * lvl;
        let attack = 35 * lvl;
        let defence = 35 * lvl;
        let damage = 25 * lvl;
        let drop = vec![Items::Guts];
        Self {
            etype: Enemies::Golem,
            sname: "Golem".to_string(),
            pos,
            steps,
            step_grp,
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
