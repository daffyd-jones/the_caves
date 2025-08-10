use crate::enums::{Cells, Door, EnvInter, FeatureType, NPCWrap};
use crate::item::{rand_hermit_item, Item};
use crate::parsing::{parse_map, tile_to_chars};

use rand::prelude::SliceRandom;
use rand::Rng;
use std::collections::HashMap;

const STREAM: &str = r#"
________________
________________
________________
________________
________________
________________
________________
________________
"#;

const STREAM_SOURCE_L: &str = r#"
################
___________#####
_~~~~____╔╦╗____
~~~~~~~~~~~╣_###
~~~~~~~~~~╦╣_###
~____~~~_____###
___________#####
##########_#####
"#;

const STREAM_SOURCE_R: &str = r#"
################
####____________
####_╔╦╗__~~~~__
####_╠~~~~~~~~~~
####_╠╦~~~~~~~~~
________~~~__~~~
#######_________
################
"#;

const STREAM_SOURCE_B: &str = r#"
################
####________####
______╔╦╦╗__####
####__╠~~╣___###
####__~~~~__####
###__~~~~~__####
####_~~~~~~__###
####__~~~~~_####
"#;

const STREAM_SOURCE_U: &str = r#"
###__~~~~~__####
####_~~~~~~__###
###___~~~~~_####
####__~~~~__####
####__╣~~╠___###
###___╚╦╦╝__####
####____________
################
"#;

const STREAM_TRANS: &str = r#"
################
################
################
################
################
################
################
################
"#;

const STREAM_UR: &str = r#"
####__~~~~__####
####__~~~~______
###__~~~~~__~~~_
###__~~~~~~~~~~~
####__~~~~~~~~~~
###_____~~~~____
####____________
################
"#;

const STREAM_UL: &str = r#"
####__~~~~__####
#_#____~~~~__###
______~~~~~__###
~~~_~~~~~~~__###
~~~~~~~~~~__####
__~~~~~______###
____________####
################
"#;

const STREAM_BL: &str = r#"
################
_________#######
__~~~~~____#####
~~~~~~~~~~__####
~~~__~~~~~~__###
______~~~~~__###
##___~~~~~~__###
###___~~~~__####
"#;

const STREAM_BR: &str = r#"
################
#######_________
#####____~~~~___
####___~~~~~~~~~
####__~~~~~~~~~~
###__~~~~~______
###__~~~~____###
####__~~~~__####
"#;

const STREAM_VERT: &str = r#"
####__~~~~__####
###__~~~~~__####
###__~~~~~~__###
###__~~~~~~__###
####__~~~~~__###
###__~~~~~~__###
###__~~~~~__####
####__~~~~__####
"#;

const STREAM_HORZ: &str = r#"
################
#______#_______#
__~~~~___~~~~~__
~~~~~~~~~~~~~~~~
~~~~~~~~~~~~~~~~
_____~~~~~______
###________#####
################
"#;

#[derive(Clone, PartialEq, Eq, Copy, PartialOrd, Ord)]
enum Stream {
    Horz,
    Vert,
    SourceU,
    SourceB,
    SourceL,
    SourceR,
    UR,
    UL,
    BL,
    BR,
    Trans,
    Null,
}

const STREAM_UP_FULL: [Stream; 4] = [Stream::Vert, Stream::BL, Stream::BR, Stream::SourceB];

const STREAM_UP_EMPTY: [Stream; 7] = [
    Stream::Horz,
    Stream::UR,
    Stream::UL,
    Stream::Trans,
    Stream::SourceU,
    Stream::SourceR,
    Stream::SourceL,
];

const STREAM_LEFT_FULL: [Stream; 4] = [Stream::Horz, Stream::UR, Stream::BR, Stream::SourceR];

const STREAM_LEFT_EMPTY: [Stream; 7] = [
    Stream::Vert,
    Stream::UL,
    Stream::BL,
    Stream::Trans,
    Stream::SourceU,
    Stream::SourceB,
    Stream::SourceL,
];

