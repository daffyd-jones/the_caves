// feature_state

use crate::enums::Location;
use crate::features::Feature;
use crate::gamestate::GameState;
use crate::map::{MAP_H, MAP_W};

impl GameState {
    pub fn update_feature(&mut self, mut feature: Feature) -> Location {
        let fpos = feature.pos;
        let pos = self.dist_fo;
        let dx = (fpos.0 + pos.0) as usize;
        let dy = (fpos.1 + pos.1) as usize;
        if dx < MAP_W && dy < MAP_H && !feature.cont_sent {
            let items = feature.items.clone();
            for (_, mut i) in items {
                let ipos = i.get_pos();
                let npos = (
                    (pos.0 + ipos.0 as i16 + fpos.0) as usize,
                    (pos.1 + ipos.1 as i16 + fpos.1) as usize,
                );
                i.set_pos(npos);
                self.items.insert(npos, i.clone());
            }
            let env_inters = feature.env_inters.clone();
            for ((x, y), env) in env_inters {
                let nwpos = (
                    (self.dist_fo.0 + x as i16 + fpos.0) as usize,
                    (self.dist_fo.1 + y as i16 + fpos.1) as usize,
                );
                self.env_inters.insert(nwpos, env);
            }
            feature.cont_sent = true;
        }
        Location::Feature(feature)
    }
}
