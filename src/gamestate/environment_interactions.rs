//environment_interactions

use crate::gamestate::GameState;

use crate::enums::{EnvInter, GameMode, Items, Plants, PuzzleType};
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
                            _ => {
                                let _ = self.key(event.code);
                            }
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
                            _ => {
                                let _ = self.key(event.code);
                            }
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
                            _ => {
                                let _ = self.key(event.code);
                            }
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
                            _ => {
                                let _ = self.key(event.code);
                            }
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
                            _ => {
                                let _ = self.key(event.code);
                            }
                        }
                    }
                }
            }
        }
        self.game_mode = GameMode::Play;

        true
    }

    fn herbalist_sells(&mut self) {
        self.gui.reset_cursor();
        loop {
            self.gui.herbalist_draw(
                "I sell a few things, mostly potions and ingredients. I have a cauldron over there that you're free to use. I can also identify any plants you find and tell you how you can brew them to make a potion.#I see, thanks.".to_string(),
                None,
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
                            _ => {
                                let _ = self.key(event.code);
                            }
                        }
                    }
                }
            }
        }
    }

    fn plant_identify(&mut self, plant: Items) {
        let plant_str = match plant {
            Items::Plants(Plants::Moss) => "That is Moss. It grows in patches here and there. It can be used to make a salve for healing wounds. It takes 10 Moss to make one Salve.".to_string(),
            Items::Plants(Plants::LuminousMushroom) => "That one is a Luminous Mushroom. They emit a light glow that helps light the caves. If you have 5 you can make a Vitality Potion.".to_string(),
            Items::Plants(Plants::LichenousGrowth) => "That is Lichenous Growth. It grows from the walls in little clumps. Its kinda spongey. An Antidote can be made from 8 of them.".to_string(),
            Items::Plants(Plants::VineBulb) => "That looks to be a Vine Bulb. They grow on the vines that cover some of the walls. If you have 5 of those, you can make an Agility Potion.".to_string(),
            Items::Plants(Plants::LampenPetals) => "That looks to be some Lampen Petals. They're from the Lampen Flower that grows in the dark. You can make a Health Potion from 5 flowers worth.".to_string(),
            Items::Plants(Plants::LuckyClover) => "That's a Lucky Clover. They grow here and there, a bit harder to find. They can be used to make a Luck Potion if you have 5.".to_string(),
            Items::Plants(Plants::Shroom) => "That's what we call a Shroom. They're pretty hard to find, they are quite coveted. You can use 5 of them to make a Magic Potion that numbs pain for a bit.".to_string(),
            _ => "That doesnt seem to be a plant.".to_string(),
        };
        self.gui.reset_cursor();
        loop {
            self.gui.herbalist_draw(
                format!("{}#{}", plant_str, "Oh, I see. Thanks!"),
                None,
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
                            _ => {
                                let _ = self.key(event.code);
                            }
                        }
                    }
                }
            }
        }
    }

    fn identify_plant(&mut self) {
        let items = self.player.inventory.clone();
        let mut plants = Vec::new();
        let mut plant_strs = Vec::new();
        for i in items {
            if let Items::Plants(_) = i.itype {
                plants.push(i.itype);
                plant_strs.push(i.sname);
            }
        }

        self.gui.reset_cursor();
        loop {
            self.gui.herbalist_draw(
                "Of course I can! What would you like me to identify?".to_string(),
                Some(plant_strs.clone()),
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
                                let cur = self.gui.get_cursor();
                                self.plant_identify(plants[cur.1]);
                                // match cur.1 {
                                //     0 => self.herbalist_sells(),
                                //     1 => self.identify_plant(),
                                //     _ => {}
                                // }
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

    fn herbalist(&mut self) -> bool {
        self.gui.reset_cursor();
        loop {
            self.gui.herbalist_draw(
                "Hey there! what can I help you with?#What do you sell here?#Can you idenitfy some plants for me?".to_string(),
                None,
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
                                let cur = self.gui.get_cursor();
                                match cur.1 {
                                    0 => self.herbalist_sells(),
                                    1 => self.identify_plant(),
                                    _ => {}
                                }
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
            EnvInter::Herbalist => self.herbalist(),
            _ => todo!(),
        }
    }
}
