use crate::enums::{Cells, Door, EnvInter, FeatureType, NPCWrap};
use crate::item::{rand_hermit_item, Item};
use crate::npc::{new_comm_npc, new_conv_npc, new_shop_npc, Convo, ShopConvos, ShopData};
use crate::parsing::{parse_map, tile_to_chars};

use rand::prelude::SliceRandom;
use rand::Rng;
use std::collections::HashMap;
use std::f64::consts;
use std::fs;

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

const GRASS_PATCH_TRANS: &str = r#"
',',',',',',',',
',",',',',',",',
',',',###,',',',
',','#####',',',
',',#####,',',',
',",'###',',',',
',',',',',",',',
',',',',',',',',
"#;

const GRASS_EMPTY: &str = r#"
________________
________________
_____"__________
________________
_____________'__
__,_____________
___________*____
________________
"#;

const GRASS_EMPTY_TRANS: &str = r#"
####______######
####________####
###__"________##
________________
_____________'__
__,_____________
###________*____
####____________
"#;

const GRASS_IN_CORNER_UL: &str = r#"
',',',',',',',',
',",',',',',",',
',',',',',',',',
',',',",'_','_',
',',','______,__
',",','__,______
',',',",______'_
',',','____*____
"#;

const GRASS_IN_CORNER_BL: &str = r#"
',',',',__._____
',",','_____,___
',',',",________
',',','__"_____.
',',',",_,',_,',
',',',',',',',',
',",',',',',",',
',',',',',',',',
"#;

const GRASS_IN_CORNER_UR: &str = r#"
',',',',',',',',
',',",',',',",',
',',',',',',',',
'_','_',",',',',
_,___.__',',',',
___,____',',",',
_______"_,',',',
__,___,_',',',',
"#;

const GRASS_IN_CORNER_BR: &str = r#"
_,______',',',',
___"____',',",',
______.__,',',',
__'_____',',',',
',',_,',",',',',
',',',',',',',',
',',",',',',",',
',',',',',',',',
"#;

const GRASS_OUT_CORNER_UL: &str = r#"
________________
_,____'______"__
__________,_____
___._____,_,_,__
_,_____,',",','"
_______,',',',',
____"___',",',',
_______,',',',',
"#;

const GRASS_OUT_CORNER_BL: &str = r#"
_______,',",',',
_"_____,',',',',
____.__,',",',',
________',',',',
__'______'__'_'_
______._________
____________,___
___"____._______
"#;

const GRASS_OUT_CORNER_BR: &str = r#"
',',',','_______
',',',",_____"__
',",',','_,_____
',',',',________
_'__'_'_______._
________,_______
_"__________,___
_____.__________
"#;

const GRASS_OUT_CORNER_UR: &str = r#"
________________
__,__________,__
_______"________
_,__,_,____'____
',',',','_______
',',',",______,_
',",',','_._____
',',',',________
"#;

const GRASS_HORZ_U: &str = r#"
__,_____________
_____._____'____
_'____________"_
________"_______
',',',_,',',','_
',",',',',',',',
',',',',',",',',
',',',',',',',',
"#;

const GRASS_HORZ_B: &str = r#"
',",',',',',',',
',',',',',",',',
',',',',',',',',
',',',_,',',','_
_______.________
_"________,_____
______________"_
_____'___.______
"#;

const GRASS_VERT_L: &str = r#"
_______,',',',',
_,_____,',',",',
____"__,',',',',
________',',',',
__'____,',',',',
_____._,',',',',
________',",',',
___'___,',',',',
"#;

const GRASS_VERT_R: &str = r#"
',',',','_______
',',",',__.___'_
',',',','_______
',',',',"___"___
',',',','_____,_
',',',',__,_____
',",',','____'__
',',',','_______
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

#[derive(Clone, PartialEq, Eq, Copy, PartialOrd, Ord)]
enum Field {
    Normal,
    NormalTrans,
    InCornerUL,
    InCornerUR,
    InCornerDL,
    InCornerDR,
    OutCornerUL,
    OutCornerUR,
    OutCornerDL,
    OutCornerDR,
    HorzEdgeU,
    HorzEdgeD,
    VertEdgeL,
    VertEdgeR,
    Shrub,
    Empty,
    EmptyTrans,
    Null,
}

