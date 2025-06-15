//settle_state

use crate::enums::{GUIMode, GameMode, Interactable, Location, NPCWrap};
use crate::gamestate::GameState;
use crate::gui_utils::GuiArgs;
use crate::map::{MAP_H, MAP_W};
use crate::npc::NPC;
use crate::settlement::Settlement;
use crate::shop::Shop;

use crate::item::Item;
use std::time::Instant;

use crate::gamestate::loc_shop_items;
use crate::npc_utils::box_npc;
use crate::npc_utils::wrap_nbox;
use ratatui::crossterm::event::{poll, read, Event, KeyCode};

impl GameState {
    pub fn get_cur_settle_name(&mut self) -> String {
        let loc = self.location.clone();
        match loc {
            Location::Settlement(mut settle) => settle.get_sname().replace(" ", "_"),
            _ => "oops".to_string(),
        }
    }

    pub fn get_shop_from_item(&mut self, mut item: Item) -> Shop {
        let ipos = item.get_pos();
        // log::info!("shop item \n{:?}", item.clone());
        match self.location.clone() {
            Location::Settlement(mut settle) => {
                if let Some(shop) = settle.get_shop_from_item_pos((
                    ipos.0 as i16 - self.dist_fo.0,
                    ipos.1 as i16 - self.dist_fo.1,
                )) {
                    shop
                } else {
                    Shop::default()
                }
            }
            _ => todo!(),
        }
    }

    pub fn update_settlement(&mut self, mut settle: Settlement) -> Location {
        let lpos = settle.get_pos();
        let pos = self.dist_fo;
        let dx = (lpos.0 + pos.0) as usize;
        let dy = (lpos.1 + pos.1) as usize;
        if dx < MAP_W && dy < MAP_H && !settle.get_npcs_sent() {
            let sitems = settle.get_items();
            for ((_x, _y), mut i) in sitems {
                let ipos = i.get_pos();
                if pos == (0, 0) {
                    let npos = (
                        (self.dist_fo.0 + ipos.0 as i16 + lpos.0) as usize,
                        (self.dist_fo.1 + ipos.1 as i16 + lpos.1) as usize,
                    );
                    i.set_pos(npos);
                    self.items.insert(npos, i.clone());
                } else {
                    let npos = (
                        (self.dist_fo.0 + ipos.0 as i16 + lpos.0) as usize,
                        (self.dist_fo.1 + ipos.1 as i16 + lpos.1) as usize,
                    );
                    i.set_pos(npos);
                    self.items.insert(npos, i.clone());
                }
            }
            let tnpcs = settle.get_npcs();
            for ((x, y), n) in tnpcs {
                let mut nbox = box_npc(n);
                let nwpos = (
                    (self.dist_fo.0 + x as i16 + lpos.0) as usize,
                    (self.dist_fo.1 + y as i16 + lpos.1) as usize,
                );
                nbox.set_pos(nwpos);
                self.npcs.insert(nwpos, wrap_nbox(nbox));
            }
            let ten_inters = settle.get_env_inters();
            for ((x, y), ei) in ten_inters {
                let nwpos = (
                    (self.dist_fo.0 + x as i16 + lpos.0) as usize,
                    (self.dist_fo.1 + y as i16 + lpos.1) as usize,
                );
                self.env_inters.insert(nwpos, ei);
            }

            settle.tog_npcs_sent();
        }
        Location::Settlement(settle.clone())
    }

