//gui
use crate::enums::{Cells, Enemies, Items, NPCs, GUIMode, InterSteps, Interactable, InterOpt, EncOpt};
use crate::map::Map;
use crate::player::Player;
use crate::enemy::{Enemy};
use crate::npc::{NPC};
use crate::item::Item;
use crate::notebook::{Quest, Stage, Place, Person, Lore};
mod gui_man_draw;



use ratatui::crossterm::event::{read, Event, KeyCode, KeyEvent, poll};
use ratatui::crossterm::terminal;
use ratatui::crossterm::event::KeyEventKind::{Press, Release};
use std::io::stdout;
// use std::time::Duration;
use rand::Rng;
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use ratatui::prelude::Line;
use ratatui::prelude::Rect;
use ratatui::widgets::{Block, Borders, Paragraph, Wrap, Padding};
use ratatui::layout::{Layout, Constraint, Direction, Margin};
use ratatui::style::{Color, Style};
use ratatui::text::{Text, Span};
use ratatui::widgets::Row;
use ratatui::widgets::Table;
use ratatui::widgets::Cell;

use std::collections::HashMap;
use std::collections::HashSet;


fn draw_map<'a>(mut map: Map, player: Player, enemies: HashMap<(usize, usize), Enemy>, items: HashMap<(usize, usize), Item>, npcs: HashMap<String, Box<dyn NPC>>) -> Paragraph<'a> {
    let start_row = map.viewport_y;
    let end_row = (map.viewport_y + map.viewport_height).min(map.cells.len());
    let start_col = map.viewport_x;
    let end_col = (map.viewport_x + map.viewport_width).min(map.cells[0].len());
    let mut text = Vec::new();
    for (j, row) in map.cells[start_row..end_row].iter().enumerate() {
        let mut line = Vec::new();
        for (i, &cell) in row[start_col..end_col].iter().enumerate() {
            let (symbol, color) = {
                let ix = i + start_col;
                let jy = j + start_row;
                if (ix, jy) == (player.x, player.y) {
                    ('&', Color::Blue)
                } else if let Some(enemy) = enemies.get(&(ix, jy)) {
                    match enemy.etype {
                        Enemies::Bug => ('B', Color::Red),
                        Enemies::GoblinMan => ('G', Color::Red),
                        Enemies::CrazedExplorer => ('C', Color::Red),
                        _ => todo!(),
                    }
                } else if let Some(npc) = npcs.get(&(ix, jy)) {
                    match npc.base.etype {
                        NPCs::CommNPC=> ('$', Color::Blue),
                        NPCs::ConvNPC=> ('$', Color::LightBlue),
                        NPCs::QuestNPC=> ('$', Color::Cyan),
                        _ => todo!(),
                    }
                } else if let Some(item) = items.get(&(ix, jy)) {
                    match item.itype {
                        Items::Rock => ('o', Color::Cyan),
                        Items::EdibleRoot => ('o', Color::Cyan),
                        Items::Apple => ('o', Color::Cyan),
                        Items::MetalScrap => ('o', Color::Cyan),
                        Items::BugBits => ('o', Color::Cyan),
                        _ => todo!(),
                    }
                } else {
                    match cell {
                        Cells::Empty => (' ', Color::White),
                        Cells::Dirt1 => ('\'', Color::DarkGray),
                        Cells::Dirt2 => ('.', Color::DarkGray),
                        Cells::Grass1 => (',', Color::Green),
                        Cells::Grass2 => ('\'', Color::LightGreen),
                        Cells::Rock => ('*', Color::DarkGray),
                        Cells::Wall => ('▒', Color::LightCyan),
                        Cells::Tunnel => ('@', Color::Blue),
                    }
                }
            };
            let span = Span::styled(
                symbol.to_string(),
                Style::new().fg(color)
            );
            line.push(span);
        }
        let line: Line = Line::from(line);
        text.push(line);
    }
    let texts: Text<'a> = text.into_iter().collect();
    Paragraph::new(texts).block(Block::default().borders(Borders::NONE).padding(Padding{left: 1, right: 1, top: 1, bottom: 1}).style(Style::default().bg(Color::Black)))
}



