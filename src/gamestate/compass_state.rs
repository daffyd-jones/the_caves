//compass_state

use crate::enums::CompMode;
use crate::gamestate::GameState;

impl GameState {
    pub fn set_comp(&mut self) {
        let copt = self.gui.get_comp_opt();
        if copt == "Search" {
            self.comp_mode = CompMode::Search;
            self.compass_check();
            return;
        } else {
            self.comp_mode = CompMode::Location;
        }
        for (p, n) in &self.comp_list {
            if *n == copt {
                self.comp_head = *p;
                break;
            }
        }
    }

    pub fn sort_comp_list(&mut self) -> Vec<String> {
        let mut hyp_list = Vec::new();
        let comp_list = self.comp_list.clone();
        let dfo = self.dist_fo;
        for (pos, name) in comp_list {
            let hyp = ((((pos.0 - -dfo.0) as i32).pow(2) + ((pos.1 - -dfo.1) as i32).pow(2)) as f64)
                .sqrt() as u16;
            hyp_list.push((hyp, name));
        }
        let mut names = Vec::new();
        hyp_list.sort_by(|a, b| a.0.cmp(&b.0));
        for (_, name) in hyp_list {
            names.push(name);
        }
        names
    }
}
