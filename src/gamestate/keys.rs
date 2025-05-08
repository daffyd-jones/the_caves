//keys

use crate::enums::{EncMode, FightSteps, GUIMode, GameMode, InterSteps};
use crate::gamestate::GameState;
use ratatui::crossterm::event::KeyCode;

impl GameState {
    pub fn play_key(&mut self, code: KeyCode) -> bool {
        match code {
            KeyCode::Up if *self.pressed_keys.get(&KeyCode::Up).unwrap_or(&false) => {
                if self.collision("UP") {
                } else {
                    if self.player.y - 1 <= self.map.viewport_y + (self.map.viewport_height / 7) {
                        self.shift_enemies("UP");
                        self.shift_items("UP");
                        self.shift_npcs("UP");
                        self.shift_env_inters("UP");
                        self.shift_portals("UP");
                        self.map.shift("UP");
                        self.dist_fo.1 += 1;
                        self.gui.set_comp_head((
                            self.comp_head.0 - self.dist_fo.0 * -1,
                            self.comp_head.1 - self.dist_fo.1 * -1,
                        ));
                    } else {
                        self.player.y -= 1;
                    }
                }
            }
            KeyCode::Down if *self.pressed_keys.get(&KeyCode::Down).unwrap_or(&false) => {
                if self.collision("DN") {
                } else {
                    if self.player.y + 1
                        >= (self.map.viewport_height + self.map.viewport_y)
                            - (self.map.viewport_height / 7)
                    {
                        self.shift_enemies("DN");
                        self.shift_items("DN");
                        self.shift_npcs("DN");
                        self.shift_env_inters("DN");
                        self.shift_portals("DN");
                        self.map.shift("DN");
                        self.dist_fo.1 -= 1;
                        self.gui.set_comp_head((
                            self.comp_head.0 - self.dist_fo.0 * -1,
                            self.comp_head.1 - self.dist_fo.1 * -1,
                        ));
                    } else {
                        self.player.y += 1;
                    }
                }
            }
            KeyCode::Left if *self.pressed_keys.get(&KeyCode::Left).unwrap_or(&false) => {
                if self.collision("LF") {
                } else {
                    if self.player.x - 1 <= self.map.viewport_x + (self.map.viewport_width / 7) {
                        self.shift_enemies("LF");
                        self.shift_items("LF");
                        self.shift_npcs("LF");
                        self.shift_env_inters("LF");
                        self.shift_portals("LF");
                        self.map.shift("LF");
                        self.dist_fo.0 += 1;
                        self.gui.set_comp_head((
                            self.comp_head.0 - self.dist_fo.0 * -1,
                            self.comp_head.1 - self.dist_fo.1 * -1,
                        ));
                    } else {
                        self.player.x -= 1;
                    }
                }
            }
            KeyCode::Right if *self.pressed_keys.get(&KeyCode::Right).unwrap_or(&false) => {
                if self.collision("RT") {
                } else {
                    if self.player.x + 1
                        >= (self.map.viewport_width + self.map.viewport_x)
                            - (self.map.viewport_width / 7)
                    {
                        self.shift_enemies("RT");
                        self.shift_items("RT");
                        self.shift_npcs("RT");
                        self.shift_env_inters("RT");
                        self.shift_portals("RT");
                        self.map.shift("RT");
                        self.dist_fo.0 -= 1;
                        self.gui.set_comp_head((
                            self.comp_head.0 - self.dist_fo.0 * -1,
                            self.comp_head.1 - self.dist_fo.1 * -1,
                        ));
                    } else {
                        self.player.x += 1;
                    }
                }
            }
            KeyCode::Char('h') => self.gui.toggle_help(),
            KeyCode::Char('p') => self.gui.set_info_mode(GUIMode::Bug),
            KeyCode::Char('o') => self.gui.set_info_mode(GUIMode::Normal),
            KeyCode::Char('q') => self.gui.set_info_mode(GUIMode::Normal),
            KeyCode::Char('w') => {
                self.gui.set_info_mode(GUIMode::Map);
                //self.gui.set_comp_head(self.comp_head);
                let comp_names = self.sort_comp_list();
                self.gui.set_comp_list(comp_names);
                self.gui.set_comp_head((
                    self.comp_head.0 - -self.dist_fo.0,
                    self.comp_head.1 - -self.dist_fo.1,
                ));
            }
            KeyCode::Char('e') => {
                self.gui.set_info_mode(GUIMode::Inventory);
                self.gui.set_inventory(self.player.get_inventory());
                self.gui.reset_cursor();
            }
            KeyCode::Char('r') => {
                self.gui.set_info_mode(GUIMode::Notes);
                self.gui.set_notes(self.notebook.get_notes());
            }
            KeyCode::Char('a') => self.gui.move_cursor("LF"),
            KeyCode::Char('s') => self.gui.move_cursor("UP"),
            KeyCode::Char('d') => self.gui.move_cursor("DN"),
            KeyCode::Char('f') => self.gui.move_cursor("RT"),
            KeyCode::Char(' ') => self.start_interact(),
            KeyCode::Enter => {
                let gmode = self.gui.get_mode();
                match gmode {
                    GUIMode::Normal => {}
                    GUIMode::Inventory => {
                        //put use_opts here, use_inv_item in opts
                        self.use_inv_item();
                    }
                    GUIMode::Map => {
                        self.set_comp();
                        let comp_names = self.sort_comp_list();
                        self.gui.set_comp_list(comp_names);
                        self.gui.set_comp_head((
                            self.comp_head.0 - -self.dist_fo.0,
                            self.comp_head.1 - -self.dist_fo.1,
                        ));
                    }
                    GUIMode::Notes => {
                        self.gui.menu_lvl("DN");
                    }
                    _ => {}
                }
            }
            KeyCode::Backspace => {
                let gmode = self.gui.get_mode();
                match gmode {
                    GUIMode::Normal => {}
                    GUIMode::Inventory => {}
                    GUIMode::Map => {}
                    GUIMode::Notes => {
                        self.gui.menu_lvl("UP");
                    }
                    _ => {}
                }
            }
            KeyCode::Esc => return false,
            _ => {}
        }
        true
    }

