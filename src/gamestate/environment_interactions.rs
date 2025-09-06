//environment_interactions
use crate::assets::{
    get_ascii, get_comms, get_convos, get_npc_name, get_shop_convos, get_shops, Ascii, Comms,
    Convos, Npcs,
};

use crate::gamestate::GameState;

use crate::enums::{
    Door, EnvInter, GameMode, Interactable, Items, Location, Plants, PuzzleType, Shops, TaskEnv,
};
use crate::gui_utils::{DisplayStats, GuiArgs};
use crate::item::Item;
use crate::shop::Shop;
use crate::tasks::{self, Task, TaskType};
use crate::utils::comb_conv;
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
                ascii: Some(&get_ascii(Ascii::Npcs(Npcs::Settler))),
                ani_stats: &self.get_ani_stats(),
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
                    ani_stats: &self.get_ani_stats(),
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
            ppost_strings.push(format!(
                r#"
---- ---- nl
There's a {} that has been spotted {} of here. nl
nl
A couple guild members checked it out earlier, but didn't find anything. nl
---- nl
"#,
                ptype_string, dir_string
            ));
        }
        let mut tasks = self.tasks.board_tasks[0..2].to_vec().clone();

        let mut task_posts = Vec::new();
        for task in &mut tasks {
            task_posts.push(task.board_post());
        }
        let mut already_tasked = false;
        self.gui.reset_cursor();
        loop {
            self.gui.guild_post_draw(
                ppost_strings.clone(),
                task_posts.clone(),
                already_tasked,
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
                    ani_stats: &self.get_ani_stats(),
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
                                if already_tasked {
                                    already_tasked = false;
                                    continue;
                                }
                                if self.gui.get_menu_lvl() == 1 && self.gui.get_cursor_hold().1 == 0
                                {
                                    let cursor = self.gui.get_cursor().1;
                                    if self.tasks.active_board_task == None {
                                        self.pick_board_task(tasks[cursor].clone());
                                        break;
                                    } else {
                                        already_tasked = true;
                                    }
                                } else {
                                    break;
                                }
                            }
                            KeyCode::Right => {
                                if self.gui.get_menu_lvl() == 0 {
                                    self.gui.menu_lvl("DN");
                                }
                            }
                            KeyCode::Left => {
                                if self.gui.get_menu_lvl() == 1 {
                                    self.gui.menu_lvl("UP");
                                }
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
                    (xdir, ydir) if xdir <= 0 && ydir <= 0 => "North West".to_string(),
                    (xdir, ydir) if xdir <= 0 && ydir > 0 => "South West".to_string(),
                    (xdir, ydir) if xdir > 0 && ydir <= 0 => "North East".to_string(),
                    (xdir, ydir) if xdir > 0 && ydir > 0 => "South East".to_string(),
                    _ => "dunno".to_string(),
                };
                // let settle_name = s.get_sname();
                // let settle_str = format!("{}#{}", settle_name.clone(), dir_string.clone());
                let stats = s.get_stats();
                let mut string = stats.1;
                string.push_str(&format!(
                    r#"
Direction:
{}
                    "#,
                    dir_string
                ));
                settles.push((stats.0, string, (dx as f64, -dy as f64)));
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
                    ani_stats: &self.get_ani_stats(),
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
                    ascii: Some(&self.npc_asciis[2].clone()),
                    ani_stats: &self.get_ani_stats(),
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
                    ascii: Some(&self.npc_asciis[1].clone()),
                    ani_stats: &self.get_ani_stats(),
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
            Items::Plants(Plants::LampenFlower) => "That looks to be some Lampen Petals. They're from the Lampen Flower that grows in the dark. You can make a Health Potion from 5 flowers worth.".to_string(),
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
                    ascii: Some(&self.npc_asciis[1].clone()),
                    ani_stats: &self.get_ani_stats(),
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
                    ascii: Some(&self.npc_asciis[1].clone()),
                    ani_stats: &self.get_ani_stats(),
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
                    ascii: Some(&self.npc_asciis[1].clone()),
                    ani_stats: &self.get_ani_stats(),
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

    fn construction(&mut self) -> bool {
        self.gui.reset_cursor();
        loop {
            self.gui.npc_comm_draw(
                "Guild Worker#Hey sorry, you can't come through here. We're doing construction on the cave walls, you're going to have to go around.".to_string(),
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
                    ani_stats: &self.get_ani_stats(),
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

    pub fn report_board_task(&mut self) {
        let task = self.tasks.active_board_task.clone().unwrap();
        self.gui.reset_cursor();
        loop {
            self.gui.npc_comm_draw(
                format!(
                    "Guild Head#It looks like you finished a task. Heres your reward! {} gold ",
                    task.reward().properties["value"]
                ),
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
                    ani_stats: &self.get_ani_stats(),
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
        self.player.inc_money(task.reward().properties["value"]);
        self.tasks.active_board_task = None;
    }

    fn shop_npc(&mut self, shop_type: Shops) -> bool {
        // check here for plot/task convos
        let loc = self.location.clone();
        let snpc = match loc {
            Location::Settlement(settle) => &settle.shops.get(&shop_type).unwrap().npc.clone(),
            _ => todo!(),
        };
        let name = match shop_type {
            Shops::Item => "Shop Keeper".to_string(),
            Shops::Guild => "Guild Head".to_string(),
            Shops::Church => "Obsidian Steward".to_string(),
            Shops::Clinic => "Clinic".to_string(),
            Shops::Herbalist => "Herbalist".to_string(),
            Shops::Weapon => "Weapon Smith".to_string(),
            Shops::Armor => "Armourer".to_string(),
            Shops::Consignment => "Shop Keeper".to_string(),
            Shops::Null => todo!(),
        };
        match shop_type {
            Shops::Guild => {
                if let Some(mut task) = self.tasks.active_board_task.clone() {
                    if task.is_complete() {
                        self.report_board_task();
                    }
                }
            }
            _ => {}
        }
        let conv = self.conv_step(
            snpc.convo.clone(),
            "0".to_string(),
            name.clone(),
            Vec::new(),
        );
        self.notebook.enter_convo(&comb_conv(name, conv));
        true
    }

    fn unlock_door(&mut self, door: Door) {
        let adj = [
            (self.player.x - 1, self.player.y),
            (self.player.x + 1, self.player.y),
            (self.player.x, self.player.y - 1),
            (self.player.x, self.player.y + 1),
        ];

        let env_temp = self.env_inters.clone();

        for (pos, env) in env_temp {
            if adj.contains(&pos) && env == EnvInter::Door(door) {
                self.env_inters.insert(
                    pos,
                    match door {
                        Door::HLocked(_) => EnvInter::Door(Door::HOpen),
                        Door::VLocked(_) => EnvInter::Door(Door::VOpen),
                        _ => todo!(),
                    },
                );
                self.interactee = Interactable::Null;
            }
        }
    }

    fn locked_door(&mut self, door: Door) -> bool {
        log::info!("intee3: {:?}", door);
        let pick_level = match door {
            Door::HLocked(lvl) => lvl,
            Door::VLocked(lvl) => lvl,
            _ => 0,
        };

        if pick_level == 0 {
            self.unlock_door(door);
            self.game_mode = GameMode::Play;
            return true;
        }

        let result = if self
            .stats
            .player_xp
            .get_xp(crate::enums::ExpType::Lockpicking)
            .0 as u8
            >= pick_level
        {
            self.unlock_door(door);
            "You have unlocked the door.".to_string()
        } else {
            "You are not skilled enough to pick this lock.".to_string()
        };
        self.gui.reset_cursor();
        loop {
            self.gui.locked_draw(
                result.clone(),
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
                    ani_stats: &self.get_ani_stats(),
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

    fn task_incomplete(&mut self, name: String) {
        self.gui.reset_cursor();
        loop {
            self.gui.npc_comm_draw(
                format!("{name}#Hey, it looks like you're not done yet"),
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
                    ascii: Some(&self.npc_asciis[0].clone()),
                    ani_stats: &self.get_ani_stats(),
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
                            _ => {
                                let _ = self.key(event.code);
                            }
                        }
                    }
                }
            }
        }
    }

    fn retrieve_item_final(&mut self, mut task: Task) {
        if let Task::BoardItemWanted {
            receiver_entity_name,
            receiver_convo,
            task_items,
            ..
        } = task.clone()
        {
            if self
                .player
                .inventory
                .iter()
                .filter(|x| x.itype == task_items.0)
                .count()
                < task_items.1.into()
            {
                self.task_incomplete(receiver_entity_name)
            } else {
                let mut cnt = 0;
                let amt = task_items.1;
                self.player.inventory.retain(|itm| {
                    if itm.itype == task_items.0 && cnt < amt {
                        cnt += 1;
                        false
                    } else {
                        true
                    }
                });
                self.conv_step(
                    receiver_convo,
                    "0".to_string(),
                    receiver_entity_name,
                    Vec::new(),
                );
                self.tasks.active_board_task = Some(task);
                if let Some(ref mut active_task) = self.tasks.active_board_task {
                    active_task.complete_task();
                }
            }
        }
    }

    fn task_null_comms(&mut self) -> bool {
        let comm = self.npc_comm_inter("Stu".to_string(), "Hey.".to_string());
        self.notebook.enter_convo(&comm[0]);
        true
    }

    fn task_board_goal(&mut self) -> bool {
        if self.tasks.active_board_task.is_none() {
            self.game_mode = GameMode::Play;
            return true;
        }
        let task = self.tasks.active_board_task.clone().unwrap();
        match task {
            Task::BoardItemWanted { .. } => self.retrieve_item_final(task),
            // Task::BoardPassMessage {..} => {}
            // Task::BoardPassItem {..} => {}
            // TaskType::Plot => {},
            _ => {}
        }
        self.game_mode = GameMode::Play;
        true
    }

    pub fn env_interaction(&mut self, env_inter: EnvInter) -> bool {
        log::info!("intee2: {:?}", env_inter);
        match env_inter {
            EnvInter::Records => self.save_game(),
            EnvInter::Clinic => self.clinic(),
            EnvInter::GuildPost => self.guild_post(),
            EnvInter::ChurchPost => self.church_post(),
            EnvInter::Cauldron => self.cauldron(),
            EnvInter::Herbalist => self.herbalist(),
            EnvInter::Door(door) => self.locked_door(door),
            EnvInter::Construction => self.construction(),
            EnvInter::ShopNPC(shop_type) => self.shop_npc(shop_type),
            EnvInter::TaskEnv(TaskEnv::BoardGoalEntity) => self.task_board_goal(),
            EnvInter::TaskEnv(TaskEnv::Null) => self.task_null_comms(),
            _ => {
                // log::info!("Not entering locked_door");
                self.game_mode = GameMode::Play;
                true
            }
        }
    }
}
