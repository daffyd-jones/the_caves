use crate::enums::{Items, NPCWrap, Interactable, InterOpt, EncOpt, Equip, ItemEffect, EnvInter, Plants};
use crate::map::Map;
use crate::player::Player;
use crate::enemy::{Enemy};
use crate::npc::{NPC};
use crate::item::Item;
//use crate::notebook::{Quest, Stage, Place, Person, Lore};
use crate::gui::{GUI, draw_map, GuiArgs};
// use crate::gui::draw_map;
use std::collections::HashMap;

// use std::time::Duration;
// use rand::Rng;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::layout::{Layout, Constraint, Direction, Margin};
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::{Text, Span};
use ratatui::widgets::Row;
use ratatui::widgets::Table;
use ratatui::widgets::Cell;
use ratatui::text::Line;

fn wrap_text(text: &str, max_width: usize) -> Text {
    let mut lines = Vec::new();
    let mut current_line = String::new();
    for word in text.split_whitespace() {
        if current_line.len() + word.len() + 1 > max_width {
            //lines.push(current_line);
            lines.push(Line::from(current_line.clone()));
            current_line.clear();
        }
        if !current_line.is_empty() {
            current_line.push(' ');
        }
        current_line.push_str(word);
    }
    if !current_line.is_empty() {
        //lines.push(current_line);
        lines.push(Line::from(current_line));
    }
    //lines
    Text::from(lines)
}

impl GUI {

