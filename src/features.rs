use crate::enums::Cells;
use std::collections::HashMap;

const palette: &str = r#"
empty: ' . , ' * |
wall: ▒ |
other ▓ ░ ~ |
pipes:
═ ║ ╣ ╠ ╩ ╦ ╗ ╝ ╚ ╔ ╬
┐ └ ┴ ┬ ├ ─ ┼ ┘ ┌ ┤ │

≡ ° × ¤ ¸ ¨ · ■ ¦ ± ¡ ø Ø ©"#;

const grass_patch: &str = r#"
',',',',',',','
',",',',',',",'
',',',',',',','
',',',",',',','
',',',',',',','
',",',',',',','
',',',',',",','
',',',',',',','
"#;

const shrub_patch: &str = r#"
',',',',',',','
',",',',',',",'
',',',',',',','
',',',",',',','
',',',',',',','
',",',',',',','
',',',',',",','
',',',',',',','
"#;

const ruin_block: &str = r#"
■■■■■___■■■■■■■
■°°___________■
¦______________
■≡_____________
■≡_____________
■≡_____________
■≡≡≡__________■
■■■■■■■■■■■■■■■
"#;

pub struct Features {
    features: HashMap<(i64, i64), Vec<Vec<Cells>>>,
}

impl Features {
    pub fn new() -> Self {
        Self {
            features: HashMap::new(),
        }
    }

    pub fn new_ruin_feature(&mut self, pos: (i64, i64)) {
        let mut small_cells = vec![vec![Cells::Empty; 80]; 40];
        self.features.insert(pos, small_cells);
    }
}
