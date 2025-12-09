//gamestate
use crate::assets::{get_ascii, get_comm, get_convo, get_npc_name};
use crate::dialogue::Dialogue;
use crate::enemy::Enemy;
use crate::enums::{
    Cells, CompMode, EncMode, EncOpt, Enemies, EnvInter, FightSteps, GameMode, Interactable, Items,
    Location, NPCWrap, NodeType, PuzzlePiece,
};
use crate::features::Features;
use crate::gui::GUI;
use crate::gui_utils::{AniStats, DisplayStats, GuiArgs};
use crate::item::Item;
use crate::map::Map;
use crate::nodemap::NodeMap;
use crate::notebook::Notebook;
use crate::npc::Convo;
use crate::player::Player;
use crate::puzzles::Puzzles;
use crate::settlements::Settlements;
use crate::stats::{Season, Stats};
use crate::tasks::{Task, Tasks};
use crate::utils::{gen_broken_range, in_range, loc_shop_items};

mod compass_state;
mod enemies;
mod enemy_encounter;
mod environment_interactions;
mod feature_state;
mod interactions;
mod inventory_state;
mod item_state;
mod keys;
mod locations;
mod map_state;
mod npc_interactions;
mod npcs;
mod puzzle_state;
mod settle_state;
mod task_state;

use rand::Rng;
use ratatui::crossterm::event::{poll, read, Event, KeyCode, KeyEventKind};
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};

//#[derive(Serialize, Deserialize, Debug)]
pub struct GameState {
    game_mode: GameMode,
    notebook: Notebook,
    gui: GUI,
    map: Map,
    nodemap: NodeMap,
    tasks: Tasks,
    settles: Settlements,
    puzzles: Puzzles,
    player: Player,
    stats: Stats,
    midnight: Instant,
    features: Features,
    dist_fo: (i16, i16),
    comp_head: (i16, i16),
    comp_list: HashMap<(i16, i16), String>,
    comp_mode: CompMode,
    loc_rad: u16,
    depth: u16,
    level: u32,
    pressed_keys: HashMap<KeyCode, bool>,
    enemies: HashMap<(usize, usize), Enemy>,
    step_group: u8,
    items: HashMap<(usize, usize), Item>,
    npcs: HashMap<(usize, usize), NPCWrap>,
    env_inters: HashMap<(usize, usize), EnvInter>,
    key_debounce_dur: Duration,
    last_event_time: Instant,
    interactee: Interactable,
    location: Location,
    puzzle_pieces: HashMap<(usize, usize), PuzzlePiece>,
    portal_cool: Instant,
    loc_map: Option<Vec<Vec<Cells>>>,
    enc: EncOpt,
    enc_mode: EncMode,
}

impl GameState {
    pub fn new_menu() -> Self {
        let gui = GUI::new();
        let map = Map::new();
        let comp_list = HashMap::new();
        let player = Player::new(308, 194);
        let stats = Stats::new();
        let enemies = HashMap::new();
        let items = HashMap::new();
        let npcs = HashMap::new();
        let env_inters = HashMap::new();
        let puzzle_pieces = HashMap::new();
        let notebook = Notebook::new().unwrap();
        let nodemap = NodeMap::new();
        let settles = Settlements::demo_self();
        let puzzles = Puzzles::demo_self();
        let features = Features::new();
        let tasks = Tasks::new();
        GameState {
            game_mode: GameMode::Play,
            notebook,
            gui,
            map,
            nodemap,
            tasks,
            settles,
            puzzles,
            player,
            stats,
            midnight: Instant::now(),
            features,
            dist_fo: (0, 0),
            comp_head: (0, 0),
            comp_list,
            comp_mode: CompMode::Search,
            loc_rad: 360,
            depth: 1,
            level: 0,
            pressed_keys: HashMap::new(),
            enemies,
            step_group: 0,
            items,
            npcs,
            env_inters,
            key_debounce_dur: Duration::from_millis(150),
            last_event_time: Instant::now(),
            interactee: Interactable::Null,
            location: Location::Null,
            puzzle_pieces,
            portal_cool: Instant::now(),
            loc_map: None,
            enc: EncOpt::Null,
            enc_mode: EncMode::Null,
        }
    }

