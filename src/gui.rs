//gui
use crate::enums::{GUIMode, Interactable, InterOpt, EncOpt, Equip, ItemEffect};
use crate::item::Item;

mod gui_man_draw;
mod npc_interactions;
mod enemy_encounters;
mod environment_interactions;

use ratatui::crossterm::style::PrintStyledContent;
use ratatui::widgets::Clear;
use ratatui::layout::Flex;
use std::io::stdout;
use ratatui::{Terminal, Frame};
use ratatui::backend::CrosstermBackend;
use ratatui::prelude::Line;
use ratatui::prelude::Rect;
use ratatui::widgets::{Block, Borders, Paragraph, Gauge};
use ratatui::layout::{Layout, Constraint, Direction, Margin};
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::{Text, Span};
use ratatui::widgets::Row;
use ratatui::widgets::Table;
use ratatui::widgets::Cell;
use std::collections::HashMap;
use crate::gui_utils::{GuiArgs, draw_map};

//#[derive(Serialize, Deserialize, Debug)]
pub struct GUI {
    terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
    info_mode: GUIMode,
    ani_cnt: u8,
    ani_updt: u8,
    cursor_pos: (usize, usize),
    cursor_hold: (usize, usize),
    menu_lvl: usize,
    viewport_dim: (usize, usize),
    interactable: HashMap<(usize, usize), Option<Interactable>>,
    adj_options: (Vec<((usize, usize), String)>, Vec<((usize, usize), String)>),
    inter_opt: HashMap<InterOpt, String>,
    inter_options: (Vec<(InterOpt, String)>, Vec<(InterOpt, String)>),
    inventory: Vec<Item>,
    inv_opt: (Vec<(usize, Item)>, Vec<(usize, Item)>, Vec<(usize, Item)>),
    comp_head: (i16, i16),
    comp_list: Vec<String>,
    comp_opts: (Vec<String>, Vec<String>, Vec<String>, Vec<String>, Vec<String>, Vec<String>),
    notes_opt: (Vec<String>, Vec<String>),
    active_notes: (HashMap<String, String>, Vec<String>, HashMap<String, String>, HashMap<String, String>),
    enc_opt: (Vec<(EncOpt, String)>, Vec<(EncOpt, String)>),
    help: bool,
}


fn draw_setup(f: &mut Frame) -> (std::rc::Rc<[ratatui::layout::Rect]>, ratatui::layout::Rect) {
    let entire_screen_block = Block::default()
        .style(Style::default().bg(Color::Black))
        .borders(Borders::NONE);
    f.render_widget(entire_screen_block, f.area());

    let chunks = Layout::default()
    .direction(Direction::Vertical)
    .margin(1)
    .constraints(
        [
            Constraint::Percentage(10),
            Constraint::Percentage(80),
            Constraint::Percentage(10)
        ].as_ref()
    )
    .split(f.area());


    let game_chunks = Layout::default()
    .direction(Direction::Horizontal)
    .constraints(
        [
            Constraint::Percentage(70),
            Constraint::Percentage(30)
        ].as_ref()
    )
    .split(chunks[1]);

    let block = Block::default()
                .title("Game")
                .borders(Borders::ALL);
    f.render_widget(block.clone(), game_chunks[0]);
    let block_area = game_chunks[0];
    f.render_widget(block.clone(), block_area);
    let inner_area = block_area.inner(Margin::default());
    let in_h = inner_area.height as usize;
    let in_w = inner_area.width as usize;
    // let mut map = &gui_args.map;
    // if in_h != self.viewport_dim.1 && in_w != self.viewport_dim.0 {
    //     // map.set_viewport(in_h, in_w);
    //     self.viewport_dim = (in_w, in_h);
    // }
    (game_chunks, inner_area)
}

impl GUI {
    pub fn new() -> Self {
        let stdout = stdout();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal.clear().unwrap();
        terminal.hide_cursor().unwrap();
        let mut interactable = HashMap::new();
        let inter_opt = HashMap::new();
        interactable.insert((0_usize, 0_usize), Some(Interactable::Null));
        let adj_options = (
            vec![((0_usize, 0_usize), "".to_string()); 3],
            vec![((0_usize, 0_usize), "".to_string()); 3],
        );
        let inter_options = (
            vec![(InterOpt::Null, "".to_string()); 3],
            vec![(InterOpt::Null, "".to_string()); 3],
        );
        let i_temp = Item::default();
        // let i_temp = Item::new(Items::Null, itype, ' ', desc, iopts, false, Equip::Null, ItemEffect::Null, 0, 0, prop);
        let inv_opt = (
            vec![(0, i_temp.clone()); 25],
            vec![(0, i_temp.clone()); 25],
            vec![(0, i_temp.clone()); 25],
        );
        let inventory = Vec::new();
        let notes_opt = (
            vec!["".to_string(); 3],
            vec!["".to_string(); 3],
        );
        let enc_opt = (
            vec![(EncOpt::Null, "".to_string()); 3],
            vec![(EncOpt::Null, "".to_string()); 3],
        );

        let comp_list = Vec::new();
        let comp_opts = (
            vec!["".to_string(); 4],
            vec!["".to_string(); 4],
            vec!["".to_string(); 4],
            vec!["".to_string(); 4],
            vec!["".to_string(); 4],
            vec!["".to_string(); 4],
        );

        let help = false;

        Self {
            terminal,
            info_mode: GUIMode::Normal,
            ani_cnt: 0,
            ani_updt: 0,
            cursor_pos: (0, 0),
            cursor_hold: (0, 0),
            menu_lvl: 0,
            viewport_dim: (0, 0),
            interactable,
            adj_options,
            inter_opt,
            inter_options,
            inventory,
            inv_opt,
            comp_head: (0, 0),
            comp_list,
            comp_opts,
            notes_opt,
            active_notes: (HashMap::new(), Vec::new(), HashMap::new(), HashMap::new()),
            enc_opt,
            help,
        }
    }

    pub fn toggle_help(&mut self) {
        self.help = !self.help;
    }

