//enemy encounter

use crate::enemy::Enemy;
use crate::enums::{
    AniType, EncMode, EncOpt, EncResult, Enemies, Equip, ExpType, FightSteps, GameMode,
    Interactable, Items,
};
use crate::gamestate::{loc_shop_items, GameState};
use crate::gui_utils::{Animation, GuiArgs};
use crate::item::Item;
use ratatui::crossterm::event::{poll, read, Event, KeyCode};
use ratatui::style::Color;
use std::time::Instant;

impl GameState {
    pub fn enemy_turn(&mut self, e: Enemy) -> u16 {
        let (atk, mut dmg) = e.fight_turn();
        let pdef = self.player.get_defence();
        let dodge = self.player.get_dodge();
        let def_xp = self.stats.player_xp.get_xp(ExpType::Defence);
        if atk > pdef + def_xp.0 {
            if dodge {
                self.player.toggle_dodge();
                dmg /= 2;
            }
            self.player.apply_attack(dmg);
            self.stats.player_xp.inc_xp(ExpType::Defence, dmg);
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

        let asciis = self.enemy_asciis.clone();
        let ascii = match enemy.etype {
            Enemies::Spider => asciis.get("spider"),
            Enemies::Snake => asciis.get("snake"),
            Enemies::Slime => asciis.get("slime"),
            Enemies::Bandit => asciis.get("bandit"),
            Enemies::Goblin => asciis.get("goblin"),
            Enemies::Ghoul => asciis.get("ghoul"),
            Enemies::Bug => asciis.get("bug"),
            Enemies::CrazedExplorer => asciis.get("explorer"),
            Enemies::Golem => asciis.get("golem"),
            _ => None,
        };

        for i in ani_frames {
            self.gui.encounter_auto_content(&mut GuiArgs {
                map: &self.map,
                player: &self.player,
                stats: &self.stats.player_xp.get_xps(),
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
                ascii,
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
                stats: &self.stats.player_xp.get_xps(),
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
                ascii,
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
        let asciis = self.enemy_asciis.clone();
        let ascii = match enemy.etype {
            Enemies::Spider => asciis.get("spider"),
            Enemies::Snake => asciis.get("snake"),
            Enemies::Slime => asciis.get("slime"),
            Enemies::Bandit => asciis.get("bandit"),
            Enemies::Goblin => asciis.get("goblin"),
            Enemies::Ghoul => asciis.get("ghoul"),
            _ => None,
        };
        let pstart = false;
        if !pstart {
            let enatk = "Enemy is attacking.".to_string();
            self.gui.reset_cursor();
            loop {
                self.gui.encounter_show_content(
                    enatk.clone(),
                    vec!["Ok".to_string()],
                    &mut GuiArgs {
                        map: &self.map,
                        player: &self.player,
                        stats: &self.stats.player_xp.get_xps(),
                        enemies: &self.enemies,
                        items: &self.items,
                        npcs: &self.npcs,
                        env_inter: Some(&self.env_inters),
                        litems: Some(&loc_shop_items(self.dist_fo, self.location.clone())),
                        portals: Some(&self.portals),
                        animate: None,
                        ascii,
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
                    vec!["Ok".to_string()],
                    &mut GuiArgs {
                        map: &self.map,
                        player: &self.player,
                        stats: &self.stats.player_xp.get_xps(),
                        enemies: &self.enemies,
                        items: &self.items,
                        npcs: &self.npcs,
                        env_inter: Some(&self.env_inters),
                        litems: Some(&loc_shop_items(self.dist_fo, self.location.clone())),
                        portals: Some(&self.portals),
                        animate: None,
                        ascii,
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
                    stats: &self.stats.player_xp.get_xps(),
                    enemies: &self.enemies,
                    items: &self.items,
                    npcs: &self.npcs,
                    env_inter: Some(&self.env_inters),
                    litems: Some(&loc_shop_items(self.dist_fo, self.location.clone())),
                    portals: Some(&self.portals),
                    animate: None,
                    ascii,
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
                vec!["Ok".to_string()],
                &mut GuiArgs {
                    map: &self.map,
                    player: &self.player,
                    stats: &self.stats.player_xp.get_xps(),
                    enemies: &self.enemies,
                    items: &self.items,
                    npcs: &self.npcs,
                    env_inter: Some(&self.env_inters),
                    litems: Some(&loc_shop_items(self.dist_fo, self.location.clone())),
                    portals: Some(&self.portals),
                    animate: None,
                    ascii,
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
        let fst = format!(
            "You are being attacked by a {}. How would you like to run encounter?",
            e.get_sname()
        );
        self.gui.reset_cursor();
        let asciis = self.enemy_asciis.clone();
        let ascii = match e.etype {
            Enemies::Spider => asciis.get("spider"),
            Enemies::Bug => asciis.get("bug"),
            Enemies::Snake => asciis.get("snake"),
            Enemies::Slime => asciis.get("slime"),
            Enemies::Bandit => asciis.get("bandit"),
            Enemies::CrazedExplorer => asciis.get("explorer"),
            Enemies::Goblin => asciis.get("goblin"),
            Enemies::Ghoul => asciis.get("ghoul"),
            Enemies::Golem => asciis.get("golem"),
            _ => {
                println!("{:#?}", e.etype);
                None
            }
        };
        loop {
            self.gui.encounter_show_content(
                fst.clone(),
                vec![
                    "Auto".to_string(),
                    "Manual".to_string(),
                    "Quick".to_string(),
                ],
                &mut GuiArgs {
                    map: &self.map,
                    player: &self.player,
                    stats: &self.stats.player_xp.get_xps(),
                    enemies: &self.enemies,
                    items: &self.items,
                    npcs: &self.npcs,
                    env_inter: Some(&self.env_inters),
                    litems: Some(&loc_shop_items(self.dist_fo, self.location.clone())),
                    portals: Some(&self.portals),
                    animate: None,
                    ascii,
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
                vec!["Ok".to_string()],
                &mut GuiArgs {
                    map: &self.map,
                    player: &self.player,
                    stats: &self.stats.player_xp.get_xps(),
                    enemies: &self.enemies,
                    items: &self.items,
                    npcs: &self.npcs,
                    env_inter: Some(&self.env_inters),
                    litems: Some(&loc_shop_items(self.dist_fo, self.location.clone())),
                    portals: Some(&self.portals),
                    animate: None,
                    ascii,
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
        let atk_xp = self.stats.player_xp.get_xp(ExpType::Attack);
        if atk + atk_xp.0 > endef {
            let dmg_xp = self.stats.player_xp.get_xp(ExpType::Damage);
            enemy.apply_attack(dmg + dmg_xp.0);
            self.player.set_enc_last_turn((EncOpt::Attack, dmg));
            self.interactee = Interactable::Enemy(enemy.clone());
            self.stats.player_xp.inc_xp(ExpType::Attack, atk);
            self.stats.player_xp.inc_xp(ExpType::Damage, dmg);
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
                stats: &self.stats.player_xp.get_xps(),
                enemies: &self.enemies,
                items: &self.items,
                npcs: &self.npcs,
                env_inter: Some(&self.env_inters),
                litems: Some(&loc_shop_items(self.dist_fo, self.location.clone())),
                portals: Some(&self.portals),
                animate: None,
                ascii: None,
            });
            if poll(std::time::Duration::from_millis(100)).unwrap() {
                if let Event::Key(event) = read().unwrap() {
                    // log::info!("keykind {:?}", event.kind.clone());
                    let now = Instant::now();
                    if now.duration_since(self.last_event_time) > self.key_debounce_dur {
                        self.last_event_time = now;
                        match event.code {
                            KeyCode::Enter => {
                                self.use_inv_item();
                                self.gui.reset_enc_opt();
                                self.enc = EncOpt::Null;
                                break;
                            }
                            _ => {
                                let _ = self.key(event.code);
                            }
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
}