    pub fn new() -> Arc<Mutex<Self>> {
        let mut load_gui = GUI::new();
        let load_bool = Arc::new(AtomicBool::new(true));
        let load_cln = Arc::clone(&load_bool);
        let load_handle = thread::spawn(move || {
            while load_cln.load(std::sync::atomic::Ordering::Relaxed) {
                load_gui.load_screen();
                thread::sleep(Duration::from_millis(100));
            }
        });
        let gui = GUI::new();
        let map = Map::new();
        let comp_list = HashMap::new();
        let mut player = Player::new(308, 194);
        player.inventory.push(Item::new_luminous_mushroom(0, 0));
        player.inventory.push(Item::new_moss(0, 0));
        player.inventory.push(Item::new_lichenous_growth(0, 0));
        player.inventory.push(Item::new_vine_bulb(0, 0));
        player.inventory.push(Item::new_lampen_flower(0, 0));
        player.inventory.push(Item::new_lucky_clover(0, 0));
        player.inventory.push(Item::new_shroom(0, 0));
        let stats = Stats::new();
        let enemies = HashMap::new();
        let items = HashMap::new();
        let npcs = HashMap::new();
        let env_inters = HashMap::new();
        let puzzle_pieces = HashMap::new();
        let notebook = Notebook::new().unwrap();
        let l_rate = 100 as u64;

        let mut nodemap = NodeMap::new();
        let mut settles = Settlements::demo_self();
        let mut puzzles = Puzzles::demo_self();
        let mut features = Features::new();

        for _ in 0..10 {
            let ulnodes = nodemap.increase_depth("ul");
            for n in ulnodes {
                match n.ntype {
                    NodeType::Settlement => settles.spawn_node_settlement(n.pos, n.name),
                    NodeType::Puzzle => puzzles.spawn_node_puzzle(n.pos),
                    _ => {}
                }
            }
            let ulfeats = nodemap.add_features("ul");
            for f in ulfeats {
                features.new_rand_feature(f.pos);
            }
            let urnodes = nodemap.increase_depth("ur");
            for n in urnodes {
                match n.ntype {
                    NodeType::Settlement => settles.spawn_node_settlement(n.pos, n.name),
                    NodeType::Puzzle => puzzles.spawn_node_puzzle(n.pos),
                    _ => {}
                }
            }
            let urfeats = nodemap.add_features("ur");
            for f in urfeats {
                features.new_rand_feature(f.pos);
            }
            let dlnodes = nodemap.increase_depth("dl");
            for n in dlnodes {
                match n.ntype {
                    NodeType::Settlement => settles.spawn_node_settlement(n.pos, n.name),
                    NodeType::Puzzle => puzzles.spawn_node_puzzle(n.pos),
                    _ => {}
                }
            }
            let dlfeats = nodemap.add_features("dl");
            for f in dlfeats {
                features.new_rand_feature(f.pos);
            }
            let drnodes = nodemap.increase_depth("dr");
            for n in drnodes {
                match n.ntype {
                    NodeType::Settlement => settles.spawn_node_settlement(n.pos, n.name),
                    NodeType::Puzzle => puzzles.spawn_node_puzzle(n.pos),
                    _ => {}
                }
            }
            let drfeats = nodemap.add_features("dr");
            for f in drfeats {
                features.new_rand_feature(f.pos);
            }
        }
        let mut tasks = Tasks::new();
        load_bool.store(false, std::sync::atomic::Ordering::Relaxed);
        load_handle.join().unwrap();
        Arc::new(Mutex::new(GameState {
            game_mode: GameMode::Play,
            notebook,
            gui,
            map,
            nodemap,
            tasks,
            settles,
            puzzles,
            player,
            stats,
            midnight: Instant::now(),
            features,
            dist_fo: (0, 0),
            comp_head: (0, 0),
            comp_list,
            comp_mode: CompMode::Search,
            loc_rad: 360,
            depth: 1,
            level: 0,
            pressed_keys: HashMap::new(),
            enemies,
            step_group: 0,
            items,
            npcs,
            env_inters,
            key_debounce_dur: Duration::from_millis(60),
            last_event_time: Instant::now(),
            interactee: Interactable::Null,
            location: Location::Null,
            puzzle_pieces,
            portal_cool: Instant::now(),
            loc_map: None,
            enc: EncOpt::Null,
            enc_mode: EncMode::Null,
        }))
    }

    pub fn start_menu(&mut self) -> usize {
        self.gui.reset_cursor();
        loop {
            self.gui.start_menu();
            if poll(std::time::Duration::from_millis(100)).unwrap() {
                if let Event::Key(event) = read().unwrap() {
                    let now = Instant::now();
                    if now.duration_since(self.last_event_time) > self.key_debounce_dur {
                        self.last_event_time = now;
                        let choice = match event.code {
                            KeyCode::Enter => self.gui.get_cursor().1,
                            KeyCode::Up => {
                                self.gui.move_cursor("UP");
                                10
                            }
                            KeyCode::Down => {
                                self.gui.move_cursor("DN");
                                10
                            }
                            _ => {
                                let _ = self.comm_key(event.code);
                                10
                            }
                        };
                        if choice < 10 {
                            return choice;
                        }
                    }
                }
            }
        }
    }

