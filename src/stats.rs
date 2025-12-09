//stats

use crate::enums::{ExpType, Items, Month, NPCIntros, Plants, PlayerTraits, ToggleState};
use rand::Rng;
use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

pub struct Season {
    pub year: u16,
    pub month: Month,
    pub day: u8,
    pub str: String,
}

impl Season {
    fn new() -> Self {
        Self {
            year: 102,
            month: Month::Opal,
            day: 11,
            str: "11/Opal/102".to_string(),
        }
    }

    fn month_str(&self) -> String {
        match self.month {
            Month::Opal => "Opal".to_string(),
            Month::Quartz => "Quartz".to_string(),
            Month::Jade => "Jade".to_string(),
            Month::Bizmuth => "Bizmuth".to_string(),
        }
    }

    fn next_day(&mut self) {
        let d = &self.day;
        if d + 1 > 30 {
            match &self.month {
                Month::Opal => self.month = Month::Quartz,
                Month::Quartz => self.month = Month::Jade,
                Month::Jade => self.month = Month::Bizmuth,
                Month::Bizmuth => {
                    self.month = Month::Opal;
                    self.year += 1;
                }
            }
            self.day = 0;
        }
        self.str = format!("{}/{}/{}", self.day, self.month_str(), self.year);
    }
}

pub struct WorldStats {
    pub political_conflict: i8,
    pub economy: i8,
    pub date: Season,
}

impl WorldStats {
    fn new() -> Self {
        Self {
            political_conflict: 0,
            economy: 0,
            date: Season::new(),
        }
    }
}

pub struct PlayerStats {
    alignment: i8,
    renown: u8,
}

impl PlayerStats {
    fn new() -> Self {
        Self {
            alignment: 0,
            renown: 0,
        }
    }
}

pub struct Experience {
    attack: (u16, u16, u16),
    damage: (u16, u16, u16),
    defence: (u16, u16, u16),
    luck: (u16, u16, u16),
    trading: (u16, u16, u16),
    lockpicking: (u16, u16, u16),
    herbalism: (u16, u16, u16),
}

impl Experience {
    fn new() -> Self {
        Self {
            attack: (1, 0, 100),
            damage: (1, 0, 100),
            defence: (1, 0, 100),
            luck: (1, 0, 10),
            trading: (1, 0, 10),
            lockpicking: (10, 0, 10),
            herbalism: (1, 0, 10),
        }
    }

    pub fn inc_xp(&mut self, xp_type: ExpType, amt: u16) {
        let t = match xp_type {
            ExpType::Attack => self.attack,
            ExpType::Damage => self.damage,
            ExpType::Defence => self.defence,
            ExpType::Luck => self.luck,
            ExpType::Trading => self.trading,
            ExpType::Lockpicking => self.lockpicking,
            ExpType::Herbalism => self.herbalism,
        };

        let new = if t.1 + amt > t.2 {
            (t.0 + 1, 0, (t.2 + (t.2 >> 1)))
        } else {
            (t.0, t.1 + amt, t.2)
        };
        match xp_type {
            ExpType::Attack => self.attack = new,
            ExpType::Damage => self.damage = new,
            ExpType::Defence => self.defence = new,
            ExpType::Luck => self.luck = new,
            ExpType::Trading => self.trading = new,
            ExpType::Lockpicking => self.lockpicking = new,
            ExpType::Herbalism => self.herbalism = new,
        }
    }

    pub fn get_xp(&mut self, xp_type: ExpType) -> (u16, u16, u16) {
        match xp_type {
            ExpType::Attack => self.attack,
            ExpType::Damage => self.damage,
            ExpType::Defence => self.defence,
            ExpType::Luck => self.luck,
            ExpType::Trading => self.trading,
            ExpType::Lockpicking => self.lockpicking,
            ExpType::Herbalism => self.herbalism,
        }
    }

    pub fn get_xps(&mut self) -> Vec<u16> {
        vec![
            self.attack.0,
            self.damage.0,
            self.defence.0,
            self.luck.0,
            self.trading.0,
            self.lockpicking.0,
            self.herbalism.0,
        ]
    }
}

#[derive(Clone, Debug)]
pub enum BuffType {
    Agility,
    Vitality,
    Strength,
    Attack,
    Damage,
    Defence,
    Luck,
    Trading,
    Lockpicking,
    Herbalism,
}
#[derive(Clone, Debug)]
pub enum Buff {
    Equip {
        item: Items,
        item_str: String,
        buffs: HashMap<BuffType, i8>,
    },
    DurEffect {
        src: String,
        buffs: HashMap<BuffType, i8>,
        end: Instant,
    },
    Effect {
        src: String,
        buffs: HashMap<BuffType, i8>,
    },
}