const UP_FULL: [Field; 6] = [
    Field::Normal,
    Field::InCornerDL,
    Field::InCornerDR,
    Field::HorzEdgeU,
    Field::Shrub,
    Field::NormalTrans,
];

const UP_LEFT: [Field; 3] = [Field::VertEdgeR, Field::InCornerUL, Field::OutCornerUR];

const UP_RIGHT: [Field; 3] = [Field::VertEdgeL, Field::InCornerUR, Field::OutCornerUL];

const UP_EMPTY: [Field; 5] = [
    Field::Empty,
    Field::EmptyTrans,
    Field::HorzEdgeD,
    Field::OutCornerDL,
    Field::OutCornerDR,
];

const LEFT_FULL: [Field; 6] = [
    Field::Normal,
    Field::NormalTrans,
    Field::InCornerUR,
    Field::InCornerDR,
    Field::VertEdgeL,
    Field::Shrub,
];

const LEFT_TOP: [Field; 3] = [Field::InCornerUL, Field::OutCornerDL, Field::HorzEdgeD];

const LEFT_BOTT: [Field; 3] = [Field::InCornerDL, Field::OutCornerUL, Field::HorzEdgeU];

const LEFT_EMPTY: [Field; 5] = [
    Field::Empty,
    Field::EmptyTrans,
    Field::VertEdgeR,
    Field::OutCornerDR,
    Field::OutCornerUR,
];