    //ineract start--------
    // pub fn inter_adj_draw(&mut self, mut map: Map, player: Player, portals: HashMap<(usize, usize), (usize, usize)>, enemies: HashMap<(usize, usize), Enemy>, items: HashMap<(usize, usize), Item>, npcs: HashMap<(usize, usize), NPCWrap>, litems: HashMap<(usize, usize), Item>, env_inter: HashMap<(usize, usize), EnvInter>) {
    pub fn inter_adj_draw(&mut self, gui_args: &mut GuiArgs) {
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
            if in_h != self.viewport_dim.1 && in_w != self.viewport_dim.0 {
                // map.set_viewport(in_h, in_w);
                self.viewport_dim = (in_w, in_h);
            }
            let paragraph = draw_map(gui_args, self.ani_cnt);
            f.render_widget(paragraph, inner_area);


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
                .title("Paragraph Block")
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Black));
            let table_block = Block::default()
                .title("Table Block")
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Black));
            let paragraph = Paragraph::new(Span::styled("What would you like to interct with?", Style::default().white()))
                .block(paragraph_block)
                .wrap(ratatui::widgets::Wrap { trim: true });
            let mut adj_list = vec![];
            let mut vec1 = vec![((0_usize, 0_usize), "".to_string()); 3];
            let mut vec2 = vec![((0_usize, 0_usize), "".to_string()); 3];
            for (pos, interable) in &self.interactable {
                let Some(inter) = interable else {todo!()};
                match inter {
                    Interactable::Item(item) => adj_list.push((*pos, item.clone().get_sname())),
                    Interactable::ShopItem(item) => adj_list.push((*pos, item.clone().get_sname())),
                    Interactable::Enemy(enemy) => adj_list.push((*pos, enemy.clone().get_sname())),
                    Interactable::NPC(npc) => {
                        match npc {
                            NPCWrap::CommNPC(comm_npc) => adj_list.push((*pos, comm_npc.clone().get_sname())),
                            NPCWrap::ConvNPC(conv_npc) => adj_list.push((*pos, conv_npc.clone().get_sname())),
                            NPCWrap::ShopNPC(shop_npc) => adj_list.push((*pos, shop_npc.clone().get_sname())),
                            NPCWrap::SpawnNPC(spawn_npc) => adj_list.push((*pos, spawn_npc.clone().get_sname())),
                            NPCWrap::TradeNPC(trade_npc) => adj_list.push((*pos, trade_npc.clone().get_sname())),
                           _ => todo!(),
                        }
                        // adj_list.push((*pos, npc.clone().get_sname()));
                    },
                    Interactable::EnvInter(env_inter) => {
                        match env_inter {
                            EnvInter::Records => adj_list.push((*pos, "Local Records".to_string())),
                            EnvInter::Clinic => adj_list.push((*pos, "Clinic".to_string())),
                            EnvInter::GuildPost => adj_list.push((*pos, "Guild Posting".to_string())),
                            EnvInter::ChurchPost => adj_list.push((*pos, "Church Posting".to_string())),
                            EnvInter::Cauldron => adj_list.push((*pos, "Cauldron".to_string())),
                            _ => todo!(),
                        }
                    }
                    _ => todo!(),
                }
            }
            for (idx, entity) in adj_list.iter().enumerate() {
                if idx < 3 {
                    vec1[idx] = entity.clone();
                } else {
                    vec2[idx - 3] = entity.clone();
                }
            }
            let inter_entities = [vec1.clone(), vec2.clone()];
            self.adj_options = (vec1, vec2);
            let rows: Vec<Row> = inter_entities.iter().enumerate().map(|(j, row)| {
                let cells: Vec<Cell> = row.iter().enumerate().map(|(i, cell)| {
                    if i == self.cursor_pos.0 && j == self.cursor_pos.1 {
                        Cell::from(Span::styled(cell.clone().1, ratatui::style::Style::default().fg(ratatui::style::Color::Yellow)))
                    } else {
                        Cell::from(cell.clone().1)
                    }
                }).collect();
                Row::new(cells)
            }).collect();
            let table = Table::new(rows, &[Constraint::Percentage(50), Constraint::Percentage(50)])
                .block(table_block);
            f.render_widget(paragraph, normal_info[0]);
            f.render_widget(table, normal_info[1]);
        }).unwrap();
    }

    pub fn inter_opt_draw(&mut self, gui_args: &mut GuiArgs) {
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

            if in_h != self.viewport_dim.1 && in_w != self.viewport_dim.0 {
                // map.set_viewport(in_h, in_w);
                self.viewport_dim = (in_w, in_h);
            }
            let paragraph = draw_map(gui_args, self.ani_cnt);
            f.render_widget(paragraph, inner_area);


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
                .title("Item")
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Black));
            let table_block = Block::default()
                .title("Options")
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Black));
            let paragraph = Paragraph::new(Span::styled("What would you like to do with it?", Style::default().white()))
                .block(paragraph_block)
                .wrap(ratatui::widgets::Wrap { trim: true });
            let mut vec1 = vec![(InterOpt::Null, "".to_string()); 3];
            let mut vec2 = vec![(InterOpt::Null, "".to_string()); 3];
            let opts = self.inter_opt.clone();
            let mut opts_kys: Vec<_> = opts.keys().collect();
            opts_kys.sort();
            for (idx, a) in opts_kys.iter().enumerate() {
                if idx < 3 {
                    vec1[idx] = (**a, opts[a].clone());
                } else {
                    vec2[idx - 3] = (**a, opts[a].clone());
                }
            }
            let inter_opts = [vec1.clone(), vec2.clone()];
            self.inter_options = (vec1, vec2);
            let rows: Vec<Row> = inter_opts.iter().enumerate().map(|(j, row)| {
                let cells: Vec<Cell> = row.iter().enumerate().map(|(i, cell)| {
                    if i == self.cursor_pos.0 && j == self.cursor_pos.1 {
                        Cell::from(Span::styled(cell.clone().1, ratatui::style::Style::default().fg(ratatui::style::Color::Yellow)))
                    } else {
                        Cell::from(cell.clone().1)
                    }
                }).collect();
                Row::new(cells)
            }).collect();
            let table = Table::new(rows, &[Constraint::Percentage(50), Constraint::Percentage(50), Constraint::Percentage(50)])
                .block(table_block);
            f.render_widget(paragraph, normal_info[0]);
            f.render_widget(table, normal_info[1]);
        }).unwrap();
    }

    pub fn inter_res_draw(&mut self, gui_args: &mut GuiArgs) {
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

            if in_h != self.viewport_dim.1 && in_w != self.viewport_dim.0 {
                // map.set_viewport(in_h, in_w);
                self.viewport_dim = (in_w, in_h);
            }
            let paragraph = draw_map(gui_args, self.ani_cnt);
            // let paragraph = draw_map(map.clone(), player.clone(), portals.clone(), enemies.clone(), items.clone(), npcs.clone(), litems.clone(), env_inter.clone(), self.ani_cnt);
            f.render_widget(paragraph, inner_area);


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
                .title("Paragraph Block")
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Black));
            let table_block = Block::default()
                .title("Table Block")
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Black));
            let paragraph = Paragraph::new(Span::styled("Done", Style::default().white())).block(paragraph_block);
            let vec1 = vec!["Ok".to_string(); 1];
            let vec2 = vec!["".to_string(); 1];

            let inter_opts = [vec1.clone(), vec2.clone()];
            let rows: Vec<Row> = inter_opts.iter().enumerate().map(|(j, row)| {
                let cells: Vec<Cell> = row.iter().enumerate().map(|(i, cell)| {
                    if i == self.cursor_pos.0 && j == self.cursor_pos.1 {
                        Cell::from(Span::styled(cell.clone(), ratatui::style::Style::default().fg(ratatui::style::Color::Yellow)))
                    } else {
                        Cell::from(cell.clone())
                    }
                }).collect();
                Row::new(cells)
            }).collect();
            let table = Table::new(rows, &[Constraint::Percentage(50), Constraint::Percentage(50)])
                .block(table_block);
            f.render_widget(paragraph, normal_info[0]);
            f.render_widget(table, normal_info[1]);
        }).unwrap();
    }

    //interact end-----


    //item_used-----
    pub fn item_use_draw(&mut self, msg_str: String, iopts: String, gui_args: &mut GuiArgs) {
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

            if in_h != self.viewport_dim.1 && in_w != self.viewport_dim.0 {
                // map.set_viewport(in_h, in_w);
                self.viewport_dim = (in_w, in_h);
            }
            let paragraph = draw_map(gui_args, self.ani_cnt);
            // let paragraph = draw_map(map.clone(), player.clone(), portals.clone(), enemies.clone(), items.clone(), npcs.clone(), litems.clone(), env_inter.clone(), self.ani_cnt);
            f.render_widget(paragraph, inner_area);


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
                .title("Inventory")
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Black));
            let table_block = Block::default()
                .title("")
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Black));
            let paragraph = Paragraph::new(Span::styled(&msg_str, Style::default().white())).block(paragraph_block);
            let vec1: Vec<&str> = iopts.split("#").collect();
            let vec2 = vec![""; 1];

            let inter_opts = [vec1.clone(), vec2.clone()];
            let rows: Vec<Row> = inter_opts.iter().enumerate().map(|(j, row)| {
                let cells: Vec<Cell> = row.iter().enumerate().map(|(i, cell)| {
                    if i == self.cursor_pos.0 && j == self.cursor_pos.1 {
                        Cell::from(Span::styled(cell.to_string(), ratatui::style::Style::default().fg(ratatui::style::Color::Yellow)))
                    } else {
                        Cell::from(cell.to_string())
                    }
                }).collect();
                Row::new(cells)
            }).collect();
            let table = Table::new(rows, &[Constraint::Percentage(33), Constraint::Percentage(33), Constraint::Percentage(33)])
                .block(table_block);
            f.render_widget(paragraph, normal_info[0]);
            f.render_widget(table, normal_info[1]);
        }).unwrap();
    }

    pub fn item_used_draw(&mut self, gui_args: &mut GuiArgs) {
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

            if in_h != self.viewport_dim.1 && in_w != self.viewport_dim.0 {
                // map.set_viewport(in_h, in_w);
                self.viewport_dim = (in_w, in_h);
            }
            let paragraph = draw_map(gui_args, self.ani_cnt);
            // let paragraph = draw_map(map.clone(), player.clone(), portals.clone(), enemies.clone(), items.clone(), npcs.clone(), litems.clone(), env_inter.clone(), self.ani_cnt);
            f.render_widget(paragraph, inner_area);


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
                .title("Inventory")
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Black));
            let table_block = Block::default()
                .title("")
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Black));
            let paragraph = Paragraph::new(Span::styled("Item used", Style::default().white())).block(paragraph_block);
            let vec1 = vec!["Ok".to_string(); 1];
            let vec2 = vec!["".to_string(); 1];

            let inter_opts = [vec1.clone(), vec2.clone()];
            let rows: Vec<Row> = inter_opts.iter().enumerate().map(|(j, row)| {
                let cells: Vec<Cell> = row.iter().enumerate().map(|(i, cell)| {
                    if i == self.cursor_pos.0 && j == self.cursor_pos.1 {
                        Cell::from(Span::styled(cell.clone(), ratatui::style::Style::default().fg(ratatui::style::Color::Yellow)))
                    } else {
                        Cell::from(cell.clone())
                    }
                }).collect();
                Row::new(cells)
            }).collect();
            let table = Table::new(rows, &[Constraint::Percentage(50), Constraint::Percentage(50)])
                .block(table_block);
            f.render_widget(paragraph, normal_info[0]);
            f.render_widget(table, normal_info[1]);
        }).unwrap();
    }

    //item used----

    //encounter start----

    pub fn encounter_show_content(&mut self, cntnt: String, gui_args: &mut GuiArgs) {
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
            .split(f.area());

            let game_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(60),
                    Constraint::Percentage(40)
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
                // map.set_viewport(in_h, in_w);
                self.viewport_dim = (in_w, in_h);
            }

            let paragraph = draw_map(gui_args, self.ani_cnt);
            // let paragraph = draw_map(map.clone(), player.clone(), portals.clone(), enemies.clone(), items.clone(), npcs.clone(), litems.clone(), env_inter.clone(), self.ani_cnt);
            f.render_widget(paragraph, inner_area);


            //alksdjlfkj---


            let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(50),
                    Constraint::Percentage(50)
                ].as_ref()
            )
            .split(game_chunks[1]);

            let left_chunk = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage(50),
                    Constraint::Percentage(50)
                ].as_ref()
            )
            .split(chunks[0]);

            let right_chunk = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage(50),
                    Constraint::Percentage(50)
                ].as_ref()
            )
            .split(chunks[1]);



