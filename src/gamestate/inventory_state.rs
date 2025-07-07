//inventory_state

use crate::enums::{GUIMode, GameMode, InterOpt, ItemOpt};
use crate::gamestate::GameState;
use crate::gui_utils::GuiArgs;
use crate::item::Item;
use crate::utils::loc_shop_items;

use ratatui::crossterm::event::{poll, read, Event, KeyCode};
use std::time::Instant;

impl GameState {
    pub fn inv_use_opt(&mut self, mut item: Item) -> ItemOpt {
        let iopts = item.get_iopts();
        //iopts.remove(&ItemOpt::PickUp);
        let msg_str = format!("What would you like to do with the {}?", item.get_sname());

        let mut useable = false;
        let opts_str = if iopts.contains_key(&InterOpt::Item(ItemOpt::Use)) {
            useable = true;
            "Use#Drop#Back".to_string()
        } else {
            "Drop#Back".to_string()
        };

        self.gui.reset_cursor();
        loop {
            self.gui.item_use_draw(
                msg_str.clone(),
                opts_str.clone(),
                &mut GuiArgs {
                    map: &self.map,
                    player: &self.player,
                    // stats: &self.stats.player_xp.get_xps(),
                    enemies: &self.enemies,
                    items: &self.items,
                    npcs: &self.npcs,
                    env_inter: Some(&self.env_inters),
                    litems: Some(&loc_shop_items(self.dist_fo, self.location.clone())),
                    portals: Some(&self.portals),
                    animate: None,
                    ascii: None,
                },
            );
            if poll(std::time::Duration::from_millis(100)).unwrap() {
                if let Event::Key(event) = read().unwrap() {
                    // log::info!("keykind {:?}", event.kind.clone());
                    let now = Instant::now();
                    if now.duration_since(self.last_event_time) > self.key_debounce_dur {
                        self.last_event_time = now;
                        match event.code {
                            KeyCode::Enter => {
                                let res = self.gui.get_cursor();
                                return match res.0 {
                                    0 if useable => ItemOpt::Use,
                                    0 if !useable => ItemOpt::Drp,
                                    1 if useable => ItemOpt::Drp,
                                    1 if !useable => ItemOpt::Null,
                                    2 if useable => ItemOpt::Null,
                                    _ => todo!(),
                                };
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

    pub fn use_inv_item(&mut self) {
        let (idx, mut item) = self.gui.get_inv_opt();
        //gui, using item

        if item.is_equip() {
            if self.confirm_equip(item.clone()) {
                self.player.add_equip(item.clone());
            }
            return;
        } else {
            match self.inv_use_opt(item.clone()) {
                ItemOpt::Use => {
                    self.player.apply_item_effect(item.clone());
                    self.player.rem_inv_item(idx);
                    self.gui.set_inventory(self.player.get_inventory());
                }
                ItemOpt::Drp => {
                    self.player.rem_inv_item(idx);
                    self.gui.set_inventory(self.player.get_inventory());
                }
                ItemOpt::Null => {
                    return;
                }
                _ => todo!(),
            }
        }
        self.gui.reset_cursor();
        match self.game_mode {
            GameMode::Play => {
                loop {
                    self.gui.item_used_draw(&mut GuiArgs {
                        map: &self.map,
                        player: &self.player,
                        // stats: &self.stats.player_xp.get_xps(),
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
                                        break;
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                }
            }
            GameMode::Fight(_) => {
                let itstr = format!("You used the {}", item.clone().get_sname());
                loop {
                    self.gui.encounter_show_content(
                        itstr.clone(),
                        vec!["Ok".to_string()],
                        &mut GuiArgs {
                            map: &self.map,
                            player: &self.player,
                            // stats: &self.stats.player_xp.get_xps(),
                            enemies: &self.enemies,
                            items: &self.items,
                            npcs: &self.npcs,
                            env_inter: Some(&self.env_inters),
                            litems: Some(&loc_shop_items(self.dist_fo, self.location.clone())),
                            portals: Some(&self.portals),
                            animate: None,
                            ascii: None,
                        },
                    );
                    if poll(std::time::Duration::from_millis(100)).unwrap() {
                        if let Event::Key(event) = read().unwrap() {
                            // log::info!("keykind {:?}", event.kind.clone());
                            let now = Instant::now();
                            if now.duration_since(self.last_event_time) > self.key_debounce_dur {
                                self.last_event_time = now;
                                match event.code {
                                    KeyCode::Enter => {
                                        break;
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }

    pub fn confirm_equip(&mut self, mut item: Item) -> bool {
        let msg_str = format!("Would you like to equip the {}?", item.get_sname());
        let iopts = "Yes#No".to_string();
        self.gui.reset_cursor();
        loop {
            self.gui.item_use_draw(
                msg_str.clone(),
                iopts.clone(),
                &mut GuiArgs {
                    map: &self.map,
                    player: &self.player,
                    // stats: &self.stats.player_xp.get_xps(),
                    enemies: &self.enemies,
                    items: &self.items,
                    npcs: &self.npcs,
                    env_inter: Some(&self.env_inters),
                    litems: Some(&loc_shop_items(self.dist_fo, self.location.clone())),
                    portals: Some(&self.portals),
                    animate: None,
                    ascii: None,
                },
            );
            if poll(std::time::Duration::from_millis(100)).unwrap() {
                if let Event::Key(event) = read().unwrap() {
                    // log::info!("keykind {:?}", event.kind.clone());
                    let now = Instant::now();
                    if now.duration_since(self.last_event_time) > self.key_debounce_dur {
                        self.last_event_time = now;
                        match event.code {
                            KeyCode::Enter => {
                                return self.gui.get_ysno();
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
}