pub struct GUI {
    terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
    info_mode: GUIMode,
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
    dist_fo: (i64, i64, i64, i64),
    notes_opt: (Vec<String>, Vec<String>),
    active_notes: (Vec<Quest>, Vec<Place>, Vec<Person>, Vec<Lore>),
    enc_opt: (Vec<(EncOpt, String)>, Vec<(EncOpt, String)>),
}



impl GUI {
    pub fn new() -> Self {
        let stdout = stdout();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal.clear().unwrap();
        terminal.hide_cursor().unwrap();
        let mut interactable = HashMap::new();
        let mut inter_opt = HashMap::new();
        interactable.insert((0 as usize, 0 as usize), Some(Interactable::Null));
        let adj_options = (
            vec![((0 as usize, 0 as usize), "".to_string()); 3],
            vec![((0 as usize, 0 as usize), "".to_string()); 3],
        );
        let inter_options = (
            vec![(InterOpt::Null, "".to_string()); 3],
            vec![(InterOpt::Null, "".to_string()); 3],
        );
        let prop = HashMap::new();
        let itype = String::new();
        let desc = String::new();
        let iopts = HashMap::new();
        let i_temp = Item::new(Items::Null, itype, desc, iopts, 0, 0, prop);
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
        let quests = vec![Quest::default()];
        let places = vec![Place::default()];
        let people = vec![Person::default()];
        let lore = vec![Lore::default()];
        let enc_opt = (
            vec![(EncOpt::Null, "".to_string()); 3],
            vec![(EncOpt::Null, "".to_string()); 3],
        );

        Self {
            terminal,
            info_mode: GUIMode::Normal,
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
            dist_fo: (0, 0, 0, 0),
            notes_opt,
            active_notes: (quests, places, people, lore),
            enc_opt,
        }
    }

    pub fn reset_enc_opt(&mut self) {
        self.enc_opt = (
            vec![(EncOpt::Null, "".to_string()); 3],
            vec![(EncOpt::Null, "".to_string()); 3],
        );
    }

    pub fn set_notes(&mut self, notes: (Vec<Quest>, Vec<Place>, Vec<Person>, Vec<Lore>)) {
        self.active_notes = notes;
    }

    pub fn set_dist_fo(&mut self, temp: (i64, i64, i64, i64)) {
        self.dist_fo = temp;
    }

    pub fn set_inventory(&mut self, inv: Vec<Item>) {
        self.inventory = inv;
    }