//------
            let enc_text_block = Block::default()
                .title("Encounter")
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Black));
            let paragraph = Paragraph::new(Span::styled(&cntnt, Style::default().white()))
                .block(enc_text_block)
                .wrap(ratatui::widgets::Wrap { trim: true });

            f.render_widget(paragraph, left_chunk[0]);



//------



            let options_block = Block::default()
                .title("")
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Black));

            let vec1 = vec!["Ok".to_string(); 1];
            let vec2 = vec!["".to_string(); 1];

            let inter_opts = [vec1.clone(), vec2.clone()];
            let rows: Vec<Row> = inter_opts.iter().enumerate().map(|(j, row)| {
                let cells: Vec<Cell> = row.iter().enumerate().map(|(i, cell)| {
                    if i == self.cursor_pos.0 && j == self.cursor_pos.1 {
                        Cell::from(Span::styled(cell.clone(), ratatui::style::Style::default().fg(ratatui::style::Color::Yellow)))
                    } else {
                        Cell::from(cell.clone())
                    }
                }).collect();
                Row::new(cells)
            }).collect();
            let table = Table::new(rows, &[Constraint::Percentage(50), Constraint::Percentage(50)])
                .block(options_block);

            f.render_widget(table, left_chunk[1]);



//-----



            let entity_block = Block::default()
                .title("")
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Black));
            let paragraph = Paragraph::new(Span::raw("entity design"))
                .block(entity_block);
            f.render_widget(paragraph, right_chunk[0]);




//-----


            let stats_block = Block::default()
                .title("")
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Black));

            let rows = vec![
                Row::new(vec![
                    Span::styled("Health: ", Style::default().fg(Color::White)),
                    Span::styled(gui_args.player.health.to_string(), Style::default().fg(Color::Yellow)),
                ]),
                Row::new(vec![
                    Span::styled("Attack: ", Style::default().fg(Color::White)),
                    Span::styled(gui_args.player.attack.to_string(), Style::default().fg(Color::Yellow)),
                ]),
                Row::new(vec![
                    Span::styled("Defence: ", Style::default().fg(Color::White)),
                    Span::styled(gui_args.player.defence.to_string(), Style::default().fg(Color::Yellow)),
                ]),
                Row::new(vec![
                    Span::styled("Damage: ", Style::default().fg(Color::White)),
                    Span::styled(gui_args.player.damage.to_string(), Style::default().fg(Color::Yellow)),
                ]),
            ];
            let table = Table::new(rows, &[Constraint::Percentage(50), Constraint::Percentage(50)])
                .block(stats_block);

            // let paragraph = Paragraph::new(Span::raw("Item used"))
            //     .block(stats_block);
            f.render_widget(table, right_chunk[1]);
        }).unwrap();
    }

    pub fn encounter_auto_content(&mut self, gui_args: &mut GuiArgs) {
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
            .split(f.area());

            let game_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(60),
                    Constraint::Percentage(40)
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
                // map.set_viewport(in_h, in_w);
                self.viewport_dim = (in_w, in_h);
            }

            let paragraph = draw_map(gui_args, self.ani_cnt);
            // let paragraph = draw_map(map.clone(), player.clone(), portals.clone(), enemies.clone(), items.clone(), npcs.clone(), litems.clone(), env_inter.clone(), self.ani_cnt);
            f.render_widget(paragraph, inner_area);


            //alksdjlfkj---


            let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(50),
                    Constraint::Percentage(50)
                ].as_ref()
            )
            .split(game_chunks[1]);

            let left_chunk = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage(50),
                    Constraint::Percentage(50)
                ].as_ref()
            )
            .split(chunks[0]);

            let right_chunk = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage(50),
                    Constraint::Percentage(50)
                ].as_ref()
            )
            .split(chunks[1]);



//------
            let enc_text_block = Block::default()
                .title("Encounter")
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Black));
            let paragraph = Paragraph::new(Span::styled("AAAAA", Style::default().white()))
                .block(enc_text_block)
                .wrap(ratatui::widgets::Wrap { trim: true });

            f.render_widget(paragraph, left_chunk[0]);



//------



            let options_block = Block::default()
                .title("")
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Black));

            let vec1 = vec!["Ok".to_string(); 1];
            let vec2 = vec!["".to_string(); 1];

            let inter_opts = [vec1.clone(), vec2.clone()];
            let rows: Vec<Row> = inter_opts.iter().enumerate().map(|(j, row)| {
                let cells: Vec<Cell> = row.iter().enumerate().map(|(i, cell)| {
                    if i == self.cursor_pos.0 && j == self.cursor_pos.1 {
                        Cell::from(Span::styled(cell.clone(), ratatui::style::Style::default().fg(ratatui::style::Color::Yellow)))
                    } else {
                        Cell::from(cell.clone())
                    }
                }).collect();
                Row::new(cells)
            }).collect();
            let table = Table::new(rows, &[Constraint::Percentage(50), Constraint::Percentage(50)])
                .block(options_block);

            f.render_widget(table, left_chunk[1]);