pub fn build_field() -> String {
    let mut rng = rand::thread_rng();
    let mut cells = vec![vec![' '; 128]; 64];
    let mut temp = vec![vec![Field::Empty; 8]; 8];
    // temp[0][0] = Field::OutCornerUL;
    for j in (0..temp.len()) {
        for i in (0..temp[0].len()) {
            let up = if j > 0 { temp[j - 1][i] } else { Field::Null };
            let left = if i > 0 { temp[j][i - 1] } else { Field::Null };
            temp[j][i] = {
                match (up, left) {
                    (Field::Null, Field::Null) => Field::OutCornerUL,
                    (up, left) if j == temp.len() - 1 && i == temp[0].len() - 1 => {
                        if UP_EMPTY.contains(&up) {
                            Field::Empty
                        } else {
                            Field::OutCornerDR
                        }
                    }
                    (Field::Null, left) => {
                        if i == temp[0].len() - 1 && LEFT_BOTT.contains(&left) {
                            Field::OutCornerUR
                        } else if i == temp[0].len() - 1 && LEFT_EMPTY.contains(&left) {
                            Field::Empty
                        } else if LEFT_BOTT.contains(&left) {
                            *[
                                Field::HorzEdgeU,
                                Field::HorzEdgeU,
                                Field::HorzEdgeU,
                                Field::OutCornerUR,
                            ]
                            .choose(&mut rng)
                            .unwrap_or(&Field::HorzEdgeU)
                        } else if LEFT_EMPTY.contains(&left) {
                            Field::OutCornerUL
                        } else {
                            Field::Empty
                        }
                    }
                    (up, Field::Null) => {
                        if j == temp.len() - 1 && UP_RIGHT.contains(&up) {
                            Field::OutCornerDL
                        } else if j == temp.len() - 1 && UP_EMPTY.contains(&up) {
                            Field::Empty
                        } else if UP_RIGHT.contains(&up) {
                            *[
                                Field::VertEdgeL,
                                Field::VertEdgeL,
                                Field::VertEdgeL,
                                Field::OutCornerDL,
                            ]
                            .choose(&mut rng)
                            .unwrap_or(&Field::VertEdgeL)
                        } else {
                            Field::OutCornerUL
                        }
                    }
                    (up, left) if i == temp[0].len() - 1 => {
                        if UP_LEFT.contains(&up) && LEFT_FULL.contains(&left) {
                            Field::VertEdgeR
                        } else if UP_LEFT.contains(&up) && LEFT_TOP.contains(&left) {
                            Field::OutCornerDR
                        } else if UP_EMPTY.contains(&up) && LEFT_BOTT.contains(&left) {
                            Field::OutCornerUR
                        } else {
                            Field::Empty
                        }
                    }
                    (up, left) if j == temp.len() - 1 => {
                        if UP_FULL.contains(&up) && LEFT_TOP.contains(&left) {
                            Field::HorzEdgeD
                        } else if UP_LEFT.contains(&up) && LEFT_TOP.contains(&left) {
                            Field::OutCornerDR
                        } else if UP_RIGHT.contains(&up) && LEFT_EMPTY.contains(&left) {
                            Field::OutCornerDL
                        } else {
                            Field::Empty
                        }
                    }
                    (up, left) if UP_FULL.contains(&up) && LEFT_FULL.contains(&left) => *[
                        Field::Normal,
                        Field::Normal,
                        Field::Normal,
                        Field::NormalTrans,
                        Field::InCornerUL,
                        Field::Shrub,
                    ]
                    .choose(&mut rng)
                    .unwrap_or(&Field::Normal),
                    (up, left) if UP_FULL.contains(&up) && LEFT_TOP.contains(&left) => {
                        *[Field::HorzEdgeD, Field::InCornerUR]
                            .choose(&mut rng)
                            .unwrap_or(&Field::HorzEdgeD)
                    }
                    (up, left) if UP_EMPTY.contains(&up) && LEFT_BOTT.contains(&left) => {
                        *[Field::HorzEdgeU, Field::OutCornerUR]
                            .choose(&mut rng)
                            .unwrap_or(&Field::HorzEdgeU)
                    }
                    (up, left) if UP_EMPTY.contains(&up) && LEFT_EMPTY.contains(&left) => {
                        // *[Field::OutCornerUR]
                        //     .choose(&mut rng)
                        //     .unwrap_or(&Field::HorzEdgeU
                        Field::OutCornerUL
                    }
                    (up, left) if UP_LEFT.contains(&up) && LEFT_FULL.contains(&left) => {
                        *[Field::VertEdgeR, Field::InCornerDL]
                            .choose(&mut rng)
                            .unwrap_or(&Field::VertEdgeR)
                    }
                    (up, left) if UP_LEFT.contains(&up) && LEFT_TOP.contains(&left) => {
                        Field::OutCornerDR
                    }
                    (up, left) if UP_RIGHT.contains(&up) && LEFT_EMPTY.contains(&left) => {
                        *[Field::VertEdgeL, Field::OutCornerDL]
                            .choose(&mut rng)
                            .unwrap_or(&Field::VertEdgeL)
                    }
                    (up, left) if UP_RIGHT.contains(&up) && LEFT_BOTT.contains(&left) => {
                        Field::InCornerDR
                    }
                    _ => Field::EmptyTrans,
                }
                // Field::Null
            }
        }
    }
    for j in 0..temp.len() {
        for i in 0..temp[0].len() {
            let patch = match temp[j][i] {
                Field::Normal => GRASS_PATCH,
                Field::NormalTrans => GRASS_PATCH,
                Field::OutCornerUL => GRASS_OUT_CORNER_UL,
                Field::OutCornerUR => GRASS_OUT_CORNER_UR,
                Field::OutCornerDL => GRASS_OUT_CORNER_BL,
                Field::OutCornerDR => GRASS_OUT_CORNER_BR,
                Field::InCornerUL => GRASS_IN_CORNER_UL,
                Field::InCornerUR => GRASS_IN_CORNER_UR,
                Field::InCornerDL => GRASS_IN_CORNER_BL,
                Field::InCornerDR => GRASS_IN_CORNER_BR,
                Field::HorzEdgeU => GRASS_HORZ_U,
                Field::HorzEdgeD => GRASS_HORZ_B,
                Field::VertEdgeL => GRASS_VERT_L,
                Field::VertEdgeR => GRASS_VERT_R,
                Field::Empty => GRASS_EMPTY,
                Field::Shrub => SHRUB_PATCH,
                Field::EmptyTrans => GRASS_EMPTY_TRANS,
                _ => todo!(),
            };
            let patch_chars = tile_to_chars(patch);
            for y in 0..8 {
                for x in 0..16 {
                    cells[j * 8 + y][i * 16 + x] = patch_chars[y][x];
                }
            }
        }
    }
    std::iter::once("Null|Null|Null".to_string())
        .chain(cells.iter().map(|row| row.iter().collect::<String>()))
        .collect::<Vec<String>>()
        .join("\n")
}

pub fn make_field() -> (
    Vec<Vec<Cells>>,
    HashMap<(usize, usize), NPCWrap>,
    HashMap<(usize, usize), Item>,
    HashMap<(usize, usize), EnvInter>,
) {
    let cells = vec![vec![Cells::Empty; 128]; 64];
    parse_map(&build_field(), cells)
}