    pub fn key(&mut self, code: KeyCode) -> bool {
        match code {
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
            KeyCode::Char('p') => self.gui.set_info_mode(GUIMode::Bug),
            KeyCode::Char('o') => self.gui.set_info_mode(GUIMode::Normal),
            KeyCode::Char('i') => {
                self.gui.set_info_mode(GUIMode::Normal);
                self.game_mode = GameMode::Play;
            }
            KeyCode::Char('a') => self.gui.move_cursor("LF"),
            KeyCode::Char('s') => self.gui.move_cursor("UP"),
            KeyCode::Char('d') => self.gui.move_cursor("DN"),
            KeyCode::Char('f') => self.gui.move_cursor("RT"),
            _ => {}
        }
        true
    }

    pub fn enc_key(&mut self, code: KeyCode) -> bool {
        match code {
            KeyCode::Enter => {
                match self.game_mode {
                    GameMode::Fight(FightSteps::Open) => {
                        let cursor = self.gui.get_cursor();
                        match cursor.0 {
                            0 => self.enc_mode = EncMode::Auto,
                            1 => self.enc_mode = EncMode::Manual,
                            2 => self.enc_mode = EncMode::Quick,
                            _ => {}
                        }
                    }
                    GameMode::Fight(FightSteps::Enemy) => {}
                    GameMode::Fight(FightSteps::Player) => {
                        let opt = self.gui.get_enc_opt();
                        self.enc = opt.0;
                        self.enc_option();
                        // match opt.0 {
                        //     _ => ,
                        //     // _ => {},
                        // }
                    }
                    GameMode::Fight(FightSteps::Message) => {}
                    _ => {}
                }
                false
            }
            _ => self.key(code),
        }
    }

    pub fn comm_key(&mut self, code: KeyCode) -> bool {
        match code {
            KeyCode::Enter => {
                self.game_mode = GameMode::Play;
                self.gui.set_info_mode(GUIMode::Normal);
                false
            }
            _ => self.key(code),
        }
    }

    pub fn inter_key(&mut self, code: KeyCode) -> bool {
        match code {
            KeyCode::Enter => {
                match self.game_mode {
                    GameMode::Interact(InterSteps::AdjOpt) => {
                        self.select_adj();
                        self.game_mode = GameMode::Interact(InterSteps::IntOpt);
                    }
                    GameMode::Interact(InterSteps::IntOpt) => {
                        self.select_opt();
                        self.game_mode = GameMode::Interact(InterSteps::Feedback);
                    }
                    GameMode::Interact(InterSteps::Feedback) => {
                        self.game_mode = GameMode::Play;
                    }
                    _ => self.game_mode = GameMode::Play,
                }

                false
            }
            KeyCode::Esc => false,
            _ => self.key(code),
        }
    }
}
