//lsystem

struct LSystem {
    axiom: String,
    rules: fn(String) -> String,
}

fn main_rules (axiom: String) -> String {
    let mut new_axiom = String::new();
    for ch in axiom.chars() {
        match ch {
            'a' => new_axiom.push_str("a"),
            _ => new_axiom.push_str(""),
        }
    }
    new_axiom
}

fn side_quests (axiom: String) -> String {
    let mut new_axiom = String::new();
    for ch in axiom.chars() {
        match ch {
            'a' => new_axiom.push_str("a"),
            _ => new_axiom.push_str(""),
        }
    }
    new_axiom
}

fn enemy_rules (axiom: String) -> String {
    let mut new_axiom = String::new();
    for ch in axiom.chars() {
        match ch {
            'a' => new_axiom.push_str("a"),
            _ => new_axiom.push_str(""),
        }
    }
    new_axiom
}

fn item_rules (axiom: String) -> String {
    let mut new_axiom = String::new();
    for ch in axiom.chars() {
        match ch {
            'a' => new_axiom.push_str("a"),
            _ => new_axiom.push_str(""),
        }
    }
    new_axiom
}

pub struct LSystems {
    main_quest: LSystem,
    side_quests: LSystem,
    enemies: LSystem,
    items: LSystem,
}

impl LSystems {
    pub fn new () -> Self {
        let main_quest = LSystem {
            axiom: "".to_string(),
            rules: main_rules,
        };

        let side_quests = LSystem {
            axiom: "".to_string(),
            rules: side_quests,
        };

        let enemies = LSystem {
            axiom: "".to_string(),
            rules: enemy_rules,
        };

        let items = LSystem {
            axiom: "".to_string(),
            rules: item_rules,
        };
        Self {
            main_quest,
            side_quests,
            enemies,
            items,
        }
    }

    fn process_enemies (&mut self) {
        let new_axiom = (self.enemies.rules)(self.enemies.axiom.clone());
        self.enemies.axiom = new_axiom;
    }
}