    pub fn ingame_menu(&mut self) -> usize {
        self.gui.reset_cursor();
        loop {
            self.gui.ingame_menu(&mut GuiArgs {
                map: &self.map,
                player: &self.player,
                enemies: &self.enemies,
                items: &self.items,
                npcs: &self.npcs,
                env_inter: Some(&self.env_inters),
                litems: Some(&loc_shop_items(self.dist_fo, self.location.clone())),
                puzzle_pieces: Some(&self.puzzle_pieces),
                animate: None,
                ascii: None,
                ani_stats: &self.get_ani_stats(),
            });
            if poll(std::time::Duration::from_millis(100)).unwrap() {
                if let Event::Key(event) = read().unwrap() {
                    let now = Instant::now();
                    if now.duration_since(self.last_event_time) > self.key_debounce_dur {
                        self.last_event_time = now;
                        let choice = match event.code {
                            KeyCode::Enter => self.gui.get_cursor().1,
                            KeyCode::Up => {
                                self.gui.move_cursor("UP");
                                10
                            }
                            KeyCode::Down => {
                                self.gui.move_cursor("DN");
                                10
                            }
                            _ => {
                                let _ = self.comm_key(event.code);
                                10
                            }
                        };
                        if choice < 10 {
                            return choice;
                        }
                    }
                }
            }
        }
    }

    fn play_update(&mut self) -> bool {
        if poll(std::time::Duration::from_millis(100)).unwrap() {
            if let Event::Key(event) = read().unwrap() {
                match event.kind {
                    KeyEventKind::Press => {
                        self.pressed_keys.insert(event.code, true);
                    }
                    KeyEventKind::Release => {
                        self.pressed_keys.insert(event.code, false);
                    }
                    _ => {}
                }
                // log::info!("keykind {:?}", event.kind.clone());
                let now = Instant::now();
                if now.duration_since(self.last_event_time) > self.key_debounce_dur {
                    self.last_event_time = now;
                    self.play_key(event.code)
                } else {
                    true
                }
            } else {
                true
            }
        } else {
            true
        }
    }

