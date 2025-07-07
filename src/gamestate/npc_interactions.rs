//npc_interactions

use crate::enums::{GUIMode, GameMode, Interactable, Items, NPCWrap};
use crate::gamestate::GameState;
use crate::gamestate::Item;
use crate::gui_utils::{DisplayStats, GuiArgs};
use crate::npc::{ConvNPC, Convo, ShopNPC, SpawnNPC, TradeNPC, NPC};
use crate::npc_utils::box_npc;
use std::time::Instant;

use crate::gamestate::loc_shop_items;
use rand::prelude::SliceRandom;
use ratatui::crossterm::event::{poll, read, Event, KeyCode};

impl GameState {
    pub fn npc_comm_inter(&mut self, npc_name: String, npc_comm: String) -> Vec<String> {
        let comms = format!("{}#{}", npc_name, npc_comm);
        let mut conv_acc = Vec::new();
        conv_acc.push(npc_comm);
        self.gui.reset_cursor();
        loop {
            self.gui.npc_comm_draw(
                comms.clone(),
                &mut GuiArgs {
                    map: &self.map,
                    player: &self.player,
                    // stats: &DisplayStats {
                    //     player: Vec::new(),
                    //     notes: (String::from(""), String::from("")),
                    // },
                    enemies: &self.enemies,
                    items: &self.items,
                    npcs: &self.npcs,
                    env_inter: Some(&self.env_inters),
                    litems: Some(&loc_shop_items(self.dist_fo, self.location.clone())),
                    portals: Some(&self.portals),
                    animate: None,
                    ascii: Some(&self.npc_asciis[0].clone()),
                },
            );
            if poll(std::time::Duration::from_millis(100)).unwrap() {
                if let Event::Key(event) = read().unwrap() {
                    let now = Instant::now();
                    if now.duration_since(self.last_event_time) > self.key_debounce_dur {
                        self.last_event_time = now;
                        let res = self.comm_key(event.code);
                        if !res {
                            break;
                        }
                    }
                }
            }
        }

        conv_acc
    }

