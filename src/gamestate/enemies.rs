use crate::enemy::Enemy;
use crate::enums::{Cells, Enemies};
use crate::map::{MAP_H, MAP_W};
//use crate::npc::{NPC};
use crate::gamestate::GameState;

//use std::fs;
use rand::prelude::SliceRandom;
use rand::Rng;
use std::collections::HashMap;

impl GameState {
    pub fn shift_enemies(&mut self, dir: &str) {
        let temp_e = self.enemies.clone();
        let mut new_e = HashMap::new();
        let mw = self.map.cells[0].len();
        let mh = self.map.cells.len();
        for ((x, y), mut e) in temp_e {
            match dir {
                "UP" => {
                    if y < mh - 5 {
                        e.pos.1 += 1;
                        new_e.insert((x, y + 1), e.clone());
                    }
                }
                "DN" => {
                    if y > 5 {
                        e.pos.1 -= 1;
                        new_e.insert((x, y - 1), e.clone());
                    }
                }
                "LF" => {
                    if x < mw - 5 {
                        e.pos.0 += 1;
                        new_e.insert((x + 1, y), e.clone());
                    }
                }
                "RT" => {
                    if x > 5 {
                        e.pos.0 -= 1;
                        new_e.insert((x - 1, y), e.clone());
                    }
                }
                _ => todo!(),
            };
        }
        self.enemies = new_e;
    }

    pub fn update_enemies(&mut self, step: u8) {
        let mut e_temp = self.enemies.clone();
        let mut new_e = HashMap::new();
        let mh = self.map.cells.len();
        let mw = self.map.cells[0].len();
        let ppos = self.player.clone().get_pos();
        for ((x, y), e) in &mut e_temp {
            let dx = ppos.0 as i32 - *x as i32;
            let dy = ppos.1 as i32 - *y as i32;
            let dis = ((dx.pow(2) + dy.pow(2)) as f32).sqrt() as i32;
            let dir = {
                let dirx = if dx != 0 { dx / dx.abs() } else { 0 };
                let diry = if dy != 0 { dy / dy.abs() } else { 0 };
                (dirx, diry)
            };
            let mut rng = rand::thread_rng();
            let dch = rng.gen_range(0..20);
            if dch % 4 == 0 {
                e.steps = dch;
            }
            let (xx, yy) =
                if e.get_step_grp() != step || *x < 200 || *x > 400 || *y < 180 || *y > 225 {
                    (*x, *y)
                } else if dis < 20 {
                    //here~~~~~~~~~~~~~~~~~~
                    match dir {
                        (dirx, diry) if dirx < 0 && diry < 0 && dx.abs() < dy.abs() => {
                            e.steps += 1;
                            if *y == 0 || self.e_collision("UP", e.clone()) {
                                (*x, *y)
                            } else {
                                e.mmove("UP");
                                (*x, y - 1)
                            }
                        }
                        (dirx, diry) if dirx < 0 && diry >= 0 && dx.abs() < dy.abs() => {
                            e.steps += 1;
                            if *y >= mh - 5 || self.e_collision("DN", e.clone()) {
                                (*x, *y)
                            } else {
                                e.mmove("DN");
                                (*x, y + 1)
                            }
                        }
                        (dirx, diry) if dirx >= 0 && diry < 0 && dx.abs() < dy.abs() => {
                            e.steps += 1;
                            if *y == 0 || self.e_collision("UP", e.clone()) {
                                (*x, *y)
                            } else {
                                e.mmove("UP");
                                (*x, y - 1)
                            }
                        }
                        (dirx, diry) if dirx >= 0 && diry >= 0 && dx.abs() < dy.abs() => {
                            e.steps += 1;
                            if *y >= mh - 5 || self.e_collision("DN", e.clone()) {
                                (*x, *y)
                            } else {
                                e.mmove("DN");
                                (*x, y + 1)
                            }
                        }
                        (dirx, diry) if dirx < 0 && diry < 0 && dx.abs() >= dy.abs() => {
                            e.steps += 1;
                            if *x == 0 || self.e_collision("LF", e.clone()) {
                                (*x, *y)
                            } else {
                                e.mmove("LF");
                                (x - 1, *y)
                            }
                        }
                        (dirx, diry) if dirx < 0 && diry >= 0 && dx.abs() >= dy.abs() => {
                            e.steps += 1;
                            if *x == 0 || self.e_collision("LF", e.clone()) {
                                (*x, *y)
                            } else {
                                e.mmove("LF");
                                (x - 1, *y)
                            }
                        }
                        (dirx, diry) if dirx >= 0 && diry < 0 && dx.abs() >= dy.abs() => {
                            e.steps += 1;
                            if *x >= mw - 5 || self.e_collision("RT", e.clone()) {
                                (*x, *y)
                            } else {
                                e.mmove("RT");
                                (x + 1, *y)
                            }
                        }
                        (dirx, diry) if dirx >= 0 && diry >= 0 && dx.abs() >= dy.abs() => {
                            e.steps += 1;
                            if *x >= mw - 5 || self.e_collision("RT", e.clone()) {
                                (*x, *y)
                            } else {
                                e.mmove("RT");
                                (x + 1, *y)
                            }
                        }
                        _ => todo!(),
                    }
                } else if e.steps < 5 {
                    e.steps += 1;
                    if *y == 0 || self.e_collision("UP", e.clone()) {
                        (*x, *y)
                    } else {
                        e.mmove("UP");
                        (*x, y - 1)
                    }
                } else if e.steps >= 5 && e.steps < 10 {
                    e.steps += 1;
                    if *x == 0 || self.e_collision("LF", e.clone()) {
                        (*x, *y)
                    } else {
                        e.mmove("LF");
                        (x - 1, *y)
                    }
                } else if e.steps >= 10 && e.steps < 15 {
                    e.steps += 1;
                    if *y >= mh - 5 || self.e_collision("DN", e.clone()) {
                        (*x, *y)
                    } else {
                        e.mmove("DN");
                        (*x, y + 1)
                    }
                } else if e.steps >= 15 && e.steps < 20 {
                    e.steps += 1;
                    if *x >= mw - 5 || self.e_collision("RT", e.clone()) {
                        (*x, *y)
                    } else {
                        e.mmove("RT");
                        (x + 1, *y)
                    }
                } else if e.steps == 20 {
                    e.steps = 0;
                    (*x, *y)
                } else {
                    (*x, *y)
                };
            new_e.insert((xx, yy), e.clone());
        }
        self.enemies = new_e;
    }

