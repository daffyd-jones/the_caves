//map_state
use crate::enemy::Enemy;
use crate::gamestate::GameState;
use crate::utils::COLLISION_CELLS;
use std::collections::HashMap;

impl GameState {
    pub fn collision(&mut self, dir: &str) -> bool {
        match dir {
            "UP" => {
                let map_coll =
                    COLLISION_CELLS.contains(&self.map.cells[self.player.y - 1][self.player.x]);
                let item_coll = self.items.contains_key(&(self.player.x, self.player.y - 1));
                map_coll || item_coll
            }
            "DN" => {
                let map_coll =
                    COLLISION_CELLS.contains(&self.map.cells[self.player.y + 1][self.player.x]);
                let item_coll = self.items.contains_key(&(self.player.x, self.player.y + 1));
                map_coll || item_coll
            }
            "LF" => {
                let map_coll =
                    COLLISION_CELLS.contains(&self.map.cells[self.player.y][self.player.x - 1]);
                let item_coll = self.items.contains_key(&(self.player.x - 1, self.player.y));
                map_coll || item_coll
            }
            "RT" => {
                let map_coll =
                    COLLISION_CELLS.contains(&self.map.cells[self.player.y][self.player.x + 1]);
                let item_coll = self.items.contains_key(&(self.player.x + 1, self.player.y));
                map_coll || item_coll
            }
            _ => false,
        }
    }

    pub fn e_collision(&mut self, dir: &str, entity: Enemy) -> bool {
        match dir {
            "UP" => {
                let map_coll =
                    COLLISION_CELLS.contains(&self.map.cells[entity.pos.1 - 1][entity.pos.0]);
                let item_coll = self.items.contains_key(&(entity.pos.0, entity.pos.1 - 1));
                map_coll || item_coll
            }
            "DN" => {
                let map_coll =
                    COLLISION_CELLS.contains(&self.map.cells[entity.pos.1 + 1][entity.pos.0]);
                let item_coll = self.items.contains_key(&(entity.pos.0, entity.pos.1 + 1));
                map_coll || item_coll
            }
            "LF" => {
                let map_coll =
                    COLLISION_CELLS.contains(&self.map.cells[entity.pos.1][entity.pos.0 - 1]);
                let item_coll = self.items.contains_key(&(entity.pos.0 - 1, entity.pos.1));
                map_coll || item_coll
            }
            "RT" => {
                let map_coll =
                    COLLISION_CELLS.contains(&self.map.cells[entity.pos.1][entity.pos.0 + 1]);
                let item_coll = self.items.contains_key(&(entity.pos.0 + 1, entity.pos.1));
                map_coll || item_coll
            }
            _ => false,
        }
    }

    pub fn shift_items(&mut self, dir: &str) {
        let temp_i = self.items.clone();
        let mut new_i = HashMap::new();
        let mw = self.map.cells[0].len();
        let mh = self.map.cells.len();
        for ((x, y), mut i) in temp_i {
            match dir {
                "UP" => {
                    if y < mh {
                        i.y += 1;
                        new_i.insert((x, y + 1), i.clone());
                    }
                }
                "DN" => {
                    if y > 0 {
                        i.y -= 1;
                        new_i.insert((x, y - 1), i.clone());
                    }
                }
                "LF" => {
                    if x < mw {
                        i.x += 1;
                        new_i.insert((x + 1, y), i.clone());
                    }
                }
                "RT" => {
                    if x > 0 {
                        i.x -= 1;
                        new_i.insert((x - 1, y), i.clone());
                    }
                }
                _ => todo!(),
            };
        }
        self.items = new_i;
    }

    pub fn shift_portals(&mut self, dir: &str) {
        let temp_p = self.portals.clone();
        let mut new_p = HashMap::new();
        let mw = self.map.cells[0].len();
        let mh = self.map.cells.len();
        for ((ix, iy), (ox, oy)) in temp_p {
            match dir {
                "UP" => {
                    if iy < mh && oy < mh {
                        new_p.insert((ix, iy + 1), (ox, oy + 1));
                    }
                }
                "DN" => {
                    if iy > 0 && oy > 0 {
                        new_p.insert((ix, iy - 1), (ox, oy - 1));
                    }
                }
                "LF" => {
                    if ix < mw && ox < mw {
                        new_p.insert((ix + 1, iy), (ox + 1, oy));
                    }
                }
                "RT" => {
                    if ix > 0 && ox > 0 {
                        new_p.insert((ix - 1, iy), (ox - 1, oy));
                    }
                }
                _ => todo!(),
            };
        }
        self.portals = new_p;
    }

    pub fn shift_env_inters(&mut self, dir: &str) {
        let temp_ei = self.env_inters.clone();
        let mut new_i = HashMap::new();
        let mw = self.map.cells[0].len();
        let mh = self.map.cells.len();
        for ((x, y), i) in temp_ei {
            match dir {
                "UP" => {
                    if y < mh {
                        new_i.insert((x, y + 1), i);
                    }
                }
                "DN" => {
                    if y > 0 {
                        new_i.insert((x, y - 1), i);
                    }
                }
                "LF" => {
                    if x < mw {
                        new_i.insert((x + 1, y), i);
                    }
                }
                "RT" => {
                    if x > 0 {
                        new_i.insert((x - 1, y), i);
                    }
                }
                _ => todo!(),
            };
        }
        self.env_inters = new_i;
    }
}