    pub fn conv_step(
        &mut self,
        conv: Convo,
        step: String,
        name: String,
        mut conv_acc: Vec<String>,
    ) -> Vec<String> {
        if step == *"e" {
            self.game_mode = GameMode::Play;
            self.gui.set_info_mode(GUIMode::Normal);
            return conv_acc;
        }
        let stage = &conv.stages[&step];
        let text = &stage.text;
        conv_acc.push(text.clone());
        let opts = &stage.opts;
        let mut opts_vec = Vec::new();
        for o in opts {
            opts_vec.push(o.text.clone());
        }
        self.gui.reset_cursor();
        loop {
            self.gui.npc_conv_draw(
                name.clone(),
                text.clone(),
                opts_vec.clone(),
                &mut GuiArgs {
                    map: &self.map,
                    player: &self.player,
                    // stats: &DisplayStats {
                    //     player: Vec::new(),
                    //     notes: (String::from(""), String::from("")),
                    // },
                    enemies: &self.enemies,
                    items: &self.items,
                    npcs: &self.npcs,
                    env_inter: Some(&self.env_inters),
                    litems: Some(&loc_shop_items(self.dist_fo, self.location.clone())),
                    portals: Some(&self.portals),
                    animate: None,
                    ascii: Some(&self.npc_asciis[0].clone()),
                },
            );
            if poll(std::time::Duration::from_millis(100)).unwrap() {
                if let Event::Key(event) = read().unwrap() {
                    let now = Instant::now();
                    if now.duration_since(self.last_event_time) > self.key_debounce_dur {
                        self.last_event_time = now;
                        match event.code {
                            KeyCode::Enter => {
                                let cursor = self.gui.get_cursor();
                                let next = &opts[cursor.1].next;
                                conv_acc.push(opts_vec[cursor.1].clone());
                                return self.conv_step(
                                    conv.clone(),
                                    next.to_string(),
                                    name.clone(),
                                    conv_acc,
                                );
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

    pub fn npc_conv_inter(&mut self, mut npc: ConvNPC) -> Vec<String> {
        let convo = npc.get_conv();
        let name = npc.get_sname();
        self.conv_step(convo, "0".to_string(), name, Vec::new())
    }

    pub fn npc_spawn_inter(&mut self, mut npc: SpawnNPC) -> Vec<String> {
        if npc.is_spawned() {
            return self.npc_comm_inter(npc.get_sname(), npc.get_comm());
        }
        npc.toggle_spawned();
        let spwn_conv = npc.get_conv();
        // let ptype = npc.get_ptype();
        let name = npc.get_sname();
        self.conv_step(spwn_conv, "0".to_string(), name, Vec::new())
    }

    pub fn npc_shop_inter(&mut self, mut npc: ShopNPC) -> Vec<String> {
        let convo = npc.get_convo();
        let name = npc.get_sname();
        self.conv_step(convo, "0".to_string(), name, Vec::new())
    }

    pub fn trade_buy(&mut self, mut item: Item) -> bool {
        let mut p = self.player.clone();
        let price = item.get_properties()["value"];
        if p.add_to_inv(item.clone()) && p.dec_money(price) {
            self.player = p.clone();
            return true;
        }
        false
    }

    pub fn trade_buy_items(&mut self, mut items: Vec<Item>) -> bool {
        self.gui.reset_cursor();
        loop {
            self.gui.npc_trade_draw(
                items.clone(),
                &mut GuiArgs {
                    map: &self.map,
                    player: &self.player,
                    // stats: &DisplayStats {
                    //     player: Vec::new(),
                    //     notes: (String::from(""), String::from("")),
                    // },
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
                                let inv_opt = self.gui.get_inv_opt();
                                if !self.trade_buy(inv_opt.1) {
                                    break;
                                }
                                let mut npc = match self.interactee.clone() {
                                    Interactable::NPC(NPCWrap::TradeNPC(npc)) => npc,
                                    _ => todo!(),
                                };
                                npc.remove_item(inv_opt.0);
                                items = npc.get_items();
                                self.interactee = Interactable::NPC(NPCWrap::TradeNPC(npc));
                            }
                            KeyCode::Backspace => {
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
        true
    }

    pub fn trade_sell(&mut self, mut item: (usize, Item)) -> bool {
        let price = item.1.get_properties()["value"];
        self.player.inc_money(price);
        self.player.rem_inv_item(item.0);
        true
    }

    pub fn trade_sell_items(&mut self, mut items: Vec<Item>) -> bool {
        self.gui.reset_cursor();
        loop {
            self.gui.npc_trade_draw(
                items.clone(),
                &mut GuiArgs {
                    map: &self.map,
                    player: &self.player,
                    // stats: &DisplayStats {
                    //     player: Vec::new(),
                    //     notes: (String::from(""), String::from("")),
                    // },
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
                                let inv_opt = self.gui.get_inv_opt();
                                if !self.trade_sell(inv_opt.clone()) {
                                    break;
                                }
                                items.remove(inv_opt.0);
                            }
                            KeyCode::Backspace => {
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
        true
    }

    pub fn npc_trade_inter(&mut self, mut npc: TradeNPC) -> bool {
        let sh_conv = npc.get_sh_conv();
        // let nitems = npc.get_items();
        let pitems = self.player.get_inventory();
        let name = npc.get_sname();
        let comms = format!("{}#{}", name, sh_conv["trade_msg"]);
        self.gui.reset_cursor();
        loop {
            self.gui.npc_trade_type_draw(
                comms.clone(),
                &mut GuiArgs {
                    map: &self.map,
                    player: &self.player,
                    // stats: &DisplayStats {
                    //     player: Vec::new(),
                    //     notes: (String::from(""), String::from("")),
                    // },
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
                                let cursor = self.gui.get_cursor();
                                let choice = cursor.0;
                                match choice {
                                    0 => {
                                        let mut npc = match self.interactee.clone() {
                                            Interactable::NPC(NPCWrap::TradeNPC(npc)) => npc,
                                            _ => todo!(),
                                        };
                                        self.trade_buy_items(npc.get_items())
                                    }
                                    1 => self.trade_sell_items(pitems.clone()),
                                    2 => {
                                        self.game_mode = GameMode::Play;
                                        self.gui.set_info_mode(GUIMode::Normal);
                                        return true;
                                    }
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

    pub fn npc_interaction(&mut self) -> bool {
        fn comb_conv(name: String, convo: Vec<String>) -> String {
            let mut conv = "".to_string();
            for (i, c) in convo.into_iter().enumerate() {
                let t = if i % 2 == 0 {
                    format!("{}: {}", name, c)
                } else {
                    format!("You: {}", c)
                };
                conv.push_str(&t);
                conv.push('#');
            }
            conv
        }

        let npc = self.interactee.clone();
        match npc {
            Interactable::NPC(NPCWrap::CommNPC(mut comm_npc)) => {
                let comm = self.npc_comm_inter(comm_npc.get_sname(), comm_npc.get_comm());
                self.notebook.enter_convo(&comm[0]);
                // true
            }
            Interactable::NPC(NPCWrap::ConvNPC(mut conv_npc)) => {
                let convo = self.npc_conv_inter(conv_npc.clone());
                self.notebook
                    .enter_convo(&comb_conv(conv_npc.get_sname(), convo));
                // true
            }
            Interactable::NPC(NPCWrap::SpawnNPC(mut spawn_npc)) => {
                let convo = self.npc_spawn_inter(spawn_npc.clone());
                self.notebook
                    .enter_convo(&comb_conv(spawn_npc.get_sname(), convo));
                // true
            }
            Interactable::NPC(NPCWrap::ShopNPC(mut shop_npc)) => {
                let convo = self.npc_shop_inter(shop_npc.clone());
                self.notebook
                    .enter_convo(&comb_conv(shop_npc.get_sname(), convo));
                // true
            }
            Interactable::NPC(NPCWrap::TradeNPC(trade_npc)) => {
                self.npc_trade_inter(trade_npc);
            }
            _ => todo!(),
        }
        match self.interactee.clone() {
            Interactable::NPC(npc) => {
                // box_npc(npc).get_pos()
                self.npcs.insert(box_npc(npc.clone()).get_pos(), npc);
            }
            _ => todo!(),
        };
        true
    }

    pub fn pop_trade_items(&self) -> Vec<Item> {
        let mut items = Vec::new();
        let mut rng = rand::thread_rng();
        let i_opts = [
            Items::Rock,
            Items::HealthPotion,
            Items::Salve,
            Items::Apple,
            Items::EdibleRoot,
            Items::Guts,
            Items::BronzeClaymore,
            Items::BronzeLongsword,
            Items::BronzeShortsword,
            Items::BronzeHeavyAxe,
            Items::BronzeLightAxe,
            Items::BronzePickHammer,
            Items::BronzePickAxe,
        ];
        for _ in 0..10 {
            let i_choice = i_opts.choose(&mut rng).unwrap_or(&i_opts[0]);
            match i_choice {
                Items::Rock => items.push(Item::new_rock(0, 0)),
                Items::HealthPotion => items.push(Item::new_health_potion(0, 0)),
                Items::Salve => items.push(Item::new_salve(0, 0)),
                Items::Apple => items.push(Item::new_apple(0, 0)),
                Items::EdibleRoot => items.push(Item::new_edible_root(0, 0)),
                Items::Guts => items.push(Item::new_guts(0, 0)),
                Items::BronzeClaymore => items.push(Item::new_bronze_claymore(0, 0)),
                Items::BronzeLongsword => items.push(Item::new_bronze_longsword(0, 0)),
                Items::BronzeShortsword => items.push(Item::new_bronze_shortsword(0, 0)),
                Items::BronzeHeavyAxe => items.push(Item::new_bronze_heavy_axe(0, 0)),
                Items::BronzeLightAxe => items.push(Item::new_bronze_light_axe(0, 0)),
                Items::BronzePickHammer => items.push(Item::new_bronze_pick_hammer(0, 0)),
                Items::BronzePickAxe => items.push(Item::new_bronze_pick_axe(0, 0)),
                _ => items.push(Item::new_rock(0, 0)),
            }
        }
        items
    }
}
