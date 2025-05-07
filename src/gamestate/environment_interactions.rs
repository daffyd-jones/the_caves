//environment_interactions

use crate::gamestate::GameState;

use crate::enums::{EnvInter, GameMode, Items, PuzzleType};
use crate::gui_utils::GuiArgs;
use crate::item::Item;
use crate::utils::loc_shop_items;
use ratatui::crossterm::event::{poll, read, Event, KeyCode};
use std::collections::HashMap;
use std::time::Instant;

impl GameState {
    fn clinic(&mut self) -> bool {
        let mut paid = false;
        self.gui.reset_cursor();
        loop {
            self.gui.clinic_draw(&mut GuiArgs {
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
                            KeyCode::Up => {
                                self.gui.move_cursor("UP");
                            }
                            KeyCode::Down => {
                                self.gui.move_cursor("DN");
                            }
                            KeyCode::Left => {
                                self.gui.move_cursor("LF");
                            }
                            KeyCode::Right => {
                                self.gui.move_cursor("RT");
                            }
                            KeyCode::Char('a') => self.gui.move_cursor("LF"),
                            KeyCode::Char('s') => self.gui.move_cursor("UP"),
                            KeyCode::Char('d') => self.gui.move_cursor("DN"),
                            KeyCode::Char('f') => self.gui.move_cursor("RT"),
                            KeyCode::Enter => {
                                if self.gui.get_ysno() {
                                    if self.player.dec_money(20) {
                                        self.player.heal_player();
                                        paid = true;
                                        break;
                                    }
                                    break;
                                }
                                self.game_mode = GameMode::Play;
                                return true;
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        let resp_string = if paid {
            "Thanks for coming by! Hope you're feeling better!".to_string()
        } else {
            "It seems that you dont have enough money, sorry about that. Medical supplies are scarce down here and we cant do it for free.".to_string()
        };

        self.gui.reset_cursor();
        loop {
            self.gui.clinic_resp_draw(
                resp_string.clone(),
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
                        match event.code {
                            KeyCode::Up => {
                                self.gui.move_cursor("UP");
                            }
                            KeyCode::Down => {
                                self.gui.move_cursor("DN");
                            }
                            KeyCode::Left => {
                                self.gui.move_cursor("LF");
                            }
                            KeyCode::Right => {
                                self.gui.move_cursor("RT");
                            }
                            KeyCode::Char('a') => self.gui.move_cursor("LF"),
                            KeyCode::Char('s') => self.gui.move_cursor("UP"),
                            KeyCode::Char('d') => self.gui.move_cursor("DN"),
                            KeyCode::Char('f') => self.gui.move_cursor("RT"),
                            KeyCode::Enter => {
                                break;
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
        self.game_mode = GameMode::Play;
        true
    }

    fn guild_post(&mut self) -> bool {
        let local_puzzles = self.puzzles.get_local_puzzles(self.dist_fo);
        let mut ppost_strings = Vec::new();
        let pos = self.dist_fo;
        for (ppos, mut p) in local_puzzles {
            let dx = ppos.0 + pos.0;
            let dy = ppos.1 + pos.1;
            let xdir = dx / dx.abs();
            let ydir = dy / dy.abs();
            let dir_string = match (xdir, ydir) {
                (xdir, ydir) if xdir <= 0 && ydir <= 0 => "North East".to_string(),
                (xdir, ydir) if xdir <= 0 && ydir > 0 => "South East".to_string(),
                (xdir, ydir) if xdir > 0 && ydir <= 0 => "North West".to_string(),
                (xdir, ydir) if xdir > 0 && ydir > 0 => "South West".to_string(),
                _ => "dunno".to_string(),
            };
            let ptype_string = {
                match p.get_ptype() {
                    PuzzleType::Maze => "Maze".to_string(),
                    PuzzleType::Teleport => "Teleport".to_string(),
                    PuzzleType::Inverted => "Inverted".to_string(),
                    _ => "dunno".to_string(),
                }
            };
            let f_str = format!("{}#{}", ptype_string, dir_string);
            ppost_strings.push(f_str.clone());
        }
        self.gui.reset_cursor();
        loop {
            self.gui.guild_post_draw(
                ppost_strings.clone(),
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
                        match event.code {
                            KeyCode::Up => {
                                self.gui.move_cursor("UP");
                            }
                            KeyCode::Down => {
                                self.gui.move_cursor("DN");
                            }
                            KeyCode::Left => {
                                self.gui.move_cursor("LF");
                            }
                            KeyCode::Right => {
                                self.gui.move_cursor("RT");
                            }
                            KeyCode::Char('a') => self.gui.move_cursor("LF"),
                            KeyCode::Char('s') => self.gui.move_cursor("UP"),
                            KeyCode::Char('d') => self.gui.move_cursor("DN"),
                            KeyCode::Char('f') => self.gui.move_cursor("RT"),
                            KeyCode::Enter => {
                                break;
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
        self.game_mode = GameMode::Play;
        true
    }

    fn church_post(&mut self) -> bool {
        let pos = self.dist_fo;
        let local_settles = self.settles.get_local_settles(pos);
        let mut settles = Vec::new();
        let loc_pos = self.location_pos();
        for (spos, mut s) in local_settles {
            if spos.0 != loc_pos.0 && spos.1 != loc_pos.1 {
                let dx = spos.0 + pos.0;
                let dy = spos.1 + pos.1;
                let xdir = dx / dx.abs();
                let ydir = dy / dy.abs();
                let dir_string = match (xdir, ydir) {
                    (xdir, ydir) if xdir <= 0 && ydir <= 0 => "North East".to_string(),
                    (xdir, ydir) if xdir <= 0 && ydir > 0 => "South East".to_string(),
                    (xdir, ydir) if xdir > 0 && ydir <= 0 => "North West".to_string(),
                    (xdir, ydir) if xdir > 0 && ydir > 0 => "South West".to_string(),
                    _ => "dunno".to_string(),
                };
                let settle_name = s.get_sname();
                let settle_str = format!("{}#{}", settle_name.clone(), dir_string.clone());
                settles.push(settle_str);
            }
        }

        self.gui.reset_cursor();
        loop {
            self.gui.church_post_draw(
                settles.clone(),
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
                        match event.code {
                            KeyCode::Up => {
                                self.gui.move_cursor("UP");
                            }
                            KeyCode::Down => {
                                self.gui.move_cursor("DN");
                            }
                            KeyCode::Left => {
                                self.gui.move_cursor("LF");
                            }
                            KeyCode::Right => {
                                self.gui.move_cursor("RT");
                            }
                            KeyCode::Char('a') => self.gui.move_cursor("LF"),
                            KeyCode::Char('s') => self.gui.move_cursor("UP"),
                            KeyCode::Char('d') => self.gui.move_cursor("DN"),
                            KeyCode::Char('f') => self.gui.move_cursor("RT"),
                            KeyCode::Enter => {
                                break;
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
        self.game_mode = GameMode::Play;
        true
    }

    fn cauldron(&mut self) -> bool {
        let inv: Vec<Item> = self
            .player
            .get_inventory()
            .into_iter()
            .filter(|item| item.craft)
            .collect();
        let mut craft_tallys: HashMap<Items, (u16, u16, Items)> = HashMap::new();
        for i in &inv {
            if let Some(c) = craft_tallys.get(&i.itype) {
                craft_tallys.insert(i.itype, (c.0 + 1, c.1, c.2));
            } else {
                craft_tallys.insert(
                    i.itype,
                    (1, *i.properties.get("required").unwrap(), i.produces),
                );
            }
        }
        // println!("{:#?}", &craft_tallys);
        let mut products = Vec::new();
        for (_, tally) in &craft_tallys {
            if tally.0 >= tally.1 {
                products.push(tally.2);
            }
        }
        // println!("{:#?}", &products);

        self.gui.reset_cursor();
        loop {
            self.gui.cauldron_draw(
                &products,
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
                        match event.code {
                            KeyCode::Up => {
                                self.gui.move_cursor("UP");
                            }
                            KeyCode::Down => {
                                self.gui.move_cursor("DN");
                            }
                            KeyCode::Left => {
                                self.gui.move_cursor("LF");
                            }
                            KeyCode::Right => {
                                self.gui.move_cursor("RT");
                            }
                            KeyCode::Char('a') => self.gui.move_cursor("LF"),
                            KeyCode::Char('s') => self.gui.move_cursor("UP"),
                            KeyCode::Char('d') => self.gui.move_cursor("DN"),
                            KeyCode::Char('f') => self.gui.move_cursor("RT"),
                            KeyCode::Enter => {
                                if products.is_empty() {
                                    break;
                                }
                                let cur = self.gui.get_cursor();
                                let itype = products[cur.1];
                                let mut inventory = self.player.inventory.clone();
                                inventory.push(match itype {
                                    Items::HealthPotion => Item::new_health_potion(0, 0),
                                    Items::Salve => Item::new_salve(0, 0),
                                    _ => Item::default(),
                                });
                                let craft_cln = craft_tallys.clone();
                                let craft = {
                                    let mut item = (Items::Null, 0);
                                    for (i, c) in craft_cln {
                                        if c.2 == itype {
                                            item = (i, c.1);
                                        }
                                    }
                                    item
                                };
                                let mut cnt = 0;
                                let mut idcs = Vec::new();
                                let _ = inventory
                                    .iter()
                                    .enumerate()
                                    .map(|(x, i)| {
                                        if i.itype == craft.0 && cnt < craft.1 {
                                            idcs.push(x);
                                            cnt += 1;
                                        }
                                    })
                                    .collect::<Vec<_>>();
                                for i in idcs.iter().rev() {
                                    inventory.remove(*i);
                                }

                                self.player.inventory = inventory;

                                break;
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
        self.game_mode = GameMode::Play;

        true
    }

    pub fn env_interaction(&mut self, env_inter: EnvInter) -> bool {
        match env_inter {
            EnvInter::Records => self.save_game(),
            EnvInter::Clinic => self.clinic(),
            EnvInter::GuildPost => self.guild_post(),
            EnvInter::ChurchPost => self.church_post(),
            EnvInter::Cauldron => self.cauldron(),
            _ => todo!(),
        }
    }
}
