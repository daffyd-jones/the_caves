use crate::enums::{Cells, GUIMode, GameMode, Interactable, Items, NPCWrap, NPCs, PuzzleType};
use crate::map::{MAP_H, MAP_W};
//use crate::player::Player;
//use crate::puzzle::Puzzle;
//use crate::puzzles::Puzzles;
//use crate::enemy::{Enemy};
use crate::gamestate::GameState;
use crate::gamestate::Item;
use crate::npc::{
    new_comm_npc, new_conv_npc, new_spawn_npc, new_trade_npc, CommNPC, ConvNPC, Convo, ShopNPC,
    SpawnNPC, TradeNPC, NPC,
};
// use crate::gui_man_draw::GUI;
use std::time::Instant;

use crate::gamestate::box_npc;
use crate::gamestate::loc_shop_items;
use crate::gamestate::npc_move;
use crate::gamestate::wrap_nbox;
use rand::prelude::SliceRandom;
use rand::Rng;
use ratatui::crossterm::event::{poll, read, Event, KeyCode};
use std::collections::HashMap;

impl GameState {
    pub fn update_npcs(&mut self, step: u8) {
        let mut n_temp = self.npcs.clone();
        let mut new_n = HashMap::new();
        let mh = self.map.cells.len();
        let mw = self.map.cells[0].len();
        for ((x, y), n) in &mut n_temp {
            // log::info!("esteps: {}, eCx: {}, ey: {}", e.steps.clone(), x.clone(), y.clone());

            let mut nbox = box_npc(n.clone());
            if nbox.get_step_grp() != step || *x < 200 || *x > 400 || *y < 180 || *y > 225 {
                new_n.insert((*x, *y), wrap_nbox(nbox));
            } else {
                let (pos, nnpc) = npc_move(nbox, self.map.cells.clone(), mw, mh, *x, *y);
                let bwrp = wrap_nbox(nnpc);
                new_n.insert(pos, bwrp);
            }
        }
        self.npcs = new_n;
    }

    pub fn shift_npcs(&mut self, dir: &str) {
        let temp_n = self.npcs.clone();
        let mut new_n = HashMap::new();
        let mw = self.map.cells[0].len();
        let mh = self.map.cells.len();
        for ((x, y), n) in temp_n {
            let mut nbox = box_npc(n);
            match dir {
                "UP" => {
                    if y < mh - 10 {
                        // n.y+=1;
                        nbox.mmove("DN");
                        let npc_w = wrap_nbox(nbox);
                        new_n.insert((x, y + 1), npc_w.clone());
                    }
                }
                "DN" => {
                    if y > 10 {
                        // n.y-=1;
                        nbox.mmove("UP");
                        let npc_w = wrap_nbox(nbox);
                        new_n.insert((x, y - 1), npc_w.clone());
                    }
                }
                "LF" => {
                    if x < mw - 10 {
                        // n.x+=1;
                        nbox.mmove("RT");
                        let npc_w = wrap_nbox(nbox);
                        new_n.insert((x + 1, y), npc_w.clone());
                    }
                }
                "RT" => {
                    if x > 10 {
                        // n.x-=1;
                        nbox.mmove("LF");
                        let npc_w = wrap_nbox(nbox);
                        new_n.insert((x - 1, y), npc_w.clone());
                    }
                }
                _ => todo!(),
            };
        }
        self.npcs = new_n;
    }