    pub fn reset_enc_opt(&mut self) {
        self.enc_opt = (
            vec![(EncOpt::Null, "".to_string()); 3],
            vec![(EncOpt::Null, "".to_string()); 3],
        );
    }

    pub fn set_notes(&mut self, notes: (HashMap<String, String>, Vec<String>, HashMap<String, String>, HashMap<String, String>)) {
        self.active_notes = notes;
    }

    pub fn set_comp_head(&mut self, temp: (i16, i16)) {
        self.comp_head = ((temp.0 - 224), (temp.1 - 174));
    }

    pub fn set_comp_list(&mut self, ltemp: Vec<String>) {
        self.comp_list = ltemp;
    }

    pub fn set_inventory(&mut self, inv: Vec<Item>) {
        self.inventory = inv;
    }

    pub fn reset_cursor(&mut self) {
        self.cursor_pos = (0, 0);
        self.cursor_hold = (0, 0);
        self.menu_lvl = 0;
    }

    pub fn get_mode(&mut self) -> GUIMode {
        self.info_mode
    }

    pub fn set_inter_opt(&mut self, temp: HashMap<InterOpt, String>) {
        self.inter_opt = temp;
    }

    pub fn get_interactee(&mut self) -> ((usize, usize), &str) {
        let temp = self.cursor_pos.1;
        let adj_option = match temp {
            0 => &self.adj_options.0[self.cursor_pos.0],
            1 => &self.adj_options.1[self.cursor_pos.0],
            _ => todo!(),
        };
        (adj_option.0, &adj_option.1)
    }

    pub fn get_iopt(&mut self) -> (InterOpt, &str) {
        let temp = self.cursor_pos.1;
        let inter_option = match temp {
            0 => &self.inter_options.0[self.cursor_pos.0],
            1 => &self.inter_options.1[self.cursor_pos.0],
            _ => todo!(),
        };
        (inter_option.0, &inter_option.1)
    }

    pub fn get_inv_opt(&mut self) -> (usize, Item) {
        let temp = self.cursor_pos.0;
        let inv_option = match temp {
            0 => &self.inv_opt.0[self.cursor_pos.1],
            1 => &self.inv_opt.1[self.cursor_pos.1],
            2 => &self.inv_opt.2[self.cursor_pos.1],
            _ => todo!(),
        };
        (inv_option.0, inv_option.clone().1)
    }

    pub fn get_enc_opt(&mut self) -> (EncOpt, &str) {
        let temp = self.cursor_pos.1;
        let enc_option = match temp {
            0 => &self.enc_opt.0[self.cursor_pos.0],
            1 => &self.enc_opt.1[self.cursor_pos.0],
            _ => todo!(),
        };
        (enc_option.0, &enc_option.1)
    }

    pub fn get_comp_opt(&self) -> String {
        let temp = self.cursor_pos.1;
        let comp_option = match temp {
            0 => &self.comp_opts.0[self.cursor_pos.0],
            1 => &self.comp_opts.1[self.cursor_pos.0],
            2 => &self.comp_opts.2[self.cursor_pos.0],
            3 => &self.comp_opts.3[self.cursor_pos.0],
            4 => &self.comp_opts.4[self.cursor_pos.0],
            5 => &self.comp_opts.5[self.cursor_pos.0],
            _ => todo!(),
        };
        comp_option.to_string()
    }

    pub fn get_ysno(&mut self) -> bool {
        self.cursor_pos.0 == 0
    }

    pub fn get_cursor(&mut self) -> (usize, usize) {
        self.cursor_pos
    }

    pub fn get_cursor_hold(&mut self) -> (usize, usize) {
        self.cursor_hold
    }

    pub fn set_interactable(&mut self, temp: HashMap<(usize, usize), Option<Interactable>>) {
        self.interactable = temp;
    }

    pub fn set_info_mode(&mut self, temp: GUIMode) {
        self.info_mode = temp;
    }

    pub fn get_viewport(&mut self) -> (usize, usize) {
        self.viewport_dim
    }

    pub fn move_cursor(&mut self, dir: &str) {
        match dir {
            "UP" => if self.cursor_pos.1 > 0 {self.cursor_pos.1 -= 1},
            "LF" => if self.cursor_pos.0 > 0 {self.cursor_pos.0 -= 1},
            "RT" => self.cursor_pos.0 += 1,
            "DN" => self.cursor_pos.1 += 1,
            _ => todo!(),
        }
    }

    pub fn get_menu_lvl(&mut self) -> usize {
        self.menu_lvl
    }

    pub fn menu_lvl(&mut self, dir: &str) {
        match dir {
            "DN" => if self.menu_lvl < 1 {
                self.menu_lvl += 1;
                self.cursor_hold = self.cursor_pos;
                self.cursor_pos = (0, 0);
            },
            "UP" => if self.menu_lvl > 0 {
                self.menu_lvl -= 1;
                self.cursor_pos = self.cursor_hold;
                self.cursor_hold = (0, 0);
            },
            _ => {},
        }
    }

    

