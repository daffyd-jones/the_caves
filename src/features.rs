use crate::enums::{Cells, Door, EnvInter, FeatureType, Items, NPCWrap, ShopItem};
use crate::features::abandoned_shacks::make_abandoned_shack;
use crate::features::construction::make_construction_feature;
use crate::features::field::make_field;
use crate::features::pond::make_pond_feature;
use crate::features::ruin::make_small_ruin_feature;
use crate::features::streams::make_stream;
use crate::item::{rand_hermit_item, Item};
use crate::parsing::{ch_to_enum, parse_map};
use crate::shop::Shop;
use rand::prelude::SliceRandom;
use rand::Rng;
use std::collections::HashMap;

mod abandoned_shacks;
mod construction;
mod field;
mod pond;
mod ruin;
mod streams;

const PALETTE: &str = r#"
empty: ' . , ' * |
wall: ▒ |
other ▓ ░ ~ |
pipes:
═ ║ ╣ ╠ ╩ ╦ ╗ ╝ ╚ ╔ ╬
┐ └ ┴ ┬ ├ ─ ┼ ┘ ┌ ┤ │
ʬ ỻ Π Ħ ʭ ṑ ⑁                   
ж ѧ π
ᘉ ᘈ ᘍ ᘊ
≡ ° × ¤ ¸ ¨ · ■ ¦ ± ¡ ø Ø ©
"#;

const HERMIT_1: &str = r#"
__________
_____─┬─__
_____o│o__
_____o│o__
__x__─┴─__
__________
"#;

const HERMIT_2: &str = r#"
__________
____┌───┐_
____│o_o│_
____│o_o│_
__x_└___┘_
__________
"#;

#[derive(Clone, Debug, PartialEq)]
pub struct Feature {
    pub ftype: FeatureType,
    pub pos: (i16, i16),
    pub map: Vec<Vec<Cells>>,
    pub items: HashMap<(usize, usize), Item>,
    // pub sitems: HashMap<(usize, usize), ShopItem>,
    pub npcs: HashMap<(usize, usize), NPCWrap>,
    pub env_inters: HashMap<(usize, usize), EnvInter>,
    pub cont_sent: bool,
    pub hermit: bool,
    pub hermit_shop: Shop,
    pub hermit_pos: (usize, usize),
    pub hermit_map: Vec<Vec<Cells>>,
}

impl Feature {
    pub fn place_hermit(&mut self) {
        for j in (0..self.map.len() - 6) {
            for i in (0..self.map[0].len() - 10) {
                let check = {
                    let mut ch = true;
                    for jj in j..(j + 6) {
                        if !ch {
                            break;
                        }
                        for ii in i..(i + 10) {
                            if self.map[jj][ii] != Cells::Empty {
                                ch = false;
                                break;
                            }
                        }
                    }
                    ch
                };
                if check {
                    self.hermit_pos = (i, j);
                    break;
                }
            }
        }
    }

    pub fn place_hermit_parts(&mut self) {
        let mut map = self.map.clone();
        let hermit = HERMIT_1;
        let mut items = HashMap::new();
        let mut env_inters = HashMap::new();
        let mut scroll = false;
        for (j, line) in hermit.lines().enumerate() {
            for (i, ch) in line.chars().enumerate() {
                match ch {
                    'x' => {
                        env_inters.insert(
                            (self.hermit_pos.0 + i, self.hermit_pos.1 + j),
                            EnvInter::Hermit,
                        );
                    }
                    'o' if !scroll => {
                        items.insert(
                            (self.hermit_pos.0 + i, self.hermit_pos.1 + j),
                            ShopItem::Hermit(Item::new_scroll(
                                self.hermit_pos.0 + i,
                                self.hermit_pos.1 + j,
                            )),
                        );
                        scroll = true;
                    }
                    'o' => {
                        items.insert(
                            (self.hermit_pos.0 + i, self.hermit_pos.1 + j),
                            ShopItem::Hermit(rand_hermit_item(
                                self.hermit_pos.0 + i,
                                self.hermit_pos.1 + j,
                            )),
                        );
                    }
                    _ => {}
                }
                map[self.hermit_pos.1 + j][self.hermit_pos.0 + i] = ch_to_enum(ch);
            }
        }
        // self.items = items;
        self.hermit_shop = Shop::new_hermit(items);
        self.env_inters = env_inters;
        self.hermit_map = map;
    }
}

pub struct Features {
    features: HashMap<(i16, i16), Feature>,
}

impl Features {
    pub fn new() -> Self {
        Self {
            features: HashMap::new(),
        }
    }

    pub fn get_feature_positions(&self) -> Vec<String> {
        let mut feat_vec = Vec::new();
        for (pos, f) in self.features.clone() {
            feat_vec.push(format!("({}, {}): {:?}", pos.0, pos.1, f.ftype));
        }
        feat_vec
    }