    pub fn buy_item(&mut self) {
        let mut item = {
            match self.interactee.clone() {
                Interactable::ShopItem(sitem) => sitem,
                _ => todo!(),
            }
        };
        let mut shop = self.get_shop_from_item(item.clone());
        let price = item.get_properties()["value"];
        let paid = self.player.dec_money(price);
        if paid {
            self.player.add_to_inv(item.clone());
            let ipos = item.get_pos();
            let mut loc = match self.location.clone() {
                Location::Settlement(settle) => settle,
                _ => todo!(),
            };
            let lpos = loc.get_pos();
            shop.set_paid(true);
            shop.remove_item((
                (ipos.0 as i16 - lpos.0 - self.dist_fo.0) as usize,
                (ipos.1 as i16 - lpos.1 - self.dist_fo.1) as usize,
            ));
            loc.update_shop(shop);
            self.location = Location::Settlement(loc);
        } else {
            shop.set_paid(false);
            let mut loc = match self.location.clone() {
                Location::Settlement(settle) => settle,
                _ => todo!(),
            };
            loc.update_shop(shop);
            self.location = Location::Settlement(loc);
        }
    }

    pub fn shop_key(&mut self, code: KeyCode) -> (bool, bool) {
        match code {
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
            KeyCode::Char('p') => self.gui.set_info_mode(GUIMode::Bug),
            KeyCode::Char('o') => self.gui.set_info_mode(GUIMode::Normal),
            KeyCode::Char('z') => {
                self.gui.set_info_mode(GUIMode::Normal);
                self.game_mode = GameMode::Play;
            }
            KeyCode::Char('a') => self.gui.move_cursor("LF"),
            KeyCode::Char('s') => self.gui.move_cursor("UP"),
            KeyCode::Char('d') => self.gui.move_cursor("DN"),
            KeyCode::Char('f') => self.gui.move_cursor("RT"),
            KeyCode::Enter => {
                let buy = self.gui.get_ysno();
                if buy {
                    self.buy_item();
                    return (false, true);
                } else {
                    return (false, false);
                }
            }
            _ => {}
        }
        (true, false)
    }

    pub fn shop_item_interaction(&mut self, mut sitem: Item) -> bool {
        let shop = self.get_shop_from_item(sitem.clone());
        let npc = shop.get_npc();
        let (sname, sh_convo) = match npc {
            NPCWrap::ShopNPC(mut snpc) => (snpc.get_sname(), snpc.get_sh_conv()),
            _ => todo!(),
        };
        let iprice = sitem.get_properties()["value"].to_string();
        let dialogue_temp = &sh_convo["item_desc"];
        let sh_dialogue = dialogue_temp
            .replace("{i}", &sitem.get_sname())
            .replace("{v}", &iprice);
        let mut buy_item = false;
        self.gui.reset_cursor();
        loop {
            self.gui.shop_convo_draw(
                sname.clone(),
                sh_dialogue.clone(),
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
                    ascii: None,
                },
            );
            if poll(std::time::Duration::from_millis(100)).unwrap() {
                if let Event::Key(event) = read().unwrap() {
                    let now = Instant::now();
                    if now.duration_since(self.last_event_time) > self.key_debounce_dur {
                        self.last_event_time = now;
                        let res = self.shop_key(event.code);
                        if !res.0 {
                            buy_item = res.1;
                            break;
                        }
                    }
                }
            }
        }
        let mut nshop = self.get_shop_from_item(sitem.clone());
        let resp_dialogue = {
            if buy_item {
                if nshop.get_paid() {
                    &sh_convo["item_bought"]
                } else {
                    &sh_convo["item_broke"]
                }
            } else {
                &sh_convo["item_nbought"]
            }
        };
        self.gui.reset_cursor();
        loop {
            self.gui.shop_convo_draw(
                sname.clone(),
                resp_dialogue.clone(),
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
                    ascii: None,
                },
            );
            if poll(std::time::Duration::from_millis(100)).unwrap() {
                if let Event::Key(event) = read().unwrap() {
                    let now = Instant::now();
                    if now.duration_since(self.last_event_time) > self.key_debounce_dur {
                        self.last_event_time = now;
                        match event.code {
                            KeyCode::Enter => {
                                break;
                            }
                            _ => todo!(),
                        }
                    }
                }
            }
        }
        self.game_mode = GameMode::Play;
        true
    }
}