    pub fn draw(&mut self,
         debug: (String, String, String, String),
         gui_args: &mut GuiArgs
    ) {
        if self.ani_updt < 120 {
            self.ani_updt += 1;
            if self.ani_cnt < 120 {
                if self.ani_updt % 6 == 0 {
                    self.ani_cnt += 1;
                }
            } else {
                self.ani_cnt = 0;
            }
        } else {
            self.ani_updt = 0;
        }
        self.terminal.draw(|f| {
            let (game_chunks, inner_area) = draw_setup(f);
            let in_h = inner_area.height as usize;
            let in_w = inner_area.width as usize;
            // let mut map = &gui_args.map;
            if in_h != self.viewport_dim.1 && in_w != self.viewport_dim.0 {
                // map.set_viewport(in_h, in_w);
                self.viewport_dim = (in_w, in_h);
            }
            let paragraph = draw_map(
                // map.clone(),
                gui_args,
                self.ani_cnt
            );

            f.render_widget(paragraph, inner_area);

            //---
            let comp_str = format!("({}, {})", self.comp_head.0, self.comp_head.1);
            match self.info_mode {
                GUIMode::Bug => {
                    let info_block = Block::default()
                        .title("Information")
                        .borders(Borders::ALL)
                        .style(Style::default().bg(Color::Black));
                    let mut rows = vec![
                        Row::new(vec![
                            Span::styled("px: ", Style::default().fg(Color::White)),
                            Span::styled(gui_args.player.x.to_string(), Style::default().fg(Color::Yellow)),
                        ]),
                        Row::new(vec![
                            Span::styled("py: ", Style::default().fg(Color::White)),
                            Span::styled(gui_args.player.y.to_string(), Style::default().fg(Color::Yellow)),
                        ]),
                        Row::new(vec![
                            Span::styled("vx: ", Style::default().fg(Color::White)),
                            Span::styled(gui_args.map.viewport_x.to_string(), Style::default().fg(Color::Yellow)),
                        ]),
                        Row::new(vec![
                            Span::styled("vy: ", Style::default().fg(Color::White)),
                            Span::styled(gui_args.map.viewport_y.to_string(), Style::default().fg(Color::Yellow)),
                        ]),
                        Row::new(vec![
                            Span::styled("vw: ", Style::default().fg(Color::White)),
                            Span::styled(gui_args.map.viewport_width.to_string(), Style::default().fg(Color::Yellow)),
                        ]),
                        Row::new(vec![
                            Span::styled("vh: ", Style::default().fg(Color::White)),
                            Span::styled(gui_args.map.viewport_height.to_string(), Style::default().fg(Color::Yellow)),
                        ]),
                        Row::new(vec![
                            Span::styled("su: ", Style::default().fg(Color::White)),
                            Span::styled((gui_args.map.viewport_y + (gui_args.map.viewport_height / 7)).to_string(), Style::default().fg(Color::Yellow)),
                        ]),
                        Row::new(vec![
                            Span::styled("sd: ", Style::default().fg(Color::White)),
                            Span::styled(((gui_args.map.viewport_height + gui_args.map.viewport_y) - (gui_args.map.viewport_height / 7)).to_string(), Style::default().fg(Color::Yellow)),
                        ]),
                        Row::new(vec![
                            Span::styled("sl: ", Style::default().fg(Color::White)),
                            Span::styled((gui_args.map.viewport_x + (gui_args.map.viewport_width / 7)).to_string(), Style::default().fg(Color::Yellow)),
                        ]),
                        Row::new(vec![
                            Span::styled("sr: ", Style::default().fg(Color::White)),
                            Span::styled(((gui_args.map.viewport_width + gui_args.map.viewport_x) - (gui_args.map.viewport_width / 7)).to_string(), Style::default().fg(Color::Yellow)),
                        ]),
                        Row::new(vec![
                            Span::styled("gx: ", Style::default().fg(Color::White)),
                            Span::styled((gui_args.map.gen_x).to_string(), Style::default().fg(Color::Yellow)),
                        ]),
                        Row::new(vec![
                            Span::styled("gy: ", Style::default().fg(Color::White)),
                            Span::styled((gui_args.map.gen_y).to_string(), Style::default().fg(Color::Yellow)),
                        ]),
                        Row::new(vec![
                            Span::styled("tlen: ", Style::default().fg(Color::White)),
                            Span::styled((gui_args.map.tunnels.len()).to_string(), Style::default().fg(Color::Yellow)),
                        ]),
                        Row::new(vec![
                            Span::styled("enemies: ", Style::default().fg(Color::White)),
                            Span::styled((gui_args.enemies.len()).to_string(), Style::default().fg(Color::Yellow)),
                        ]),
                        Row::new(vec![
                            Span::styled("npcs: ", Style::default().fg(Color::White)),
                            Span::styled((gui_args.npcs.len()).to_string(), Style::default().fg(Color::Yellow)),
                        ]),
                        Row::new(vec![
                            Span::styled("items: ", Style::default().fg(Color::White)),
                            Span::styled((gui_args.items.len()).to_string(), Style::default().fg(Color::Yellow)),
                        ]),
                        Row::new(vec![
                            Span::styled("dfo: ", Style::default().fg(Color::White)),
                            Span::styled(debug.0, Style::default().fg(Color::Yellow)),
                         ]),
                        Row::new(vec![
                            Span::styled("compass: ", Style::default().fg(Color::White)),
                            Span::styled(comp_str, Style::default().fg(Color::Yellow)),
                         ]),
                        Row::new(vec![
                            Span::styled("gs_compass: ", Style::default().fg(Color::White)),
                            Span::styled(debug.2, Style::default().fg(Color::Yellow)),
                         ]),
                        // Row::new(vec![
                        //     Span::styled("settle_pos: ", Style::default().fg(Color::White)),
                        //     Span::styled(debug.1, Style::default().fg(Color::Yellow)),
                        //  ]),
                        Row::new(vec![
                            Span::styled("env_inters: ", Style::default().fg(Color::White)),
                            Span::styled((gui_args.env_inter.as_ref().unwrap().len()).to_string(), Style::default().fg(Color::Yellow)),
                         ]),
                    ];

                    let settle_pos = debug.1.split("#");

                    for i in settle_pos {
                        rows.push(
                            Row::new(vec![
                                Span::styled("-", Style::default().fg(Color::White)),
                                Span::styled(i, Style::default().fg(Color::Yellow)),
                             ])
                        );
                    }

                    let feat_pos = debug.3.split("#");

                    for i in feat_pos {
                        rows.push(
                            Row::new(vec![
                                Span::styled(":", Style::default().fg(Color::White)),
                                Span::styled(i, Style::default().fg(Color::Yellow)),
                             ])
                        );
                    }

                    let table = Table::new(rows, &[Constraint::Percentage(50), Constraint::Percentage(50)])
                                    .block(info_block);

                    f.render_widget(table, game_chunks[1]);
                },
                GUIMode::Normal => {
                    let normal_info = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(
                        [
                            Constraint::Percentage(18),
                            Constraint::Percentage(18),
                            Constraint::Percentage(14),
                            Constraint::Percentage(50)
                        ].as_ref()
                    )
                    .split(game_chunks[1]);
                    let h_block = Block::default()
                        .title(Span::styled("Health", Style::default().fg(Color::DarkGray)))
                        .borders(Borders::ALL)
                        .style(Style::default().bg(Color::Black));
                    let stat_block = Block::default()
                        .title(Span::styled("Stats", Style::default().fg(Color::DarkGray)))
                        .borders(Borders::ALL)
                        .style(Style::default().bg(Color::Black));

                    let enchant_block = Block::default()
                        .title(Span::styled("Enchantments", Style::default().fg(Color::DarkGray)))
                        .borders(Borders::ALL)
                        .style(Style::default().bg(Color::Black));

                    let weapon_block = Block::default()
                        .title(Span::styled("Weapon", Style::default().fg(Color::DarkGray)))
                        .borders(Borders::ALL)
                        .style(Style::default().bg(Color::Black));

                    let shield_block = Block::default()
                        .title(Span::styled("Shield", Style::default().fg(Color::DarkGray)))
                        .borders(Borders::ALL)
                        .style(Style::default().bg(Color::Black));

                    let armour_block = Block::default()
                        .title(Span::styled("Armour", Style::default().fg(Color::DarkGray)))
                        .borders(Borders::ALL)
                        .style(Style::default().bg(Color::Black));

                    let wearing_block = Block::default()
                        .title(Span::styled("Wearing", Style::default().fg(Color::DarkGray)))
                        .borders(Borders::ALL)
                        .style(Style::default().bg(Color::Black));

                    let equip_layout = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(
                        [
                            Constraint::Percentage(50),
                            Constraint::Percentage(50),
                        ].as_ref()
                    )
                    .split(normal_info[3]);

                    let hands_layout = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints(
                        [
                            Constraint::Percentage(50),
                            Constraint::Percentage(50),
                        ].as_ref()
                    )
                    .split(equip_layout[0]);

                    let body_layout = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints(
                        [
                            Constraint::Percentage(50),
                            Constraint::Percentage(50),
                        ].as_ref()
                    )
                    .split(equip_layout[1]);

                    let h_gauge = Gauge::default()
                        .block(Block::bordered().title("Health"))
                        .gauge_style(Style::new().light_red().on_black().italic())
                        .percent(gui_args.player.health);

                    let rows = vec![
                        Row::new(vec![
                            Span::styled("Attack: ", Style::default().fg(Color::White)),
                            Span::styled(gui_args.player.attack.to_string(), Style::default().fg(Color::Yellow)),
                            Span::styled("Attack xp: ", Style::default().fg(Color::White)),
                            Span::styled(gui_args.stats[0].to_string(), Style::default().fg(Color::Yellow)),
                            Span::styled("Trading xp: ", Style::default().fg(Color::White)),
                            Span::styled(gui_args.stats[4].to_string(), Style::default().fg(Color::Yellow)),
                        ]),
                        Row::new(vec![
                            Span::styled("Damage: ", Style::default().fg(Color::White)),
                            Span::styled(gui_args.player.damage.to_string(), Style::default().fg(Color::Yellow)),
                            Span::styled("Damage xp: ", Style::default().fg(Color::White)),
                            Span::styled(gui_args.stats[1].to_string(), Style::default().fg(Color::Yellow)),
                            Span::styled("Lockpicking xp: ", Style::default().fg(Color::White)),
                            Span::styled(gui_args.stats[5].to_string(), Style::default().fg(Color::Yellow)),
                        ]),
                        Row::new(vec![
                            Span::styled("Defence: ", Style::default().fg(Color::White)),
                            Span::styled(gui_args.player.defence.to_string(), Style::default().fg(Color::Yellow)),
                            Span::styled("Defence xp: ", Style::default().fg(Color::White)),
                            Span::styled(gui_args.stats[2].to_string(), Style::default().fg(Color::Yellow)),
                            Span::styled("Navigation xp: ", Style::default().fg(Color::White)),
                            Span::styled(gui_args.stats[6].to_string(), Style::default().fg(Color::Yellow)),
                        ]),
                        Row::new(vec![
                            Span::styled("Money: ", Style::default().fg(Color::White)),
                            Span::styled(gui_args.player.money.to_string(), Style::default().fg(Color::Yellow)),
                            Span::styled("Luck xp: ", Style::default().fg(Color::White)),
                            Span::styled(gui_args.stats[3].to_string(), Style::default().fg(Color::Yellow)),
                            Span::styled("Herbalism xp: ", Style::default().fg(Color::White)),
                            Span::styled(gui_args.stats[7].to_string(), Style::default().fg(Color::Yellow)),
                        ]),
                        Row::new(vec![
                            Span::styled("", Style::default().fg(Color::White)),
                            Span::styled("", Style::default().fg(Color::Yellow)),
                            Span::styled("", Style::default().fg(Color::White)),
                            Span::styled("", Style::default().fg(Color::Yellow)),
                            Span::styled("", Style::default().fg(Color::White)),
                            Span::styled("", Style::default().fg(Color::Yellow)),
                        ]),
                    ];
                    let stats = Table::new(rows, &[
                        Constraint::Percentage(25),
                        Constraint::Percentage(5),
                        Constraint::Percentage(25),
                        Constraint::Percentage(5),
                        Constraint::Percentage(27),
                        Constraint::Percentage(10)])
                        .block(stat_block);

                    let enchant_data = [
                        vec!["", "", ""],
                    ];
                    let en_rows: Vec<Row> = enchant_data.iter().enumerate().map(|(j, row)| {
                        let cells: Vec<Cell> = row.iter().enumerate().map(|(i, &cell)| {
                            if i == self.cursor_pos.0 && j == self.cursor_pos.1 {
                                Cell::from(Span::styled(cell, ratatui::style::Style::default().fg(ratatui::style::Color::Yellow)))
                            } else {
                                Cell::from(cell)
                            }
                        }).collect();
                        Row::new(cells)
                    }).collect();
                    let en_table = Table::new(en_rows, &[Constraint::Percentage(33), Constraint::Percentage(33), Constraint::Percentage(33)])
                        .block(enchant_block);
                    
                    let mut equip_items = HashMap::new();
                    //let mut equip_buff = Vec::new();
                    let equip = gui_args.player.get_equipped();
                    let mut keys: Vec<_> = equip.keys().collect();
                    keys.sort();
                    for k in keys {
                        let mut itm = equip[k].clone();
                        let etype = itm.get_equip_type();
                        let prop = itm.get_properties().clone();
                        let e_type = itm.get_effect();
                        let icon = itm.icon; 
                        let efct = match e_type {
                            ItemEffect::Health => format!("Health: +{} | {}", prop["health"], icon.0),
                            ItemEffect::Attack => format!("Atack: +{} | {}", prop["attack"], icon.0),
                            ItemEffect::Damage => format!("Damage: +{} | {}", prop["damage"], icon.0),
                            ItemEffect::Defence => format!("Defence: +{} | {}", prop["defence"], icon.0),
                            _ => todo!(),
                        };
                        let estr = format!("{}\n{}", itm.get_sname(), efct);
                        equip_items.insert(etype, estr);
                    }

                    let def_str = "".to_string();
                    let w_str = equip_items.get(&Equip::Weapon).unwrap_or(&def_str);
                    let weapon_para = Paragraph::new(Text::raw(w_str))
                        .block(weapon_block);
                    f.render_widget(weapon_para, hands_layout[0]);

                    let s_str = equip_items.get(&Equip::Shield).unwrap_or(&def_str);
                    let shield_para = Paragraph::new(Text::raw(s_str))
                        .block(shield_block);
                    f.render_widget(shield_para, hands_layout[1]);

                    let t_str = equip_items.get(&Equip::Armour).unwrap_or(&def_str);
                    let armour_para = Paragraph::new(Text::raw(t_str))
                        .block(armour_block);
                    f.render_widget(armour_para, body_layout[0]);

                    let f_str = equip_items.get(&Equip::Wearing).unwrap_or(&def_str);
                    let wearing_para = Paragraph::new(Text::raw(f_str))
                        .block(wearing_block);
                    f.render_widget(wearing_para, body_layout[1]);

                  
                    f.render_widget(h_block, normal_info[0]);
                    f.render_widget(h_gauge, normal_info[0]);
                    f.render_widget(stats, normal_info[1]);
                    f.render_widget(en_table, normal_info[2]);
                    //f.render_widget(eq_table, normal_info[3]);
                },
                GUIMode::Map => {
                    let normal_info = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(
                        [
                            Constraint::Percentage(80),
                            Constraint::Percentage(20)
                        ].as_ref()
                    )
                    .split(game_chunks[1]);
                    let paragraph_block = Block::default()
                        .title(Span::styled("Compass", Style::default().fg(Color::DarkGray)))
                        .borders(Borders::ALL)
                        .style(Style::default().bg(Color::Black));
                    let table_block = Block::default()
                        .title(Span::styled("Heading", Style::default().fg(Color::DarkGray)))
                        .borders(Borders::ALL)
                        .style(Style::default().bg(Color::Black));

                    let upper_region: Rect = normal_info[0];
                    let width = upper_region.width;
                    let height = upper_region.height;
                    let symbol_char = '#';
                    let mut content = String::new();
                    let cen_x = width/2;
                    let cen_y = height/2;
                    let slope = cen_y as f32 / cen_x as f32;

                    //log::info!("dist_fo: {:?}", self.dist_fo);

                    //-----------

                    
                    let map = false;
                    if map {
                        for y in 0..height {
                            for x in 0..width {
                                if y % 5 == 0 && x % 4 == 0 {
                                    content.push('#');
                                } else {
                                    content.push('.');
                                }
                            }
                            content.push('\n');
                        }
                        let compass = Paragraph::new(Text::raw(content))
                            .block(paragraph_block);
                        f.render_widget(compass, upper_region);
                    } else {
                        match self.comp_head {
                            (dx, dy) if dx > 0 && dx.abs() >= dy.abs() => {
                                for y in 0..height {
                                    for x in 0..width {
                                        if y < cen_y && (y + 1) as f32 >= slope * (width - (x + 1)) as f32 {
                                            content.push('#');
                                        } else if y >= cen_y && x > cen_x && (y + 0) as f32 <= slope * (x - 0) as f32 {
                                            content.push('#');
                                        } else {
                                            content.push('.');
                                        }
                                    }
                                    content.push('\n');
                                }
                                let compass = Paragraph::new(Text::raw(content))
                                    .block(paragraph_block);
                                f.render_widget(compass, upper_region);
                            },
                            (dx, dy) if dx < 0 && dx.abs() >= dy.abs() => {
                                for y in 0..height {
                                    for x in 0..width {
                                        if y <= cen_y && y as f32 >= slope * x as f32 {
                                            content.push('#');
                                        } else if y > cen_y && y as f32 <= slope * (width - x) as f32{
                                            content.push('#');
                                        } else {
                                            content.push('.');
                                        }
                                    }
                                    content.push('\n');
                                }
                                let compass = Paragraph::new(Text::raw(content))
                                    .block(paragraph_block);
                                f.render_widget(compass, upper_region);
                            },
                            (dx, dy) if dy > 0 && dy.abs() >= dx.abs() => {
                                for y in 0..height {
                                    for x in 0..width {
                                        if x < cen_x && y >= cen_y - 1 && (y + 1) as f32 >= slope * (width - (x + 3)) as f32 {
                                            content.push('#');
                                        } else if x >= cen_x && y >= cen_y - 1 && y as f32 >= slope * (x - 2) as f32 {
                                            content.push('#');
                                        } else {
                                            content.push('.');
                                        }
                                    }
                                    content.push('\n');
                                }
                                let compass = Paragraph::new(Text::raw(content))
                                    .block(paragraph_block);
                                f.render_widget(compass, upper_region);
                            },
                            (dx, dy) if dy < 0 &&  dy.abs() >= dx.abs() => {
                                for y in 0..height {
                                    for x in 0..width {
                                        if x < cen_x && y < cen_y && y as f32 <= slope * x as f32 {
                                            content.push('#');
                                        } else if x >= cen_x && y < cen_y && (y + 0) as f32 <= slope * (width - (x + 1)) as f32 {
                                            content.push('#');
                                        } else {
                                            content.push('.');
                                        }
                                    }
                                    content.push('\n');
                                }
                                let compass = Paragraph::new(Text::raw(content))
                                    .block(paragraph_block);
                                f.render_widget(compass, upper_region);
                            },
                            (0, 0) => {
                                for y in 0..height {
                                    for x in 0..width {
                                        let dx = (cen_x as isize - x as isize).abs();
                                        let dy = (cen_y as isize - y as isize).abs();
                                        if dx + dy <= cen_x as isize {
                                            content.push(symbol_char);
                                        } else {
                                            content.push('.');
                                        }
                                    }
                                    content.push('\n');
                                }
                                let compass = Paragraph::new(Text::raw(content))
                                    .block(paragraph_block);
                                f.render_widget(compass, upper_region);
                            },
                            _ => {},
                        }
                    }
                   
                    let xy_block = Block::default()
                        .borders(Borders::ALL)
                        .style(Style::default().bg(Color::Black));
                    let xy_area = Rect {
                        x: upper_region.x,
                        y: upper_region.y,
                        width: upper_region.width / 2,
                        height: 3,
                    };
                    let xy_str = format!("target: ({}, {})", self.comp_head.0, self.comp_head.1);
                    let xy_para = Paragraph::new(Text::raw(xy_str))
                        .block(xy_block);
                    f.render_widget(Clear, xy_area);
                    f.render_widget(xy_para, xy_area);
                    

                    let mut vec1 = vec!["".to_string(); 4];
                    let mut vec2 = vec!["".to_string(); 4];
                    let mut vec3 = vec!["".to_string(); 4];
                    let mut vec4 = vec!["".to_string(); 4];
                    let mut vec5 = vec!["".to_string(); 4];
                    let mut vec6 = vec!["".to_string(); 4];

                    let cmp_list = self.comp_list.clone();
                    let cmp_scroll = if cmp_list.len() > 23 {
                        &cmp_list[0..23]   
                    } else {
                        &cmp_list[0..]
                    }; 
                    
                    vec1[0] = "Search".to_string();
                    for (idx, names) in cmp_scroll.iter().enumerate() {
                        if idx < 3 {
                            vec1[idx+1] = names.clone();
                        } else if (3..7).contains(&idx) {
                            vec2[idx-3] = names.clone();
                        } else if (7..11).contains(&idx) {
                            vec3[idx-7] = names.clone();
                        } else if (11..15).contains(&idx) {
                            vec4[idx-11] = names.clone();
                        } else if (15..19).contains(&idx) {
                            vec5[idx-15] = names.clone();
                        } else {
                            vec6[idx-19] = names.clone();
                        }
                    }
                    self.comp_opts = (vec1.clone(), vec2.clone(), vec3.clone(), vec4.clone(), vec5.clone(), vec6.clone()); 
                    let inv_table = [vec1.clone(), vec2.clone(), vec3.clone(), vec4.clone(), vec5.clone(), vec6.clone()];
                    //
                    let rows: Vec<Row> = inv_table.iter().enumerate().map(|(j, row)| {
                        let cells: Vec<Cell> = row.iter().enumerate().map(|(i, cell)| {
                            if i == self.cursor_pos.0 && j == self.cursor_pos.1 {
                                Cell::from(Span::styled(cell, ratatui::style::Style::default().fg(ratatui::style::Color::Yellow)))
                            } else {
                                Cell::from(Span::raw(cell))
                            }
                        }).collect();
                        Row::new(cells)
                    }).collect();
                    let table = Table::new(rows, &[Constraint::Percentage(25), Constraint::Percentage(25), Constraint::Percentage(25), Constraint::Percentage(25)])
                        .block(table_block);
                    f.render_widget(table, normal_info[1]);
                },
                GUIMode::Inventory => {
                    let normal_info = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(
                        [
                            Constraint::Percentage(30),
                            Constraint::Percentage(70)
                        ].as_ref()
                    )
                    .split(game_chunks[1]);
                    let paragraph_block = Block::default()
                        .title(Span::styled("Inventory", Style::default().fg(Color::DarkGray)))
                        .borders(Borders::ALL)
                        .style(Style::default().bg(Color::Black));
                    let table_block = Block::default()
                        .title(Span::styled("Items", Style::default().fg(Color::DarkGray)))
                        .borders(Borders::ALL)
                        .style(Style::default().bg(Color::Black));

                    // let prop = HashMap::new();
                    // let itype = String::new();
                    // let desc = String::new();
                    // let iopts = HashMap::new();
                    let i_temp = Item::default();
                    let mut col1 = vec![(0, i_temp.clone()); 25];
                    let mut col2 = vec![(0, i_temp.clone()); 25];
                    let mut col3 = vec![(0, i_temp.clone()); 25];

                    for (idx, item) in self.inventory.iter().enumerate() {
                        if idx < 25 {
                            col1[idx] = (idx.clone(), item.clone());
                        } else if idx < 50 {
                            col2[idx - 25] = (idx.clone(), item.clone());
                        } else {
                            col3[idx - 50] = (idx.clone(), item.clone());
                        }
                    }
                    //xx
                    let inv_table: Vec<Vec<(usize, Item)>> = vec![col1.clone(), col2.clone(), col3.clone()];
                    self.inv_opt = (col1, col2, col3);
                    //xx
                    let rows: Vec<Row> = (0..25).map(|i| {
                        let cells: Vec<Cell> = inv_table.iter().enumerate().map(|(j, col)| {
                            if i == self.cursor_pos.1 && j == self.cursor_pos.0 {
                                Cell::from(Span::styled(col[i].1.sname.clone(), ratatui::style::Style::default().fg(ratatui::style::Color::Yellow)))
                            } else {
                                Cell::from(col[i].1.sname.clone())
                            }
                        }).collect();
                        Row::new(cells)
                    }).collect();

                    let table = Table::new(rows, &[Constraint::Percentage(33), Constraint::Percentage(33), Constraint::Percentage(33)])
                        .block(table_block);

                    let temp = self.cursor_pos.0;
                    let inv_option = match temp {
                        0 => &mut self.inv_opt.0[self.cursor_pos.1],
                        1 => &mut self.inv_opt.1[self.cursor_pos.1],
                        2 => &mut self.inv_opt.2[self.cursor_pos.1],
                        _ => todo!(),
                    };

                    let mut itm = inv_option.1.clone();
                    let i_sel = itm.clone().get_desc();
                    let mut props = Vec::new();
                    props.push(Line::from(Span::raw(i_sel)));
                    for (s, i) in &itm.get_properties() {
                        let fmt_prop = format!("{}: {}", s, i);
                        props.push(Line::from(Span::raw(fmt_prop)));
                    }
                    let fmt_prop = "┌───┐".to_string();
                    props.push(Line::from(Span::raw(fmt_prop)));
                    let fmt_prop = format!("| {} |", itm.icon.0);
                    props.push(Line::from(Span::raw(fmt_prop)));
                    let fmt_prop = "└───┘".to_string();
                    props.push(Line::from(Span::raw(fmt_prop)));


                    let paragraph = Paragraph::new(Text::from(props))
                        .block(paragraph_block)
                        .wrap(ratatui::widgets::Wrap { trim: true });


                    f.render_widget(paragraph, normal_info[0]);
                    f.render_widget(table, normal_info[1]);
                },
                GUIMode::Notes => {
                    let normal_info = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(
                        [
                            Constraint::Percentage(10),
                            Constraint::Percentage(90)
                        ].as_ref()
                    )
                    .split(game_chunks[1]);
                    let notes_block = Block::default()
                        .title(Span::styled("Notes", Style::default().fg(Color::DarkGray)))
                        .borders(Borders::ALL)
                        .style(Style::default().bg(Color::Black));
                    let note_block = Block::default()
                        .title("")
                        .borders(Borders::ALL)
                        .style(Style::default().bg(Color::Black));

                    let vec1 = vec!["Settlements".to_string(), "Conversations".to_string(), "Knowledge".to_string(), "Tasks".to_string()];
                    let vec2 = vec!["".to_string(), "".to_string(), "".to_string(), "".to_string()];

                    let inv_table: Vec<Vec<String>, > = vec![vec1.clone(), vec2.clone()];
                    self.notes_opt = (vec1, vec2);
                    let rows: Vec<Row> = inv_table.iter().enumerate().map(|(j, row)| {
                        let cells: Vec<Cell> = row.iter().enumerate().map(|(i, cell)| {
                            if self.menu_lvl > 0 {
                                if i == self.cursor_hold.0 && j == self.cursor_hold.1 {
                                    Cell::from(Span::styled(cell.as_str(), ratatui::style::Style::default().fg(ratatui::style::Color::Yellow)))
                                } else {
                                    Cell::from(cell.as_str())
                                }
                            } else {
                                if i == self.cursor_pos.0 {
                                    Cell::from(Span::styled(cell.as_str(), ratatui::style::Style::default().fg(ratatui::style::Color::Yellow)))
                                } else {
                                    Cell::from(cell.as_str())
                                }
                            }
                        }).collect();
                        Row::new(cells)
                    }).collect();
                    let table = Table::new(rows, &[Constraint::Percentage(27), Constraint::Percentage(30), Constraint::Percentage(20), Constraint::Percentage(10)])
                        .block(notes_block);

                    let c_hold = &self.cursor_hold.0;

                    let paragraph = if self.menu_lvl == 0 {
                        match self.cursor_pos.0 {
                            0 => {
                                let mut tvec = Vec::new();
                                for (i, v) in self.active_notes.0.keys().enumerate() {
                                    if self.cursor_pos.1 == i {
                                        tvec.push(Line::from(Span::styled(v, Style::default().fg(Color::Yellow))));
                                    } else {
                                        tvec.push(Line::from(Span::styled(v, Style::default().fg(Color::White))));
                                    }
                                }
                                let text = Text::from(tvec);
                                let paragraph = Paragraph::new(text)
                                    .block(note_block)
                                    .alignment(ratatui::layout::Alignment::Center)
                                    .wrap(ratatui::widgets::Wrap { trim: true });
                                paragraph
                            },
                            1 => {
                                let mut tvec = Vec::new();
                                for v in &self.active_notes.1 {
                                    let sp = v.split("#");
                                    for l in sp {
                                        tvec.push(Line::from(Span::styled(l, Style::default().fg(Color::White))));
                                        tvec.push(
                                            Line::from(Span::raw(""))
                                        );
                                    }
                                    tvec.push(
                                        Line::from(Span::raw("--"))
                                    );
                                }
                                let text = Text::from(tvec);
                                let paragraph = Paragraph::new(text)
                                    .block(note_block)
                                    .wrap(ratatui::widgets::Wrap { trim: true })
                                    .scroll((self.cursor_pos.1.try_into().expect("oope"), 0));
                                paragraph
                            },
                            2 => {
                                let mut tvec = Vec::new();
                                for (i, v) in self.active_notes.2.keys().enumerate() {
                                    if self.cursor_pos.1 == i {
                                        tvec.push(Line::from(Span::styled(v, Style::default().fg(Color::Yellow))));
                                    } else {
                                        tvec.push(Line::from(Span::styled(v, Style::default().fg(Color::White))));
                                    }
                                }
                                let text = Text::from(tvec);
                                let paragraph = Paragraph::new(text)
                                    .block(note_block)
                                    .wrap(ratatui::widgets::Wrap { trim: true });
                                paragraph
                            },
                            3 => {
                                let mut tvec = Vec::new();
                                for (i, v) in self.active_notes.3.keys().enumerate() {
                                    if self.cursor_pos.1 == i {
                                        tvec.push(Line::from(Span::styled(v, Style::default().fg(Color::Yellow))));
                                    } else {
                                        tvec.push(Line::from(Span::styled(v, Style::default().fg(Color::White))));
                                    }
                                }
                                let text = Text::from(tvec);
                                let paragraph = Paragraph::new(text)
                                    .block(note_block)
                                    .wrap(ratatui::widgets::Wrap { trim: true });
                                paragraph
                            },
                            _ => {
                                let paragraph = Paragraph::new(Span::raw("Notes for the user."))
                                    .block(note_block);
                                paragraph
                            },
                        }
                    } else {
                        match c_hold {
                            0 => {
                                // let mut tvec = Vec::new();
                                // for (k, v) in &self.active_notes.0 {
                                //     tvec.push(
                                //         Line::from(Span::raw(k))
                                //     );
                                //     tvec.push(
                                //         Line::from(Span::raw(v))
                                //     );
                                // }
                                let settle = self.active_notes.0.clone().into_values().collect::<Vec<String>>()[self.cursor_pos.1].clone();
                                let paragraph = Paragraph::new(Text::raw(settle))
                                    .block(note_block)
                                    .alignment(ratatui::layout::Alignment::Center)
                                    .wrap(ratatui::widgets::Wrap { trim: true });
                                paragraph
                            },
                            1 => {
                                let text = Text::from(Line::from(Span::raw(" ")));
                                let paragraph = Paragraph::new(text)
                                    .block(note_block)
                                    .wrap(ratatui::widgets::Wrap { trim: true });
                                paragraph
                            },
                            2 => {
                                let mut tvec = Vec::new();
                                for (k, v) in &self.active_notes.2 {
                                    tvec.push(
                                        Line::from(Span::raw(k))
                                    );
                                    tvec.push(
                                        Line::from(Span::raw(v))
                                    );
                                }
                                let text = Text::from(tvec);
                                let paragraph = Paragraph::new(text)
                                    .block(note_block)
                                    .wrap(ratatui::widgets::Wrap { trim: true });
                                paragraph
                            },
                            3 => {
                                let mut tvec = Vec::new();
                                for (k, v) in &self.active_notes.3 {
                                    tvec.push(
                                        Line::from(Span::raw(k))
                                    );
                                    tvec.push(
                                        Line::from(Span::raw(v))
                                    );
                                }
                                let text = Text::from(tvec);
                                let paragraph = Paragraph::new(text)
                                    .block(note_block)
                                    .wrap(ratatui::widgets::Wrap { trim: true });
                                paragraph
                            },
                            _ => {
                                let paragraph = Paragraph::new(Span::raw("Notes for the user."))
                                    .block(note_block);
                                paragraph
                            },
                        }
                    };

                    f.render_widget(table, normal_info[0]);
                    f.render_widget(paragraph, normal_info[1]);
                },
                // GUIMode::NPC => {},
                // GUIMode::Fight => {},
                _ => todo!(),
            }
            if self.help {
                let a = f.area();
                let b = Block::bordered();
                let (xper, yper) = (60, 20); 
                let harea = |a, xper, yper| {
                    let vertical = Layout::vertical([Constraint::Percentage(yper)]).flex(Flex::Center);
                    let horizontal = Layout::horizontal([Constraint::Percentage(xper)]).flex(Flex::Center); 
                    let [area] = vertical.areas(a);
                    let [area] = horizontal.areas(a);
                    area
                };
                let h_area = harea(a, xper, yper);
                f.render_widget(Clear, h_area);
                let h_block = Block::default()
                            .title("Game")
                            .borders(Borders::ALL);
                f.render_widget(h_block, h_area);
                
                let text = "Welcome to the caves!!\n\nHave a look around and see what you find. There are settlements scattered throughout the caves as well as ruins with puzzles and treasure! Be careful however and be sure to use your Compass! The caves constantly change and its easy to get lost!\n\nThe Caves are full of mosters and those who have lost themselves to the caves, so make sure you are careful and learn to protect yourself. Eating some items will heal you, others you can sell.\n\nHave a look around and have fun, chatting with others down here might give you more insight and point you in the right direction.\n\nMove around with the Arrow Keys, and use the 'q, w, e, r' buttons to access your menus. In standard play, the menus are navigated using the 'a, s, d, f' keys and Enter. During Encounters and Interactions, the menus are navigated using the Arrow Keys and Enter. In the notebook, Backspace is used to go up a level."; 
                let paragraph = Paragraph::new(text)
                    .wrap(ratatui::widgets::Wrap { trim: true });
                let para_area = Rect {
                    x: h_area.x + 5,
                    y: h_area.y + 2,
                    width: h_area.width - 10,
                    height: 15,
                };
                f.render_widget(paragraph, para_area);
                
                let table_area = Rect {
                    x: h_area.x + 5,
                    y: h_area.y + 20,
                    width: h_area.width - 10,
                    height: h_area.height - 25,
                };
                let rows = vec![
                    Row::new(["Key", "Action"]),
                    Row::new(["q", "Stats"]),
                    Row::new(["w", "Compass"]),
                    Row::new(["e", "Inventory"]),
                    Row::new(["r", "Notebook"]),
                    Row::new(["a", "Side Menu Left"]),
                    Row::new(["s", "Side Menu Up"]),
                    Row::new(["d", "Side Menu Down"]),
                    Row::new(["f", "Side Menu Right"]),
                    Row::new(["Space", "Interact with item/npc"]),
                    Row::new(["Enter", "Select option"]),
                    Row::new(["Up Arrow", "Move Player Up/Move Cursor Up"]),
                    Row::new(["Down Arrow", "Move Player Down/Move Cursor Down"]),
                    Row::new(["Left Arrow", "Move Player Left/Move Cursor Left"]),
                    Row::new(["Right Arrows", "Move Player Right/Move Cursor Right"]),
                ];

                let table = Table::new(rows, &[Constraint::Percentage(50), Constraint::Percentage(50)]).block(Block::bordered().title("Key Bindings"));
                f.render_widget(table, table_area);

            }
            
        }).unwrap();
    }



}