    pub fn reset_cursor(&mut self) {
        self.cursor_pos = (0, 0);
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

    pub fn get_inv_opt(&mut self) -> ((usize), Item) {
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

    pub fn menu_lvl(&mut self, dir: &str) {
        match dir {
            "DN" => {
                self.menu_lvl += 1;
                self.cursor_hold = self.cursor_pos;
            },
            "UP" => if self.menu_lvl > 0 {
                self.menu_lvl -= 1;
                self.cursor_pos = self.cursor_hold;
            },
            _ => {},
        }
    }



    pub fn draw(&mut self, mut map: Map, player: Player, enemies: HashMap<(usize, usize), Enemy>, items: HashMap<(usize, usize), Item>, npcs: HashMap<String, Box<dyn NPC>>) {
        self.terminal.draw(|f| {
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
            .split(f.size());

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

            if in_h != self.viewport_dim.1 && in_w != self.viewport_dim.0 {
                map.set_viewport(in_h, in_w);
                self.viewport_dim = (in_w, in_h);
            }
            let paragraph = draw_map(map.clone(), player.clone(), enemies.clone(), items.clone());

            f.render_widget(paragraph, inner_area);

            //---

            match self.info_mode {
                GUIMode::Bug => {
                    let info_block = Block::default()
                        .title("Information")
                        .borders(Borders::ALL)
                        .style(Style::default().bg(Color::Black));
                    let rows = vec![
                        Row::new(vec![
                            Span::styled("px: ", Style::default().fg(Color::White)),
                            Span::styled(player.x.to_string(), Style::default().fg(Color::Yellow)),
                        ]),
                        Row::new(vec![
                            Span::styled("py: ", Style::default().fg(Color::White)),
                            Span::styled(player.y.to_string(), Style::default().fg(Color::Yellow)),
                        ]),
                        Row::new(vec![
                            Span::styled("vx: ", Style::default().fg(Color::White)),
                            Span::styled(map.viewport_x.to_string(), Style::default().fg(Color::Yellow)),
                        ]),
                        Row::new(vec![
                            Span::styled("vy: ", Style::default().fg(Color::White)),
                            Span::styled(map.viewport_y.to_string(), Style::default().fg(Color::Yellow)),
                        ]),
                        Row::new(vec![
                            Span::styled("vw: ", Style::default().fg(Color::White)),
                            Span::styled(map.viewport_width.to_string(), Style::default().fg(Color::Yellow)),
                        ]),
                        Row::new(vec![
                            Span::styled("vh: ", Style::default().fg(Color::White)),
                            Span::styled(map.viewport_height.to_string(), Style::default().fg(Color::Yellow)),
                        ]),
                        Row::new(vec![
                            Span::styled("su: ", Style::default().fg(Color::White)),
                            Span::styled((map.viewport_y + (map.viewport_height / 7)).to_string(), Style::default().fg(Color::Yellow)),
                        ]),
                        Row::new(vec![
                            Span::styled("sd: ", Style::default().fg(Color::White)),
                            Span::styled(((map.viewport_height + map.viewport_y) - (map.viewport_height / 7)).to_string(), Style::default().fg(Color::Yellow)),
                        ]),
                        Row::new(vec![
                            Span::styled("sl: ", Style::default().fg(Color::White)),
                            Span::styled((map.viewport_x + (map.viewport_width / 7)).to_string(), Style::default().fg(Color::Yellow)),
                        ]),
                        Row::new(vec![
                            Span::styled("sr: ", Style::default().fg(Color::White)),
                            Span::styled(((map.viewport_width + map.viewport_x) - (map.viewport_width / 7)).to_string(), Style::default().fg(Color::Yellow)),
                        ]),
                        Row::new(vec![
                            Span::styled("gx: ", Style::default().fg(Color::White)),
                            Span::styled((map.gen_x).to_string(), Style::default().fg(Color::Yellow)),
                        ]),
                        Row::new(vec![
                            Span::styled("gy: ", Style::default().fg(Color::White)),
                            Span::styled((map.gen_y).to_string(), Style::default().fg(Color::Yellow)),
                        ]),
                        Row::new(vec![
                            Span::styled("tlen: ", Style::default().fg(Color::White)),
                            Span::styled((map.tunnels.len()).to_string(), Style::default().fg(Color::Yellow)),
                        ]),
                        Row::new(vec![
                            Span::styled("dtlen: ", Style::default().fg(Color::White)),
                            Span::styled((map.dead_tunnels.len()).to_string(), Style::default().fg(Color::Yellow)),
                        ]),
                    ];

                    let table = Table::new(rows, &[Constraint::Percentage(50), Constraint::Percentage(50)])
                                    .block(info_block);

                    f.render_widget(table, game_chunks[1]);
                },
                GUIMode::Normal => {
                    let normal_info = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(
                        [
                            Constraint::Percentage(70),
                            Constraint::Percentage(30)
                        ].as_ref()
                    )
                    .split(game_chunks[1]);
                    let paragraph_block = Block::default()
                        .title("Game Stats")
                        .borders(Borders::ALL)
                        .style(Style::default().bg(Color::Black));
                    let table_block = Block::default()
                        .title("")
                        .borders(Borders::ALL)
                        .style(Style::default().bg(Color::Black));

                    let rows = vec![
                        Row::new(vec![
                            Span::styled("Health: ", Style::default().fg(Color::White)),
                            Span::styled(player.health.to_string(), Style::default().fg(Color::Yellow)),
                        ]),
                        // Row::new(vec![
                        //     Span::styled("py: ", Style::default().fg(Color::White)),
                        //     Span::styled(player.y.to_string(), Style::default().fg(Color::Yellow)),
                        // ]),
                        // Row::new(vec![
                        //     Span::styled("vx: ", Style::default().fg(Color::White)),
                        //     Span::styled(map.viewport_x.to_string(), Style::default().fg(Color::Yellow)),
                        // ]),
                        // Row::new(vec![
                        //     Span::styled("vy: ", Style::default().fg(Color::White)),
                        //     Span::styled(map.viewport_y.to_string(), Style::default().fg(Color::Yellow)),
                        // ]),
                        // Row::new(vec![
                        //     Span::styled("vw: ", Style::default().fg(Color::White)),
                        //     Span::styled(map.viewport_width.to_string(), Style::default().fg(Color::Yellow)),
                        // ]),
                        // Row::new(vec![
                        //     Span::styled("vh: ", Style::default().fg(Color::White)),
                        //     Span::styled(map.viewport_height.to_string(), Style::default().fg(Color::Yellow)),
                        // ]),
                        // Row::new(vec![
                        //     Span::styled("su: ", Style::default().fg(Color::White)),
                        //     Span::styled((map.viewport_y + (map.viewport_height / 7)).to_string(), Style::default().fg(Color::Yellow)),
                        // ]),
                        // Row::new(vec![
                        //     Span::styled("sd: ", Style::default().fg(Color::White)),
                        //     Span::styled(((map.viewport_height + map.viewport_y) - (map.viewport_height / 7)).to_string(), Style::default().fg(Color::Yellow)),
                        // ]),
                        // Row::new(vec![
                        //     Span::styled("sl: ", Style::default().fg(Color::White)),
                        //     Span::styled((map.viewport_x + (map.viewport_width / 7)).to_string(), Style::default().fg(Color::Yellow)),
                        // ]),
                        // Row::new(vec![
                        //     Span::styled("sr: ", Style::default().fg(Color::White)),
                        //     Span::styled(((map.viewport_width + map.viewport_x) - (map.viewport_width / 7)).to_string(), Style::default().fg(Color::Yellow)),
                        // ]),
                        // Row::new(vec![
                        //     Span::styled("gx: ", Style::default().fg(Color::White)),
                        //     Span::styled((map.gen_x).to_string(), Style::default().fg(Color::Yellow)),
                        // ]),
                        // Row::new(vec![
                        //     Span::styled("gy: ", Style::default().fg(Color::White)),
                        //     Span::styled((map.gen_y).to_string(), Style::default().fg(Color::Yellow)),
                        // ]),
                        // Row::new(vec![
                        //     Span::styled("tlen: ", Style::default().fg(Color::White)),
                        //     Span::styled((map.tunnels.len()).to_string(), Style::default().fg(Color::Yellow)),
                        // ]),
                        // Row::new(vec![
                        //     Span::styled("dtlen: ", Style::default().fg(Color::White)),
                        //     Span::styled((map.dead_tunnels.len()).to_string(), Style::default().fg(Color::Yellow)),
                        // ]),
                    ];
                    let stats = Table::new(rows, &[Constraint::Percentage(50), Constraint::Percentage(50)])
                                    .block(paragraph_block);
                    let table_data = vec![
                        vec!["", "", ""],
                    ];
                    let rows: Vec<Row> = table_data.iter().enumerate().map(|(j, row)| {
                        let cells: Vec<Cell> = row.iter().enumerate().map(|(i, &cell)| {
                            if i == self.cursor_pos.0 && j == self.cursor_pos.1 {
                                Cell::from(Span::styled(cell, ratatui::style::Style::default().fg(ratatui::style::Color::Yellow)))
                            } else {
                                Cell::from(cell)
                            }
                        }).collect();
                        Row::new(cells)
                    }).collect();
                    let table = Table::new(rows, &[Constraint::Percentage(33), Constraint::Percentage(33), Constraint::Percentage(33)])
                        .block(table_block);
                    f.render_widget(stats, normal_info[0]);
                    f.render_widget(table, normal_info[1]);
                },
                GUIMode::Interact => {
                    // match inter_step {
                    //     InterSteps::AdjOpt => {
                    //         let normal_info = Layout::default()
                    //         .direction(Direction::Vertical)
                    //         .constraints(
                    //             [
                    //                 Constraint::Percentage(70),
                    //                 Constraint::Percentage(30)
                    //             ].as_ref()
                    //         )
                    //         .split(game_chunks[1]);
                    //         let paragraph_block = Block::default()
                    //             .title("Paragraph Block")
                    //             .borders(Borders::ALL)
                    //             .style(Style::default().bg(Color::Black));
                    //         let table_block = Block::default()
                    //             .title("Table Block")
                    //             .borders(Borders::ALL)
                    //             .style(Style::default().bg(Color::Black));
                    //         let paragraph = Paragraph::new(Span::raw("What would you like to interct with?"))
                    //             .block(paragraph_block);
                    //         let mut adj_list = vec![];
                    //         let mut vec1 = vec![((0 as usize, 0 as usize), "".to_string()); 2];
                    //         let mut vec2 = vec![((0 as usize, 0 as usize), "".to_string()); 2];
                    //         for (pos, interable) in &self.interactable {
                    //             let Some(inter) = interable else {todo!()};
                    //             match inter {
                    //                 Interactable::Item(item) => adj_list.push((*pos, item.clone().get_sname())),
                    //                 Interactable::Enemy(enemy) => adj_list.push((*pos, enemy.clone().get_sname())),
                    //                 Interactable::Item(npc) => adj_list.push((*pos, npc.clone().get_sname())),
                    //             _ => todo!(),
                    //             }
                    //         }
                    //         for (idx, entity) in adj_list.iter().enumerate() {
                    //             if idx < 2 {
                    //                 vec1[idx] = entity.clone();
                    //             } else {
                    //                 vec2[idx - 2] = entity.clone();
                    //             }
                    //         }
                    //         let inter_entities = vec![vec1.clone(), vec2.clone()];
                    //         self.adj_options = (vec1, vec2);
                    //         let rows: Vec<Row> = inter_entities.iter().enumerate().map(|(j, row)| {
                    //             let cells: Vec<Cell> = row.iter().enumerate().map(|(i, &ref cell)| {
                    //                 if i == self.cursor_pos.0 && j == self.cursor_pos.1 {
                    //                     Cell::from(Span::styled(cell.clone().1, ratatui::style::Style::default().fg(ratatui::style::Color::Yellow)))
                    //                 } else {
                    //                     Cell::from(cell.clone().1)
                    //                 }
                    //             }).collect();
                    //             Row::new(cells)
                    //         }).collect();
                    //         let table = Table::new(rows, &[Constraint::Percentage(50), Constraint::Percentage(50)])
                    //             .block(table_block);
                    //         f.render_widget(paragraph, normal_info[0]);
                    //         f.render_widget(table, normal_info[1]);
                    //     },
                    //     InterSteps::IntOpt => {
                    //         let normal_info = Layout::default()
                    //             .direction(Direction::Vertical)
                    //             .constraints(
                    //             [
                    //                 Constraint::Percentage(70),
                    //                 Constraint::Percentage(30)
                    //             ].as_ref()
                    //         )
                    //         .split(game_chunks[1]);
                    //         let paragraph_block = Block::default()
                    //             .title("Paragraph Block")
                    //             .borders(Borders::ALL)
                    //             .style(Style::default().bg(Color::Black));
                    //         let table_block = Block::default()
                    //             .title("Table Block")
                    //             .borders(Borders::ALL)
                    //             .style(Style::default().bg(Color::Black));
                    //         let paragraph = Paragraph::new(Span::raw("What would you like to interct with?"))
                    //             .block(paragraph_block);
                    //         let mut adj_list = vec![];
                    //         let mut vec1 = vec![((0 as usize, 0 as usize), "".to_string()); 2];
                    //         let mut vec2 = vec![((0 as usize, 0 as usize), "".to_string()); 2];
                    //         for (pos, interable) in &self.interactable {
                    //             let Some(inter) = interable else {todo!()};
                    //             match inter {
                    //                 Interactable::Item(item) => adj_list.push((*pos, item.clone().get_sname())),
                    //                 Interactable::Enemy(enemy) => adj_list.push((*pos, enemy.clone().get_sname())),
                    //                 Interactable::Item(npc) => adj_list.push((*pos, npc.clone().get_sname())),
                    //             _ => todo!(),
                    //             }
                    //         }
                    //         for (idx, entity) in adj_list.iter().enumerate() {
                    //             if idx < 2 {
                    //                 vec1[idx] = entity.clone();
                    //             } else {
                    //                 vec2[idx - 2] = entity.clone();
                    //             }
                    //         }
                    //         let inter_entities = vec![vec1.clone(), vec2.clone()];
                    //         self.adj_options = (vec1, vec2);
                    //         let rows: Vec<Row> = inter_entities.iter().enumerate().map(|(j, row)| {
                    //             let cells: Vec<Cell> = row.iter().enumerate().map(|(i, &ref cell)| {
                    //                 if i == self.cursor_pos.0 && j == self.cursor_pos.1 {
                    //                     Cell::from(Span::styled(cell.clone().1, ratatui::style::Style::default().fg(ratatui::style::Color::Yellow)))
                    //                 } else {
                    //                     Cell::from(cell.clone().1)
                    //                 }
                    //             }).collect();
                    //             Row::new(cells)
                    //         }).collect();
                    //         let table = Table::new(rows, &[Constraint::Percentage(50), Constraint::Percentage(50)])
                    //             .block(table_block);
                    //         f.render_widget(paragraph, normal_info[0]);
                    //         f.render_widget(table, normal_info[1]);
                    //     },
                    //     _ => todo!(),
                    // }

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
                        .title("Map")
                        .borders(Borders::ALL)
                        .style(Style::default().bg(Color::Black));
                    let table_block = Block::default()
                        .title("Map Type")
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

                    log::info!("dist_fo: {:?}", self.dist_fo);

                    match self.dist_fo {
                        ( u, d, l, r) if l > r && l.abs() >= u.abs() && l.abs() >= d.abs() => {
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
                        (u, d, l, r) if r > l && r.abs() >= u.abs() && r.abs() >= d.abs() => {
                            for y in 0..height {
                                for x in 0..width {
                                    if y <= cen_y && y as f32 >= slope * x as f32 {
                                        content.push('#');
                                    } else if y > cen_y && y as f32 <= slope * (width - x)  as f32{
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
                        (u, d, l, r) if u > d && u.abs() >= l.abs() && u.abs() >= r.abs() => {
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
                        (u, d, l, r) if d > u && d.abs() >= l.abs() && d.abs() >= r.abs() => {
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
                        (0, 0, 0, 0) => {
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

                    let mut vec1 = vec!["compass"];
                    let mut vec2 = vec!["", ""];

                    let inv_table = vec![vec1.clone(), vec2.clone()];
                    let rows: Vec<Row> = inv_table.iter().enumerate().map(|(j, row)| {
                        let cells: Vec<Cell> = row.iter().enumerate().map(|(i, cell)| {
                            if i == self.cursor_pos.0 && j == self.cursor_pos.1 {
                                Cell::from(Span::styled(*cell, ratatui::style::Style::default().fg(ratatui::style::Color::Yellow)))
                            } else {
                                Cell::from(*cell)
                            }
                        }).collect();
                        Row::new(cells)
                    }).collect();
                    let table = Table::new(rows, &[Constraint::Percentage(50), Constraint::Percentage(50)])
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
                        .title("Inventory")
                        .borders(Borders::ALL)
                        .style(Style::default().bg(Color::Black));
                    let table_block = Block::default()
                        .title("Items")
                        .borders(Borders::ALL)
                        .style(Style::default().bg(Color::Black));

                    let prop = HashMap::new();
                    let itype = String::new();
                    let desc = String::new();
                    let iopts = HashMap::new();
                    let i_temp = Item::new(Items::Null, itype, desc, iopts, 0, 0, prop);
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
                    let rows: Vec<Row> = (0..12).map(|i| {
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
                        let fmt_prop = format!("{}: {}", s, i.to_string());
                        props.push(Line::from(Span::raw(fmt_prop)));
                    }


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
                            Constraint::Percentage(20),
                            Constraint::Percentage(80)
                        ].as_ref()
                    )
                    .split(game_chunks[1]);
                    let notes_block = Block::default()
                        .title("Notes")
                        .borders(Borders::ALL)
                        .style(Style::default().bg(Color::Black));
                    let note_block = Block::default()
                        .title("")
                        .borders(Borders::ALL)
                        .style(Style::default().bg(Color::Black));

                    let mut vec1 = vec!["Quests".to_string(), "Places".to_string(), "People".to_string(), "Lore".to_string()];
                    let mut vec2 = vec!["".to_string(), "".to_string(), "".to_string(), "".to_string()];

                    let inv_table: Vec<Vec<(String)>, > = vec![vec1.clone(), vec2.clone()];
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
                                if i == self.cursor_pos.0 && j == self.cursor_pos.1 {
                                    Cell::from(Span::styled(cell.as_str(), ratatui::style::Style::default().fg(ratatui::style::Color::Yellow)))
                                } else {
                                    Cell::from(cell.as_str())
                                }
                            }

                        }).collect();
                        Row::new(cells)
                    }).collect();
                    let table = Table::new(rows, &[Constraint::Percentage(25), Constraint::Percentage(25), Constraint::Percentage(25), Constraint::Percentage(25)])
                        .block(notes_block);

                    let c_hold = &self.cursor_hold.0;

                    let paragraph = if self.menu_lvl > 0 {
                        match c_hold {
                            0 => {
                                let qsts = &self.active_notes.0;
                                let qst = &qsts[self.cursor_pos.1];
                                let Some(stage) = ({
                                    let mut a_s = None;
                                    for (_, s) in &qst.stages {
                                        if s.active {
                                            a_s = Some(s);
                                        }
                                    }
                                    a_s
                                }) else {todo!()};
                                let text = Text::from(vec![
                                    Line::from(Span::raw(qst.name.clone())),
                                    Line::from(Span::raw(stage.text.clone())),
                                ]);
                                let paragraph = Paragraph::new(text)
                                    .block(note_block)
                                    .wrap(ratatui::widgets::Wrap { trim: true });
                                paragraph
                            },
                            1 => {
                                let plcs = &self.active_notes.1;
                                let plc = &plcs[self.cursor_pos.1];
                                let text = Text::from(vec![
                                    Line::from(Span::raw(plc.name.clone())),
                                    Line::from(Span::raw(plc.text.clone())),
                                ]);
                                let paragraph = Paragraph::new(text)
                                    .block(note_block)
                                    .wrap(ratatui::widgets::Wrap { trim: true });
                                paragraph
                            },
                            2 => {
                                let ppl = &self.active_notes.2;
                                let prsn = &ppl[self.cursor_pos.1];
                                let text = Text::from(vec![
                                    Line::from(Span::raw(prsn.name.clone())),
                                    Line::from(Span::raw(prsn.desc.clone())),
                                ]);
                                let paragraph = Paragraph::new(text)
                                    .block(note_block)
                                    .wrap(ratatui::widgets::Wrap { trim: true });
                                paragraph
                            },
                            3 => {
                                let lore = &self.active_notes.3;
                                let lre = &lore[self.cursor_pos.1];
                                let text = Text::from(vec![
                                    Line::from(Span::raw(lre.name.clone())),
                                    Line::from(Span::raw(lre.desc.clone())),
                                ]);
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
                        let paragraph = Paragraph::new(Span::raw("Notes for the user."))
                        .block(note_block);
                        paragraph
                    };


                    f.render_widget(table, normal_info[0]);
                    f.render_widget(paragraph, normal_info[1]);
                },
                // GUIMode::NPC => {},
                // GUIMode::Fight => {},
                _ => todo!(),
            }

        }).unwrap();
    }
}
