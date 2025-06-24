//item_state

use crate::enums::{Cells, Items, Month, Plants};
use crate::gamestate::GameState;
use crate::item::Item;
use crate::map::{MAP_H, MAP_W};
use rand::prelude::SliceRandom;
use rand::Rng;

const EMPTY_CELLS: [Cells; 8] = [
    Cells::Empty,
    Cells::Grass1,
    Cells::Grass2,
    Cells::Grass3,
    Cells::Dirt1,
    Cells::Dirt2,
    Cells::Dirt3,
    Cells::Rock,
];

impl GameState {
    fn place_moss(&mut self, x: usize, y: usize) {
        // let rng = rand::thread_rng();
        let tmap = self.map.cells.clone();
        for j in 0..12 {
            for i in 0..20 {
                let yy = j + y;
                let xx = i + x;
                if EMPTY_CELLS.contains(&tmap[yy][xx]) && !EMPTY_CELLS.contains(&tmap[yy - 1][xx])
                    || EMPTY_CELLS.contains(&tmap[yy][xx])
                        && !EMPTY_CELLS.contains(&tmap[yy + 1][xx])
                    || EMPTY_CELLS.contains(&tmap[yy][xx])
                        && !EMPTY_CELLS.contains(&tmap[yy][xx - 1])
                    || EMPTY_CELLS.contains(&tmap[yy][xx])
                        && !EMPTY_CELLS.contains(&tmap[yy][xx + 1])
                {
                    self.items.insert((xx, yy), Item::new_moss(xx, yy));
                }
            }
        }
    }

    fn place_vine(&mut self, x: usize, y: usize) {
        let mut rng = rand::thread_rng();
        let tmap = self.map.cells.clone();
        for j in 0..12 {
            for i in 0..20 {
                let yy = j + y;
                let xx = i + x;
                if xx > tmap[0].len() - 5 || yy > tmap.len() - 5 {
                    return;
                }
                if EMPTY_CELLS.contains(&tmap[yy][xx]) && !EMPTY_CELLS.contains(&tmap[yy - 1][xx])
                    || EMPTY_CELLS.contains(&tmap[yy][xx])
                        && !EMPTY_CELLS.contains(&tmap[yy + 1][xx])
                    || EMPTY_CELLS.contains(&tmap[yy][xx])
                        && !EMPTY_CELLS.contains(&tmap[yy][xx - 1])
                    || EMPTY_CELLS.contains(&tmap[yy][xx])
                        && !EMPTY_CELLS.contains(&tmap[yy][xx + 1])
                {
                    self.map.cells[yy][xx] = *[
                        Cells::Bramble1,
                        Cells::Bramble2,
                        Cells::Bramble3,
                        Cells::Bramble4,
                    ]
                    .choose(&mut rng)
                    .unwrap_or(&Cells::Bramble1);
                    if rng.gen_range(0..5) == 0 {
                        self.items.insert((xx, yy), Item::new_vine_bulb(xx, yy));
                    }
                }
            }
        }
    }

    fn place_area_plants(&mut self, x: usize, y: usize) -> bool {
        let mut rng = rand::thread_rng();
        let types = [Items::Plants(Plants::Moss), Items::Plants(Plants::VineBulb)];
        if let Some(i_type) = types.choose(&mut rng) {
            match i_type {
                Items::Plants(Plants::Moss) => self.place_moss(x, y),
                Items::Plants(Plants::VineBulb) => self.place_vine(x, y),
                _ => todo!(),
            }
            return true;
        }
        false
    }