    pub fn check_place_enemies(&mut self, x: usize, y: usize) -> bool {
        let (x1, y1, x2, y2) = match &self.location {
            crate::enums::Location::Settlement(settle) => (
                settle.pos.0,
                settle.pos.1,
                settle.map[0].len(),
                settle.map.len(),
            ),
            _ => (0, 0, 0, 0),
        };
        let dfo = self.dist_fo;
        if ((dfo.0 + x1 as i16) as usize..(dfo.0 + x2 as i16) as usize).contains(&x)
            && ((dfo.1 + y1 as i16) as usize..(dfo.1 + y2 as i16) as usize).contains(&y)
        {
            return false;
        }
        let mut rng = rand::thread_rng();
        let l_types = vec![
            Enemies::Bug,
            Enemies::Slime,
            Enemies::Snake,
            Enemies::Spider,
        ];
        let h_types = vec![
            Enemies::Goblin,
            Enemies::CrazedExplorer,
            Enemies::Golem,
            Enemies::Ghoul,
            Enemies::Bandit,
        ];
        if self.map.cells[y][x] == Cells::Empty
            && !self.in_loc_check((x, y))
            && !self.npcs.contains_key(&(x, y))
            && !self.items.contains_key(&(x, y))
        {
            let en_type = {
                match rng.gen_range(0..2) {
                    0 => l_types,
                    1 => h_types,
                    _ => l_types,
                }
            };
            if let Some(en_type) = en_type.choose(&mut rng) {
                match en_type {
                    Enemies::Bug => {
                        self.enemies
                            .insert((x, y), Enemy::new_bug((x, y), self.depth));
                    }
                    Enemies::Slime => {
                        self.enemies
                            .insert((x, y), Enemy::new_slime((x, y), self.depth));
                    }
                    Enemies::Snake => {
                        self.enemies
                            .insert((x, y), Enemy::new_snake((x, y), self.depth));
                    }
                    Enemies::Spider => {
                        self.enemies
                            .insert((x, y), Enemy::new_spider((x, y), self.depth));
                    }
                    Enemies::Goblin => {
                        self.enemies
                            .insert((x, y), Enemy::new_goblin((x, y), self.depth));
                    }
                    Enemies::Bandit => {
                        self.enemies
                            .insert((x, y), Enemy::new_bandit((x, y), self.depth));
                    }
                    Enemies::CrazedExplorer => {
                        self.enemies
                            .insert((x, y), Enemy::new_crazed_explorer((x, y), self.depth));
                    }
                    Enemies::Ghoul => {
                        self.enemies
                            .insert((x, y), Enemy::new_ghoul((x, y), self.depth));
                    }
                    Enemies::Golem => {
                        self.enemies
                            .insert((x, y), Enemy::new_golem((x, y), self.depth));
                    }
                    _ => todo!(),
                };
                return true;
            }
        }
        false
    }