    pub fn new_rand_feature(&mut self, pos: (i16, i16)) {
        // let mut rng = rand::thread_rng();
        // let choice = *[
        //     FeatureType::AbandonedShack,
        //     FeatureType::Field,
        //     FeatureType::Ruin,
        //     FeatureType::Stream,
        //     FeatureType::Construction,
        //     FeatureType::Pond,
        // ]
        // .choose(&mut rng)
        // .unwrap_or(&FeatureType::AbandonedShack);
        let choice = FeatureType::Construction;
        match choice {
            FeatureType::AbandonedShack => self.new_abandoned_shack(pos),
            FeatureType::Field => self.new_field_feature(pos),
            FeatureType::Ruin => self.new_small_ruin_feature(pos),
            FeatureType::Stream => self.new_stream_feature(pos),
            FeatureType::Construction => self.new_construction_feature(pos),
            FeatureType::Pond => self.new_pond_feature(pos),
            _ => self.new_abandoned_shack(pos),
        }
    }

    pub fn new_small_ruin_feature(&mut self, pos: (i16, i16)) {
        let (map, npcs, items, env_inters) = make_small_ruin_feature();
        self.features.insert(
            pos,
            Feature {
                ftype: FeatureType::Ruin,
                pos,
                map,
                items,
                npcs,
                env_inters,
                cont_sent: false,
                hermit: false,
                hermit_shop: Shop::default(),
                hermit_pos: (0, 0),
                hermit_map: Vec::new(),
            },
        );
    }

    pub fn new_construction_feature(&mut self, pos: (i16, i16)) {
        let (map, npcs, items, env_inters) = make_construction_feature();
        self.features.insert(
            pos,
            Feature {
                ftype: FeatureType::Construction,
                pos,
                map,
                items,
                npcs,
                env_inters,
                cont_sent: false,
                hermit: false,
                hermit_shop: Shop::default(),
                hermit_pos: (0, 0),
                hermit_map: Vec::new(),
            },
        );
    }

    pub fn new_pond_feature(&mut self, pos: (i16, i16)) {
        let (map, npcs, items, env_inters) = make_pond_feature();
        self.features.insert(
            pos,
            Feature {
                ftype: FeatureType::Pond,
                pos,
                map,
                items,
                npcs,
                env_inters,
                cont_sent: false,
                hermit: false,
                hermit_shop: Shop::default(),
                hermit_pos: (0, 0),
                hermit_map: Vec::new(),
            },
        );
    }

    pub fn new_field_feature(&mut self, pos: (i16, i16)) {
        let (map, npcs, items, env_inters) = make_field();
        self.features.insert(
            pos,
            Feature {
                ftype: FeatureType::Field,
                pos,
                map,
                items,
                npcs,
                env_inters,
                cont_sent: false,
                hermit: false,
                hermit_shop: Shop::default(),
                hermit_pos: (0, 0),
                hermit_map: Vec::new(),
            },
        );
    }

    pub fn new_stream_feature(&mut self, pos: (i16, i16)) {
        let (map, npcs, items, env_inters) = make_stream();
        self.features.insert(
            pos,
            Feature {
                ftype: FeatureType::Stream,
                pos,
                map,
                items,
                npcs,
                env_inters,
                cont_sent: false,
                hermit: false,
                hermit_shop: Shop::default(),
                hermit_pos: (0, 0),
                hermit_map: Vec::new(),
            },
        );
    }

    pub fn new_abandoned_shack(&mut self, pos: (i16, i16)) {
        let (map, npcs, items, env_inters) = make_abandoned_shack();
        self.features.insert(
            pos,
            Feature {
                ftype: FeatureType::AbandonedShack,
                pos,
                map,
                items,
                npcs,
                env_inters,
                cont_sent: false,
                hermit: false,
                hermit_shop: Shop::default(),
                hermit_pos: (0, 0),
                hermit_map: Vec::new(),
            },
        );
    }

    pub fn check_location(&self, bpos: (i16, i16), rad: u16) -> Option<Feature> {
        for (spos, s) in &self.features {
            let xx = (spos.0 - bpos.0 * -1) as i32;
            let yy = (spos.1 - bpos.1 * -1) as i32;
            let hyp = ((xx.pow(2) + yy.pow(2)) as f64).sqrt() as u64;
            if hyp <= rad.into() {
                return Some(s.clone());
            }
        }
        return None;
    }

    pub fn update_feature(&mut self, feature: Feature) {
        self.features.insert(feature.pos, feature);
    }

    pub fn feature_check(&mut self, pos: (i16, i16)) -> bool {
        let dir = (pos.0 / pos.0.abs(), pos.1 / pos.1.abs());
        let space = {
            match dir {
                (x, y) if x >= 0 && y >= 0 => (((pos.0 + 800), (pos.1 + 800)), pos),
                (x, y) if x < 0 && y >= 0 => (((pos.0 - 800), (pos.1 + 800)), pos),
                (x, y) if x >= 0 && y < 0 => (((pos.0 + 800), (pos.1 - 800)), pos),
                (x, y) if x < 0 && y < 0 => (((pos.0 - 800), (pos.1 - 800)), pos),
                _ => todo!(),
            }
        };
        for (k, _) in self.features.clone() {
            let xrange: Vec<i16> = {
                let mut xa = space.0 .0;
                let mut xb = space.1 .0;
                if xa > xb {
                    std::mem::swap(&mut xa, &mut xb);
                }
                (xa..xb).collect()
            };
            let yrange: Vec<i16> = {
                let mut ya = space.0 .0;
                let mut yb = space.1 .0;
                if ya > yb {
                    std::mem::swap(&mut ya, &mut yb);
                }
                (ya..yb).collect()
            };

            if xrange.contains(&k.0) && yrange.contains(&k.1) {
                return false;
            };
        }
        true
    }
}
