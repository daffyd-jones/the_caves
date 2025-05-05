use crate::enemy::Enemy;
use crate::enums::{
    AniType, Cells, EncMode, EncOpt, EncResult, Enemies, Equip, FightSteps, GameMode, Interactable,
    Items,
};
use crate::map::{MAP_H, MAP_W};
//use crate::npc::{NPC};
use crate::gamestate::GameState;

use crate::gamestate::loc_shop_items;
use crate::gui::{Animation, GuiArgs};
use crate::item::Item;
use std::time::Instant;
//use std::fs;
use rand::prelude::SliceRandom;
use rand::Rng;
use ratatui::crossterm::event::{poll, read, Event, KeyCode};
use ratatui::style::Color;
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
                        // log::info!("new key {:?}", (x, y+1));
                        // log::info!("new en {:?}", e);
                    }
                }
                "DN" => {
                    if y > 5 {
                        e.pos.1 -= 1;
                        new_e.insert((x, y - 1), e.clone());
                        // log::info!("new key {:?}", (x, y+1));
                        // log::info!("new en {:?}", e);
                    }
                }
                "LF" => {
                    if x < mw - 5 {
                        e.pos.0 += 1;
                        new_e.insert((x + 1, y), e.clone());
                        // log::info!("new key {:?}", (x, y+1));
                        // log::info!("new en {:?}", e);
                    }
                }
                "RT" => {
                    if x > 5 {
                        e.pos.0 -= 1;
                        new_e.insert((x - 1, y), e.clone());
                        // log::info!("new key {:?}", (x, y+1));
                        // log::info!("new en {:?}", e);
                    }
                }
                _ => todo!(),
            };
        }
        self.enemies = new_e;
    }

    pub fn pursue_player() {}

    pub fn update_enemies(&mut self, step: u8) {
        let mut e_temp = self.enemies.clone();
        let mut new_e = HashMap::new();
        let mh = self.map.cells.len();
        let mw = self.map.cells[0].len();
        let ppos = self.player.clone().get_pos();
        //let dx = ppos.0 -
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
            // new_e.insert((xx, yy), e.clone());
            new_e.insert((xx, yy), e.clone());
            // }
        }
        // self.enemies = new_e.into_iter().map(|(k, v)| (k, v.clone())).collect();
        self.enemies = new_e;
    }

    pub fn enemy_turn(&mut self, e: Enemy) -> u16 {
        let (atk, mut dmg) = e.fight_turn();
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
            Some(Items::Guts) => Item::new_guts(x, y),
            Some(Items::Apple) => Item::new_apple(x, y),
            Some(Items::MetalScrap) => Item::new_metal_scrap(x, y),
            Some(Items::Salve) => Item::new_salve(x, y),
            Some(Items::HealthPotion) => Item::new_health_potion(x, y),
            _ => todo!(),
        };
        self.items.insert((x, y), itm.clone());
    }

    fn auto_encounter(&mut self) -> EncResult {
        let Interactable::Enemy(enemy) = self.interactee.clone() else {
            todo!()
        };
        let turn = self.enemy_turn(enemy.clone());
        let t_str: Vec<char> = turn.to_string().chars().collect();
        let mut ani_frames = vec![('\\', Color::White), ('X', Color::Red), ('/', Color::White)];
        if let Some(weap) = self.player.get_equipped().get(&Equip::Shield) {
            ani_frames.push(weap.icon);
        }
        for i in t_str {
            ani_frames.push((i, Color::Red));
        }
        for i in ani_frames {
            self.gui.encounter_auto_content(&mut GuiArgs {
                map: &self.map,
                player: &self.player,
                enemies: &self.enemies,
                items: &self.items,
                npcs: &self.npcs,
                env_inter: Some(&self.env_inters),
                litems: Some(&loc_shop_items(self.dist_fo, self.location.clone())),
                portals: Some(&self.portals),
                animate: Some(&Animation {
                    atype: AniType::Player,
                    pos: (0, 0),
                    char: Some(i),
                    frame: None,
                }),
            });
            if poll(std::time::Duration::from_millis(500)).unwrap() {
                if let Event::Key(event) = read().unwrap() {
                    // log::info!("keykind {:?}", event.kind.clone());
                    let now = Instant::now();
                    if now.duration_since(self.last_event_time) > self.key_debounce_dur {
                        self.last_event_time = now;
                        let res = self.enc_key(event.code);
                    }
                }
            }
        }
        if self.player.get_health() == 0 {
            self.game_mode = GameMode::Fight(FightSteps::Null);
            return EncResult::Lose;
        }

        self.player_attack();
        let lturn = self.player.get_last_turn().1;
        let t_str: Vec<char> = lturn.to_string().chars().collect();
        let mut ani_frames = vec![
            ('\\', Color::White),
            ('X', Color::Green),
            ('/', Color::White),
        ];
        if let Some(weap) = self.player.get_equipped().get(&Equip::Weapon) {
            ani_frames.push(weap.icon);
        }
        for i in t_str {
            ani_frames.push((i, Color::Green));
        }
        let ppos = enemy.pos;
        for i in ani_frames {
            self.gui.encounter_auto_content(&mut GuiArgs {
                map: &self.map,
                player: &self.player,
                enemies: &self.enemies,
                items: &self.items,
                npcs: &self.npcs,
                env_inter: Some(&self.env_inters),
                litems: Some(&loc_shop_items(self.dist_fo, self.location.clone())),
                portals: Some(&self.portals),
                animate: Some(&Animation {
                    atype: AniType::Char,
                    pos: ppos,
                    char: Some(i),
                    frame: None,
                }),
            });
            if poll(std::time::Duration::from_millis(500)).unwrap() {
                if let Event::Key(event) = read().unwrap() {
                    // log::info!("keykind {:?}", event.kind.clone());
                    let now = Instant::now();
                    if now.duration_since(self.last_event_time) > self.key_debounce_dur {
                        self.last_event_time = now;
                        let res = self.enc_key(event.code);
                    }
                }
            }
        }
        let Interactable::Enemy(mut enemy) = self.interactee.clone() else {
            todo!()
        };
        if enemy.health == 0 {
            let epos = enemy.get_pos();
            self.enemies.remove(&epos);
            self.game_mode = GameMode::Fight(FightSteps::Null);
            return EncResult::Win;
        }
        EncResult::Cont
    }

    fn quick_encounter(&mut self) -> EncResult {
        let Interactable::Enemy(enemy) = self.interactee.clone() else {
            todo!()
        };
        // let mut e = ;
        let turn = self.enemy_turn(enemy.clone());
        if self.player.get_health() == 0 {
            self.game_mode = GameMode::Fight(FightSteps::Null);
            return EncResult::Lose;
        }

        self.player_attack();
        let lturn = self.player.get_last_turn();
        let Interactable::Enemy(mut enemy) = self.interactee.clone() else {
            todo!()
        };
        if enemy.health == 0 {
            let epos = enemy.get_pos();
            self.enemies.remove(&epos);
            self.game_mode = GameMode::Fight(FightSteps::Null);
            return EncResult::Win;
        }

        EncResult::Cont
    }

    fn manual_encounter(&mut self) -> EncResult {
        let Interactable::Enemy(enemy) = self.interactee.clone() else {
            todo!()
        };
        let mut e = enemy.clone();
        let pstart = false;
        if !pstart {
            let enatk = "Enemy is attacking.".to_string();
            loop {
                self.gui.encounter_show_content(
                    enatk.clone(),
                    &mut GuiArgs {
                        map: &self.map,
                        player: &self.player,
                        enemies: &self.enemies,
                        items: &self.items,
                        npcs: &self.npcs,
                        env_inter: Some(&self.env_inters),
                        litems: Some(&loc_shop_items(self.dist_fo, self.location.clone())),
                        portals: Some(&self.portals),
                        animate: None,
                    },
                );
                if poll(std::time::Duration::from_millis(100)).unwrap() {
                    if let Event::Key(event) = read().unwrap() {
                        // log::info!("keykind {:?}", event.kind.clone());
                        let now = Instant::now();
                        if now.duration_since(self.last_event_time) > self.key_debounce_dur {
                            self.last_event_time = now;
                            let res = self.enc_key(event.code);
                            if !res {
                                break;
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
                self.gui.encounter_show_content(
                    trn_res.clone(),
                    &mut GuiArgs {
                        map: &self.map,
                        player: &self.player,
                        enemies: &self.enemies,
                        items: &self.items,
                        npcs: &self.npcs,
                        env_inter: Some(&self.env_inters),
                        litems: Some(&loc_shop_items(self.dist_fo, self.location.clone())),
                        portals: Some(&self.portals),
                        animate: None,
                    },
                );
                if poll(std::time::Duration::from_millis(100)).unwrap() {
                    if let Event::Key(event) = read().unwrap() {
                        // log::info!("keykind {:?}", event.kind.clone());
                        let now = Instant::now();
                        if now.duration_since(self.last_event_time) > self.key_debounce_dur {
                            self.last_event_time = now;
                            let res = self.enc_key(event.code);
                            if !res {
                                break;
                            }
                        }
                    }
                }
            }
            if self.player.get_health() == 0 {
                self.game_mode = GameMode::Fight(FightSteps::Null);
                return EncResult::Lose;
            }
            self.game_mode = GameMode::Fight(FightSteps::Player);
        }
        //player turn
        //-player choice
        let popt = self.player.get_enc_opt();
        self.gui.reset_cursor();
        loop {
            self.gui.encounter_user_options(
                popt.clone(),
                &mut GuiArgs {
                    map: &self.map,
                    player: &self.player,
                    enemies: &self.enemies,
                    items: &self.items,
                    npcs: &self.npcs,
                    env_inter: Some(&self.env_inters),
                    litems: Some(&loc_shop_items(self.dist_fo, self.location.clone())),
                    portals: Some(&self.portals),
                    animate: None,
                },
            );
            if poll(std::time::Duration::from_millis(100)).unwrap() {
                if let Event::Key(event) = read().unwrap() {
                    // log::info!("keykind {:?}", event.kind.clone());
                    let now = Instant::now();
                    if now.duration_since(self.last_event_time) > self.key_debounce_dur {
                        self.last_event_time = now;
                        let res = self.enc_key(event.code);
                        if !res {
                            break;
                        }
                    }
                }
            }
        }
        let lturn = self.player.get_last_turn();
        self.player.set_enc_last_turn((EncOpt::Null, 0));
        let mut itm = false;
        let trn_res = match lturn {
            (EncOpt::Dodge, _) => "You dodged in an attempt to evade attack.".to_string(),
            (EncOpt::Attack, 0) => "You attempted an attack, but missed.".to_string(),
            (EncOpt::Attack, _) => {
                let ehp = if e.health > lturn.1 {
                    e.health - lturn.1
                } else {
                    0
                };
                let fmts = format!(
                    "You successfully attacked the {} for {}hp. They have an hp of: {}",
                    e.clone().get_sname(),
                    lturn.1,
                    ehp
                );
                fmts
            }
            (EncOpt::UseItem, _) => {
                itm = true;
                "".to_string()
            }
            _ => "OOPS!".to_string(),
        };
        self.gui.reset_cursor();
        loop {
            if itm {
                break;
            }
            self.gui.encounter_show_content(
                trn_res.clone(),
                &mut GuiArgs {
                    map: &self.map,
                    player: &self.player,
                    enemies: &self.enemies,
                    items: &self.items,
                    npcs: &self.npcs,
                    env_inter: Some(&self.env_inters),
                    litems: Some(&loc_shop_items(self.dist_fo, self.location.clone())),
                    portals: Some(&self.portals),
                    animate: None,
                },
            );
            if poll(std::time::Duration::from_millis(100)).unwrap() {
                if let Event::Key(event) = read().unwrap() {
                    // log::info!("keykind {:?}", event.kind.clone());
                    let now = Instant::now();
                    if now.duration_since(self.last_event_time) > self.key_debounce_dur {
                        self.last_event_time = now;
                        let res = self.enc_key(event.code);
                        if !res {
                            break;
                        }
                    }
                }
            }
        }
        let Interactable::Enemy(enemy) = self.interactee.clone() else {
            todo!()
        };
        e = enemy.clone();
        if e.health == 0 {
            let epos = e.get_pos();
            self.enemies.remove(&epos);
            self.game_mode = GameMode::Fight(FightSteps::Null);
            return EncResult::Win;
        }
        self.game_mode = GameMode::Fight(FightSteps::Enemy);
        EncResult::Cont
    }

    pub fn enemy_encounter(&mut self, mut e: Enemy) -> bool {
        //you are in fight
        let fst = format!("You are being attacked by a {}", e.get_sname());
        self.gui.reset_cursor();
        loop {
            self.gui.encounter_show_content(
                fst.clone(),
                &mut GuiArgs {
                    map: &self.map,
                    player: &self.player,
                    enemies: &self.enemies,
                    items: &self.items,
                    npcs: &self.npcs,
                    env_inter: Some(&self.env_inters),
                    litems: Some(&loc_shop_items(self.dist_fo, self.location.clone())),
                    portals: Some(&self.portals),
                    animate: None,
                },
            );
            if poll(std::time::Duration::from_millis(100)).unwrap() {
                if let Event::Key(event) = read().unwrap() {
                    // log::info!("keykind {:?}", event.kind.clone());
                    let now = Instant::now();
                    if now.duration_since(self.last_event_time) > self.key_debounce_dur {
                        self.last_event_time = now;
                        let res = self.enc_key(event.code);
                        if !res {
                            break;
                        }
                    }
                }
            }
        }
        //fight start
        self.game_mode = GameMode::Fight(FightSteps::Player);
        let enc_res = loop {
            let res = match self.enc_mode {
                EncMode::Auto => self.auto_encounter(),
                EncMode::Manual => self.manual_encounter(),
                EncMode::Quick => self.quick_encounter(),
                _ => self.manual_encounter(),
            };
            if res != EncResult::Cont {
                break res;
            }
        };
        //fight over
        let win_msg = if enc_res == EncResult::Win {
            self.enemy_drop(e.clone());
            format!("You defeated the {}!", e.get_sname())
        } else {
            format!("You were killed by the {}! You are dead", e.get_sname())
        };
        self.gui.reset_cursor();
        loop {
            self.gui.encounter_show_content(
                win_msg.clone(),
                &mut GuiArgs {
                    map: &self.map,
                    player: &self.player,
                    enemies: &self.enemies,
                    items: &self.items,
                    npcs: &self.npcs,
                    env_inter: Some(&self.env_inters),
                    litems: Some(&loc_shop_items(self.dist_fo, self.location.clone())),
                    portals: Some(&self.portals),
                    animate: None,
                },
            );
            if poll(std::time::Duration::from_millis(100)).unwrap() {
                if let Event::Key(event) = read().unwrap() {
                    // log::info!("keykind {:?}", event.kind.clone());
                    let now = Instant::now();
                    if now.duration_since(self.last_event_time) > self.key_debounce_dur {
                        self.last_event_time = now;
                        let res = self.inter_key(event.code);
                        if !res {
                            break;
                        }
                    }
                }
            }
        }
        if enc_res == EncResult::Win {
            true
        } else {
            self.game_mode = GameMode::Dead;
            false
        }
    }

    pub fn player_attack(&mut self) {
        let (atk, dmg) = self.player.get_enc_turn();
        let Interactable::Enemy(mut enemy) = self.interactee.clone() else {
            todo!()
        };
        let endef = enemy.get_defence();
        if atk > endef {
            enemy.apply_attack(dmg);
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
            self.gui.encounter_pick_item(&mut GuiArgs {
                map: &self.map,
                player: &self.player,
                enemies: &self.enemies,
                items: &self.items,
                npcs: &self.npcs,
                env_inter: Some(&self.env_inters),
                litems: Some(&loc_shop_items(self.dist_fo, self.location.clone())),
                portals: Some(&self.portals),
                animate: None,
            });
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
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    pub fn enc_option(&mut self) {
        let opt = self.enc;
        match opt {
            EncOpt::Attack => {
                self.player_attack();
            }
            EncOpt::UseItem => {
                self.enc_use_item();
                self.player.set_enc_last_turn((EncOpt::UseItem, 0));
            }
            EncOpt::Dodge => {
                self.player.toggle_dodge();
                self.player.set_enc_last_turn((EncOpt::Dodge, 0));
            }
            _ => {}
        }
    }

    pub fn check_place_enemies(&mut self, x: usize, y: usize) -> bool {
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
        //let nt = self.npcs.clone();
    }
}
