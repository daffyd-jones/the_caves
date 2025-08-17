//npc_interactions
use crate::enemy::Enemy;
use crate::enums::{
    EncOpt, EnvInter, Equip, InterOpt, Interactable, ItemEffect, Items, NPCWrap, Plants,
};
use crate::gui::GUI;
use crate::gui_utils::{draw_map, wrap_text, GuiArgs};
use crate::item::Item;
use ratatui::layout::Flex;
use ratatui::layout::{Constraint, Direction, Layout, Margin};
use ratatui::prelude::Rect;
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::Line;
use ratatui::text::{Span, Text};
use ratatui::widgets::Cell;
use ratatui::widgets::Clear;
use ratatui::widgets::Row;
use ratatui::widgets::Table;
use ratatui::widgets::{Block, Borders, Paragraph};

impl GUI {
    pub fn npc_commdraw(&mut self, comms: String, gui_args: &mut GuiArgs) {
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

                let mut ascii_str = Vec::new();
                ascii_str.push(Span::styled(npc_str[1], Style::default().white()));
                // let padding = " ".repeat(((&normal_info[0].width - 60) / 2) as usize);
                let padding = " ".repeat((normal_info[0].width.saturating_sub(60) / 2) as usize);

                let ascii = gui_args.ascii.unwrap();

                for i in 0..(ascii.len() / 60) {
                    let line = &ascii[i * 60..(i * 60 + 60)];
                    let padded_line = format!("{}{}", padding, line);
                    ascii_str.push(Span::styled(padded_line, Style::default().white()));
                }

                let texts: Text = ascii_str.into_iter().collect();

                let npc = Paragraph::new(texts)
                    .block(paragraph_block)
                    .wrap(ratatui::widgets::Wrap { trim: false });
                let plyr = Paragraph::new(Span::raw("")).block(table_block);
                f.render_widget(npc, normal_info[0]);
                f.render_widget(plyr, normal_info[1]);
            })
            .unwrap();
    }

    pub fn npc_comm_draw(&mut self, comms: String, gui_args: &mut GuiArgs) {
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

                let npc_str: Vec<&str> = comms.split("#").collect();
                let name = npc_str[0];

                let info_block = Block::default()
                    .title("")
                    .borders(Borders::ALL)
                    .style(Style::default().bg(Color::Black));
                f.render_widget(info_block, game_chunks[1]);

                let a = f.area();
                let b = Block::bordered()
                    .title(name)
                    .style(Style::default().bg(Color::Black));
                let (xper, yper) = (80, 20);
                let harea = |a, xper, yper| {
                    let vertical =
                        Layout::vertical([Constraint::Percentage(yper)]).flex(Flex::Center);
                    let horizontal =
                        Layout::horizontal([Constraint::Percentage(xper)]).flex(Flex::Center);
                    let [area] = vertical.areas(a);
                    let [area] = horizontal.areas(a);
                    area
                };
                let h_area = harea(a, xper, yper);
                f.render_widget(Clear, h_area);
                f.render_widget(b, h_area);

                let paragraph = Paragraph::new(npc_str[1])
                    .block(Block::bordered())
                    .style(Style::default().bg(Color::Black))
                    .wrap(ratatui::widgets::Wrap { trim: true });
                let para_area = Rect {
                    x: h_area.x + 2,
                    y: h_area.y + 2,
                    width: h_area.width / 2 - 3,
                    height: (h_area.height / 3) * 2,
                };
                f.render_widget(paragraph, para_area);

                let opts = Paragraph::new(Span::raw(""))
                    .block(Block::bordered())
                    .style(Style::default().bg(Color::Black))
                    .wrap(ratatui::widgets::Wrap { trim: true });
                let opts_area = Rect {
                    x: h_area.x + 2,
                    y: h_area.y + 2 + (h_area.height / 3) * 2,
                    width: h_area.width / 2 - 3,
                    height: (h_area.height / 3) - 4,
                };
                f.render_widget(opts, opts_area);

                let table_area = Rect {
                    x: h_area.x + h_area.width / 2 + 2,
                    y: h_area.y + 2,
                    width: h_area.width / 2 - 4,
                    height: h_area.height - 4,
                };
                let mut ascii_str = Vec::new();
                let padding = " ".repeat((table_area.width.saturating_sub(60) / 2) as usize);
                let ascii = gui_args.ascii.unwrap();
                for i in 0..(ascii.len() / 60) {
                    let line = &ascii[i * 60..(i * 60 + 60)];
                    let padded_line = format!("{}{}", padding, line);
                    ascii_str.push(Span::styled(padded_line, Style::default().white()));
                }
                let f_ascii: Text = ascii_str.into_iter().collect();
                let plyr = Paragraph::new(f_ascii)
                    .block(Block::bordered())
                    .style(Style::default().bg(Color::Black));
                f.render_widget(plyr, table_area);
            })
            .unwrap();
    }

    pub fn npc_trade_type_draw(&mut self, comms: String, gui_args: &mut GuiArgs) {
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
                self.cursor_bounds = vec![3];
                let rows: Vec<Row> = opts
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
                        Constraint::Percentage(33),
                        Constraint::Percentage(33),
                        Constraint::Percentage(33),
                    ],
                )
                .block(table_block);

                let comm = npc_str[1];
                let npc = Paragraph::new(Span::styled(comm, Style::default().white()))
                    .block(paragraph_block)
                    .wrap(ratatui::widgets::Wrap { trim: true });
                //let plyr = Paragraph::new(Span::raw(""))
                //  .block(table_block);
                f.render_widget(npc, normal_info[0]);
                f.render_widget(table, normal_info[1]);
            })
            .unwrap();
    }

    pub fn npc_trade_draw(&mut self, titems: Vec<Item>, gui_args: &mut GuiArgs) {
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
                    .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
                    .split(game_chunks[1]);

                let norm_top = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
                    .split(normal_info[0]);

                let desc_block = Block::default()
                    .title(Span::styled(
                        "Inventory",
                        Style::default().fg(Color::DarkGray),
                    ))
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
                let rows: Vec<Row> = (0..25)
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
                let table = Table::new(
                    rows,
                    &[
                        Constraint::Percentage(33),
                        Constraint::Percentage(33),
                        Constraint::Percentage(33),
                    ],
                )
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
            })
            .unwrap();
    }

    pub fn npc_conv_draw(
        &mut self,
        name: String,
        text: String,
        opts_vec: Vec<String>,
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
                // let paragraph = draw_map(map.clone(), player.clone(), portals.clone(), enemies.clone(), items.clone(), npcs.clone(), litems.clone(), env_inter.clone(), self.ani_cnt);
                f.render_widget(paragraph, inner_area);

                // let normal_info = Layout::default()
                //     .direction(Direction::Vertical)
                //     .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
                //     .split(game_chunks[1]);

                // let paragraph_block = Block::default()
                //     .title(name)
                //     .borders(Borders::ALL)
                //     .style(Style::default().bg(Color::Black));
                // let table_block = Block::default()
                //     .title("")
                //     .borders(Borders::ALL)
                //     .style(Style::default().bg(Color::Black));

                // let table_inner = table_block.inner(normal_info[1]);
                // let table_width = table_inner.width;

                // let comm = npc_str[1];

                // let mut ascii_str = Vec::new();
                // ascii_str.push(Span::styled(npc_str[1], Style::default().white()));
                // let padding = " ".repeat(((&normal_info[0].width - 60) / 2) as usize);

                // let ascii = gui_args.ascii.unwrap();

                // for i in 0..(ascii.len() / 60) {
                //     let line = &ascii[i * 60..(i * 60 + 60)];
                //     let padded_line = format!("{}{}", padding, line);
                //     ascii_str.push(Span::styled(padded_line, Style::default().white()));
                // }

                // let texts: Text = ascii_str.into_iter().collect();

                let info_block = Block::default()
                    .title("")
                    .borders(Borders::ALL)
                    .style(Style::default().bg(Color::Black));
                f.render_widget(info_block, game_chunks[1]);

                let a = f.area();
                let b = Block::bordered()
                    .title(name)
                    .style(Style::default().bg(Color::Black));
                let (xper, yper) = (80, 20);
                let harea = |a, xper, yper| {
                    let vertical =
                        Layout::vertical([Constraint::Percentage(yper)]).flex(Flex::Center);
                    let horizontal =
                        Layout::horizontal([Constraint::Percentage(xper)]).flex(Flex::Center);
                    let [area] = vertical.areas(a);
                    let [area] = horizontal.areas(a);
                    area
                };
                let h_area = harea(a, xper, yper);
                f.render_widget(Clear, h_area);
                f.render_widget(b, h_area);

                let paragraph = Paragraph::new(text)
                    .block(Block::bordered())
                    .style(Style::default().bg(Color::Black))
                    .wrap(ratatui::widgets::Wrap { trim: true });
                let para_area = Rect {
                    x: h_area.x + 2,
                    y: h_area.y + 2,
                    width: h_area.width / 2 - 3,
                    height: (h_area.height / 3) * 2,
                };
                f.render_widget(paragraph, para_area);

                let table_area = Rect {
                    x: h_area.x + h_area.width / 2 + 2,
                    y: h_area.y + 2,
                    width: h_area.width / 2 - 4,
                    height: h_area.height - 4,
                };
                let mut ascii_str = Vec::new();
                let padding = " ".repeat((table_area.width.saturating_sub(60) / 2) as usize);
                let ascii = gui_args.ascii.unwrap();
                for i in 0..(ascii.len() / 60) {
                    let line = &ascii[i * 60..(i * 60 + 60)];
                    let padded_line = format!("{}{}", padding, line);
                    ascii_str.push(Span::styled(padded_line, Style::default().white()));
                }
                let f_ascii: Text = ascii_str.into_iter().collect();
                let plyr = Paragraph::new(f_ascii)
                    .block(Block::bordered())
                    .style(Style::default().bg(Color::Black));
                f.render_widget(plyr, table_area);

                let opts_area = Rect {
                    x: h_area.x + 2,
                    y: h_area.y + 2 + (h_area.height / 3) * 2,
                    width: h_area.width / 2 - 3,
                    height: (h_area.height / 3) - 4,
                };

                let table_block = Block::default()
                    .title("")
                    .borders(Borders::ALL)
                    .style(Style::default().bg(Color::Black));
                let table_inner = table_block.inner(opts_area);
                let table_width = table_inner.width;
                let rows: Vec<Row> = opts_vec
                    .iter()
                    .enumerate()
                    .map(|(j, cell)| {
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
                    })
                    .collect();
                let table = Table::new(rows, &[Constraint::Percentage(100)]).block(table_block);
                f.render_widget(table, opts_area);
            })
            .unwrap();
    }

    pub fn shop_convo_draw(
        &mut self,
        sname: String,
        dialogue: String,
        buy: bool,
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
                // let paragraph = draw_map(map.clone(), player.clone(), portals.clone(), enemies.clone(), items.clone(), npcs.clone(), litems.clone(), env_inter.clone(), self.ani_cnt);
                f.render_widget(paragraph, inner_area);

                let normal_info = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
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
                let (vec1, cursor_bounds) = if buy {
                    (vec!["Yes", "No"], vec![2])
                } else {
                    (vec!["Thanks"], vec![1])
                };
                let opts = [vec1.clone()];
                self.cursor_bounds = cursor_bounds;
                let rows: Vec<Row> = opts
                    .iter()
                    .enumerate()
                    .map(|(j, row)| {
                        let cells: Vec<Cell> = row
                            .iter()
                            .enumerate()
                            .map(|(i, cell)| {
                                if i == self.cursor_pos.0 && j == self.cursor_pos.1 {
                                    Cell::from(Span::styled(
                                        *cell,
                                        ratatui::style::Style::default()
                                            .fg(ratatui::style::Color::Yellow),
                                    ))
                                } else {
                                    Cell::from(*cell)
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