    fn game_save(&mut self, filename: String) -> bool {
        let path = format!("src/save/{}", filename);
        //let json = serde_json::to_string(self).expect("failed to save 1");
        let json = r#"{
            "name": "gamestate",
            "works": "yes"
        }"#;
        //let mut file = std::fs::File::create(path).expect("failed to save 2");
        //fs::remove_file(path.clone()).expect("failed to delete previous save");
        let mut sfile = OpenOptions::new()
            .write(true)
            .create(true)
            .open(path)
            .expect("failed to open save file");
        sfile.write_all(json.as_bytes());
        let fn_list_path = "src/save/index.txt";
        let mut nfile = OpenOptions::new()
            .append(true)
            .open(fn_list_path)
            .expect("failed to save fname");
        writeln!(nfile, "{}", filename.clone());
        self.game_mode = GameMode::Play;
        true
    }

    fn save_game(&mut self) -> bool {
        let file = File::open("src/save/index.txt").expect("failed to open savelist");
        let reader = BufReader::new(file);
        let mut savelist = Vec::new();

        for line in reader.lines() {
            let line = line.unwrap_or("ERR".to_string());
            savelist.push(line);
        }
        let save_str = if savelist.len() == 0 {
            "It seems you dont have any logs, would you like to log your progress?".to_string()
        } else {
            savelist.push("new save".to_string());
            "Would you like to overwrite a previous log?".to_string()
        };

        let day = self.stats.world_stats.date.day;
        let month = self.stats.world_stats.date.month;
        let year = self.stats.world_stats.date.year;
        self.gui.reset_cursor();
        loop {
            self.gui.guild_records_draw(
                save_str.clone(),
                savelist.clone(),
                &mut GuiArgs {
                    map: &self.map,
                    player: &self.player,
                    // stats: &self.stats.player_xp.get_xps(),
                    enemies: &self.enemies,
                    items: &self.items,
                    npcs: &self.npcs,
                    env_inter: Some(&self.env_inters),
                    litems: Some(&loc_shop_items(self.dist_fo, self.location.clone())),
                    puzzle_pieces: Some(&self.puzzle_pieces),
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
                            KeyCode::Enter => match savelist.len() {
                                0 => {
                                    if self.gui.get_ysno() {
                                        let settle_name = self.get_cur_settle_name();
                                        let save_name = format!("save_1_{}.json", settle_name);
                                        if self.game_save(save_name) {
                                            return true;
                                        }
                                    } else {
                                        self.game_mode = GameMode::Play;
                                        return true;
                                    };
                                }
                                _ => {
                                    let choice = self.gui.get_cursor();
                                    if choice.0 == savelist.len() - 1 {
                                        let settle_name = self.get_cur_settle_name();
                                        let save_name = format!("save_1_{}.json", settle_name);
                                        if self.game_save(save_name) {
                                            return true;
                                        }
                                    } else {
                                        let save_name = savelist[choice.0].clone();
                                        let sn_parts: Vec<&str> = save_name.split("_").collect();
                                        let settle_name = self.get_cur_settle_name();
                                        let new_save_name =
                                            format!("save_{}_{}.json", sn_parts[1], settle_name);
                                        if self.game_save(new_save_name) {
                                            return true;
                                        }
                                    };
                                }
                            },
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    pub fn start_update_threads(game_state: Arc<Mutex<Self>>) {
        let game_clone = Arc::clone(&game_state);
        thread::spawn(move || loop {
            {
                let mut game = game_clone.lock().unwrap();
                let step = game.step_group;
                if game.game_mode == GameMode::Play {
                    game.update_enemies(step);
                    game.update_npcs(step);
                    if step < 15 {
                        game.step_group += 1;
                    } else if step > 30 {
                    } else {
                        game.step_group = 0;
                    }
                }
            }
            thread::sleep(Duration::from_millis(125));
        });
    }

    pub fn update(&mut self) -> bool {
        let (vw, vh) = self.gui.get_viewport();
        self.map.set_viewport(vh, vw);
        // log::info!("update");
        let res = match self.game_mode {
            GameMode::Play => self.play_update(),
            GameMode::Interact(_) => self.interaction(),
            GameMode::Fight(_) => {
                let Interactable::Enemy(e) = self.interactee.clone() else {
                    todo!()
                };
                self.enemy_encounter(e)
            }
            GameMode::Dead => false,
            _ => todo!(),
        };

        if !res && self.ingame_menu() == 1 {
            return false;
        }

        if self.items.len() < 80 {
            self.repop_items();
            self.repop_area_plants();
            // self.repop_plants();
        }

        if self.npcs.len() < 30 {
            self.repop_npcs();
        }

        if self.enemies.len() < 60 {
            self.repop_enemies();
        }

        if self.tasks.locals.len() < 2 {
            self.tasks.locals = {
                let settles = self
                    .settles
                    .get_local_settles((self.dist_fo.0, self.dist_fo.1));
                let mut locals = Vec::new();
                for s in settles {
                    locals.push((s.0, s.1.sname));
                }
                locals
            };
            self.tasks.new_board_task();
            self.tasks.new_board_task();
            self.tasks.new_board_task();
        }

        let adj = [
            (self.player.x - 1, self.player.y),
            (self.player.x + 1, self.player.y),
            (self.player.x, self.player.y - 1),
            (self.player.x, self.player.y + 1),
        ];
        for i in adj {
            if let Some(e) = self.enemies.get(&i) {
                self.interactee = Interactable::Enemy(e.clone());
                self.game_mode = GameMode::Fight(FightSteps::Open);
            }
        }

        let now = Instant::now();
        if now.duration_since(self.midnight) >= Duration::from_secs(3600) {
            self.midnight = Instant::now();
            self.stats.next_day();
            self.stats.roll_world_stats();
        }

        self.stats.update_buffs();

        self.compass_check();
        true
    }

    pub fn get_ani_stats(&self) -> AniStats {
        let day = self.stats.world_stats.date.day;
        let month = self.stats.world_stats.date.month;
        let year = self.stats.world_stats.date.year;
        AniStats {
            season: Season {
                day,
                month,
                year,
                str: "".to_string(),
            },
        }
    }

    pub fn draw(&mut self) {
        self.location_check();
        let litems = if self.location != Location::Null {
            self.update_location();
            loc_shop_items(self.dist_fo, self.location.clone())
        } else {
            HashMap::new()
        };
        self.map_location();
        let debug_strs = {
            let dist_fo = format!("({}, {})", self.dist_fo.0, self.dist_fo.1);
            let comp = format!("({}, {})", self.comp_head.0, self.comp_head.1);
            //let spos_list = self.settles.get_settle_pos();
            // let spos_list = &self.comp_list;
            let spos_s = self
                .comp_list
                .clone()
                .iter()
                .map(|((x, y), s)| format!("({}, {}): {}", x, y, s))
                .collect::<Vec<String>>()
                .join("#");
            let fpos_s = self.features.get_feature_positions().join("#");
            (dist_fo, spos_s, comp, fpos_s)
        };
        self.gui.draw(
            debug_strs.clone(),
            DisplayStats {
                player: self.stats.player_xp.get_xps(),
                notes: self.stats.get_display_stats(),
                buffs: self.stats.get_display_buffs(),
            },
            &mut GuiArgs {
                map: &self.map,
                player: &self.player,
                // stats: &self.stats.player_xp.get_xps(),
                enemies: &self.enemies,
                items: &self.items,
                npcs: &self.npcs,
                env_inter: Some(&self.env_inters),
                litems: Some(&litems),
                puzzle_pieces: Some(&self.puzzle_pieces),
                animate: None,
                ascii: None,
                ani_stats: &self.get_ani_stats(),
            },
        );
    }
}
