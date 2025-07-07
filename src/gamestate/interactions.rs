//interactions
use crate::enums::{
    FightSteps, GUIMode, GameMode, InterOpt, InterSteps, Interactable, ItemOpt, Location,
};
use crate::gamestate::GameState;
use crate::gui_utils::GuiArgs;
use crate::item::Item;
use crate::utils::loc_shop_items;

use ratatui::crossterm::event::{poll, read, Event};
use std::collections::HashMap;
use std::time::Instant;

impl GameState {
    pub fn start_interact(&mut self) {
        let (px, py) = self.player.pos();
        let adj = vec![
            (px, (py as isize - 1) as usize),
            (px, py + 1),
            ((px as isize - 1) as usize, py),
            (px + 1, py),
        ];
        let mut adj_inter = HashMap::new();
        for (x, y) in &adj {
            if let Some(item) = self.items.get(&(*x, *y)) {
                adj_inter.insert((*x, *y), Some(Interactable::Item(item.clone())));
            }
            if let Some(enemy) = self.enemies.get(&(*x, *y)) {
                adj_inter.insert((*x, *y), Some(Interactable::Enemy(enemy.clone())));
            }
            if let Some(npc) = self.npcs.get(&(*x, *y)) {
                adj_inter.insert((*x, *y), Some(Interactable::NPC(npc.clone())));
            }
            if let Some(env_inter) = self.env_inters.get(&(*x, *y)) {
                adj_inter.insert((*x, *y), Some(Interactable::EnvInter(*env_inter)));
            }
            if self.location != Location::Null {
                let st = loc_shop_items(self.dist_fo, self.location.clone());
                if let Some(sitm) = st.get(&(*x, *y)) {
                    adj_inter.insert((*x, *y), Some(Interactable::ShopItem(sitm.clone())));
                }
            }
        }
        if !adj_inter.is_empty() {
            self.game_mode = GameMode::Interact(InterSteps::AdjOpt);
            // self.gui.set_info_mode(GUIMode::Interact);
            self.gui.set_interactable(adj_inter);
        }
    }

    pub fn get_interactee(&mut self, pos: (usize, usize)) -> Option<Interactable> {
        if let Some(item) = self.items.get(&pos) {
            Some(Interactable::Item(item.clone()))
        } else if let Some(sitem) = loc_shop_items(self.dist_fo, self.location.clone()).get(&pos) {
            Some(Interactable::ShopItem(sitem.clone()))
        } else if let Some(enemy) = self.enemies.get(&pos) {
            Some(Interactable::Enemy(enemy.clone()))
        } else if let Some(npc) = self.npcs.get(&pos) {
            Some(Interactable::NPC(npc.clone()))
        } else if let Some(env_inter) = self.env_inters.get(&pos) {
            Some(Interactable::EnvInter(*env_inter))
        } else {
            Some(Interactable::Null)
        }
    }

    pub fn select_adj(&mut self) {
        let (pos, st) = self.gui.get_interactee();
        let Some(intee) = self.get_interactee(pos) else {
            todo!()
        };
        log::info!("intee: {:?}", intee);
        self.interactee = intee.clone();
        match intee {
            Interactable::Item(item) => {
                // self.item_opt(item.clone());
                self.gui.set_inter_opt(item.iopts);
            }
            Interactable::ShopItem(sitem) => {}
            Interactable::Enemy(enemy) => {}
            Interactable::NPC(npc) => {}
            Interactable::EnvInter(env_inter) => {}
            Interactable::Null => {}
            _ => todo!(),
        }
    }

    pub fn pickup_item(&mut self, item: Item) {
        self.player.add_to_inv(item.clone());
        if let Some(itm) = self.items.remove(&(item.x, item.y)) {}
    }

    pub fn select_opt(&mut self) {
        let (opt, _) = self.gui.get_iopt();
        match opt {
            InterOpt::Item(item_opt) => match item_opt {
                ItemOpt::PickUp => {
                    let Interactable::Item(item) = self.interactee.clone() else {
                        todo!()
                    };
                    self.pickup_item(item);
                }
                ItemOpt::Drp => {}
                ItemOpt::Use => {}
                _ => todo!(),
            },
            _ => todo!(),
        }
    }

    pub fn item_interaction(&mut self) -> bool {
        self.gui.reset_cursor();
        loop {
            self.gui.inter_opt_draw(&mut GuiArgs {
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
                        let res = self.inter_key(event.code);
                        if !res {
                            break;
                        }
                    }
                }
            }
        }
        self.gui.reset_cursor();
        loop {
            self.gui.inter_res_draw(&mut GuiArgs {
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
                        let res = self.inter_key(event.code);
                        if !res {
                            break;
                        }
                    }
                }
            }
        }
        self.gui.reset_cursor();
        // self.gui.set_info_mode(GUIMode::Normal);
        true
    }

    pub fn interaction(&mut self) -> bool {
        self.gui.reset_cursor();
        loop {
            self.gui.inter_adj_draw(&mut GuiArgs {
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
                        let res = self.inter_key(event.code);
                        if !res {
                            break;
                        }
                    }
                }
            }
        }

        let intee = self.interactee.clone();
        log::info!("intee1: {:#?}", intee);
        let res = match intee {
            Interactable::Item(_) => self.item_interaction(),
            Interactable::ShopItem(si) => self.shop_item_interaction(si),
            Interactable::NPC(_) => self.npc_interaction(),
            Interactable::Enemy(e) => {
                self.game_mode = GameMode::Fight(FightSteps::Open);
                self.enemy_encounter(e)
            }
            Interactable::EnvInter(env_inter) => self.env_interaction(env_inter),
            Interactable::Null => false,
            _ => todo!(),
        };
        self.gui.set_info_mode(GUIMode::Normal);
        if !res {
            return false;
        }
        true
    }
}
