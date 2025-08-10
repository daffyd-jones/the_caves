use crate::enums::{Cells, Door, EnvInter, FeatureType, NPCWrap};
use crate::item::{rand_hermit_item, Item};
use crate::npc::{new_comm_npc, new_conv_npc, new_shop_npc, Convo, ShopConvos, ShopData};
use crate::parsing::{parse_map, tile_to_chars};

use rand::prelude::SliceRandom;
use rand::Rng;
use std::collections::HashMap;

const CONSTRUCTION_BLANK: &str = r#"Null|Null|Null
________________________________________
________________________________________
________________________________________
________________________________________
________________________________________
________________________________________
________________________________________
________________________________________
________________________________________
________________________________________
________________________________________
________________________________________
"#;

const CONSTRUCTION_1: &str = r#"CommNPC CommNPC CommNPC CommNPC CommNPC|Null|Null
######░░░░░░░░░░▓▓▓▓  . ,  C ·  ▓▓▓▓####
######░░░░░░░░░░▓▓▓▓┬┬┬┬┬┐┌┬┬┬┬┬▓▓▓▓▓▓##
░░░░░░░🬗░░░░░░░░▓▓🬐🬑        '  │▓▓▓▓▓▓##
░░░░░░░░░░░░░░░░▓▓🬮🬡@      *   │▓▓▓▓▓▓##
,   ░░░░     ·, │ @ ··'   ·    ├┬┬┬┐▓▓##
    ░░░░     ,  │   ,   ·   ⑁  ┌─┐ │▓▓##
.  ·░░░░ *.     ├┬┬┐    @ ѧ  ⑁ ├╦┤ ├▓▓##
    ░░░░   '   , , ├┬┬┐   ' ,,   @  ▓▓##
,   ░░░░   . ,      * ├┐┌┐     @🬮🬡🬑▓▓🬗▓▓
    ░░░░   .          C  ├┬┬┐ · ▓▓▓▓🬗▓▓▓
    ░░░░                '   ├┬┬┬▓▓▓▓▓🬗▓▓
    🬁░░░               , ·      ▓▓▓▓▓▓▓▓
"#;

const CONSTRUCTION_2: &str = r#"CommNPC CommNPC CommNPC CommNPC CommNPC CommNPC|Null|Null
' ######### 🬞▓▓▓     C   '  ▓▓▓▓▓▓▓▓▓▓▓🭏
##        , ▓▓▓▓┌┬┬┬┬┐┌┬┬┬┬┐▓▓▓▓▓▓▓▓▓▓▓▓
## *        ▓▓▓▓         .  ▓▓▓▓▓▓▓▓▓▓▓▓
##.         ▓▓🬑🬗   @  ⑁    @🬑▓▓▓▓▓▓▓▓▓▓▓
##       , '▓▓▓🬡     ѧ ⑁    🬡🬮▓▓   ·    
##          ▓▓▓🬗@   ▓▓▓🭏   @🬦▓▓▓    '   
##          ▓▓▓▓π@  ▓▓▓▓··  ▓▓▓▓   .  , 
## *    '   ▓▓▓▓,   ▓▓▓▓   ·🬡🬗▓▓        
▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓·  @🬑▓▓▓▓▓▓▓▓▓▓🬿
▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓   *▓▓▓▓▓▓▓▓▓▓▓▓
▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓┌┬┬┐▓▓▓▓▓▓▓▓▓▓▓▓
▓▓▓▓▓▓▓ඉ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓🭠 C  ▓▓▓▓▓▓▓▓▓▓▓▓
"#;

const CONSTRUCTION_3: &str = r#"CommNPC CommNPC CommNPC CommNPC CommNPC|Null|Null
## '   ,,    '   ,,#########▓▓▓▓   *▓▓▓▓
## . ,    * *           ▓▓▓▓▓🬗▓▓▓▓▓▓▓▓▓▓
## .                *  ·▓▓▓▓🬗▓▓▓▓▓▓▓▓▓▓▓
##          '        ┌┬┐▓▓▓▓▓🬗▓▓▓▓▓▓▓▓▓▓
##              ,C·┌┬┤  ▓▓🬡🬑🬑▓▓▓▓▓▓▓▓▓▓ඉ
.'     ,   ,  ┌┬┐┌┬┤   Ʌ .@ 🬮▓▓▓    ▓▓▓▓
            ┌┬┤             ▓▓▓▓    ▓▓▓▓
   *      ┌┬┤        ѧ ⑁ '' 🬑▓▓▓  . ▓▓▓▓
     · ,  │  *  @  @     @ @🬗▓▓▓.   ▓▓▓▓
▒▒▒▒▒▒▒▒▓▓▓▓▓▓▓🬑🬡🬑🬗🬑🬮▓▓▓🬑🬮🬡🬮🬑▓▓▓ *  ▓▓▓▓
▒▒▒▒▒▒▒▒▓▓▓▓▓▓▓▓🬑🬮🬡▓▓▓▓▓▓▓▓▓▓▓▓▓    ▓▓▓▓
▒▒▒▒▒▒▒▒▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓    ▓▓▓▓
"#;

const CONSTRUCTION: [&str; 3] = [CONSTRUCTION_1, CONSTRUCTION_2, CONSTRUCTION_3];

pub fn make_construction_feature() -> (
    Vec<Vec<Cells>>,
    HashMap<(usize, usize), NPCWrap>,
    HashMap<(usize, usize), Item>,
    HashMap<(usize, usize), EnvInter>,
) {
    let cells = vec![vec![Cells::Empty; 40]; 12];
    let mut rng = rand::thread_rng();
    parse_map(
        CONSTRUCTION.choose(&mut rng).unwrap_or(&CONSTRUCTION_1),
        cells,
    )
}
