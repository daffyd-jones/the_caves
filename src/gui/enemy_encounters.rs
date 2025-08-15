//enemy_encounters

use crate::enums::{
    EncOpt, EnvInter, Equip, InterOpt, Interactable, ItemEffect, Items, NPCWrap, Plants,
};
use crate::gui::GUI;
use crate::gui_utils::{draw_map, GuiArgs};
use crate::item::Item;
use crate::npc::NPC;
use ratatui::layout::{Constraint, Direction, Layout, Margin};
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::Line;
use ratatui::text::{Span, Text};
use ratatui::widgets::Cell;
use ratatui::widgets::Row;
use ratatui::widgets::Table;
use ratatui::widgets::{Block, Borders, Paragraph};
use std::collections::HashMap;

impl GUI {
    pub fn encounter_show_content(
        &mut self,
        cntnt: String,
        opts: Vec<String>,
        gui_args: &mut GuiArgs,
    ) {
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

                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(
                        [
                            Constraint::Percentage(70),
                            Constraint::Percentage(10),
                            Constraint::Percentage(20),
                        ]
                        .as_ref(),
                    )
                    .split(game_chunks[1]);

                //------

                let chunk_w = chunks[0].inner(Margin::new(0, 0)).width;
                let fit = chunk_w as i8 - 60;
                let ascii = gui_args.ascii.unwrap();
                let mut ascii_str = Vec::new();
                ascii_str.push(Span::styled(cntnt, Style::default().white()));
                for i in 0..(ascii.len() / 60) {
                    let line = &ascii[i * 60..(i * 60 + 60)];
                    let crop_line = if fit < 0 {
                        // line[-fit as usize..-fit as usize].to_string()
                        line[(-fit * 2) as usize..].to_string()
                    } else {
                        line.to_string()
                    };
                    ascii_str.push(Span::styled(crop_line, Style::default().white()));
                }
                let texts: Text = ascii_str.into_iter().collect();

                let paragraph_block = Block::default()
                    .title("Encounter")
                    .borders(Borders::ALL)
                    .style(Style::default().bg(Color::Black));

                let paragraph = Paragraph::new(texts)
                    .block(paragraph_block)
                    .wrap(ratatui::widgets::Wrap { trim: false });

                f.render_widget(paragraph, chunks[0]);

                //------

                let options_block = Block::default()
                    .title("")
                    .borders(Borders::ALL)
                    .style(Style::default().bg(Color::Black));

                let vec1 = opts.clone();
                let vec2 = vec!["".to_string(); 1];

                let inter_opts = [vec1.clone(), vec2.clone()];
                let cur_bounds = vec![opts.len()];
                self.cursor_bounds = cur_bounds;
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
                    &[
                        Constraint::Percentage(30),
                        Constraint::Percentage(30),
                        Constraint::Percentage(30),
                    ],
                )
                .block(options_block);

                f.render_widget(table, chunks[1]);

                //-----

                let stats_block = Block::default()
                    .title("")
                    .borders(Borders::ALL)
                    .style(Style::default().bg(Color::Black));

                let rows = vec![
                    Row::new(vec![
                        Span::styled("Health: ", Style::default().fg(Color::White)),
                        Span::styled(
                            gui_args.player.health.to_string(),
                            Style::default().fg(Color::Yellow),
                        ),
                    ]),
                    Row::new(vec![
                        Span::styled("Attack: ", Style::default().fg(Color::White)),
                        Span::styled(
                            gui_args.player.attack.to_string(),
                            Style::default().fg(Color::Yellow),
                        ),
                    ]),
                    Row::new(vec![
                        Span::styled("Defence: ", Style::default().fg(Color::White)),
                        Span::styled(
                            gui_args.player.defence.to_string(),
                            Style::default().fg(Color::Yellow),
                        ),
                    ]),
                    Row::new(vec![
                        Span::styled("Damage: ", Style::default().fg(Color::White)),
                        Span::styled(
                            gui_args.player.damage.to_string(),
                            Style::default().fg(Color::Yellow),
                        ),
                    ]),
                ];
                let table = Table::new(
                    rows,
                    &[Constraint::Percentage(50), Constraint::Percentage(50)],
                )
                .block(stats_block);