    pub fn repop_enemies(&mut self) {
        let mut rng = rand::thread_rng();
        let (vx, vy, vw, vh) = self.map.get_viewport();
        //xx
        match (-self.map.gen_x, -self.map.gen_y) {
            (x, y) if x < 0 && y == 0 => {
                for _ in 0..20 {
                    loop {
                        let x = rng.gen_range(10..vx - 5);
                        let y = rng.gen_range(10..MAP_H - 10);
                        let res = self.check_place_enemies(x, y);
                        if res {
                            break;
                        }
                    }
                }
            }
            (x, y) if x > 0 && y == 0 => {
                for _ in 0..20 {
                    loop {
                        let x = rng.gen_range((vx + vw + 5)..MAP_W - 10);
                        let y = rng.gen_range(10..MAP_H - 10);
                        let res = self.check_place_enemies(x, y);
                        if res {
                            break;
                        }
                    }
                }
            }
            (x, y) if y < 0 && x == 0 => {
                for _ in 0..20 {
                    loop {
                        let x = rng.gen_range(10..MAP_W - 10);
                        let y = rng.gen_range(10..vy - 5);
                        let res = self.check_place_enemies(x, y);
                        if res {
                            break;
                        }
                    }
                }
            }
            (x, y) if y > 0 && x == 0 => {
                for _ in 0..20 {
                    loop {
                        let x = rng.gen_range(10..MAP_W - 10);
                        let y = rng.gen_range((vy + vh + 5)..MAP_H - 10);
                        let res = self.check_place_enemies(x, y);
                        if res {
                            break;
                        }
                    }
                }
            } // asdf
            (x, y) if x > 0 && y > 0 => {
                for _ in 0..10 {
                    loop {
                        let x = rng.gen_range((vx + vw + 5)..MAP_W - 10);
                        let y = rng.gen_range(10..MAP_H - 10);
                        let res = self.check_place_enemies(x, y);
                        if res {
                            break;
                        }
                    }
                }
                for _ in 0..10 {
                    loop {
                        let x = rng.gen_range(10..MAP_W - 10);
                        let y = rng.gen_range((vy + vh + 5)..MAP_H - 10);
                        let res = self.check_place_enemies(x, y);
                        if res {
                            break;
                        }
                    }
                }
            }
            (x, y) if x > 0 && y < 0 => {
                for _ in 0..10 {
                    loop {
                        let x = rng.gen_range((vx + vw + 5)..MAP_W - 10);
                        let y = rng.gen_range(10..MAP_H - 10);
                        let res = self.check_place_enemies(x, y);
                        if res {
                            break;
                        }
                    }
                }
                for _ in 0..10 {
                    loop {
                        let x = rng.gen_range(10..MAP_W - 10);
                        let y = rng.gen_range(10..vy - 5);
                        let res = self.check_place_enemies(x, y);
                        if res {
                            break;
                        }
                    }
                }
            }
            (x, y) if x < 0 && y > 0 => {
                for _ in 0..10 {
                    loop {
                        let x = rng.gen_range(10..vx - 5);
                        let y = rng.gen_range(10..MAP_H - 10);
                        let res = self.check_place_enemies(x, y);
                        if res {
                            break;
                        }
                    }
                }
                for _ in 0..10 {
                    loop {
                        let x = rng.gen_range(10..MAP_W - 10);
                        let y = rng.gen_range((vy + vh + 5)..MAP_H - 10);
                        let res = self.check_place_enemies(x, y);
                        if res {
                            break;
                        }
                    }
                }
            }
            (x, y) if x < 0 && y < 0 => {
                for _ in 0..10 {
                    loop {
                        let x = rng.gen_range(10..vx - 5);
                        let y = rng.gen_range(10..MAP_H - 10);
                        let res = self.check_place_enemies(x, y);
                        if res {
                            break;
                        }
                    }
                }
                for _ in 0..10 {
                    loop {
                        let x = rng.gen_range(10..MAP_W - 10);
                        let y = rng.gen_range(10..vy - 5);
                        let res = self.check_place_enemies(x, y);
                        if res {
                            break;
                        }
                    }
                }
            }
            _ => {}
        }
    }
}