pub struct Stats {
    pub world_stats: WorldStats,
    pub state_toggle: HashMap<ToggleState, bool>,
    pub player_stats: PlayerStats,
    pub player_xp: Experience,
    pub buffs: Vec<Buff>,
}

fn build_state_toggle() -> HashMap<ToggleState, bool> {
    HashMap::from([
        (ToggleState::PlayerTraits(PlayerTraits::Poisoned), false),
        (ToggleState::PlayerTraits(PlayerTraits::Agility), false),
        (ToggleState::PlayerTraits(PlayerTraits::Vitality), false),
        (ToggleState::PlayerTraits(PlayerTraits::Invisible), false),
        (ToggleState::NPCIntros(NPCIntros::Herbalist), false),
        (ToggleState::Plants(Plants::Moss), false),
        (ToggleState::Plants(Plants::LuminousMushroom), false),
        (ToggleState::Plants(Plants::LichenousGrowth), false),
        (ToggleState::Plants(Plants::VineBulb), false),
        (ToggleState::Plants(Plants::LampenFlower), false),
        (ToggleState::Plants(Plants::LuckyClover), false),
        (ToggleState::Plants(Plants::Shroom), false),
    ])
}

impl Stats {
    pub fn new() -> Self {
        Self {
            world_stats: WorldStats::new(),
            state_toggle: build_state_toggle(),
            player_stats: PlayerStats::new(),
            player_xp: Experience::new(),
            buffs: Vec::new(),
        }
    }

    pub fn roll_world_stats(&mut self) {
        let mut rng = rand::thread_rng();
        let politics_roll = rng.gen_range(-10..10);
        let economy_roll = rng.gen_range(-10..10);
        self.world_stats.political_conflict += politics_roll;
        self.world_stats.economy += economy_roll;
    }

    pub fn get_display_stats(&self) -> (String, String) {
        let date = self.world_stats.date.str.clone();
        let economy = self.world_stats.economy.to_string();
        (date, economy)
    }

    fn btype_to_string(&self, btype: BuffType) -> String {
        match btype {
            BuffType::Attack => "Attack".to_string(),
            BuffType::Damage => "Damage".to_string(),
            BuffType::Defence => "Defence".to_string(),
            BuffType::Luck => "Luck".to_string(),
            BuffType::Trading => "Trading".to_string(),
            BuffType::Lockpicking => "Lockpicking".to_string(),
            BuffType::Herbalism => "Herbalism".to_string(),
            BuffType::Agility => "Agility".to_string(),
            BuffType::Vitality => "Vitality".to_string(),
            BuffType::Strength => "Strength".to_string(),
        }
    }

    pub fn get_display_buffs(&self) -> Vec<String> {
        let mut temp = Vec::new();
        for i in &self.buffs {
            match i {
                Buff::DurEffect { btype, amt, end } => {
                    let bt_str = self.btype_to_string(btype.clone());
                    let dur = *end - Instant::now();
                    let dur_sec = dur.as_secs();
                    let min = dur_sec / 60;
                    let sec = dur_sec % 60;
                    let sign = if *amt > 0 { "+" } else { " " };
                    let str = format!("{}: {}{} - {}:{}", bt_str, sign, amt, min, sec);
                    temp.push(str);
                }
                _ => todo!(),
            }
        }
        temp
    }

    pub fn update_buffs(&mut self) {
        let mut temp = Vec::new();
        for (idx, buff) in self.buffs.iter().enumerate() {
            match buff {
                Buff::DurEffect { btype, amt, end } => {
                    if *end < Instant::now() {
                        temp.push(idx);
                    }
                }
                _ => todo!(),
            }
        }
        temp.reverse();
        temp.iter().for_each(|i| {
            self.buffs.remove(*i);
        });
    }

    pub fn next_day(&mut self) {
        self.world_stats.date.next_day();
    }

    pub fn add_timed_buff(&mut self, btype: BuffType, amt: i8, dur: Duration) {
        let now = Instant::now();
        let temp = now.checked_add(dur);
        self.buffs.push(Buff::DurEffect {
            btype,
            amt,
            end: temp.unwrap_or(now),
        });
    }
}
