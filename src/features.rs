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

const GRASS_PATCH: &str = r#"
',',',',',',',',
',",',',',',",',
',',',',',',',',
',',',",',',',',
',',',',',',',',
',",',',',',',',
',',',',',",',',
',',',',',',',',
"#;

const GRASS_PATCH_IN_CORNER: &str = r#"
',',',',',',',',
',",',',',',",',
',',',',',',',',
',',',",' ',' ',
',','           
',",',          
',','           
',',',          
"#;

const GRASS_PATCH_OUT_CORNER: &str = r#"
                
                
                
       , ' , ,  
      ',',",','"
     ,',',',',',
      ',',",',',
      ',',',',',
"#;

const GRASS_PATCH_HORZ_EDGE: &str = r#"
                
                
                
                
',',', ,',',',' 
',",',',',',',',
',',',',',",',',
',',',',',',',',
"#;

const GRASS_PATCH_VERT_EDGE: &str = r#"
      ',',',',',
       ,',',",',
      ',',',',',
     ,",',',',',
     ,',',',',',
      ',',',',',
      ',',",',',
     ,',',',',',
"#;

const SHRUB_PATCH: &str = r#"
',',',',',',',',
',",'&&&',',",',
','&&&&&&&',',',
',&&&&&&&&',',',
','&&&&&&&&,',',
',",'&&&&,',',',
',',',',',",',',
',',',',',',',',
"#;

const RUIN_BLOCK: &str = r#"
■■■■■___■■■■■■■
■°°___________■
¦______________
■≡_____________
■≡_____________
■≡_____________
■≡≡≡__________■
■■■■■■■■■■■■■■■
"#;

fn make_ruin_feature() -> Vec<Vec<Cells>> {
    let cells = vec![vec![Cells::Empty; 80]; 40];
    cells
}

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

    pub fn new_field_feature(&mut self, pos: (i64, i64)) {
        let mut small_cells = vec![vec![Cells::Empty; 80]; 40];
        self.features.insert(pos, small_cells);
    }
}
