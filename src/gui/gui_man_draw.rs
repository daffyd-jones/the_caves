use crate::enums::{Cells, Enemies, Items, NPCWrap, GUIMode, InterSteps, Interactable, InterOpt, EncOpt};
use crate::map::Map;
use crate::player::Player;
use crate::enemy::{Enemy};
use crate::npc::{NPC, CommNPC};
use crate::item::Item;
use crate::notebook::{Quest, Stage, Place, Person, Lore};
use crate::gui::GUI;
use crate::gui::draw_map;

use std::collections::HashMap;

// use std::time::Duration;
// use rand::Rng;
use ratatui::widgets::{Block, Borders, Paragraph, Wrap, Padding};
use ratatui::layout::{Layout, Constraint, Direction, Margin};
use ratatui::style::{Color, Style};
use ratatui::text::{Text, Span};
use ratatui::widgets::Row;
use ratatui::widgets::Table;
use ratatui::widgets::Cell;

impl GUI {

    //ineract start--------
    pub fn inter_adj_draw(&mut self, mut map: Map, map_vec: Vec<Vec<Cells>>, player: Player, enemies: HashMap<(usize, usize), Enemy>, items: HashMap<(usize, usize), Item>, npcs: HashMap<(usize, usize), NPCWrap>) {
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
            let paragraph = draw_map(map.clone(), map_vec.clone(), player.clone(), enemies.clone(), items.clone(), npcs.clone());
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
            let paragraph = Paragraph::new(Span::raw("What would you like to interct with?"))
                .block(paragraph_block);
            let mut adj_list = vec![];
            let mut vec1 = vec![((0 as usize, 0 as usize), "".to_string()); 3];
            let mut vec2 = vec![((0 as usize, 0 as usize), "".to_string()); 3];
            for (pos, interable) in &self.interactable {
                let Some(inter) = interable else {todo!()};
                match inter {
                    Interactable::Item(item) => adj_list.push((*pos, item.clone().get_sname())),
                    Interactable::Enemy(enemy) => adj_list.push((*pos, enemy.clone().get_sname())),
                    Interactable::NPC(npc) => {
                        match npc {
                            NPCWrap::CommNPC(comm_npc) => adj_list.push((*pos, comm_npc.clone().get_sname())),
                            NPCWrap::ConvNPC(conv_npc) => adj_list.push((*pos, conv_npc.clone().get_sname())),
                            NPCWrap::QuestNPC(quest_npc) => adj_list.push((*pos, quest_npc.clone().get_sname())),
                           _ => todo!(),
                        }
                        // adj_list.push((*pos, npc.clone().get_sname()));
                    },
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
            let inter_entities = vec![vec1.clone(), vec2.clone()];
            self.adj_options = (vec1, vec2);
            let rows: Vec<Row> = inter_entities.iter().enumerate().map(|(j, row)| {
                let cells: Vec<Cell> = row.iter().enumerate().map(|(i, &ref cell)| {
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

    pub fn inter_opt_draw(&mut self, mut map: Map, map_vec: Vec<Vec<Cells>>, player: Player, enemies: HashMap<(usize, usize), Enemy>, items: HashMap<(usize, usize), Item>, npcs: HashMap<(usize, usize), NPCWrap>) {
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
            let paragraph = draw_map(map.clone(), map_vec.clone(), player.clone(), enemies.clone(), items.clone(), npcs.clone());
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
            let paragraph = Paragraph::new(Span::raw("What would you like to do with it?"))
                .block(paragraph_block);
            let mut vec1 = vec![(InterOpt::Null, "".to_string()); 3];
            let mut vec2 = vec![(InterOpt::Null, "".to_string()); 3];

            for (idx, (a, b)) in self.inter_opt.iter().enumerate() {
                if idx < 3 {
                    vec1[idx] = (a.clone(), b.clone());
                } else {
                    vec2[idx - 3] = (a.clone(), b.clone());
                }
            }
            let inter_opts = vec![vec1.clone(), vec2.clone()];
            self.inter_options = (vec1, vec2);
            let rows: Vec<Row> = inter_opts.iter().enumerate().map(|(j, row)| {
                let cells: Vec<Cell> = row.iter().enumerate().map(|(i, &ref cell)| {
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

    pub fn inter_res_draw(&mut self, mut map: Map, map_vec: Vec<Vec<Cells>>, player: Player, enemies: HashMap<(usize, usize), Enemy>, items: HashMap<(usize, usize), Item>, npcs: HashMap<(usize, usize), NPCWrap>) {
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
            let paragraph = draw_map(map.clone(), map_vec.clone(), player.clone(), enemies.clone(), items.clone(), npcs.clone());
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
            let paragraph = Paragraph::new(Span::raw("Done"))
                .block(paragraph_block);
            let mut vec1 = vec!["Ok".to_string(); 1];
            let mut vec2 = vec!["".to_string(); 1];

            let inter_opts = vec![vec1.clone(), vec2.clone()];
            let rows: Vec<Row> = inter_opts.iter().enumerate().map(|(j, row)| {
                let cells: Vec<Cell> = row.iter().enumerate().map(|(i, &ref cell)| {
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

    pub fn item_used_draw(&mut self, mut map: Map, map_vec: Vec<Vec<Cells>>, player: Player, enemies: HashMap<(usize, usize), Enemy>, items: HashMap<(usize, usize), Item>, npcs: HashMap<(usize, usize), NPCWrap>) {
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
            let paragraph = draw_map(map.clone(), map_vec.clone(), player.clone(), enemies.clone(), items.clone(), npcs.clone());
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
            let paragraph = Paragraph::new(Span::raw("Item used"))
                .block(paragraph_block);
            let mut vec1 = vec!["Ok".to_string(); 1];
            let mut vec2 = vec!["".to_string(); 1];

            let inter_opts = vec![vec1.clone(), vec2.clone()];
            let rows: Vec<Row> = inter_opts.iter().enumerate().map(|(j, row)| {
                let cells: Vec<Cell> = row.iter().enumerate().map(|(i, &ref cell)| {
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

    pub fn encounter_show_content(&mut self, cntnt: String, mut map: Map, map_vec: Vec<Vec<Cells>>, player: Player, enemies: HashMap<(usize, usize), Enemy>, items: HashMap<(usize, usize), Item>, npcs: HashMap<(usize, usize), NPCWrap>) {
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
                    Constraint::Percentage(30),
                    Constraint::Percentage(70)
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

            let paragraph = draw_map(map.clone(), map_vec.clone(), player.clone(), enemies.clone(), items.clone(), npcs.clone());
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
            let paragraph = Paragraph::new(Span::raw(&cntnt))
                .block(enc_text_block)
                .wrap(ratatui::widgets::Wrap { trim: true });

            f.render_widget(paragraph, left_chunk[0]);



//------



            let options_block = Block::default()
                .title("")
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Black));

            let mut vec1 = vec!["Ok".to_string(); 1];
            let mut vec2 = vec!["".to_string(); 1];

            let inter_opts = vec![vec1.clone(), vec2.clone()];
            let rows: Vec<Row> = inter_opts.iter().enumerate().map(|(j, row)| {
                let cells: Vec<Cell> = row.iter().enumerate().map(|(i, &ref cell)| {
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
                    Span::styled(player.health.to_string(), Style::default().fg(Color::Yellow)),
                ]),
                Row::new(vec![
                    Span::styled("Attack: ", Style::default().fg(Color::White)),
                    Span::styled(player.attack.to_string(), Style::default().fg(Color::Yellow)),
                ]),
                Row::new(vec![
                    Span::styled("Defence: ", Style::default().fg(Color::White)),
                    Span::styled(player.defence.to_string(), Style::default().fg(Color::Yellow)),
                ]),
                Row::new(vec![
                    Span::styled("Damage: ", Style::default().fg(Color::White)),
                    Span::styled(player.damage.to_string(), Style::default().fg(Color::Yellow)),
                ]),
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
            let table = Table::new(rows, &[Constraint::Percentage(50), Constraint::Percentage(50)])
                .block(stats_block);

            // let paragraph = Paragraph::new(Span::raw("Item used"))
            //     .block(stats_block);
            f.render_widget(table, right_chunk[1]);





        }).unwrap();
    }

    pub fn encounter_user_options(&mut self, enc_opt: HashMap<EncOpt, String>, mut map: Map, map_vec: Vec<Vec<Cells>>, player: Player, enemies: HashMap<(usize, usize), Enemy>, items: HashMap<(usize, usize), Item>, npcs: HashMap<(usize, usize), NPCWrap>) {
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
                    Constraint::Percentage(30),
                    Constraint::Percentage(70)
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
            let paragraph = draw_map(map.clone(), map_vec.clone(), player.clone(), enemies.clone(), items.clone(), npcs.clone());
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
            let paragraph = Paragraph::new(Span::raw("What would you like to do?"))
                .block(enc_text_block);

            f.render_widget(paragraph, left_chunk[0]);



//------



            let options_block = Block::default()
                .title("")
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Black));

            let mut vec1 = vec![(EncOpt::Null, "".to_string()); 3];
            let mut vec2 = vec![(EncOpt::Null, "".to_string()); 3];

            for (idx, (a, b)) in enc_opt.iter().enumerate() {
                if idx < 3 {
                    vec1[idx] = (a.clone(), b.clone());
                } else {
                    vec2[idx - 3] = (a.clone(), b.clone());
                }
            }
            let enc_opts = vec![vec1.clone(), vec2.clone()];
            self.enc_opt = (vec1, vec2);

            // let mut vec1 = vec!["Ok".to_string(); 1];
            // let mut vec2 = vec!["".to_string(); 1];

            // let enc_opts = vec![vec1.clone(), vec2.clone()];
            let rows: Vec<Row> = enc_opts.iter().enumerate().map(|(j, row)| {
                let cells: Vec<Cell> = row.iter().enumerate().map(|(i, &ref cell)| {
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
                    Span::styled(player.health.to_string(), Style::default().fg(Color::Yellow)),
                ]),
                Row::new(vec![
                    Span::styled("Attack: ", Style::default().fg(Color::White)),
                    Span::styled(player.attack.to_string(), Style::default().fg(Color::Yellow)),
                ]),
                Row::new(vec![
                    Span::styled("Defence: ", Style::default().fg(Color::White)),
                    Span::styled(player.defence.to_string(), Style::default().fg(Color::Yellow)),
                ]),
                Row::new(vec![
                    Span::styled("Damage: ", Style::default().fg(Color::White)),
                    Span::styled(player.damage.to_string(), Style::default().fg(Color::Yellow)),
                ]),
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
            let table = Table::new(rows, &[Constraint::Percentage(50), Constraint::Percentage(50)])
                .block(stats_block);

            // let paragraph = Paragraph::new(Span::raw("Item used"))
            //     .block(stats_block);
            f.render_widget(table, right_chunk[1]);





        }).unwrap();
    }

    pub fn encounter_pick_item(&mut self, mut map: Map, map_vec: Vec<Vec<Cells>>, player: Player, enemies: HashMap<(usize, usize), Enemy>, items: HashMap<(usize, usize), Item>, npcs: HashMap<(usize, usize), NPCWrap>) {
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
                    Constraint::Percentage(30),
                    Constraint::Percentage(70)
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
            let paragraph = draw_map(map.clone(), map_vec.clone(), player.clone(), enemies.clone(), items.clone(), npcs.clone());
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
            let paragraph = Paragraph::new(Span::raw("What item to use?"))
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

            let prop = HashMap::new();
            let itype = String::new();
            let desc = String::new();
            let iopts = HashMap::new();
            let i_temp = Item::new(Items::Null, itype, desc, iopts, 0, 0, prop);
            let mut col1 = vec![(0, i_temp.clone()); 25];
            let mut col2 = vec![(0, i_temp.clone()); 25];
            let mut col3 = vec![(0, i_temp.clone()); 25];
            //dd
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
                    Span::styled(player.health.to_string(), Style::default().fg(Color::Yellow)),
                ]),
                Row::new(vec![
                    Span::styled("Attack: ", Style::default().fg(Color::White)),
                    Span::styled(player.attack.to_string(), Style::default().fg(Color::Yellow)),
                ]),
                Row::new(vec![
                    Span::styled("Defence: ", Style::default().fg(Color::White)),
                    Span::styled(player.defence.to_string(), Style::default().fg(Color::Yellow)),
                ]),
                Row::new(vec![
                    Span::styled("Damage: ", Style::default().fg(Color::White)),
                    Span::styled(player.damage.to_string(), Style::default().fg(Color::Yellow)),
                ]),
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
            let table = Table::new(rows, &[Constraint::Percentage(50), Constraint::Percentage(50)])
                .block(stats_block);

            // let paragraph = Paragraph::new(Span::raw("Item used"))
            //     .block(stats_block);
            f.render_widget(table, right_chunk[1]);





        }).unwrap();
    }


    pub fn npc_comm_draw(&mut self, comms: String, mut map: Map, map_vec: Vec<Vec<Cells>>, player: Player, enemies: HashMap<(usize, usize), Enemy>, items: HashMap<(usize, usize), Item>, npcs: HashMap<(usize, usize), NPCWrap>) {
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
            let paragraph = draw_map(map.clone(), map_vec.clone(), player.clone(), enemies.clone(), items.clone(), npcs.clone());
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

            let npc = Paragraph::new(Span::raw(comm))
                .block(paragraph_block)
                .wrap(ratatui::widgets::Wrap { trim: true });
            let plyr = Paragraph::new(Span::raw(""))
                .block(table_block);
            // let mut vec1 = vec![(InterOpt::Null, "".to_string()); 3];
            // let mut vec2 = vec![(InterOpt::Null, "".to_string()); 3];
            //
            // for (idx, (a, b)) in self.inter_opt.iter().enumerate() {
            //     if idx < 3 {
            //         vec1[idx] = (a.clone(), b.clone());
            //     } else {
            //         vec2[idx - 3] = (a.clone(), b.clone());
            //     }
            // }
            // let inter_opts = vec![vec1.clone(), vec2.clone()];
            // self.inter_options = (vec1, vec2);
            // let rows: Vec<Row> = inter_opts.iter().enumerate().map(|(j, row)| {
            //     let cells: Vec<Cell> = row.iter().enumerate().map(|(i, &ref cell)| {
            //         if i == self.cursor_pos.0 && j == self.cursor_pos.1 {
            //             Cell::from(Span::styled(cell.clone().1, ratatui::style::Style::default().fg(ratatui::style::Color::Yellow)))
            //         } else {
            //             Cell::from(cell.clone().1)
            //         }
            //     }).collect();
            //     Row::new(cells)
            // }).collect();
            // let table = Table::new(rows, &[Constraint::Percentage(50), Constraint::Percentage(50), Constraint::Percentage(50)])
            //     .block(table_block);
            f.render_widget(npc, normal_info[0]);
            f.render_widget(plyr, normal_info[1]);
        }).unwrap();
    }

}