//-----



            let entity_block = Block::default()
                .title("")
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Black));
            let paragraph = Paragraph::new(Span::raw("entity design"))
                .block(entity_block);
            f.render_widget(paragraph, right_chunk[0]);




//-----


            let stats_block = Block::default()
                .title("")
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Black));

            let rows = vec![
                Row::new(vec![
                    Span::styled("Health: ", Style::default().fg(Color::White)),
                    Span::styled(gui_args.player.health.to_string(), Style::default().fg(Color::Yellow)),
                ]),
                Row::new(vec![
                    Span::styled("Attack: ", Style::default().fg(Color::White)),
                    Span::styled(gui_args.player.attack.to_string(), Style::default().fg(Color::Yellow)),
                ]),
                Row::new(vec![
                    Span::styled("Defence: ", Style::default().fg(Color::White)),
                    Span::styled(gui_args.player.defence.to_string(), Style::default().fg(Color::Yellow)),
                ]),
                Row::new(vec![
                    Span::styled("Damage: ", Style::default().fg(Color::White)),
                    Span::styled(gui_args.player.damage.to_string(), Style::default().fg(Color::Yellow)),
                ]),
            ];
            let table = Table::new(rows, &[Constraint::Percentage(50), Constraint::Percentage(50)])
                .block(stats_block);

            // let paragraph = Paragraph::new(Span::raw("Item used"))
            //     .block(stats_block);
            f.render_widget(table, right_chunk[1]);
        }).unwrap();
    }

    pub fn encounter_user_options(&mut self, enc_opt: HashMap<EncOpt, String>, gui_args: &mut GuiArgs) {
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
            .split(f.area());

            let game_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(60),
                    Constraint::Percentage(40)
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
                // map.set_viewport(in_h, in_w);
                self.viewport_dim = (in_w, in_h);
            }
            let paragraph = draw_map(gui_args, self.ani_cnt);
            // let paragraph = draw_map(map.clone(), player.clone(), portals.clone(), enemies.clone(), items.clone(), npcs.clone(), litems.clone(), env_inter.clone(), self.ani_cnt);
            f.render_widget(paragraph, inner_area);


            let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(50),
                    Constraint::Percentage(50)
                ].as_ref()
            )
            .split(game_chunks[1]);

            let left_chunk = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage(50),
                    Constraint::Percentage(50)
                ].as_ref()
            )
            .split(chunks[0]);

            let right_chunk = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage(50),
                    Constraint::Percentage(50)
                ].as_ref()
            )
            .split(chunks[1]);



//------
            let enc_text_block = Block::default()
                .title("Encounter")
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Black));
            let paragraph = Paragraph::new(Span::styled("What would you like to do?", Style::default().white()))
                .block(enc_text_block);

            f.render_widget(paragraph, left_chunk[0]);



//------



            let options_block = Block::default()
                .title("")
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Black));

            let mut vec1 = vec![(EncOpt::Null, "".to_string()); 3];
            let mut vec2 = vec![(EncOpt::Null, "".to_string()); 3];
            let opts = enc_opt.clone();
            let mut opts_kys: Vec<_> = opts.keys().collect();
            opts_kys.sort();
            for (idx, a) in opts_kys.iter().enumerate() {
                if idx < 3 {
                    vec1[idx] = (**a, opts[a].clone());
                } else {
                    vec2[idx - 3] = (**a, opts[a].clone());
                }
            }
            let enc_opts = [vec1.clone(), vec2.clone()];
            self.enc_opt = (vec1, vec2);

            // let mut vec1 = vec!["Ok".to_string(); 1];
            // let mut vec2 = vec!["".to_string(); 1];

            // let enc_opts = vec![vec1.clone(), vec2.clone()];
            let rows: Vec<Row> = enc_opts.iter().enumerate().map(|(j, row)| {
                let cells: Vec<Cell> = row.iter().enumerate().map(|(i, cell)| {
                    if i == self.cursor_pos.0 && j == self.cursor_pos.1 {
                        Cell::from(Span::styled(cell.1.clone(), ratatui::style::Style::default().fg(ratatui::style::Color::Yellow)))
                    } else {
                        Cell::from(cell.1.clone())
                    }
                }).collect();
                Row::new(cells)
            }).collect();
            let table = Table::new(rows, &[Constraint::Percentage(33), Constraint::Percentage(33), Constraint::Percentage(33)])
                .block(options_block);

            f.render_widget(table, left_chunk[1]);



//-----



            let entity_block = Block::default()
                .title("")
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Black));
            let paragraph = Paragraph::new(Span::raw("Enemy thing."))
                .block(entity_block);
            f.render_widget(paragraph, right_chunk[0]);




//-----


            let stats_block = Block::default()
                .title("")
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Black));


            let rows = vec![
                Row::new(vec![
                    Span::styled("Health: ", Style::default().fg(Color::White)),
                    Span::styled(gui_args.player.health.to_string(), Style::default().fg(Color::Yellow)),
                ]),
                Row::new(vec![
                    Span::styled("Attack: ", Style::default().fg(Color::White)),
                    Span::styled(gui_args.player.attack.to_string(), Style::default().fg(Color::Yellow)),
                ]),
                Row::new(vec![
                    Span::styled("Defence: ", Style::default().fg(Color::White)),
                    Span::styled(gui_args.player.defence.to_string(), Style::default().fg(Color::Yellow)),
                ]),
                Row::new(vec![
                    Span::styled("Damage: ", Style::default().fg(Color::White)),
                    Span::styled(gui_args.player.damage.to_string(), Style::default().fg(Color::Yellow)),
                ]),
            ];
            let table = Table::new(rows, &[Constraint::Percentage(50), Constraint::Percentage(50)])
                .block(stats_block);

            // let paragraph = Paragraph::new(Span::raw("Item used"))
            //     .block(stats_block);
            f.render_widget(table, right_chunk[1]);
        }).unwrap();
    }

    pub fn encounter_pick_item(&mut self, gui_args: &mut GuiArgs) {
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
            .split(f.area());

            let game_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(60),
                    Constraint::Percentage(40)
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
                // map.set_viewport(in_h, in_w);
                self.viewport_dim = (in_w, in_h);
            }
            let paragraph = draw_map(gui_args, self.ani_cnt);
            // let paragraph = draw_map(map.clone(), player.clone(), portals.clone(), enemies.clone(), items.clone(), npcs.clone(), litems.clone(), env_inter.clone(), self.ani_cnt);
            f.render_widget(paragraph, inner_area);


            let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(50),
                    Constraint::Percentage(50)
                ].as_ref()
            )
            .split(game_chunks[1]);

            let left_chunk = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage(50),
                    Constraint::Percentage(50)
                ].as_ref()
            )
            .split(chunks[0]);

            let right_chunk = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage(50),
                    Constraint::Percentage(50)
                ].as_ref()
            )
            .split(chunks[1]);



