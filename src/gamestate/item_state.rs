//item_state

use crate::enums::{Cells, Items, Plants};
use crate::gamestate::GameState;
use crate::item::Item;
use crate::map::{MAP_H, MAP_W};
use rand::prelude::SliceRandom;
use rand::Rng;

impl GameState {
    pub fn check_place_item(&mut self, x: usize, y: usize) -> bool {
        let mut rng = rand::thread_rng();
        let types = [
            Items::Rock,
            Items::EdibleRoot,
            Items::Apple,
            Items::MetalScrap,
            Items::Plants(Plants::LuminousMushroom),
            Items::Plants(Plants::LichenousGrowth),
            Items::Plants(Plants::LampenPetals),
            Items::Plants(Plants::LuckyClover),
            Items::Plants(Plants::Shroom),
        ];
        if self.map.cells[y][x] == Cells::Empty
            && !self.in_loc_check((x, y))
            && !self.enemies.contains_key(&(x, y))
            && !self.items.contains_key(&(x, y))
        {
            if let Some(i_type) = types.choose(&mut rng) {
                match i_type {
                    Items::EdibleRoot => {
                        self.items.insert((x, y), Item::new_edible_root(x, y));
                    }
                    Items::Apple => {
                        self.items.insert((x, y), Item::new_apple(x, y));
                    }
                    Items::MetalScrap => {
                        self.items.insert((x, y), Item::new_metal_scrap(x, y));
                    }
                    Items::Rock => {
                        self.items.insert((x, y), Item::new_rock(x, y));
                    }
                    Items::Plants(Plants::LuminousMushroom) => {
                        self.items.insert((x, y), Item::new_luminous_mushroom(x, y));
                    }
                    Items::Plants(Plants::LichenousGrowth) => {
                        self.items.insert((x, y), Item::new_lichenous_growth(x, y));
                    }
                    Items::Plants(Plants::LampenPetals) => {
                        self.items
                            .insert((x, y), Item::new_lampen_flower_petals(x, y));
                    }
                    Items::Plants(Plants::LuckyClover) => {
                        self.items.insert((x, y), Item::new_lucky_clover(x, y));
                    }
                    Items::Plants(Plants::Shroom) => {
                        self.items.insert((x, y), Item::new_shroom(x, y));
                    }
                    _ => todo!(),
                };
                return true;
            }
        }
        false
    }

    pub fn repop_items(&mut self) {
        let mut rng = rand::thread_rng();
        let (vx, vy, vw, vh) = self.map.get_viewport();
        //xx
        match (-self.map.gen_x, -self.map.gen_y) {
            (x, y) if x < 0 && y == 0 => {
                for _ in 0..30 {
                    loop {
                        let x = rng.gen_range(10..vx - 5);
                        let y = rng.gen_range(10..MAP_H - 10);
                        let res = self.check_place_item(x, y);
                        if res {
                            break;
                        }
                    }
                }
            }
            (x, y) if x > 0 && y == 0 => {
                for _ in 0..30 {
                    loop {
                        let x = rng.gen_range((vx + vw + 5)..MAP_W - 10);
                        let y = rng.gen_range(10..MAP_H - 10);
                        let res = self.check_place_item(x, y);
                        if res {
                            break;
                        }
                    }
                }
            }
            (x, y) if y < 0 && x == 0 => {
                for _ in 0..30 {
                    loop {
                        let x = rng.gen_range(10..MAP_W - 10);
                        let y = rng.gen_range(10..vy - 5);
                        let res = self.check_place_item(x, y);
                        if res {
                            break;
                        }
                    }
                }
            }
            (x, y) if y > 0 && x == 0 => {
                for _ in 0..30 {
                    loop {
                        let x = rng.gen_range(10..MAP_W - 10);
                        let y = rng.gen_range((vy + vh + 5)..MAP_H - 10);
                        let res = self.check_place_item(x, y);
                        if res {
                            break;
                        }
                    }
                }
            } // asdf
            (x, y) if x > 0 && y > 0 => {
                for _ in 0..15 {
                    loop {
                        let x = rng.gen_range((vx + vw + 5)..MAP_W - 10);
                        let y = rng.gen_range(10..MAP_H - 10);
                        let res = self.check_place_item(x, y);
                        if res {
                            break;
                        }
                    }
                }
                for _ in 0..15 {
                    loop {
                        let x = rng.gen_range(10..MAP_W - 10);
                        let y = rng.gen_range((vy + vh + 5)..MAP_H - 10);
                        let res = self.check_place_item(x, y);
                        if res {
                            break;
                        }
                    }
                }
            }
            (x, y) if x > 0 && y < 0 => {
                for _ in 0..15 {
                    loop {
                        let x = rng.gen_range((vx + vw + 5)..MAP_W - 10);
                        let y = rng.gen_range(10..MAP_H - 10);
                        let res = self.check_place_item(x, y);
                        if res {
                            break;
                        }
                    }
                }
                for _ in 0..15 {
                    loop {
                        let x = rng.gen_range(10..MAP_W - 10);
                        let y = rng.gen_range(10..vy - 5);
                        let res = self.check_place_item(x, y);
                        if res {
                            break;
                        }
                    }
                }
            }
            (x, y) if x < 0 && y > 0 => {
                for _ in 0..15 {
                    loop {
                        let x = rng.gen_range(10..vx - 5);
                        let y = rng.gen_range(10..MAP_H - 10);
                        let res = self.check_place_item(x, y);
                        if res {
                            break;
                        }
                    }
                }
                for _ in 0..15 {
                    loop {
                        let x = rng.gen_range(10..MAP_W - 10);
                        let y = rng.gen_range((vy + vh + 5)..MAP_H - 10);
                        let res = self.check_place_item(x, y);
                        if res {
                            break;
                        }
                    }
                }
            }
            (x, y) if x < 0 && y < 0 => {
                for _ in 0..15 {
                    loop {
                        let x = rng.gen_range(10..vx - 5);
                        let y = rng.gen_range(10..MAP_H - 10);
                        let res = self.check_place_item(x, y);
                        if res {
                            break;
                        }
                    }
                }
                for _ in 0..15 {
                    loop {
                        let x = rng.gen_range(10..MAP_W - 10);
                        let y = rng.gen_range(10..vy - 5);
                        let res = self.check_place_item(x, y);
                        if res {
                            break;
                        }
                    }
                }
            }
            _ => {}
        }
    }
}