pub fn build_stream() -> String {
    let mut rng = rand::thread_rng();
    let mut cells = vec![vec![' '; 128]; 64];
    let mut temp = vec![vec![Stream::Trans; 8]; 8];
    // temp[0][0] = Field::OutCornerUL;
    for j in (0..temp.len()) {
        for i in (0..temp[0].len()) {
            let up = if j > 0 { temp[j - 1][i] } else { Stream::Null };
            let left = if i > 0 { temp[j][i - 1] } else { Stream::Null };
            temp[j][i] = {
                match (up, left) {
                    (Stream::Null, Stream::Null) => *[Stream::SourceB, Stream::SourceR]
                        .choose(&mut rng)
                        .unwrap_or(&Stream::SourceB),
                    (up, right) if j == temp.len() - 1 && i == temp[0].len() - 1 => {
                        if STREAM_UP_FULL.contains(&up) {
                            Stream::SourceU
                        } else if STREAM_LEFT_FULL.contains(&right) {
                            Stream::SourceL
                        } else {
                            Stream::Trans
                        }
                    }
                    (Stream::Null, left) => {
                        if i == temp[0].len() - 1 && STREAM_LEFT_FULL.contains(&left) {
                            Stream::BL
                        } else if i == temp[0].len() - 1 && STREAM_LEFT_EMPTY.contains(&left) {
                            Stream::Trans
                        } else if STREAM_LEFT_FULL.contains(&left) {
                            *[Stream::Horz, Stream::BL]
                                .choose(&mut rng)
                                .unwrap_or(&Stream::Horz)
                        } else {
                            Stream::Trans
                        }
                    }
                    (up, Stream::Null) => {
                        if j == temp.len() - 1 && STREAM_UP_FULL.contains(&up) {
                            Stream::UR
                        } else if j == temp.len() - 1 && STREAM_UP_EMPTY.contains(&up) {
                            Stream::Trans
                        } else if STREAM_UP_FULL.contains(&up) {
                            *[Stream::Vert, Stream::UR]
                                .choose(&mut rng)
                                .unwrap_or(&Stream::Vert)
                        } else {
                            Stream::Trans
                        }
                    }
                    (up, left) if i == temp[0].len() - 1 => {
                        if STREAM_UP_FULL.contains(&up) && STREAM_LEFT_FULL.contains(&left) {
                            Stream::UL
                        } else if STREAM_UP_FULL.contains(&up) && STREAM_LEFT_EMPTY.contains(&left)
                        {
                            Stream::Vert
                        } else if STREAM_UP_EMPTY.contains(&up) && STREAM_LEFT_FULL.contains(&left)
                        {
                            Stream::BL
                        } else {
                            Stream::Trans
                        }
                    }
                    (up, left) if j == temp.len() - 1 => {
                        if STREAM_UP_FULL.contains(&up) && STREAM_LEFT_FULL.contains(&left) {
                            Stream::UL
                        } else if STREAM_UP_EMPTY.contains(&up) && STREAM_LEFT_FULL.contains(&left)
                        {
                            Stream::Horz
                        } else if STREAM_UP_FULL.contains(&up) && STREAM_LEFT_EMPTY.contains(&left)
                        {
                            Stream::UR
                        } else {
                            Stream::Trans
                        }
                    }
                    (up, left)
                        if STREAM_UP_FULL.contains(&up) && STREAM_LEFT_FULL.contains(&left) =>
                    {
                        Stream::UL
                    }
                    (up, left)
                        if STREAM_UP_FULL.contains(&up) && STREAM_LEFT_EMPTY.contains(&left) =>
                    {
                        *[Stream::Vert, Stream::UR]
                            .choose(&mut rng)
                            .unwrap_or(&Stream::Vert)
                    }
                    (up, left)
                        if STREAM_UP_EMPTY.contains(&up) && STREAM_LEFT_FULL.contains(&left) =>
                    {
                        *[Stream::Horz, Stream::BL]
                            .choose(&mut rng)
                            .unwrap_or(&Stream::Horz)
                    }
                    (up, left)
                        if STREAM_UP_EMPTY.contains(&up) && STREAM_LEFT_EMPTY.contains(&left) =>
                    {
                        *[Stream::SourceB, Stream::SourceR, Stream::Trans, Stream::BR]
                            .choose(&mut rng)
                            .unwrap_or(&Stream::Trans)
                    }
                    _ => Stream::Trans,
                }
            }
        }
    }
    for j in 0..temp.len() {
        for i in 0..temp[0].len() {
            let patch = match temp[j][i] {
                Stream::Horz => STREAM_HORZ,
                Stream::Vert => STREAM_VERT,
                Stream::SourceU => STREAM_SOURCE_U,
                Stream::SourceB => STREAM_SOURCE_B,
                Stream::SourceL => STREAM_SOURCE_L,
                Stream::SourceR => STREAM_SOURCE_R,
                Stream::UL => STREAM_UL,
                Stream::UR => STREAM_UR,
                Stream::BL => STREAM_BL,
                Stream::BR => STREAM_BR,
                Stream::Trans => STREAM_TRANS,
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

pub fn make_stream() -> (
    Vec<Vec<Cells>>,
    HashMap<(usize, usize), NPCWrap>,
    HashMap<(usize, usize), Item>,
    HashMap<(usize, usize), EnvInter>,
) {
    let cells = vec![vec![Cells::Empty; 128]; 64];
    parse_map(&build_stream(), cells)
}
