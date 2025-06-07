//environment_interactions.rs
use crate::enums::Items;
use crate::gui::GUI;
use crate::gui_utils::{GuiArgs, draw_map};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::layout::{Layout, Constraint, Direction, Margin};
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::Span;
use ratatui::widgets::Row;
use ratatui::widgets::Table;
use ratatui::widgets::Cell;



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
                .title("Herbalist")
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Black));
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
            
            let save_text = Paragraph::new(Span::styled(str, Style::default().white()))
                .block(paragraph_block)
                .wrap(ratatui::widgets::Wrap { trim: true });
          
            // let opts = [opt];
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
            f.render_widget(table, normal_info[1]);
            f.render_widget(save_text, normal_info[0]);
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

    pub fn church_post_draw(&mut self, post_strings: Vec<(String, String, Vec<String>, Vec<String>)>, gui_args: &mut GuiArgs) {
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
                .title("Church Posting Board")
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Black));
            let table_block = Block::default()
                .title("Done")
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Black));

            let mut snames = Vec::new();
            for i in (0..post_strings.len()).step_by(2) {
                let st2 = if i + 1 == post_strings.len() {
                    "".to_string()
                } else {
                    post_strings[i + 1].1.clone()
                };
                snames.push(vec![post_strings[i].1.clone(), st2]);
            }
            
            let idx = self.cursor_pos.0*2 + self.cursor_pos.1;
            let settle = post_strings[idx].clone();
            
            let mut rows = vec![
                Row::new(vec![
                    Span::styled(settle.0, Style::default().fg(Color::White)),
                    // Span::styled(gui_args.player.x.to_string(), Style::default().fg(Color::Yellow)),
                ]),
                Row::new(vec![
                    Span::styled(settle.1, Style::default().fg(Color::White)),
                    // Span::styled(gui_args.player.x.to_string(), Style::default().fg(Color::Yellow)),
                ]),
                Row::new(vec![
                    Span::styled("------".to_string(), Style::default().fg(Color::White)),
                    // Span::styled(gui_args.player.x.to_string(), Style::default().fg(Color::Yellow)),
                ]),
                Row::new(vec![
                    Span::styled("".to_string(), Style::default().fg(Color::White)),
                    // Span::styled(gui_args.player.x.to_string(), Style::default().fg(Color::Yellow)),
                ]),
                Row::new(vec![
                    Span::styled("Shops:".to_string(), Style::default().fg(Color::White)),
                    // Span::styled(gui_args.player.x.to_string(), Style::default().fg(Color::Yellow)),
                ]),
            ];
            for i in settle.2 {
                rows.push(
                    Row::new(vec![
                        Span::styled(i, Style::default().fg(Color::White)),
                        // Span::styled(gui_args.player.x.to_string(), Style::default().fg(Color::Yellow)),
                    ]),
                );
            }

            rows.push(
                Row::new(vec![
                    Span::styled("".to_string(), Style::default().fg(Color::White)),
                    // Span::styled(gui_args.player.x.to_string(), Style::default().fg(Color::Yellow)),
                ]),
            );

            rows.push(
                Row::new(vec![
                    Span::styled("Residents:".to_string(), Style::default().fg(Color::White)),
                    // Span::styled(gui_args.player.x.to_string(), Style::default().fg(Color::Yellow)),
                ]),
            );

            for i in settle.3 {
                rows.push(
                    Row::new(vec![
                        Span::styled(i, Style::default().fg(Color::White)),
                        // Span::styled(gui_args.player.x.to_string(), Style::default().fg(Color::Yellow)),
                    ]),
                );
            }

            let p_table = Table::new(rows, &[Constraint::Percentage(100)])
                .block(paragraph_block);
            
            // let opts = [snames];
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
                .block(table_block);
            f.render_widget(p_table, normal_info[0]);
            f.render_widget(table, normal_info[1]);
        }).unwrap();
    }
}