//------
            let enc_text_block = Block::default()
                .title("Encounter")
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Black));
            let paragraph = Paragraph::new(Span::styled("What item to use?", Style::default().white()))
                .block(enc_text_block);

            f.render_widget(paragraph, left_chunk[0]);



//------



            let options_block = Block::default()
                .title("")
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Black));

            // let mut vec1 = vec![(EncOpt::Null, "".to_string()); 3];
            // let mut vec2 = vec![(EncOpt::Null, "".to_string()); 3];
            //
            // for (idx, (a, b)) in enc_opt.iter().enumerate() {
            //     if idx < 3 {
            //         vec1[idx] = (a.clone(), b.clone());
            //     } else {
            //         vec2[idx - 3] = (a.clone(), b.clone());
            //     }
            // }
            // let enc_opts = vec![vec1.clone(), vec2.clone()];
            // self.enc_opt = (vec1, vec2);
            //
            // // let mut vec1 = vec!["Ok".to_string(); 1];
            // // let mut vec2 = vec!["".to_string(); 1];
            //
            // // let enc_opts = vec![vec1.clone(), vec2.clone()];
            // let rows: Vec<Row> = enc_opts.iter().enumerate().map(|(j, row)| {
            //     let cells: Vec<Cell> = row.iter().enumerate().map(|(i, &ref cell)| {
            //         if i == self.cursor_pos.0 && j == self.cursor_pos.1 {
            //             Cell::from(Span::styled(cell.1.clone(), ratatui::style::Style::default().fg(ratatui::style::Color::Yellow)))
            //         } else {
            //             Cell::from(cell.1.clone())
            //         }
            //     }).collect();
            //     Row::new(cells)
            // }).collect();
            // let table = Table::new(rows, &[Constraint::Percentage(50), Constraint::Percentage(50)])
            //     .block(options_block);

            // let prop = HashMap::new();
            // let itype = String::new();
            // let desc = String::new();
            // let iopts = HashMap::new();
            let i_temp = Item::default();
            let mut col1 = vec![(0, i_temp.clone()); 25];
            let mut col2 = vec![(0, i_temp.clone()); 25];
            let mut col3 = vec![(0, i_temp.clone()); 25];
            //dd
            for (idx, item) in self.inventory.iter().enumerate() {
                if idx < 25 {
                    col1[idx] = (idx, item.clone());
                } else if idx < 50 {
                    col2[idx - 25] = (idx, item.clone());
                } else {
                    col3[idx - 50] = (idx, item.clone());
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
            //dd
            let table = Table::new(rows, &[Constraint::Percentage(33), Constraint::Percentage(33), Constraint::Percentage(33)])
                .block(options_block);

            f.render_widget(table, left_chunk[1]);



//-----



            let entity_block = Block::default()
                .title("")
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Black));
            let paragraph = Paragraph::new(Span::raw("Enemy thing."))
                .block(entity_block);
            f.render_widget(paragraph, right_chunk[0]);




