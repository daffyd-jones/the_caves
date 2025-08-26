use std::env::current_dir;

//environment_interactions.rs
use crate::enums::Items;
use crate::gui::GUI;
use crate::gui_utils::{draw_map, wrap_text, GuiArgs};
use ratatui::symbols::Marker;
use ratatui::widgets::{Axis, Block, Borders, Chart, Dataset, GraphType, Padding, Paragraph};
use ratatui::layout::{Layout, Constraint, Direction, Margin};
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::{Span, Text};
use ratatui::widgets::Row;
use ratatui::widgets::Table;
use ratatui::widgets::Cell;
use ratatui::prelude::Rect;
use ratatui::layout::Flex;
use ratatui::widgets::Clear;


impl GUI {

    pub fn guild_records_draw(&mut self, save_str: String, savelist: Vec<String>, gui_args: &mut GuiArgs) {
        self.terminal.draw(|f| {
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

            if in_h != self.viewport_dim.1 && in_w != self.viewport_dim.0 {
                // map.set_viewport(in_h, in_w);
                self.viewport_dim = (in_w, in_h);
            }
            let paragraph = draw_map(gui_args, self.ani_cnt);
            f.render_widget(paragraph, inner_area);

            let info_block = Block::default()
                .title("")
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Black));
            f.render_widget(info_block, game_chunks[1]);

            let a = f.area();
            let b = Block::bordered()
                .title("")
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

            let paragraph_block = Block::default()
                .title("Cauldron")
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Black));
            let para_area = Rect {
                x: h_area.x + 2,
                y: h_area.y + 2,
                width: h_area.width / 2 - 3,
                height: (h_area.height / 3) * 2,
            };
            let cauldron_msg = if product_list.is_empty() {
                "It looks like you dont have enoug of anythin to brew anything."
            } else {
                "The following things can be made with what you have"
            };
            let para = Paragraph::new(Span::styled(cauldron_msg, Style::default().white()))
                .block(paragraph_block)
                .wrap(ratatui::widgets::Wrap { trim: true });
            f.render_widget(para, para_area);
            
            let table_block = Block::default()
                .title("")
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Black));

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
            self.cursor_bounds = vec![product_str.len()];
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
            let table = Table::new(rows, &[Constraint::Percentage(33), Constraint::Percentage(33), Constraint::Percentage(33)])
                .block(table_block);
            let opts_area = Rect {
                x: h_area.x + 2,
                y: h_area.y + 2 + (h_area.height / 3) * 2,
                width: h_area.width / 2 - 3,
                height: (h_area.height / 3) - 4,
            };
            f.render_widget(table, opts_area);

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


        }).unwrap();
    }

    pub fn herbalist_draw(&mut self, herbalist_msg: String, plants: Option<Vec<String>>, gui_args: &mut GuiArgs) {
        self.terminal.draw(|f| {
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

            if in_h != self.viewport_dim.1 && in_w != self.viewport_dim.0 {
                self.viewport_dim = (in_w, in_h);
            }
            let paragraph = draw_map(gui_args, self.ani_cnt);
            f.render_widget(paragraph, inner_area);


            // let normal_info = Layout::default()
            // .direction(Direction::Vertical)
            // .constraints(
            //     [
            //         Constraint::Percentage(70),
            //         Constraint::Percentage(30)
            //     ].as_ref()
            // )
            // .split(game_chunks[1]);
            let info_block = Block::default()
                .title("")
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Black));
            f.render_widget(info_block, game_chunks[1]);

            let a = f.area();
            let b = Block::bordered()
                .title("")
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

            let paragraph_block = Block::default()
                .title("Herbalist")
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Black));

            // let paragraph = Paragraph::new()
            //     .block(paragraph_block)
            //     .style(Style::default().bg(Color::Black))
            //     .wrap(ratatui::widgets::Wrap { trim: true });
            
            let table_block = Block::default()
                .title("Options")
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Black));

            let (str, opts) = if let Some(plnts) = plants {
                (herbalist_msg, plnts)
            } else {
                let tmsg = herbalist_msg.split("#").collect::<Vec<&str>>();
                let op_vec: Vec<String> = tmsg[1..].iter().map(|s| s.to_string()).collect();
                (tmsg[0].to_string(), op_vec)
            };
            
            let para = Paragraph::new(Span::styled(str, Style::default().white()))
                .block(paragraph_block)
                .wrap(ratatui::widgets::Wrap { trim: true });
            let para_area = Rect {
                x: h_area.x + 2,
                y: h_area.y + 2,
                width: h_area.width / 2 - 3,
                height: (h_area.height / 3) * 2,
            };
            f.render_widget(para, para_area);
          
            let opts_area = Rect {
                x: h_area.x + 2,
                y: h_area.y + 2 + (h_area.height / 3) * 2,
                width: h_area.width / 2 - 3,
                height: (h_area.height / 3) - 4,
            };

            let mut cur_bounds = Vec::new();
            for _ in 0..opts.len() {
                cur_bounds.push(1);
            }
            self.cursor_bounds = cur_bounds;

            let rows: Vec<Row> = opts.iter().enumerate().map(|(j, cell)| {
                    let row = if j == self.cursor_pos.1 {
                        vec![Cell::from(Span::styled(cell, ratatui::style::Style::default().fg(ratatui::style::Color::Yellow)))]
                    } else {
                        vec![Cell::from(Span::styled(cell, Style::default().white()))]
                    };
                Row::new(row)
            }).collect();
            let table = Table::new(rows, &[Constraint::Percentage(100)])
                .block(table_block);
            f.render_widget(table, opts_area);

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
            
            // f.render_widget(table, normal_info[1]);
            // f.render_widget(save_text, normal_info[0]);
        }).unwrap();
    }

    pub fn clinic_draw(&mut self, gui_args: &mut GuiArgs) {
        self.terminal.draw(|f| {
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
            self.cursor_bounds = vec![2];
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

    pub fn locked_draw(&mut self, result: String, gui_args: &mut GuiArgs) {
        self.terminal.draw(|f| {
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
                .title("Locked Door")
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Black));
            let table_block = Block::default()
                .title("")
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Black));
            
            let save_text = Paragraph::new(Span::styled(result, Style::default().white()))
                .block(paragraph_block)
                .wrap(ratatui::widgets::Wrap { trim: true });
            
            let vec1 = vec![""];
            let opts = [vec1.clone()];
            self.cursor_bounds = vec![1];
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
            self.cursor_bounds = vec![1];
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

    pub fn guild_post_draw(&mut self, post_strings: Vec<String>, task_posts: Vec<String>, already_tasked: bool,  gui_args: &mut GuiArgs) {
        self.terminal.draw(|f| {
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

            if in_h != self.viewport_dim.1 && in_w != self.viewport_dim.0 {
                // map.set_viewport(in_h, in_w);
                self.viewport_dim = (in_w, in_h);
            }
            let paragraph = draw_map(gui_args, self.ani_cnt);
            f.render_widget(paragraph, inner_area);

            let info_block = Block::default()
                .title("")
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Black));
            f.render_widget(info_block, game_chunks[1]);

            let a = f.area();
            let b = Block::bordered()
                .title("")
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


            let paragraph_block = Block::default()
                .title("Guild Posting Board")
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Black));
           
            let rows: Vec<Row> = ["Jobs", "Discoveries"].iter().enumerate().map(|(j, cell)| {
                let row = if self.menu_lvl == 0 {
                    if j == self.cursor_pos.1 {
                        Cell::from(Span::styled(*cell, ratatui::style::Style::default().fg(ratatui::style::Color::Yellow)))
                    } else {
                        Cell::from(Span::styled(*cell, ratatui::style::Style::default().fg(ratatui::style::Color::White)))
                    }
                } else {
                    if j == self.cursor_hold.1 {
                        Cell::from(Span::styled(*cell, ratatui::style::Style::default().fg(ratatui::style::Color::Yellow)))
                    } else {
                        Cell::from(Span::styled(*cell, ratatui::style::Style::default().fg(ratatui::style::Color::White)))
                    }
                };
                Row::new(vec![row])
            }).collect();

            let p_table = Table::new(rows, &[Constraint::Percentage(100)])
                .block(paragraph_block);
            let para_area = Rect {
                x: h_area.x + 2,
                y: h_area.y + 2,
                width: h_area.width / 5,
                height: h_area.height - 4,
            };
            f.render_widget(p_table, para_area);
           
            let table_area = Rect {
                x: h_area.x + h_area.width / 5 + 3,
                y: h_area.y + 2,
                width: (h_area.width / 5)*4 - 4,
                height: h_area.height - 4,
            };
            let table_block = Block::default()
                .title("")
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Black));
            
            let opts = match (self.menu_lvl, self.cursor_pos.1, self.cursor_hold.1) {
                (0, 0, _) => task_posts,
                (0, 1, _) => post_strings,
                (1, _, 0) => task_posts,
                (1, _, 1) => post_strings,
                _ => vec!["".to_string()]
            };
            self.cursor_bounds = if self.menu_lvl == 1 {
                vec![1; opts.len()]
            } else {
                vec![1; 2]
            };
            let rows: Vec<Row> = opts.iter().enumerate().map(|(j, cell)| {
                let wrapped_text = wrap_text(cell, table_block.inner(table_area).width.into());
                if self.menu_lvl == 1 && j == self.cursor_pos.1 {
                    Row::new(vec![Cell::from(wrapped_text.alignment(ratatui::layout::Alignment::Center))])
                        .style(Style::default().fg(Color::Yellow))
                        .height(10)
                } else {
                    Row::new(vec![Cell::from(wrapped_text.alignment(ratatui::layout::Alignment::Center))])
                        .style(Style::default().fg(Color::White))
                        .height(10)
                }
            }).collect();
            let table = Table::new(rows, &[Constraint::Percentage(100)])
                .block(table_block);
            f.render_widget(table, table_area);

            if already_tasked {
                let popup_area = Rect {
                    x: h_area.x + h_area.width / 3,
                    y: h_area.y + h_area.height / 3,
                    width: h_area.width / 3 ,
                    height: h_area.height / 4,
                };
                let popup_block = Block::default()
                    .title("")
                    .borders(Borders::ALL)
                    .padding(Padding::new(0, 0, 2, 0))
                    .style(Style::default().bg(Color::Black));
                let popup_msg = Paragraph::new(Text::raw("It looks like you already have an active board task. You can only take on one board task at a time."))
                    .style(Style::new().white())
                    .block(popup_block)
                    .alignment(ratatui::layout::Alignment::Center)
                    .wrap(ratatui::widgets::Wrap { trim: true });
                f.render_widget(Clear, popup_area);
                f.render_widget(popup_msg, popup_area);
            }
        }).unwrap();
    }

    pub fn church_post_draw(&mut self, post_strings: Vec<(String, String, (f64, f64))>, gui_args: &mut GuiArgs) {
        self.terminal.draw(|f| {
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

            if in_h != self.viewport_dim.1 && in_w != self.viewport_dim.0 {
                // map.set_viewport(in_h, in_w);
                self.viewport_dim = (in_w, in_h);
            }
            let paragraph = draw_map(gui_args, self.ani_cnt);
            // let paragraph = draw_map(map.clone(), player.clone(), portals.clone(), enemies.clone(), items.clone(), npcs.clone(), litems.clone(), env_inter.clone(), self.ani_cnt);
            f.render_widget(paragraph, inner_area);


            // let normal_info = Layout::default()
            // .direction(Direction::Vertical)
            // .constraints(
            //     [
            //         Constraint::Percentage(70),
            //         Constraint::Percentage(30)
            //     ].as_ref()
            // )
            // .split(game_chunks[1]);
            
            let info_block = Block::default()
                .title("")
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Black));
            f.render_widget(info_block, game_chunks[1]);

            let a = f.area();
            let b = Block::bordered()
                .title("")
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



            
            let opt_block = Block::default()
                .title("")
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Black));

            let opt_area = Rect {
                x: h_area.x + 2,
                y: h_area.y + (h_area.height/ 4)*3,
                width: (h_area.width / 4),
                height: h_area.height/ 4,
            };

            self.cursor_bounds = if post_strings.len() % 2 == 0
                && (post_strings.len() / 2) < (opt_area.height - 2).into() {
                vec![2; post_strings.len()/2]
            } else if ((post_strings.len() + 1) / 2) < (opt_area.height - 2).into() {
                let mut temp = vec![2; post_strings.len() / 2];
                temp.push(1);
                temp
            } else {
                vec![2; (opt_area.height - 2) as usize]
            }; 
            
            let mut snames = Vec::new();
            for i in (0..post_strings.len()).step_by(2) {
                let st2 = if i + 1 == post_strings.len() {
                    "".to_string()
                } else {
                    post_strings[i + 1].0.clone()
                };
                snames.push(vec![post_strings[i].0.clone(), st2]);
            }
            
            
            
            let rows: Vec<Row> = snames.iter().enumerate().map(|(j, row)| {
                let cells: Vec<Cell> = row.iter().enumerate().map(|(i, cell)| {
                    if i == self.cursor_pos.0 && j == self.cursor_pos.1 {
                        Cell::from(Span::styled(cell, ratatui::style::Style::default().fg(ratatui::style::Color::Yellow)))
                    } else {
                        Cell::from(Span::styled(cell, ratatui::style::Style::default().fg(ratatui::style::Color::White)))
                    }
                }).collect();
                Row::new(cells)
            }).collect();
            let table = Table::new(rows, &[Constraint::Percentage(50), Constraint::Percentage(50)])
                .block(opt_block);
            f.render_widget(table, opt_area);

            
            let idx = self.cursor_pos.1*2 + self.cursor_pos.0;
            let settle = post_strings[idx].1.clone();
            
            let paragraph_block = Block::default()
                .title("Church Posting Board")
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Black));

            let para_area = Rect {
                x: h_area.x + 2,
                y: h_area.y + 2,
                width: h_area.width / 4,
                height: (h_area.height / 4)*3 - 2,
            };
            let s_info = Paragraph::new(Text::raw(settle).style(Style::new().white()))
                .block(paragraph_block)
                .wrap(ratatui::widgets::Wrap { trim: true });
            f.render_widget(s_info, para_area);
           
            let mut points = Vec::new();
            let point = {
                let mut p = (0.0, 0.0);
                for i in 0..post_strings.len() {
                    if i == idx {
                        p = post_strings[i].2;
                    } else {
                        points.push(post_strings[i].2);
                    }
                }
                [p]
            };
            points.push((0.0, 0.0));

            let map_block = Block::default()
                .title("Map")
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Black));
            
            let map_area = Rect {
                x: h_area.x + h_area.width / 4 + 4,
                y: h_area.y + 2,
                width: (h_area.width / 4)*3 - 6,
                height: h_area.height - 4,
            };
            let datasets = vec![
                Dataset::default()
                    .name("settlements")
                    .marker(Marker::HalfBlock)
                    .graph_type(GraphType::Scatter)
                    .style(Style::default().fg(Color::Gray))
                    .data(&points),
                Dataset::default()
                    .marker(Marker::HalfBlock)
                    .graph_type(GraphType::Scatter)
                    .style(Style::default().fg(Color::Yellow))
                    .data(&point),
                Dataset::default()
                    .marker(Marker::Dot)
                    .graph_type(GraphType::Scatter)
                    .style(Style::default().fg(Color::DarkGray))
                    .data(&[
                        (500.0, 500.0),
                        (1000.0, 1000.0),
                        (1500.0, 1500.0),
                        (2000.0, 2000.0),
                        // (2500.0, 2500.0),
                        // (3000.0, 3000.0),
                        // (3500.0, 3500.0),
                        (-500.0, 500.0),
                        (-1000.0, 1000.0),
                        (-1500.0, 1500.0),
                        (-2000.0, 2000.0),
                        // (-2500.0, 2500.0),
                        // (-3000.0, 3000.0),
                        // (-3500.0, 3500.0),
                        (500.0, -500.0),
                        (1000.0, -1000.0),
                        (1500.0, -1500.0),
                        (2000.0, -2000.0),
                        // (2500.0, -2500.0),
                        // (3000.0, -3000.0),
                        // (3500.0, -3500.0),
                        (-500.0, -500.0),
                        (-1000.0, -1000.0),
                        (-1500.0, -1500.0),
                        (-2000.0, -2000.0),
                        // (-2500.0, -2500.0),
                        // (-3000.0, -3000.0),
                        // (-3500.0, -3500.0),
                        (0.0, -500.0),
                        (0.0, -1000.0),
                        (0.0, -1500.0),
                        (0.0, -2500.0),
                        (0.0, -2000.0),
                        // (0.0, -3000.0),
                        // (0.0, -3500.0),
                        (0.0, 500.0),
                        (0.0, 1000.0),
                        (0.0, 1500.0),
                        (0.0, 2000.0),
                        // (0.0, 2500.0),
                        // (0.0, 3000.0),
                        // (0.0, 3500.0),
                        (-500.0, 0.0),
                        (-1000.0, 0.0),
                        (-1500.0, 0.0),
                        (-2000.0, 0.0),
                        // (-2500.0, 0.0),
                        // (-3000.0, 0.0),
                        // (-3500.0, 0.0),
                        (500.0, 0.0),
                        (1000.0, 0.0),
                        (1500.0, 0.0),
                        (2000.0, 0.0),
                        // (2500.0, 0.0),
                        // (3000.0, 0.0),
                        // (3500.0, 0.0),
                    ]),
            ];

            let x_axis = Axis::default()
                .style(Style::default().white())
                .bounds([-2000.0, 2000.0]);
                // .bounds([-3000.0, 3000.0]);

            let y_axis = Axis::default()
                .style(Style::default().white())
                .bounds([-2000.0, 2000.0]);
                // .bounds([-3000.0, 3000.0]);

            let map = Chart::new(datasets)
                .block(map_block)
                .style(Style::default().bg(Color::Black))
                .x_axis(x_axis)
                .y_axis(y_axis);
            f.render_widget(map, map_area);
        }).unwrap();
    }
}
