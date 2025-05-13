use crate::enums::{EnvInter, InterOpt, Interactable, NPCWrap};
use crate::gui::GUI;
use crate::gui_utils::{draw_map, GuiArgs};
use crate::npc::NPC;
use ratatui::layout::{Constraint, Direction, Layout, Margin};
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::Span;
use ratatui::widgets::Cell;
use ratatui::widgets::Row;
use ratatui::widgets::Table;
use ratatui::widgets::{Block, Borders, Paragraph};

impl GUI {
    //ineractions
    pub fn inter_adj_draw(&mut self, gui_args: &mut GuiArgs) {
        self.terminal
            .draw(|f| {
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
                            Constraint::Percentage(10),
                        ]
                        .as_ref(),
                    )
                    .split(f.area());

                let game_chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
                    .split(chunks[1]);

                let block = Block::default().title("Game").borders(Borders::ALL);
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
                    .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
                    .split(game_chunks[1]);
                let paragraph_block = Block::default()
                    .title("Paragraph Block")
                    .borders(Borders::ALL)
                    .style(Style::default().bg(Color::Black));
                let table_block = Block::default()
                    .title("Table Block")
                    .borders(Borders::ALL)
                    .style(Style::default().bg(Color::Black));
                let paragraph = Paragraph::new(Span::styled(
                    "What would you like to interct with?",
                    Style::default().white(),
                ))
                .block(paragraph_block)
                .wrap(ratatui::widgets::Wrap { trim: true });
                let mut adj_list = vec![];
                let mut vec1 = vec![((0_usize, 0_usize), "".to_string()); 3];
                let mut vec2 = vec![((0_usize, 0_usize), "".to_string()); 3];
                for (pos, interable) in &self.interactable {
                    let Some(inter) = interable else { todo!() };
                    match inter {
                        Interactable::Item(item) => adj_list.push((*pos, item.clone().get_sname())),
                        Interactable::ShopItem(item) => {
                            adj_list.push((*pos, item.clone().get_sname()))
                        }
                        Interactable::Enemy(enemy) => {
                            adj_list.push((*pos, enemy.clone().get_sname()))
                        }
                        Interactable::NPC(npc) => {
                            match npc {
                                NPCWrap::CommNPC(comm_npc) => {
                                    adj_list.push((*pos, comm_npc.clone().get_sname()))
                                }
                                NPCWrap::ConvNPC(conv_npc) => {
                                    adj_list.push((*pos, conv_npc.clone().get_sname()))
                                }
                                NPCWrap::ShopNPC(shop_npc) => {
                                    adj_list.push((*pos, shop_npc.clone().get_sname()))
                                }
                                NPCWrap::SpawnNPC(spawn_npc) => {
                                    adj_list.push((*pos, spawn_npc.clone().get_sname()))
                                }
                                NPCWrap::TradeNPC(trade_npc) => {
                                    adj_list.push((*pos, trade_npc.clone().get_sname()))
                                }
                                _ => todo!(),
                            }
                            // adj_list.push((*pos, npc.clone().get_sname()));
                        }
                        Interactable::EnvInter(env_inter) => match env_inter {
                            EnvInter::Records => adj_list.push((*pos, "Local Records".to_string())),
                            EnvInter::Clinic => adj_list.push((*pos, "Clinic".to_string())),
                            EnvInter::GuildPost => {
                                adj_list.push((*pos, "Guild Posting".to_string()))
                            }
                            EnvInter::ChurchPost => {
                                adj_list.push((*pos, "Church Posting".to_string()))
                            }
                            EnvInter::Cauldron => adj_list.push((*pos, "Cauldron".to_string())),
                            _ => todo!(),
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
                let inter_entities = [vec1.clone(), vec2.clone()];
                self.adj_options = (vec1, vec2);
                let rows: Vec<Row> = inter_entities
                    .iter()
                    .enumerate()
                    .map(|(j, row)| {
                        let cells: Vec<Cell> = row
                            .iter()
                            .enumerate()
                            .map(|(i, cell)| {
                                if i == self.cursor_pos.0 && j == self.cursor_pos.1 {
                                    Cell::from(Span::styled(
                                        cell.clone().1,
                                        ratatui::style::Style::default()
                                            .fg(ratatui::style::Color::Yellow),
                                    ))
                                } else {
                                    Cell::from(cell.clone().1)
                                }
                            })
                            .collect();
                        Row::new(cells)
                    })
                    .collect();
                let table = Table::new(
                    rows,
                    &[Constraint::Percentage(50), Constraint::Percentage(50)],
                )
                .block(table_block);
                f.render_widget(paragraph, normal_info[0]);
                f.render_widget(table, normal_info[1]);
            })
            .unwrap();
    }

    pub fn inter_opt_draw(&mut self, gui_args: &mut GuiArgs) {
        self.terminal
            .draw(|f| {
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
                            Constraint::Percentage(10),
                        ]
                        .as_ref(),
                    )
                    .split(f.area());

                let game_chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
                    .split(chunks[1]);

                let block = Block::default().title("Game").borders(Borders::ALL);
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
                    .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
                    .split(game_chunks[1]);
                let paragraph_block = Block::default()
                    .title("Item")
                    .borders(Borders::ALL)
                    .style(Style::default().bg(Color::Black));
                let table_block = Block::default()
                    .title("Options")
                    .borders(Borders::ALL)
                    .style(Style::default().bg(Color::Black));
                let paragraph = Paragraph::new(Span::styled(
                    "What would you like to do with it?",
                    Style::default().white(),
                ))
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
                let rows: Vec<Row> = inter_opts
                    .iter()
                    .enumerate()
                    .map(|(j, row)| {
                        let cells: Vec<Cell> = row
                            .iter()
                            .enumerate()
                            .map(|(i, cell)| {
                                if i == self.cursor_pos.0 && j == self.cursor_pos.1 {
                                    Cell::from(Span::styled(
                                        cell.clone().1,
                                        ratatui::style::Style::default()
                                            .fg(ratatui::style::Color::Yellow),
                                    ))
                                } else {
                                    Cell::from(cell.clone().1)
                                }
                            })
                            .collect();
                        Row::new(cells)
                    })
                    .collect();
                let table = Table::new(
                    rows,
                    &[
                        Constraint::Percentage(50),
                        Constraint::Percentage(50),
                        Constraint::Percentage(50),
                    ],
                )
                .block(table_block);
                f.render_widget(paragraph, normal_info[0]);
                f.render_widget(table, normal_info[1]);
            })
            .unwrap();
    }

    pub fn inter_res_draw(&mut self, gui_args: &mut GuiArgs) {
        self.terminal
            .draw(|f| {
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
                            Constraint::Percentage(10),
                        ]
                        .as_ref(),
                    )
                    .split(f.area());

                let game_chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
                    .split(chunks[1]);

                let block = Block::default().title("Game").borders(Borders::ALL);
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
                    .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
                    .split(game_chunks[1]);
                let paragraph_block = Block::default()
                    .title("Paragraph Block")
                    .borders(Borders::ALL)
                    .style(Style::default().bg(Color::Black));
                let table_block = Block::default()
                    .title("Table Block")
                    .borders(Borders::ALL)
                    .style(Style::default().bg(Color::Black));
                let paragraph = Paragraph::new(Span::styled("Done", Style::default().white()))
                    .block(paragraph_block);
                let vec1 = vec!["Ok".to_string(); 1];
                let vec2 = vec!["".to_string(); 1];

                let inter_opts = [vec1.clone(), vec2.clone()];
                let rows: Vec<Row> = inter_opts
                    .iter()
                    .enumerate()
                    .map(|(j, row)| {
                        let cells: Vec<Cell> = row
                            .iter()
                            .enumerate()
                            .map(|(i, cell)| {
                                if i == self.cursor_pos.0 && j == self.cursor_pos.1 {
                                    Cell::from(Span::styled(
                                        cell.clone(),
                                        ratatui::style::Style::default()
                                            .fg(ratatui::style::Color::Yellow),
                                    ))
                                } else {
                                    Cell::from(cell.clone())
                                }
                            })
                            .collect();
                        Row::new(cells)
                    })
                    .collect();
                let table = Table::new(
                    rows,
                    &[Constraint::Percentage(50), Constraint::Percentage(50)],
                )
                .block(table_block);
                f.render_widget(paragraph, normal_info[0]);
                f.render_widget(table, normal_info[1]);
            })
            .unwrap();
    }

    //item_use
    pub fn item_use_draw(&mut self, msg_str: String, iopts: String, gui_args: &mut GuiArgs) {
        self.terminal
            .draw(|f| {
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
                            Constraint::Percentage(10),
                        ]
                        .as_ref(),
                    )
                    .split(f.area());

                let game_chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
                    .split(chunks[1]);

                let block = Block::default().title("Game").borders(Borders::ALL);
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
                    .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
                    .split(game_chunks[1]);
                let paragraph_block = Block::default()
                    .title("Inventory")
                    .borders(Borders::ALL)
                    .style(Style::default().bg(Color::Black));
                let table_block = Block::default()
                    .title("")
                    .borders(Borders::ALL)
                    .style(Style::default().bg(Color::Black));
                let paragraph = Paragraph::new(Span::styled(&msg_str, Style::default().white()))
                    .block(paragraph_block);
                let vec1: Vec<&str> = iopts.split("#").collect();
                let vec2 = vec![""; 1];

                let inter_opts = [vec1.clone(), vec2.clone()];
                let rows: Vec<Row> = inter_opts
                    .iter()
                    .enumerate()
                    .map(|(j, row)| {
                        let cells: Vec<Cell> = row
                            .iter()
                            .enumerate()
                            .map(|(i, cell)| {
                                if i == self.cursor_pos.0 && j == self.cursor_pos.1 {
                                    Cell::from(Span::styled(
                                        cell.to_string(),
                                        ratatui::style::Style::default()
                                            .fg(ratatui::style::Color::Yellow),
                                    ))
                                } else {
                                    Cell::from(cell.to_string())
                                }
                            })
                            .collect();
                        Row::new(cells)
                    })
                    .collect();
                let table = Table::new(
                    rows,
                    &[
                        Constraint::Percentage(33),
                        Constraint::Percentage(33),
                        Constraint::Percentage(33),
                    ],
                )
                .block(table_block);
                f.render_widget(paragraph, normal_info[0]);
                f.render_widget(table, normal_info[1]);
            })
            .unwrap();
    }

    pub fn item_used_draw(&mut self, gui_args: &mut GuiArgs) {
        self.terminal
            .draw(|f| {
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
                            Constraint::Percentage(10),
                        ]
                        .as_ref(),
                    )
                    .split(f.area());

                let game_chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
                    .split(chunks[1]);

                let block = Block::default().title("Game").borders(Borders::ALL);
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
                    .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
                    .split(game_chunks[1]);
                let paragraph_block = Block::default()
                    .title("Inventory")
                    .borders(Borders::ALL)
                    .style(Style::default().bg(Color::Black));
                let table_block = Block::default()
                    .title("")
                    .borders(Borders::ALL)
                    .style(Style::default().bg(Color::Black));
                let paragraph = Paragraph::new(Span::styled("Item used", Style::default().white()))
                    .block(paragraph_block);
                let vec1 = vec!["Ok".to_string(); 1];
                let vec2 = vec!["".to_string(); 1];

                let inter_opts = [vec1.clone(), vec2.clone()];
                let rows: Vec<Row> = inter_opts
                    .iter()
                    .enumerate()
                    .map(|(j, row)| {
                        let cells: Vec<Cell> = row
                            .iter()
                            .enumerate()
                            .map(|(i, cell)| {
                                if i == self.cursor_pos.0 && j == self.cursor_pos.1 {
                                    Cell::from(Span::styled(
                                        cell.clone(),
                                        ratatui::style::Style::default()
                                            .fg(ratatui::style::Color::Yellow),
                                    ))
                                } else {
                                    Cell::from(cell.clone())
                                }
                            })
                            .collect();
                        Row::new(cells)
                    })
                    .collect();
                let table = Table::new(
                    rows,
                    &[Constraint::Percentage(50), Constraint::Percentage(50)],
                )
                .block(table_block);
                f.render_widget(paragraph, normal_info[0]);
                f.render_widget(table, normal_info[1]);
            })
            .unwrap();
    }
}