    pub fn repop_area_plants(&mut self) {
        let mut rng = rand::thread_rng();
        let (vx, vy, vw, vh) = self.map.get_viewport();
        //xx
        match (-self.map.gen_x, -self.map.gen_y) {
            (x, y) if x < 0 => {
                for _ in 0..5 {
                    loop {
                        let x = rng.gen_range(10..vx - 5);
                        let y = rng.gen_range(10..MAP_H - 10);
                        let res = self.place_area_plants(x, y);
                        if res {
                            break;
                        }
                    }
                }
                if y < 0 {
                    for _ in 0..5 {
                        loop {
                            let x = rng.gen_range(10..MAP_W - 10);
                            let y = rng.gen_range(10..vy - 5);
                            let res = self.place_area_plants(x, y);
                            if res {
                                break;
                            }
                        }
                    }
                }
                if y > 0 {
                    for _ in 0..5 {
                        loop {
                            let x = rng.gen_range(10..MAP_W - 10);
                            let y = rng.gen_range((vy + vh + 5)..MAP_H - 10);
                            let res = self.place_area_plants(x, y);
                            if res {
                                break;
                            }
                        }
                    }
                }
            }
            (x, y) if x > 0 => {
                for _ in 0..5 {
                    loop {
                        let x = rng.gen_range((vx + vw + 5)..MAP_W - 10);
                        let y = rng.gen_range(10..MAP_H - 10);
                        let res = self.place_area_plants(x, y);
                        if res {
                            break;
                        }
                    }
                }
                if y < 0 {
                    for _ in 0..5 {
                        loop {
                            let x = rng.gen_range(10..MAP_W - 10);
                            let y = rng.gen_range(10..vy - 5);
                            let res = self.place_area_plants(x, y);
                            if res {
                                break;
                            }
                        }
                    }
                }
                if y > 0 {
                    for _ in 0..5 {
                        loop {
                            let x = rng.gen_range(10..MAP_W - 10);
                            let y = rng.gen_range((vy + vh + 5)..MAP_H - 10);
                            let res = self.place_area_plants(x, y);
                            if res {
                                break;
                            }
                        }
                    }
                }
            }
            (x, y) if y < 0 => {
                for _ in 0..5 {
                    loop {
                        let x = rng.gen_range(10..MAP_W - 10);
                        let y = rng.gen_range(10..vy - 5);
                        let res = self.place_area_plants(x, y);
                        if res {
                            break;
                        }
                    }
                }
                if x < 0 {
                    for _ in 0..5 {
                        loop {
                            let x = rng.gen_range(10..vx - 5);
                            let y = rng.gen_range(10..MAP_H - 10);
                            let res = self.place_area_plants(x, y);
                            if res {
                                break;
                            }
                        }
                    }
                }
                if x > 0 {
                    for _ in 0..5 {
                        loop {
                            let x = rng.gen_range((vx + vw + 5)..MAP_W - 10);
                            let y = rng.gen_range(10..MAP_H - 10);
                            let res = self.place_area_plants(x, y);
                            if res {
                                break;
                            }
                        }
                    }
                }
            }
            (x, y) if y > 0 => {
                for _ in 0..5 {
                    loop {
                        let x = rng.gen_range(10..MAP_W - 10);
                        let y = rng.gen_range((vy + vh + 5)..MAP_H - 10);
                        let res = self.place_area_plants(x, y);
                        if res {
                            break;
                        }
                    }
                }
                if x < 0 {
                    for _ in 0..5 {
                        loop {
                            let x = rng.gen_range(10..vx - 5);
                            let y = rng.gen_range(10..MAP_H - 10);
                            let res = self.place_area_plants(x, y);
                            if res {
                                break;
                            }
                        }
                    }
                }
                if x > 0 {
                    for _ in 0..5 {
                        loop {
                            let x = rng.gen_range((vx + vw + 5)..MAP_W - 10);
                            let y = rng.gen_range(10..MAP_H - 10);
                            let res = self.place_area_plants(x, y);
                            if res {
                                break;
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }
    pub fn check_place_item(&mut self, x: usize, y: usize) -> bool {
        let mut rng = rand::thread_rng();
        let mut types = vec![
            Items::Rock,
            Items::EdibleRoot,
            Items::Apple,
            Items::MetalScrap,
        ];
        match self.stats.world_stats.date.month {
            Month::Opal => types.append(&mut vec![
                // damp
                Items::Plants(Plants::Shroom),
                Items::Plants(Plants::LuminousMushroom),
                Items::Plants(Plants::LichenousGrowth),
            ]),
            Month::Quartz => types.append(&mut vec![
                // drying
                Items::Plants(Plants::LuminousMushroom),
                Items::Plants(Plants::LampenFlower),
                Items::Plants(Plants::LuckyClover),
            ]),
            Month::Jade => types.append(&mut vec![
                // dry
                Items::Plants(Plants::VioletShadow),
                Items::Plants(Plants::LampenFlower),
                Items::Plants(Plants::LuckyClover),
            ]),
            Month::Bizmuth => types.append(&mut vec![
                // damping
                Items::Plants(Plants::LichenousGrowth),
                Items::Plants(Plants::VioletShadow),
                Items::Plants(Plants::Shroom),
            ]),
        }
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
                    Items::Plants(Plants::LampenFlower) => {
                        self.items.insert((x, y), Item::new_lampen_flower(x, y));
                    }
                    Items::Plants(Plants::LuckyClover) => {
                        self.items.insert((x, y), Item::new_lucky_clover(x, y));
                    }
                    Items::Plants(Plants::Shroom) => {
                        self.items.insert((x, y), Item::new_shroom(x, y));
                    }
                    Items::Plants(Plants::VioletShadow) => {
                        self.items.insert((x, y), Item::new_violet_shadow(x, y));
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