//-----


            let stats_block = Block::default()
                .title("")
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Black));


            let rows = vec![
                Row::new(vec![
                    Span::styled("Health: ", Style::default().fg(Color::White)),
                    Span::styled(gui_args.player.health.to_string(), Style::default().fg(Color::Yellow)),
                ]),
                Row::new(vec![
                    Span::styled("Attack: ", Style::default().fg(Color::White)),
                    Span::styled(gui_args.player.attack.to_string(), Style::default().fg(Color::Yellow)),
                ]),
                Row::new(vec![
                    Span::styled("Defence: ", Style::default().fg(Color::White)),
                    Span::styled(gui_args.player.defence.to_string(), Style::default().fg(Color::Yellow)),
                ]),
                Row::new(vec![
                    Span::styled("Damage: ", Style::default().fg(Color::White)),
                    Span::styled(gui_args.player.damage.to_string(), Style::default().fg(Color::Yellow)),
                ]),
            ];
            let table = Table::new(rows, &[Constraint::Percentage(50), Constraint::Percentage(50)])
                .block(stats_block);
            f.render_widget(table, right_chunk[1]);
        }).unwrap();
    }


    pub fn npc_comm_draw(&mut self, comms: String, gui_args: &mut GuiArgs) {
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

            if in_h != self.viewport_dim.1 && in_w != self.viewport_dim.0 {
                // map.set_viewport(in_h, in_w);
                self.viewport_dim = (in_w, in_h);
            }
            let paragraph = draw_map(gui_args, self.ani_cnt);
            // let paragraph = draw_map(map.clone(), player.clone(), portals.clone(), enemies.clone(), items.clone(), npcs.clone(), litems.clone(), env_inter.clone(), self.ani_cnt);
            f.render_widget(paragraph, inner_area);


            let normal_info = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage(70),
                    Constraint::Percentage(30)
                ].as_ref()
            )
            .split(game_chunks[1]);

            let npc_str: Vec<&str> = comms.split("#").collect();

            let name = npc_str[0];

            let paragraph_block = Block::default()
                .title(name)
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Black));
            let table_block = Block::default()
                .title("")
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Black));


            let comm = npc_str[1];
            let npc = Paragraph::new(Span::styled(comm, Style::default().white()))
                .block(paragraph_block)
                .wrap(ratatui::widgets::Wrap { trim: true });
            let plyr = Paragraph::new(Span::raw(""))
                .block(table_block);
            f.render_widget(npc, normal_info[0]);
            f.render_widget(plyr, normal_info[1]);
        }).unwrap();
    }

    pub fn npc_trade_type_draw(&mut self, comms: String, gui_args: &mut GuiArgs) {
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

            if in_h != self.viewport_dim.1 && in_w != self.viewport_dim.0 {
                // map.set_viewport(in_h, in_w);
                self.viewport_dim = (in_w, in_h);
            }
            let paragraph = draw_map(gui_args, self.ani_cnt);
            // let paragraph = draw_map(map.clone(), player.clone(), portals.clone(), enemies.clone(), items.clone(), npcs.clone(), litems.clone(), env_inter.clone(), self.ani_cnt);
            f.render_widget(paragraph, inner_area);


            let normal_info = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage(70),
                    Constraint::Percentage(30)
                ].as_ref()
            )
            .split(game_chunks[1]);

            let npc_str: Vec<&str> = comms.split("#").collect();

            let name = npc_str[0];

            let paragraph_block = Block::default()
                .title(name)
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Black));
            let table_block = Block::default()
                .title("")
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Black));


            let vec1 = vec!["Buy", "Sell", "Leave"];
            let opts = [vec1.clone()];
            let rows: Vec<Row> = opts.iter().enumerate().map(|(j, row)| {
                let cells: Vec<Cell> = row.iter().enumerate().map(|(i, cell)| {
                    if i == self.cursor_pos.0 && j == self.cursor_pos.1 {
                        Cell::from(Span::styled(cell.clone(), ratatui::style::Style::default().fg(ratatui::style::Color::Yellow)))
                    } else {
                        Cell::from(cell.clone())
                    }
                }).collect();
                Row::new(cells)
            }).collect();
            let table = Table::new(rows, &[Constraint::Percentage(33), Constraint::Percentage(33), Constraint::Percentage(33)])
                .block(table_block);

            let comm = npc_str[1];
            let npc = Paragraph::new(Span::styled(comm, Style::default().white()))
                .block(paragraph_block)
                .wrap(ratatui::widgets::Wrap { trim: true });
            //let plyr = Paragraph::new(Span::raw(""))
              //  .block(table_block);
            f.render_widget(npc, normal_info[0]);
            f.render_widget(table, normal_info[1]);
        }).unwrap();
    }

    pub fn npc_trade_draw(&mut self, titems: Vec<Item>, gui_args: &mut GuiArgs) {
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

            if in_h != self.viewport_dim.1 && in_w != self.viewport_dim.0 {
                // map.set_viewport(in_h, in_w);
                self.viewport_dim = (in_w, in_h);
            }
            let paragraph = draw_map(gui_args, self.ani_cnt);
            // let paragraph = draw_map(map.clone(), player.clone(), portals.clone(), enemies.clone(), items.clone(), npcs.clone(), litems.clone(), env_inter.clone(), self.ani_cnt);
            f.render_widget(paragraph, inner_area);


            let normal_info = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage(30),
                    Constraint::Percentage(70)
                ].as_ref()
            )
            .split(game_chunks[1]);

            let norm_top = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(70),
                    Constraint::Percentage(30)
                ].as_ref()
            )
            .split(normal_info[0]);
            
        
            let desc_block = Block::default()
                .title(Span::styled("Inventory", Style::default().fg(Color::DarkGray)))
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Black));

            let info_block = Block::default()
                .title(Span::styled("", Style::default().fg(Color::DarkGray)))
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Black));

            let table_block = Block::default()
                .title(Span::styled("Items", Style::default().fg(Color::DarkGray)))
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Black));
            let i_temp = Item::default();
            let mut col1 = vec![(0, i_temp.clone()); 25];
            let mut col2 = vec![(0, i_temp.clone()); 25];
            let mut col3 = vec![(0, i_temp.clone()); 25];
            //for (idx, item) in self.inventory.iter().enumerate() {
            for (idx, item) in titems.iter().enumerate() {
                if idx < 25 {
                    col1[idx] = (idx, item.clone());
                } else if idx < 50 {
                    col2[idx - 25] = (idx, item.clone());
                } else {
                    col3[idx - 50] = (idx, item.clone());
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
            let desc = Paragraph::new(Text::from(props))
                .block(desc_block)
                .wrap(ratatui::widgets::Wrap { trim: true });
            let money = gui_args.player.money;
            let money_str = format!("money: {}", money);
            let back = "BS for back".to_string();
            // let mut mvec = Vec::new();
            let mvec = vec![
                Line::from(Span::raw(money_str)),
                Line::from(Span::raw(back)),
            ];


            let money_info = Paragraph::new(Text::from(mvec))
                .block(info_block)
                .wrap(ratatui::widgets::Wrap { trim: true });

            f.render_widget(desc, norm_top[0]);
            f.render_widget(money_info, norm_top[1]);
            f.render_widget(table, normal_info[1]);
        }).unwrap();
    }    
    
    pub fn npc_conv_draw(&mut self, name: String, text: String, opts_vec: Vec<String>, gui_args: &mut GuiArgs) {
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

            if in_h != self.viewport_dim.1 && in_w != self.viewport_dim.0 {
                // map.set_viewport(in_h, in_w);
                self.viewport_dim = (in_w, in_h);
            }
            let paragraph = draw_map(gui_args, self.ani_cnt);
            // let paragraph = draw_map(map.clone(), player.clone(), portals.clone(), enemies.clone(), items.clone(), npcs.clone(), litems.clone(), env_inter.clone(), self.ani_cnt);
            f.render_widget(paragraph, inner_area);


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
                .title(name)
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Black));
            let table_block = Block::default()
                .title("")
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Black));
            
            let table_inner = table_block.inner(normal_info[1]);
            let table_width = table_inner.width;
            
            // let comm = npc_str[1];
            //
            let npc = Paragraph::new(Span::styled(text, Style::default().white()))
                .block(paragraph_block)
                .wrap(ratatui::widgets::Wrap { trim: true });

            let rows: Vec<Row> = opts_vec.iter().enumerate().map(|(j, cell)| {
                let wrapped_text = wrap_text(cell, table_width.into());
                //let styled_text = wrapped_text.iter().map(|line| Span::raw(line.clone())).collect::<Vec<_>>();
                if j == self.cursor_pos.1 {
                    //Row::new(vec![Cell::from(Span::styled(cell.clone(), Style::default().fg(Color::Yellow)))]).height(2)
                    Row::new(vec![Cell::from(wrapped_text)])
                        .style(Style::default().fg(Color::Yellow))
                        .height(2)
                } else {
                    Row::new(vec![Cell::from(wrapped_text)]).height(2)
                    //Row::new(vec![Cell::from(Span::raw(cell.clone()))]).height(2)

                }
            }).collect();
            let table = Table::new(rows, &[Constraint::Percentage(100)])
                .block(table_block);
            f.render_widget(npc, normal_info[0]);
            f.render_widget(table, normal_info[1]);
        }).unwrap();
    }

    pub fn shop_convo_draw(&mut self, sname: String, dialogue: String, gui_args: &mut GuiArgs) {
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

            if in_h != self.viewport_dim.1 && in_w != self.viewport_dim.0 {
                // map.set_viewport(in_h, in_w);
                self.viewport_dim = (in_w, in_h);
            }
            let paragraph = draw_map(gui_args, self.ani_cnt);
            // let paragraph = draw_map(map.clone(), player.clone(), portals.clone(), enemies.clone(), items.clone(), npcs.clone(), litems.clone(), env_inter.clone(), self.ani_cnt);
            f.render_widget(paragraph, inner_area);


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
                .title(&*sname)
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Black));
            let table_block = Block::default()
                .title("Buy")
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Black));
            let paragraph = Paragraph::new(Span::styled(&dialogue, Style::default().white()))
                .block(paragraph_block)
                .wrap(ratatui::widgets::Wrap { trim: true });
            // let mut adj_list = vec![];
            let vec1 = vec!["Yes", "No"];
            let opts = [vec1.clone()];
            let rows: Vec<Row> = opts.iter().enumerate().map(|(j, row)| {
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
            f.render_widget(paragraph, normal_info[0]);
            f.render_widget(table, normal_info[1]);
        }).unwrap();
    }

    pub fn guild_records_draw(&mut self, save_str: String, savelist: Vec<String>, gui_args: &mut GuiArgs) {
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

            if in_h != self.viewport_dim.1 && in_w != self.viewport_dim.0 {
                // map.set_viewport(in_h, in_w);
                self.viewport_dim = (in_w, in_h);
            }
            let paragraph = draw_map(gui_args, self.ani_cnt);
            // let paragraph = draw_map(map.clone(), player.clone(), portals.clone(), enemies.clone(), items.clone(), npcs.clone(), litems.clone(), env_inter.clone(), self.ani_cnt);
            f.render_widget(paragraph, inner_area);


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
                .title("Guild Local Records")
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Black));
            let table_block = Block::default()
                .title("Save")
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Black));
            
            let save_text = Paragraph::new(Span::styled(save_str, Style::default().white()))
                .block(paragraph_block)
                .wrap(ratatui::widgets::Wrap { trim: true });
            
            match savelist.len() {
                0 => {
                    let vec1 = vec!["Yes", "No"];
                    let opts = [vec1.clone()];
                    let rows: Vec<Row> = opts.iter().enumerate().map(|(j, row)| {
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
                _ => {
                    let opts = [savelist.clone()];
                    let rows: Vec<Row> = opts.iter().enumerate().map(|(j, row)| {
                        let cells: Vec<Cell> = row.iter().enumerate().map(|(i, cell)| {
                            if j == self.cursor_pos.0 && i == self.cursor_pos.1 {
                                Cell::from(Span::styled(cell, ratatui::style::Style::default().fg(ratatui::style::Color::Yellow)))
                            } else {
                                Cell::from(cell.clone())
                            }
                        }).collect();
                        Row::new(cells)
                    }).collect();
                    let table = Table::new(rows, &[Constraint::Percentage(100)])
                        .block(table_block);
                    f.render_widget(table, normal_info[1]);
                }
            }
            f.render_widget(save_text, normal_info[0]);
        }).unwrap();
    }

    pub fn cauldron_draw(&mut self, product_list: &Vec<Items>, gui_args: &mut GuiArgs) {
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

            if in_h != self.viewport_dim.1 && in_w != self.viewport_dim.0 {
                // map.set_viewport(in_h, in_w);
                self.viewport_dim = (in_w, in_h);
            }
            let paragraph = draw_map(gui_args, self.ani_cnt);
            f.render_widget(paragraph, inner_area);


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
                .title("Cauldron")
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Black));
            let table_block = Block::default()
                .title("Save")
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Black));
            let cauldron_msg = if product_list.is_empty() {
                "It looks like you dont have enoug of anythin to brew anything."
            } else {
                "The following things can be made with what you have"
            };
            let save_text = Paragraph::new(Span::styled(cauldron_msg, Style::default().white()))
                .block(paragraph_block)
                .wrap(ratatui::widgets::Wrap { trim: true });

            let mut product_str = Vec::new();
            for i in product_list {
                product_str.push(
                    match i {
                        Items::HealthPotion => "Health Potion",
                        Items::Salve => "Salve",
                        // Items::HealthPotion => "Health Potion",
                        _ => "",
                    }
                );
            }
            let opts = [product_str];
            let rows: Vec<Row> = opts.iter().enumerate().map(|(j, row)| {
                let cells: Vec<Cell> = row.iter().enumerate().map(|(i, cell)| {
                    if j == self.cursor_pos.1 && i == self.cursor_pos.0 {
                        Cell::from(Span::styled(*cell, ratatui::style::Style::default().fg(ratatui::style::Color::Yellow)))
                    } else {
                        Cell::from(*cell)
                    }
                }).collect();
                Row::new(cells)
            }).collect();
            let table = Table::new(rows, &[Constraint::Percentage(100), Constraint::Percentage(100), Constraint::Percentage(100)])
                .block(table_block);
            f.render_widget(table, normal_info[1]);
            f.render_widget(save_text, normal_info[0]);
        }).unwrap();
    }

    pub fn clinic_draw(&mut self, gui_args: &mut GuiArgs) {
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

            if in_h != self.viewport_dim.1 && in_w != self.viewport_dim.0 {
                // map.set_viewport(in_h, in_w);
                self.viewport_dim = (in_w, in_h);
            }
            let paragraph = draw_map(gui_args, self.ani_cnt);
            // let paragraph = draw_map(map.clone(), player.clone(), portals.clone(), enemies.clone(), items.clone(), npcs.clone(), litems.clone(), env_inter.clone(), self.ani_cnt);
            f.render_widget(paragraph, inner_area);


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
                .title("Guild Clinic")
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Black));
            let table_block = Block::default()
                .title("Heal")
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Black));
            
            let save_text = Paragraph::new(Span::styled("Would you like us to heal you? You will be treated to full health for the cost of 20g.", Style::default().white()))
                .block(paragraph_block)
                .wrap(ratatui::widgets::Wrap { trim: true });
            
            let vec1 = vec!["Yes", "No"];
            let opts = [vec1.clone()];
            let rows: Vec<Row> = opts.iter().enumerate().map(|(j, row)| {
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
            f.render_widget(save_text, normal_info[0]);
        }).unwrap();
    }

    pub fn clinic_resp_draw(&mut self, heal_resp: String, gui_args: &mut GuiArgs) {
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

            if in_h != self.viewport_dim.1 && in_w != self.viewport_dim.0 {
                // map.set_viewport(in_h, in_w);
                self.viewport_dim = (in_w, in_h);
            }
            let paragraph = draw_map(gui_args, self.ani_cnt);
            // let paragraph = draw_map(map.clone(), player.clone(), portals.clone(), enemies.clone(), items.clone(), npcs.clone(), litems.clone(), env_inter.clone(), self.ani_cnt);
            f.render_widget(paragraph, inner_area);


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
                .title("Guild Clinic")
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Black));
            let table_block = Block::default()
                .title("Heal")
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Black));
            
            let save_text = Paragraph::new(Span::styled(&heal_resp, Style::default().white()))
                .block(paragraph_block)
                .wrap(ratatui::widgets::Wrap { trim: true });
            
            let vec1 = vec!["Ok"];
            let opts = [vec1.clone()];
            let rows: Vec<Row> = opts.iter().enumerate().map(|(j, row)| {
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
            f.render_widget(save_text, normal_info[0]);
        }).unwrap();
    }

    pub fn guild_post_draw(&mut self, post_strings: Vec<String>, gui_args: &mut GuiArgs) {
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

            if in_h != self.viewport_dim.1 && in_w != self.viewport_dim.0 {
                // map.set_viewport(in_h, in_w);
                self.viewport_dim = (in_w, in_h);
            }
            let paragraph = draw_map(gui_args, self.ani_cnt);
            // let paragraph = draw_map(map.clone(), player.clone(), portals.clone(), enemies.clone(), items.clone(), npcs.clone(), litems.clone(), env_inter.clone(), self.ani_cnt);
            f.render_widget(paragraph, inner_area);


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
                .title("Guild Posting Board")
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Black));
            let table_block = Block::default()
                .title("Done")
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Black));

            let mut v1 = Vec::new();

            for s in post_strings {
                let s_temp = s.clone();
                let sparts = s_temp.split("#").collect::<Vec<&str>>();
                let v2 = [
                    sparts[0].to_string(),
                    sparts[1].to_string(),
                ];
                v1.push(v2.clone());
            }



            let rows: Vec<Row> = v1.iter().enumerate().map(|(j, row)| {
                let cells: Vec<Cell> = row.iter().enumerate().map(|(i, cell)| {
                    if i == self.cursor_pos.0 && j == self.cursor_pos.1 {
                        Cell::from(Span::styled(cell, ratatui::style::Style::default().fg(ratatui::style::Color::Yellow)))
                    } else {
                        Cell::from(Span::styled(cell, ratatui::style::Style::default().fg(ratatui::style::Color::White)))
                    }
                }).collect();
                Row::new(cells)
            }).collect();
            let p_table = Table::new(rows, &[Constraint::Percentage(30), Constraint::Percentage(70)])
                .block(paragraph_block);
            
            //let save_text = Paragraph::new(Span::styled(&heal_resp, Style::default().white()))
              //  .block(paragraph_block)
                //.wrap(ratatui::widgets::Wrap { trim: true });
            
            let vec1 = vec!["Ok"];
            let opts = [vec1.clone()];
            let rows: Vec<Row> = opts.iter().enumerate().map(|(j, row)| {
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
            f.render_widget(p_table, normal_info[0]);
            f.render_widget(table, normal_info[1]);
        }).unwrap();
    }


    pub fn church_post_draw(&mut self, post_strings: Vec<String>, gui_args: &mut GuiArgs) {
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

            if in_h != self.viewport_dim.1 && in_w != self.viewport_dim.0 {
                // map.set_viewport(in_h, in_w);
                self.viewport_dim = (in_w, in_h);
            }
            let paragraph = draw_map(gui_args, self.ani_cnt);
            // let paragraph = draw_map(map.clone(), player.clone(), portals.clone(), enemies.clone(), items.clone(), npcs.clone(), litems.clone(), env_inter.clone(), self.ani_cnt);
            f.render_widget(paragraph, inner_area);


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
                .title("Church Posting Board")
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Black));
            let table_block = Block::default()
                .title("Done")
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Black));

            let mut v1 = Vec::new();

            for s in post_strings {
                let s_temp = s.clone();
                let sparts = s_temp.split("#").collect::<Vec<&str>>();
                let v2 = [
                    sparts[0].to_string(),
                    sparts[1].to_string(),
                ];
                v1.push(v2.clone());
            }



            let rows: Vec<Row> = v1.iter().enumerate().map(|(j, row)| {
                let cells: Vec<Cell> = row.iter().enumerate().map(|(i, cell)| {
                    if i == self.cursor_pos.0 && j == self.cursor_pos.1 {
                        Cell::from(Span::styled(cell, ratatui::style::Style::default().fg(ratatui::style::Color::Yellow)))
                    } else {
                        Cell::from(Span::styled(cell, ratatui::style::Style::default().fg(ratatui::style::Color::White)))
                    }
                }).collect();
                Row::new(cells)
            }).collect();
            let p_table = Table::new(rows, &[Constraint::Percentage(30), Constraint::Percentage(70)])
                .block(paragraph_block);
            
            //let save_text = Paragraph::new(Span::styled(&heal_resp, Style::default().white()))
              //  .block(paragraph_block)
                //.wrap(ratatui::widgets::Wrap { trim: true });
            
            let vec1 = vec!["Ok"];
            let opts = [vec1.clone()];
            let rows: Vec<Row> = opts.iter().enumerate().map(|(j, row)| {
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
            f.render_widget(p_table, normal_info[0]);
            f.render_widget(table, normal_info[1]);
        }).unwrap();
    }
}
