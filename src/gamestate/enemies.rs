use crate::enums::{Cells, Enemies, Items, GameMode, FightSteps, Interactable, EncOpt};
use crate::map::{MAP_W, MAP_H};
use crate::enemy::{Enemy};
//use crate::npc::{NPC};
use crate::gamestate::GameState;

use crate::gamestate::loc_shop_items;
// use crate::gui_man_draw::GUI;
use crate::item::Item;
use std::time::{Instant};
//use std::fs;
use std::collections::HashMap;
use rand::Rng;
use rand::prelude::SliceRandom;
use ratatui::crossterm::event::{read, Event, KeyCode, poll};


impl GameState {
    pub fn shift_enemies(&mut self, dir: &str) {
        let temp_e = self.enemies.clone();
        let mut new_e = HashMap::new();
        let mw = self.map.cells[0].len();
        let mh = self.map.cells.len();
        for ((x, y), mut e) in temp_e {
            match dir {
                "UP" => if y < mh - 5 {
                    e.y+=1;
                    new_e.insert((x, y+1), e.clone());
                    // log::info!("new key {:?}", (x, y+1));
                    // log::info!("new en {:?}", e);

                },
                "DN" => if y > 5 {
                    e.y-=1;
                    new_e.insert((x, y-1), e.clone());
                    // log::info!("new key {:?}", (x, y+1));
                    // log::info!("new en {:?}", e);
                },
                "LF" => if x < mw - 5 {
                    e.x+=1;
                    new_e.insert((x+1, y), e.clone());
                    // log::info!("new key {:?}", (x, y+1));
                    // log::info!("new en {:?}", e);
                },
                "RT" => if x > 5 {
                    e.x-=1;
                    new_e.insert((x-1, y), e.clone());
                    // log::info!("new key {:?}", (x, y+1));
                    // log::info!("new en {:?}", e);
                },
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
        for ((x, y), mut e) in &mut e_temp {
            let mut rng = rand::thread_rng();
            let dch = rng.gen_range(0..20);
            if dch % 4 == 0 {
                e.steps = dch;
            }
            let (xx, yy) = if e.get_step_grp() != step {
                (*x, *y)
            } else if e.steps < 5 {
                e.steps += 1;
                if *y == 0 || self.e_collision("UP", e.clone()) {(*x, *y)} else {
                    e.mmove("UP");
                    (*x, y - 1)
                }
            } else if e.steps >= 5 && e.steps < 10 {
                e.steps += 1;
                if *x == 0 || self.e_collision("LF", e.clone()) {(*x, *y)} else {
                    e.mmove("LF");
                    (x - 1, *y)
                }
            } else if e.steps >= 10 && e.steps < 15 {
                e.steps += 1;
                if *y >= mh-5 || self.e_collision("DN", e.clone()) {(*x, *y)} else {
                    e.mmove("DN");
                    (*x, y + 1)
                }
            } else if e.steps >= 15 && e.steps < 20 {
                e.steps += 1;
                if *x >= mw-5 || self.e_collision("RT", e.clone()) {(*x, *y)} else {
                    e.mmove("RT");
                    (x + 1, *y)
                }
            } else if e.steps == 20 {
                e.steps = 0;
                (*x, *y)
            } else {(*x, *y)};
            new_e.insert((xx.try_into().unwrap(), yy.try_into().unwrap()), e.clone());
           // }
        }
        // self.enemies = new_e.into_iter().map(|(k, v)| (k, v.clone())).collect();
        self.enemies = new_e;
    }

    pub fn enemy_turn(&mut self, e: Enemy) -> u16 {
        let (mut atk, mut dmg) = e.fight_turn();
        let pdef = self.player.get_defence();
        let dodge = self.player.get_dodge();
        if atk > pdef {
            if dodge {
                self.player.toggle_dodge();
                dmg /= 2;
            }
            self.player.apply_attack(dmg);
            return dmg;
        }
        0
    }

    pub fn enemy_drop(&mut self, mut e: Enemy) {
        let mut drps = e.get_drop();
        let i = drps.pop();
        let (x, y) = e.get_pos();
        let itm = match i {
            Some(Items::BugBits) => Item::new_bug_bits(x.clone(), y.clone()),
            Some(Items::Apple) => Item::new_apple(x.clone(), y.clone()),
            Some(Items::MetalScrap) => Item::new_metal_scrap(x.clone(), y.clone()),
            Some(Items::Salve) => Item::new_salve(x.clone(), y.clone()),
            Some(Items::HealthPotion) => Item::new_health_potion(x.clone(), y.clone()),
            _ => todo!(),
        };
        self.items.insert((x, y), itm.clone());
    }

    pub fn enemy_encounter(&mut self, mut e: Enemy) {
        //you are in fight
        let fst = format!("You are being attacked by a {}", e.get_sname());
        self.gui.reset_cursor();
        loop {
            self.gui.encounter_show_content(fst.clone(), self.map.clone(), self.player.clone(), self.enemies.clone(), self.items.clone(), self.npcs.clone(), loc_shop_items(self.dist_fo.clone(), self.location.clone()), self.env_inters.clone());
            if poll(std::time::Duration::from_millis(100)).unwrap() {
                if let Event::Key(event) = read().unwrap() {
                    // log::info!("keykind {:?}", event.kind.clone());
                    let now = Instant::now();
                    if now.duration_since(self.last_event_time) > self.key_debounce_dur {
                        self.last_event_time = now;
                        let res = self.enc_key(event.code);
                        if !res {
                            break
                        }
                    }
                }
            }
        }
        //fight start
        let mut pstart = true;
        self.game_mode = GameMode::Fight(FightSteps::Player);
        let mut fight = true;
        let mut win = None;
        while fight {
            let Interactable::Enemy(mut enemy) = self.interactee.clone() else {todo!()};
            e = enemy.clone();
            if !pstart {
                let enatk = "Enemy is attacking.".to_string();
                loop {
                    self.gui.encounter_show_content(enatk.clone(), self.map.clone(), self.player.clone(), self.enemies.clone(), self.items.clone(), self.npcs.clone(), loc_shop_items(self.dist_fo.clone(), self.location.clone()), self.env_inters.clone());
                    if poll(std::time::Duration::from_millis(100)).unwrap() {
                        if let Event::Key(event) = read().unwrap() {
                            // log::info!("keykind {:?}", event.kind.clone());
                            let now = Instant::now();
                            if now.duration_since(self.last_event_time) > self.key_debounce_dur {
                                self.last_event_time = now;
                                let res = self.enc_key(event.code);
                                if !res {
                                    break
                                }
                            }
                        }
                    }
                }
                //enemy turn
                let turn = self.enemy_turn(e.clone());
                let trn_res = if turn == 0 {
                    "The enemy attempted an attack, but missed.".to_string()
                } else {
                    let fmts = format!("The enemy atacked you for {}hp.", turn.clone());
                    fmts
                };
                self.gui.reset_cursor();
                loop {
                    self.gui.encounter_show_content(trn_res.clone(), self.map.clone(), self.player.clone(), self.enemies.clone(), self.items.clone(), self.npcs.clone(), loc_shop_items(self.dist_fo.clone(), self.location.clone()), self.env_inters.clone());
                    if poll(std::time::Duration::from_millis(100)).unwrap() {
                        if let Event::Key(event) = read().unwrap() {
                            // log::info!("keykind {:?}", event.kind.clone());
                            let now = Instant::now();
                            if now.duration_since(self.last_event_time) > self.key_debounce_dur {
                                self.last_event_time = now;
                                let res = self.enc_key(event.code);
                                if !res {
                                    break
                                }
                            }
                        }
                    }
                }
                if self.player.get_health() == 0 {
                    win = Some(false);
                    self.game_mode = GameMode::Fight(FightSteps::Null);
                    break;
                }
                self.game_mode = GameMode::Fight(FightSteps::Player);
            }
            if pstart {
                pstart = false;
            }
            //player turn
            //-player choice
            let popt = self.player.get_enc_opt();
            self.gui.reset_cursor();
            loop {
                self.gui.encounter_user_options(popt.clone(), self.map.clone(), self.player.clone(), self.enemies.clone(), self.items.clone(), self.npcs.clone(), loc_shop_items(self.dist_fo.clone(), self.location.clone()), self.env_inters.clone());
                if poll(std::time::Duration::from_millis(100)).unwrap() {
                    if let Event::Key(event) = read().unwrap() {
                        // log::info!("keykind {:?}", event.kind.clone());
                        let now = Instant::now();
                        if now.duration_since(self.last_event_time) > self.key_debounce_dur {
                            self.last_event_time = now;
                            let res = self.enc_key(event.code);
                            if !res {
                                break
                            }
                        }
                    }
                }
            }
            let lturn = self.player.get_last_turn();
            self.player.set_enc_last_turn((EncOpt::Null, 0));
            let mut itm = false;
            let trn_res = match lturn {
                (EncOpt::Dodge, _) => {
                    "You dodged in an attempt to evade attack.".to_string()
                },
                (EncOpt::Attack, 0) => {
                    "You attempted an attack, but missed.".to_string()
                },
                (EncOpt::Attack, _) => {
                    let ehp = if e.health > lturn.1 {e.health - lturn.clone().1} else {0};
                    let fmts = format!("You successfully attacked the {} for {}hp. They have an hp of: {}", e.clone().get_sname(), lturn.clone().1, ehp);
                    fmts
                },
                (EncOpt::UseItem, _) => {
                    itm = true;
                    "".to_string()
                },
                _ => "OOPS!".to_string(),

            };
            self.gui.reset_cursor();
            loop {
                if itm {break;}
                self.gui.encounter_show_content(trn_res.clone(), self.map.clone(), self.player.clone(), self.enemies.clone(), self.items.clone(), self.npcs.clone(), loc_shop_items(self.dist_fo.clone(), self.location.clone()), self.env_inters.clone());
                if poll(std::time::Duration::from_millis(100)).unwrap() {
                    if let Event::Key(event) = read().unwrap() {
                        // log::info!("keykind {:?}", event.kind.clone());
                        let now = Instant::now();
                        if now.duration_since(self.last_event_time) > self.key_debounce_dur {
                            self.last_event_time = now;
                            let res = self.enc_key(event.code);
                            if !res {
                                break
                            }
                        }
                    }
                }
            }
            let Interactable::Enemy(mut enemy) = self.interactee.clone() else {todo!()};
            e = enemy.clone();
            if e.health == 0 {
                win = Some(true);
                let epos = e.get_pos();
                self.enemies.remove(&epos);
                self.game_mode = GameMode::Fight(FightSteps::Null);
                break;
            }
            self.game_mode = GameMode::Fight(FightSteps::Enemy);
            //round end
        }
        //fight over
        let win_msg = if win.unwrap() {
            self.enemy_drop(e.clone());
            format!("You defeated the {}!", e.get_sname())
        } else {
            format!("You were killed by the {}! You are dead", e.get_sname())
        };
        self.gui.reset_cursor();
        loop {
            self.gui.encounter_show_content(win_msg.clone(), self.map.clone(), self.player.clone(), self.enemies.clone(), self.items.clone(), self.npcs.clone(), loc_shop_items(self.dist_fo.clone(), self.location.clone()), self.env_inters.clone());
            if poll(std::time::Duration::from_millis(100)).unwrap() {
                if let Event::Key(event) = read().unwrap() {
                    // log::info!("keykind {:?}", event.kind.clone());
                    let now = Instant::now();
                    if now.duration_since(self.last_event_time) > self.key_debounce_dur {
                        self.last_event_time = now;
                        let res = self.inter_key(event.code);
                        if !res {
                            break
                        }
                    }
                }
            }
        }
    }

    pub fn player_attack(&mut self) {
        let (atk, dmg) = self.player.get_enc_turn();
        let Interactable::Enemy(mut enemy) = self.interactee.clone() else {todo!()};
        let endef = enemy.get_defence();
        if atk > endef {
            enemy.apply_attack(dmg.clone());
            self.player.set_enc_last_turn((EncOpt::Attack, dmg));
            self.interactee = Interactable::Enemy(enemy.clone());
        } else {
            self.player.set_enc_last_turn((EncOpt::Attack, 0));
        }
        self.gui.reset_enc_opt();
    }

    pub fn enc_use_item(&mut self) {
        // let inventory = self.player.get_inventory();
        self.gui.set_inventory(self.player.get_inventory());
        self.gui.reset_cursor();
        loop {
            self.gui.encounter_pick_item(self.map.clone(), self.player.clone(), self.enemies.clone(), self.items.clone(), self.npcs.clone(), loc_shop_items(self.dist_fo.clone(), self.location.clone()), self.env_inters.clone());
            if poll(std::time::Duration::from_millis(100)).unwrap() {
                if let Event::Key(event) = read().unwrap() {
                    // log::info!("keykind {:?}", event.kind.clone());
                    let now = Instant::now();
                    if now.duration_since(self.last_event_time) > self.key_debounce_dur {
                        self.last_event_time = now;
                            match event.code {
                                KeyCode::Up => self.gui.move_cursor("UP"),
                                KeyCode::Down => self.gui.move_cursor("DN"),
                                KeyCode::Left => self.gui.move_cursor("LF"),
                                KeyCode::Right => self.gui.move_cursor("RT"),
                                KeyCode::Char('a') => self.gui.move_cursor("LF"),
                                KeyCode::Char('s') => self.gui.move_cursor("UP"),
                                KeyCode::Char('d') => self.gui.move_cursor("DN"),
                                KeyCode::Char('f') => self.gui.move_cursor("RT"),
                                KeyCode::Enter => {
                                    self.use_inv_item();
                                    self.gui.reset_enc_opt();
                                    self.enc = EncOpt::Null;
                                    break;
                                },
                                _ => {},
                            }
                    }
                }
            }
        }
    }

    pub fn enc_option(&mut self) {
        let opt = self.enc.clone();
        match opt {
            EncOpt::Attack => {
                self.player_attack();
            },
            EncOpt::UseItem => {
                self.enc_use_item();
                self.player.set_enc_last_turn((EncOpt::UseItem, 0));
            },
            EncOpt::Dodge => {
                self.player.toggle_dodge();
                self.player.set_enc_last_turn((EncOpt::Dodge, 0));
            },
            _ => {},
        }
    }

    pub fn check_place_enemies(&mut self, x: usize, y: usize) -> bool {
        let mut rng = rand::thread_rng();
        let l_types = vec![Enemies::Bug, Enemies::Slime];
        let h_types = vec![Enemies::GoblinMan, Enemies::CrazedExplorer, Enemies::Golem];
        if self.map.cells[y][x] == Cells::Empty && !self.in_loc_check((x, y)) && !self.npcs.contains_key(&(x, y)) && !self.items.contains_key(&(x, y)) {
            let en_type = {
                match rng.gen_range(0..2) {
                    0 => l_types,
                    1 => h_types,
                    _ => l_types,
                }
            };
            if let Some(en_type) = en_type.choose(&mut rng){
                match en_type {
                    Enemies::Bug => {
                        let drop = vec![Items::BugBits];
                        self.enemies.insert((x, y), Enemy::new_bug(x, y, 20, 10, 10, 5, drop));
                    },
                    Enemies::Slime => {
                        let drop = vec![Items::Salve];
                        self.enemies.insert((x, y), Enemy::new_slime(x, y, 20, 12, 12, 7, drop));
                    },
                    Enemies::GoblinMan => {
                        let drop = vec![Items::MetalScrap];
                        self.enemies.insert((x, y), Enemy::new_goblin_man(x, y, 25, 12, 15, 9, drop));
                    },
                    Enemies::CrazedExplorer => {
                        let drop = vec![Items::Apple];
                        self.enemies.insert((x, y), Enemy::new_crazed_explorer(x, y, 30, 15, 20, 12, drop));
                    },
                    Enemies::Golem => {
                        let drop = vec![Items::HealthPotion];
                        self.enemies.insert((x, y), Enemy::new_golem(x, y, 35, 20, 25, 15, drop));
                    },
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
        match (self.map.gen_x * - 1, self.map.gen_y * - 1) {
            (x, y) if x < 0 && y == 0 => {
                for _ in 0..20 {
                    loop {
                        let x = rng.gen_range(10..vx-5);
                        let y = rng.gen_range(10..MAP_H-10);
                        let res = self.check_place_enemies(x, y);
                        if res {break;}
                    }
                }
            },
            (x, y) if x > 0 && y == 0 => {
                for _ in 0..20 {
                    loop {
                        let x = rng.gen_range((vx + vw + 5)..MAP_W-10);
                        let y = rng.gen_range(10..MAP_H-10);
                        let res = self.check_place_enemies(x, y);
                        if res {break;}
                    }
                }
            },
            (x, y) if y < 0 && x == 0 => {
                for _ in 0..20 {
                    loop {
                        let x = rng.gen_range(10..MAP_W-10);
                        let y = rng.gen_range(10..vy-5);
                        let res = self.check_place_enemies(x, y);
                        if res {break;}
                    }
                }
            },
            (x, y) if y > 0 && x == 0 => {
                for _ in 0..20 {
                    loop {
                        let x = rng.gen_range(10..MAP_W-10);
                        let y = rng.gen_range((vy + vh + 5)..MAP_H-10);
                        let res = self.check_place_enemies(x, y);
                        if res {break;}
                    }
                }
            }, // asdf
            (x, y) if x > 0 && y > 0 => {
                for _ in 0..10 {
                    loop {
                        let x = rng.gen_range((vx + vw + 5)..MAP_W-10);
                        let y = rng.gen_range(10..MAP_H-10);
                        let res = self.check_place_enemies(x, y);
                        if res {break;}
                    }
                }
                for _ in 0..10 {
                    loop {
                        let x = rng.gen_range(10..MAP_W-10);
                        let y = rng.gen_range((vy + vh + 5)..MAP_H-10);
                        let res = self.check_place_enemies(x, y);
                        if res {break;}
                    }
                }
            },
            (x, y) if x > 0 && y < 0 => {
                for _ in 0..10 {
                    loop {
                        let x = rng.gen_range((vx + vw + 5)..MAP_W-10);
                        let y = rng.gen_range(10..MAP_H-10);
                        let res = self.check_place_enemies(x, y);
                        if res {break;}
                    }
                }
                for _ in 0..10 {
                    loop {
                        let x = rng.gen_range(10..MAP_W-10);
                        let y = rng.gen_range(10..vy-5);
                        let res = self.check_place_enemies(x, y);
                        if res {break;}
                    }
                }
            },
            (x, y) if x < 0 && y > 0 => {
                for _ in 0..10 {
                    loop {
                        let x = rng.gen_range(10..vx-5);
                        let y = rng.gen_range(10..MAP_H-10);
                        let res = self.check_place_enemies(x, y);
                        if res {break;}
                    }
                }
                for _ in 0..10 {
                    loop {
                        let x = rng.gen_range(10..MAP_W-10);
                        let y = rng.gen_range((vy + vh + 5)..MAP_H-10);
                        let res = self.check_place_enemies(x, y);
                        if res {break;}
                    }
                }
            },
            (x, y) if x < 0 && y < 0 => {
                for _ in 0..10 {
                    loop {
                        let x = rng.gen_range(10..vx-5);
                        let y = rng.gen_range(10..MAP_H-10);
                        let res = self.check_place_enemies(x, y);
                        if res {break;}
                    }
                }
                for _ in 0..10 {
                    loop {
                        let x = rng.gen_range(10..MAP_W-10);
                        let y = rng.gen_range(10..vy-5);
                        let res = self.check_place_enemies(x, y);
                        if res {break;}
                    }
                }
            },
            _ => {},
        }
        //let nt = self.npcs.clone();
    }
}