                f.render_widget(table, chunks[2]);
            })
            .unwrap();
    }

    pub fn encounter_auto_content(&mut self, gui_args: &mut GuiArgs) {
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

                //alksdjlfkj---

                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(
                        [
                            Constraint::Percentage(70),
                            Constraint::Percentage(10),
                            Constraint::Percentage(20),
                        ]
                        .as_ref(),
                    )
                    .split(game_chunks[1]);

                //------
                let chunk_w = chunks[0].inner(Margin::new(0, 0)).width;
                let fit = chunk_w as i8 - 60;
                let ascii = gui_args.ascii.unwrap();
                let mut ascii_str = Vec::new();
                ascii_str.push(Span::styled("", Style::default().white()));
                for i in 0..(ascii.len() / 60) {
                    let line = &ascii[i * 60..(i * 60 + 60)];
                    let crop_line = if fit < 0 {
                        line[(-fit * 2) as usize..].to_string()
                    } else {
                        line.to_string()
                    };
                    ascii_str.push(Span::styled(crop_line, Style::default().white()));
                }
                let texts: Text = ascii_str.into_iter().collect();

                let paragraph_block = Block::default()
                    .title("Encounter")
                    .borders(Borders::ALL)
                    .style(Style::default().bg(Color::Black));
                let paragraph = Paragraph::new(texts)
                    .block(paragraph_block)
                    .wrap(ratatui::widgets::Wrap { trim: false });

                f.render_widget(paragraph, chunks[0]);

                //------

                let options_block = Block::default()
                    .title("")
                    .borders(Borders::ALL)
                    .style(Style::default().bg(Color::Black));

                let vec1 = vec!["Ok".to_string(); 1];
                let vec2 = vec!["".to_string(); 1];

                let inter_opts = [vec1.clone(), vec2.clone()];
                self.cursor_bounds = vec![1];
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
                .block(options_block);

                f.render_widget(table, chunks[1]);

                //-----

                // let entity_block = Block::default()
                //     .title("")
                //     .borders(Borders::ALL)
                //     .style(Style::default().bg(Color::Black));
                // let paragraph = Paragraph::new(Span::raw("entity design")).block(entity_block);
                // f.render_widget(paragraph, right_chunk[0]);

                //-----

                let stats_block = Block::default()
                    .title("")
                    .borders(Borders::ALL)
                    .style(Style::default().bg(Color::Black));

                let rows = vec![
                    Row::new(vec![
                        Span::styled("Health: ", Style::default().fg(Color::White)),
                        Span::styled(
                            gui_args.player.health.to_string(),
                            Style::default().fg(Color::Yellow),
                        ),
                    ]),
                    Row::new(vec![
                        Span::styled("Attack: ", Style::default().fg(Color::White)),
                        Span::styled(
                            gui_args.player.attack.to_string(),
                            Style::default().fg(Color::Yellow),
                        ),
                    ]),
                    Row::new(vec![
                        Span::styled("Defence: ", Style::default().fg(Color::White)),
                        Span::styled(
                            gui_args.player.defence.to_string(),
                            Style::default().fg(Color::Yellow),
                        ),
                    ]),
                    Row::new(vec![
                        Span::styled("Damage: ", Style::default().fg(Color::White)),
                        Span::styled(
                            gui_args.player.damage.to_string(),
                            Style::default().fg(Color::Yellow),
                        ),
                    ]),
                ];
                let table = Table::new(
                    rows,
                    &[Constraint::Percentage(50), Constraint::Percentage(50)],
                )
                .block(stats_block);

                // let paragraph = Paragraph::new(Span::raw("Item used"))
                //     .block(stats_block);
                f.render_widget(table, chunks[2]);
            })
            .unwrap();
    }

    pub fn encounter_user_options(
        &mut self,
        enc_opt: HashMap<EncOpt, String>,
        gui_args: &mut GuiArgs,
    ) {
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

                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(
                        [
                            Constraint::Percentage(70),
                            Constraint::Percentage(10),
                            Constraint::Percentage(20),
                        ]
                        .as_ref(),
                    )
                    .split(game_chunks[1]);

                //------

                let chunk_w = chunks[0].inner(Margin::new(0, 0)).width;
                let fit = chunk_w as i8 - 60;
                let ascii = gui_args.ascii.unwrap();
                let mut ascii_str = Vec::new();
                ascii_str.push(Span::styled("", Style::default().white()));
                for i in 0..(ascii.len() / 60) {
                    let line = &ascii[i * 60..(i * 60 + 60)];
                    let crop_line = if fit < 0 {
                        line[(-fit * 2) as usize..].to_string()
                    } else {
                        line.to_string()
                    };
                    ascii_str.push(Span::styled(crop_line, Style::default().white()));
                }
                let texts: Text = ascii_str.into_iter().collect();

                let paragraph_block = Block::default()
                    .title("Encounter")
                    .borders(Borders::ALL)
                    .style(Style::default().bg(Color::Black));
                let paragraph = Paragraph::new(texts).block(paragraph_block);

                f.render_widget(paragraph, chunks[0]);

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
                self.enc_opt = (vec1.clone(), vec2.clone());
                self.cursor_bounds = vec![
                    vec1.iter().filter(|x| x.0 != EncOpt::Null).count(),
                    vec2.iter().filter(|x| x.0 != EncOpt::Null).count(),
                ];
                let rows: Vec<Row> = enc_opts
                    .iter()
                    .enumerate()
                    .map(|(j, row)| {
                        let cells: Vec<Cell> = row
                            .iter()
                            .enumerate()
                            .map(|(i, cell)| {
                                if i == self.cursor_pos.0 && j == self.cursor_pos.1 {
                                    Cell::from(Span::styled(
                                        cell.1.clone(),
                                        ratatui::style::Style::default()
                                            .fg(ratatui::style::Color::Yellow),
                                    ))
                                } else {
                                    Cell::from(cell.1.clone())
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
                .block(options_block);

                f.render_widget(table, chunks[1]);

                //-----

                let stats_block = Block::default()
                    .title("")
                    .borders(Borders::ALL)
                    .style(Style::default().bg(Color::Black));

                let rows = vec![
                    Row::new(vec![
                        Span::styled("Health: ", Style::default().fg(Color::White)),
                        Span::styled(
                            gui_args.player.health.to_string(),
                            Style::default().fg(Color::Yellow),
                        ),
                    ]),
                    Row::new(vec![
                        Span::styled("Attack: ", Style::default().fg(Color::White)),
                        Span::styled(
                            gui_args.player.attack.to_string(),
                            Style::default().fg(Color::Yellow),
                        ),
                    ]),
                    Row::new(vec![
                        Span::styled("Defence: ", Style::default().fg(Color::White)),
                        Span::styled(
                            gui_args.player.defence.to_string(),
                            Style::default().fg(Color::Yellow),
                        ),
                    ]),
                    Row::new(vec![
                        Span::styled("Damage: ", Style::default().fg(Color::White)),
                        Span::styled(
                            gui_args.player.damage.to_string(),
                            Style::default().fg(Color::Yellow),
                        ),
                    ]),
                ];
                let table = Table::new(
                    rows,
                    &[Constraint::Percentage(50), Constraint::Percentage(50)],
                )
                .block(stats_block);

                f.render_widget(table, chunks[2]);
            })
            .unwrap();
    }

    pub fn encounter_pick_item(&mut self, gui_args: &mut GuiArgs) {
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
                    .constraints([Constraint::Percentage(60), Constraint::Percentage(40)].as_ref())
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

                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(
                        [
                            Constraint::Percentage(10),
                            Constraint::Percentage(70),
                            Constraint::Percentage(20),
                        ]
                        .as_ref(),
                    )
                    .split(game_chunks[1]);

                //------
                let paragraph_block = Block::default()
                    .title("Encounter")
                    .borders(Borders::ALL)
                    .style(Style::default().bg(Color::Black));
                let paragraph =
                    Paragraph::new(Span::styled("What item to use?", Style::default().white()))
                        .block(paragraph_block);

                f.render_widget(paragraph, chunks[0]);

                //------

                let options_block = Block::default()
                    .title("")
                    .borders(Borders::ALL)
                    .style(Style::default().bg(Color::Black));

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
                let inv_table: Vec<Vec<(usize, Item)>> =
                    vec![col1.clone(), col2.clone(), col3.clone()];
                self.inv_opt = (col1, col2, col3);
                let mut cur_bounds = Vec::new();
                for i in 0..inv_table[0].len() {
                    if inv_table[0][i].1.itype == Items::Null {
                        break;
                    }
                    cur_bounds.push(1);
                }
                for i in 0..inv_table[1].len() {
                    if inv_table[1][i].1.itype == Items::Null {
                        break;
                    }
                    cur_bounds[i] += 1;
                }
                for i in 0..inv_table[2].len() {
                    if inv_table[2][i].1.itype == Items::Null {
                        break;
                    }
                    cur_bounds[i] += 1;
                }
                self.cursor_bounds = cur_bounds;

                //xx
                let rows: Vec<Row> = (0..12)
                    .map(|i| {
                        let cells: Vec<Cell> = inv_table
                            .iter()
                            .enumerate()
                            .map(|(j, col)| {
                                if i == self.cursor_pos.1 && j == self.cursor_pos.0 {
                                    Cell::from(Span::styled(
                                        col[i].1.sname.clone(),
                                        ratatui::style::Style::default()
                                            .fg(ratatui::style::Color::Yellow),
                                    ))
                                } else {
                                    Cell::from(col[i].1.sname.clone())
                                }
                            })
                            .collect();
                        Row::new(cells)
                    })
                    .collect();
                //dd
                let table = Table::new(
                    rows,
                    &[
                        Constraint::Percentage(33),
                        Constraint::Percentage(33),
                        Constraint::Percentage(33),
                    ],
                )
                .block(options_block);

                f.render_widget(table, chunks[1]);

                //-----

                let stats_block = Block::default()
                    .title("")
                    .borders(Borders::ALL)
                    .style(Style::default().bg(Color::Black));

                let rows = vec![
                    Row::new(vec![
                        Span::styled("Health: ", Style::default().fg(Color::White)),
                        Span::styled(
                            gui_args.player.health.to_string(),
                            Style::default().fg(Color::Yellow),
                        ),
                    ]),
                    Row::new(vec![
                        Span::styled("Attack: ", Style::default().fg(Color::White)),
                        Span::styled(
                            gui_args.player.attack.to_string(),
                            Style::default().fg(Color::Yellow),
                        ),
                    ]),
                    Row::new(vec![
                        Span::styled("Defence: ", Style::default().fg(Color::White)),
                        Span::styled(
                            gui_args.player.defence.to_string(),
                            Style::default().fg(Color::Yellow),
                        ),
                    ]),
                    Row::new(vec![
                        Span::styled("Damage: ", Style::default().fg(Color::White)),
                        Span::styled(
                            gui_args.player.damage.to_string(),
                            Style::default().fg(Color::Yellow),
                        ),
                    ]),
                ];
                let table = Table::new(
                    rows,
                    &[Constraint::Percentage(50), Constraint::Percentage(50)],
                )
                .block(stats_block);
                f.render_widget(table, chunks[2]);
            })
            .unwrap();
    }
}