    // pub fn npc_comm_inter(&mut self, mut npc: CommNPC) -> bool {
    pub fn npc_comm_inter(&mut self, npc_name: String, npc_comm: String) -> bool {
        //let comms = format!("{}#{}", npc.get_sname(), npc.get_comm());
        let comms = format!("{}#{}", npc_name, npc_comm);
        self.gui.reset_cursor();
        loop {
            self.gui.npc_comm_draw(
                comms.clone(),
                self.map.clone(),
                self.player.clone(),
                self.portals.clone(),
                self.enemies.clone(),
                self.items.clone(),
                self.npcs.clone(),
                loc_shop_items(self.dist_fo, self.location.clone()),
                self.env_inters.clone(),
            );
            if poll(std::time::Duration::from_millis(100)).unwrap() {
                if let Event::Key(event) = read().unwrap() {
                    // log::info!("keykind {:?}", event.kind.clone());
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

        true
    }

    pub fn conv_step(&mut self, conv: Convo, step: String, name: String) -> bool {
        //log::info!("stage: {:?}", step.clone());
        if step == *"e" {
            //log::info!("Going home");
            self.game_mode = GameMode::Play;
            self.gui.set_info_mode(GUIMode::Normal);
            return true;
        }
        let stage = &conv.stages[&step];
        let text = &stage.text;
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
                self.map.clone(),
                self.player.clone(),
                self.portals.clone(),
                self.enemies.clone(),
                self.items.clone(),
                self.npcs.clone(),
                loc_shop_items(self.dist_fo, self.location.clone()),
                self.env_inters.clone(),
            );
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
                                let cursor = self.gui.get_cursor();
                                let next = &opts[cursor.1].next;
                                // let next = opts[opts_choice.1];
                                return self.conv_step(
                                    conv.clone(),
                                    next.to_string(),
                                    name.clone(),
                                );
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    pub fn npc_conv_inter(&mut self, mut npc: ConvNPC) -> bool {
        let convo = npc.get_conv();
        let name = npc.get_sname();
        self.conv_step(convo, "0".to_string(), name)
    }

    pub fn npc_spawn_inter(&mut self, mut npc: SpawnNPC) -> bool {
        if npc.is_spawned() {
            return self.npc_comm_inter(npc.get_sname(), npc.get_comm());
        }
        npc.toggle_spawned();
        let spwn_conv = npc.get_conv();
        let ptype = npc.get_ptype();
        let name = npc.get_sname();
        let pos = self.dist_fo;
        self.puzzles.spawn_new_puzzle(pos, ptype.clone());
        self.conv_step(spwn_conv, "0".to_string(), name)
    }

    pub fn npc_shop_inter(&mut self, mut npc: ShopNPC) -> bool {
        let convo = npc.get_convo();
        let name = npc.get_sname();
        self.conv_step(convo, "0".to_string(), name)
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
        //put stuff here
        //let mut inv_opt = (0, Item::default());
        self.gui.reset_cursor();
        loop {
            self.gui.npc_trade_draw(
                items.clone(),
                self.map.clone(),
                self.player.clone(),
                self.portals.clone(),
                self.enemies.clone(),
                self.items.clone(),
                self.npcs.clone(),
                loc_shop_items(self.dist_fo, self.location.clone()),
                self.env_inters.clone(),
            );
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
                                let inv_opt = self.gui.get_inv_opt();
                                if !self.trade_buy(inv_opt.1) {
                                    //show no money
                                    break;
                                }
                                items.remove(inv_opt.0);
                            }
                            KeyCode::Backspace => {
                                break;
                            }
                            _ => {}
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
                self.map.clone(),
                self.player.clone(),
                self.portals.clone(),
                self.enemies.clone(),
                self.items.clone(),
                self.npcs.clone(),
                loc_shop_items(self.dist_fo, self.location.clone()),
                self.env_inters.clone(),
            );
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
                                let inv_opt = self.gui.get_inv_opt();
                                if !self.trade_sell(inv_opt.clone()) {
                                    //show no money
                                    break;
                                }
                                items.remove(inv_opt.0);
                            }
                            KeyCode::Backspace => {
                                break;
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
        true
    }

    pub fn npc_trade_inter(&mut self, mut npc: TradeNPC) -> bool {
        let sh_conv = npc.get_sh_conv();
        let nitems = npc.get_items();
        let pitems = self.player.get_inventory();
        let name = npc.get_sname();
        let comms = format!("{}#{}", name, sh_conv["trade_msg"]);
        self.gui.reset_cursor();
        loop {
            self.gui.npc_trade_type_draw(
                comms.clone(),
                self.map.clone(),
                self.player.clone(),
                self.portals.clone(),
                self.enemies.clone(),
                self.items.clone(),
                self.npcs.clone(),
                loc_shop_items(self.dist_fo, self.location.clone()),
                self.env_inters.clone(),
            );
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
                                let cursor = self.gui.get_cursor();
                                let choice = cursor.0;
                                match choice {
                                    0 => self.trade_buy_items(nitems.clone()),
                                    1 => self.trade_sell_items(pitems.clone()),
                                    2 => {
                                        self.game_mode = GameMode::Play;
                                        self.gui.set_info_mode(GUIMode::Normal);
                                        return true;
                                    }
                                    _ => todo!(),
                                };
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    pub fn npc_interaction(&mut self) -> bool {
        let npc = self.interactee.clone();
        match npc {
            Interactable::NPC(NPCWrap::CommNPC(mut comm_npc)) => {
                self.npc_comm_inter(comm_npc.get_sname(), comm_npc.get_comm())
            }
            Interactable::NPC(NPCWrap::ConvNPC(conv_npc)) => self.npc_conv_inter(conv_npc),
            Interactable::NPC(NPCWrap::SpawnNPC(spawn_npc)) => self.npc_spawn_inter(spawn_npc),
            Interactable::NPC(NPCWrap::ShopNPC(shop_npc)) => self.npc_shop_inter(shop_npc),
            Interactable::NPC(NPCWrap::TradeNPC(trade_npc)) => self.npc_trade_inter(trade_npc),
            _ => todo!(),
        }
    }

    fn pop_trade_items(&self) -> Vec<Item> {
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

    pub fn check_place_npcs(&mut self, x: usize, y: usize) -> bool {
        let mut rng = rand::thread_rng();
        let types = {
            let rnd = rng.gen_range(0..30);
            if rnd == 0 {
                vec![NPCs::CommNPC, NPCs::ConvNPC, NPCs::SpawnNPC]
            } else {
                vec![NPCs::CommNPC, NPCs::ConvNPC, NPCs::TradeNPC]
            }
        };
        if self.map.cells[y][x] == Cells::Empty
            && !self.in_loc_check((x, y))
            && !self.enemies.contains_key(&(x, y))
            && !self.items.contains_key(&(x, y))
            && !self.npcs.contains_key(&(x, y))
        {
            if let Some(i_type) = types.choose(&mut rng) {
                let def_name = "Kevthony".to_string();
                let npc = match i_type {
                    NPCs::CommNPC => {
                        let rnd_comms = {
                            let mut tvec = Vec::new();
                            for _ in 0..4 {
                                let tidx = rng.gen_range(0..self.npc_comms.len());
                                tvec.push(self.npc_comms[tidx].clone());
                            }
                            tvec
                        };
                        let name = self
                            .npc_names
                            .choose(&mut rng)
                            .unwrap_or(&def_name.clone())
                            .clone();
                        NPCWrap::CommNPC(new_comm_npc(name.to_string(), x, y, rnd_comms))
                    }
                    NPCs::ConvNPC => {
                        let conv: Convo = self
                            .npc_convos
                            .choose(&mut rng)
                            .unwrap_or(&self.npc_convos[0].clone())
                            .clone();
                        let name = self
                            .npc_names
                            .choose(&mut rng)
                            .unwrap_or(&def_name.clone())
                            .clone();
                        NPCWrap::ConvNPC(new_conv_npc(name.to_string(), x, y, conv))
                    }
                    NPCs::SpawnNPC => {
                        let rnd_comms = {
                            let mut tvec = Vec::new();
                            for _ in 0..4 {
                                let tidx = rng.gen_range(0..self.npc_spcomms.len());
                                tvec.push(self.npc_spcomms[tidx].clone());
                            }
                            tvec
                        };
                        let conv: Convo = self
                            .npc_spconvos
                            .choose(&mut rng)
                            .unwrap_or(&self.npc_spconvos[0].clone())
                            .clone();
                        let pt_str = conv.id.clone();
                        let ptype = match pt_str {
                            pt if pt.contains("maze") => PuzzleType::Maze,
                            pt if pt.contains("teleport") => PuzzleType::Teleport,
                            pt if pt.contains("inverted") => PuzzleType::Inverted,
                            _ => todo!(),
                        };
                        let name = self
                            .npc_names
                            .choose(&mut rng)
                            .unwrap_or(&def_name.clone())
                            .clone();
                        NPCWrap::SpawnNPC(new_spawn_npc(
                            name.to_string(),
                            x,
                            y,
                            conv,
                            rnd_comms,
                            ptype,
                        ))
                    }
                    NPCs::TradeNPC => {
                        let name = self
                            .npc_names
                            .choose(&mut rng)
                            .unwrap_or(&def_name.clone())
                            .clone();
                        let conv = self
                            .npc_trade
                            .choose(&mut rng)
                            .unwrap_or(&self.npc_trade[0].clone())
                            .clone();
                        let items = self.pop_trade_items();
                        NPCWrap::TradeNPC(new_trade_npc(name.to_string(), x, y, items, conv))
                    }
                    _ => todo!(),
                };
                self.npcs.insert((x, y), npc);
                return true;
            }
        }
        false
    }

    pub fn repop_npcs(&mut self) {
        let mut rng = rand::thread_rng();
        let (vx, vy, vw, vh) = self.map.get_viewport();
        //xx
        match (-self.map.gen_x, -self.map.gen_y) {
            (x, y) if x < 0 && y == 0 => {
                for _ in 0..20 {
                    loop {
                        let x = rng.gen_range(10..vx - 5);
                        let y = rng.gen_range(10..MAP_H - 10);
                        let res = self.check_place_npcs(x, y);
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
                        let res = self.check_place_npcs(x, y);
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
                        let res = self.check_place_npcs(x, y);
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
                        let res = self.check_place_npcs(x, y);
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
                        let res = self.check_place_npcs(x, y);
                        if res {
                            break;
                        }
                    }
                }
                for _ in 0..10 {
                    loop {
                        let x = rng.gen_range(10..MAP_W - 10);
                        let y = rng.gen_range((vy + vh + 5)..MAP_H - 10);
                        let res = self.check_place_npcs(x, y);
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
                        let res = self.check_place_npcs(x, y);
                        if res {
                            break;
                        }
                    }
                }
                for _ in 0..10 {
                    loop {
                        let x = rng.gen_range(10..MAP_W - 10);
                        let y = rng.gen_range(10..vy - 5);
                        let res = self.check_place_npcs(x, y);
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
                        let res = self.check_place_npcs(x, y);
                        if res {
                            break;
                        }
                    }
                }
                for _ in 0..10 {
                    loop {
                        let x = rng.gen_range(10..MAP_W - 10);
                        let y = rng.gen_range((vy + vh + 5)..MAP_H - 10);
                        let res = self.check_place_npcs(x, y);
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
                        let res = self.check_place_npcs(x, y);
                        if res {
                            break;
                        }
                    }
                }
                for _ in 0..10 {
                    loop {
                        let x = rng.gen_range(10..MAP_W - 10);
                        let y = rng.gen_range(10..vy - 5);
                        let res = self.check_place_npcs(x, y);
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
