use crate::enums::{Cells, EnvInter, FeatureType, NPCWrap};
use crate::item::Item;
use crate::npc::{new_comm_npc, new_conv_npc, new_shop_npc, Convo, ShopConvos, ShopData};
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

// Abandoned shacks

const ABANDONED_SHACK_1: &str = r#"Null|Null|BronzeWarAxe
###___________________________________##
______________________________________##
____▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒___',___,',',____##
____▒_π____│________▒___',",',',',____##
____▒_____O│______.¨≡°___,',',",',._____
____▒───_──┘________▒___',',',',',."____
____▒_______________▒___',",',',",'_____
#___▒▒▒▒▒▒▒▒▒▒__▒▒▒▒▒____,',",',','_____
#____________"__",_____,',',',',',,_____
#_____.ѧ._____________',',",',',",'_____
#_____"*'_____________',",',',',',______
#_______________________________________
"#;

const ABANDONED_SHACK_2: &str = r#"Null|Null|BronzeGreatsword
#####__________________________________#
__,______.,"__,___________________,__._#
_"┌┬┬┬┬┬┬┬┬┬┐_____'__▒▒▒▒▒▒▒▒▒▒▒▒▒▒,___#
__├",*______┤__,_____▒_________≡≡_▒_____
__├',"______┤_____,"_▒____________▒_____
__├'________┤________▒___┌──┬──___▒___"_
__├_________┤___'____▒___│__2_____▒_____
__├┬┬┬┬┐_┌┬┬┤_"__________├──┘__≡≡O▒__,__
#_._____________'____▒___│_____≡≡≡▒_____
#_______,__.__*______▒___│____≡≡≡_▒,____
#___'____._______.___▒▒▒▒▒▒▒▒▒▒▒▒▒▒",___
_"__________.______*_______"",'___""____
"#;

const ABANDONED_SHACK_3: &str = r#"Null|Null|BronzeGreatsword
________________#########_______________
_,______Ʌ___Ʌ_______'___,__:┌─────┐:_,__
_____ж_Ʌ_________,_________:│~~~~~│:____
__"__жж__ѧ___Ʌ________"____:│~~~~~│:__'_
_,__________Ʌ___________.__:│~~~~~│:____
_"▒▒▒▒▒▒▒▒▒,____,",',',____:└─────┘:_,__
__▒O__│___▒"____,',',",',_____±_±______#
__▒___│___▒_____,',',',',',',_,_,_,',__#
_,▒─__┘_________,",',',",",',',",",',__#
__▒≡_____≡▒__'__,',",',',',",',',',"____
_,▒▒▒▒▒▒▒▒▒,_____',',',',',',',',','__'_
_____'____"_____________________________
"#;

const ABANDONED_SHACK_4: &str = r#"Null|Null|BronzeGreatsword
______________,___________._____########
_ʬʬʬʬʬʬʬʬʬʬʬ___________,________________
_ʬʬ,ʬʬʬʬʬʬʬʬ____._____"▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒"_
_ʬʬʬʬʬʬʬʬʬʬʬ___________▒_____│_______▒__
_ʬʬʬʬʬʬʬ"ʬʬʬ_┤_____,___▒__________π__▒,_
_ʬʬʬʬʬʬʬʬʬʬʬ_┤_________▒O____│_______▒__
_ʬʬʬʬʬʬʬʬʬʬʬ_┤_________▒─────┼────_──▒__
_ʬʬʬ.ʬʬʬʬʬʬʬ_┤_______________________▒,_
_ʬʬʬʬʬʬʬʬʬʬʬ_┤__"______▒_____│_____≡≡▒__
_ʬʬʬʬʬʬʬʬʬʬʬ_┤________,▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒__
_______┌┬┬┬┬┬┤____.____'___________'"___
###______,________________._____________
"#;

const ABANDONED_SHACK_5: &str = r#"Null|Null|BronzeLightAxe BronzeGreatsword
#_____________________________________##
_,.~~~~,__________,____________________#
_"~~~~~~~,__________ʭΠỻΠỻΠΠỻΠỻΠʭ____.___
_~~~~~~~~~,___╔═══╗_ʭ________O_ʭ________
_~~~~~~~~~.___1___║_ʭ__________ʭ________
_~~~~~~~~~~___║__O║_ʭ__________ʭ________
_"~~~~~~~~~'__╚═══╝_ʭ__________ʭ________
__,~~~~~~~,_________ỻΠỻΠĦ__ĦΠỻΠỻ________
___.~~~~~,.______________________,______
_______'"______"________________________
_______________________________________#
##_____________________________________#
"#;

const ABANDONED_SHACKS: [&str; 5] = [
    ABANDONED_SHACK_1,
    ABANDONED_SHACK_2,
    ABANDONED_SHACK_3,
    ABANDONED_SHACK_4,
    ABANDONED_SHACK_5,
];

const SMALL_RUIN_1: &str = r#"Null|Null|Null
###___________________##########____________________________##########__________
______________________##########___,________________________##########__________
_,_ʭʭʭʭʭʭʭʭʭʭʭʭʭʭʭʭ______.________________________'_____________________________
___ʭ≡≡___±©©±_____ʭ______________________________________,___________________"__
__.ʭ_____________"ʭ_________╔════════╗__╔════════╗__╔════════╗__╔════════╗______
___ʭ______::______ʭ_________║~~~~~~~~║__║~~~~~~~~║__║~~~~~~~~║__║~~~~~~~~║____##
___ʭ______::______ʭ_________║~~~~~~~~║__║~~~~~~~~║__║~~~~~~~~║__║~~~~~~~~║____##
___ʭ"_____::______ʭ_________║~~~~~~~~║__║~~~~~~~~║__║~~~~~~~~║__║~~~~~~~~║_,__##
___ʭ_____::::_____ʭ,________╚════════╝__╚════════╝__╚════════╝__╚════════╝____##
_______:::__:::________________________________"______________________________##
_______:::_":::________"___________,_________________________________,________##
_"_____::::::::_____________╔════════╗__╔════════╗__╔════════╗__╔════════╗____##
____________________________║~~~~~~~~║__║~~~~~~~~║__║~~~~~~~~║__║~~~~~~~~║______
___.:____Ħ::Ħ____:__________║~~~~~~~~║__║~~~~~~~~║__║~~~~~~~~║__║~~~~~~~~║___.__
____:,____::_____:,_______._║~~~~~~~~║__║~~~~~~~~║_.║~~~~~~~~║__║~~~~~~~~║______
____::::::::::::::__________╚════════╝__╚════════╝__╚════════╝__╚════════╝______
____:_____::"____:________________________""___________________________"________
_________::::_________,____________________________,____________________________
__'_:ỻ__Π____Π__ỻ:_____________"___##########________________,_______________###
___________________________________##########________________________________###
"#;

const SMALL_RUIN_2: &str = r#"Null|Null|Null
##______________________________________________________________________########
##___,______ᘉᘈᘊ_______┌┬┬┬┬┬┬┬┬┬┬┬┬┬┬┬┐_________________________________########
____╔═════╦═══════╗___├______Ħ±Ħ______┤___╔══════╦═══════════╗________________##
____║≡≡___║_±°¤°±_║___├_┌───┐___┌───┐_┤"__║≡≡≡___║_______≡≡≡≡║________*_______##
____║_____║__:·:__║___├_│~~~│___│~~~│_┤___║_________________≡║__________________
____║________:·:__║___├_│~~~│___│~~~│_┤___║______║___________║__________________
____╠═__══╝__:·:__║___├_└───┘___└───┘_┤___╠════__╝______.____║__________________
____║________:·:__║___├_______________┤___║ᘉᘍᘈᘊ______________║____________,_____
____║_____________║___├┬┬┬┬┬┬Ħ_Ħ┬┬┬┬┬┬┤___║ᘈᘊ___________________________________
___'╚_____________╝_______________,'_____"║ᘉ____________________________________
__________:·:_______ᘉᘍᘈᘊ__________________║ᘍ_______________________~~~__________
##________:·:______ᘈᘍᘈᘊᘉᘍ__________________________:·:_&&________~~~~~~~~",_____
##________:·:_______ᘈᘊᘉᘍᘈ__________________________:·:&&&&______~~~~~~~~~~',____
##________:·:________ᘈᘊᘉ___________________________:·:_&&&_______~~~~~~~~~~.____
##________:·_··················_________,__________:·:__________~~~~~~~~~~______
__________:·······················________________________________~~~~~~~_______
___,'_____.·····_.·······_······.___________·______________________~~~______###
________________________________________________________'____________________###
#____________________________________________________________________________###
#####________________________________________________________________________###
"#;

const SMALL_RUIN_3: &str = r#"Null|Null|Null
######_______________________________________________________________________###
________________________________________________________________________________
__,____________________________________,,________________┌┬┬┬┬┬┬┬┬┬┬┬┬┬┬┬┬┬┬┬┐__
___'__╔═════════════════╦════════════════════════════╗___├___,_______________┤__
______║___│__±°¤°±__│___║≡≡≡≡≡≡≡≡_____________"______║,__├__┌────┐:·:┌────┐._┤__
______║___└─────────┘___║≡≡__________________________║___├__│~~~~│:·:│~~~~│__┤__
______║____:::::::::____║____________________________║___├__│~~~~│:·:│~~~~│__┤__
____,"║____:·:·:·:·:____║____________________________║___├__└────┘:·:└─~~~┘__┤__
______║____:¨:·:·:¨:____║,___________________________║___├________:·:_~~~~~__┤__
______║____:·:¨:¨:·:____╩____________.___________________├_.______:·:__~~~___┤__
______║____:¨:·:·:¨:_____________________________________├,________________,"┤__
______║_____"____________________________________________├┬┬┬┬┬┬┬Ħ___Ħ┬┬┬┬┬┬┬┤__
___.__║_________________________________________________________________________
______║________________________________________________.________________________
______╚══*,_________________══════__________══════______________________________
________"_____________________________________________________________ѧ_________
##___________________________________________'__________________________________
##____________________________,_______________________________________________##
##____________________________________________________________________________##
##_______________________________________________________________________#######
"#;

const palette: &str = r#"
empty: ' . , ' * |
wall: ▒ |
other ▓ ░ ~ |
pipes:
═ ║ ╣ ╠ ╩ ╦ ╗ ╝ ╚ ╔ ╬
┐ └ ┴ ┬ ├ ─ ┼ ┘ ┌ ┤ │

ʬ ỻ Π Ħ ʭ                     
ж ѧ π
ᘉ ᘈ ᘍ ᘊ

≡ ° × ¤ ¸ ¨ · ■ ¦ ± ¡ ø Ø ©

i ̾¡  ͔¡  ͊¡  ͛¡  ̷¡  ̸¡  ̚¡  ͆¡ ¡˞ ¡ˡ  ̢¡ ¡     
"#;

const SMALL_RUIN_0: &str = r#"
________________________________________________________________________________
________________________________________________________________________________
________________________________________________________________________________
________________________________________________________________________________
________________________________________________________________________________
________________________________________________________________________________
________________________________________________________________________________
________________________________________________________________________________
________________________________________________________________________________
________________________________________________________________________________
________________________________________________________________________________
________________________________________________________________________________
________________________________________________________________________________
________________________________________________________________________________
________________________________________________________________________________
________________________________________________________________________________
________________________________________________________________________________
________________________________________________________________________________
________________________________________________________________________________
________________________________________________________________________________
"#;

const ralette: &str = r#"
empty: ' . , ' * |
wall: ▒ |
other ▓ ░ ~ |
pipes:
═ ║ ╣ ╠ ╩ ╦ ╗ ╝ ╚ ╔ ╬
┐ └ ┴ ┬ ├ ─ ┼ ┘ ┌ ┤ │

ʬ ỻ Π Ħ ʭ                     
ж ѧ π
 
≡ ° × ¤ ¸ ¨ · ■ ¦ ± ¡ ø Ø ©

i ̾¡  ͔¡  ͊¡  ͛¡  ̷¡  ̸¡  ̚¡  ͆¡ ¡˞ ¡ˡ  ̢¡ ¡     

"#;

const TEMP: &str = r#"
________________________________________________________________________________
________________________________________________________________________________
________________________________________________________________________________
________________________________________________________________________________
________________________________________________________________________________
________________________________________________________________________________
________________________________________________________________________________
________________________________________________________________________________
________________________________________________________________________________
________________________________________________________________________________
________________________________________________________________________________
________________________________________________________________________________
________________________________________________________________________________
________________________________________________________________________________
________________________________________________________________________________
________________________________________________________________________________
________________________________________________________________________________
________________________________________________________________________________
________________________________________________________________________________
________________________________________________________________________________
"#;

const SMALL_RUINS: [&str; 3] = [SMALL_RUIN_1, SMALL_RUIN_2, SMALL_RUIN_3];

fn parse_map(
    s_map: &str,
    mut cells: Vec<Vec<Cells>>,
) -> (
    Vec<Vec<Cells>>,
    HashMap<(usize, usize), NPCWrap>,
    HashMap<(usize, usize), Item>,
    HashMap<(usize, usize), EnvInter>,
) {
    // let mut cells: Vec<Vec<Cells>> = Vec::new();
    let mut rng = rand::thread_rng();
    let map_codet = s_map.lines().next().unwrap_or("");
    let map_code: Vec<&str> = map_codet.split("|").collect();
    let npc_types: Vec<&str> = map_code.clone()[0].split(" ").collect();
    let item_types: Vec<&str> = map_code.clone()[2].split(" ").collect();

    let data1 = fs::read_to_string("src/npcs/npc_names.json");
    //log::info!("{:?}", &data1);
    let names: Vec<String> = match data1 {
        Ok(content) => serde_json::from_str(&content).unwrap(),
        Err(e) => {
            log::info!("{:?}", e);
            Vec::new()
        }
    };

    let data2 = fs::read_to_string("src/npcs/npc_comms.json");
    //log::info!("{:?}", &data2);
    let comms: Vec<String> = match data2 {
        Ok(content) => serde_json::from_str(&content).unwrap(),
        Err(e) => {
            log::info!("{:?}", e);
            Vec::new()
        }
    };

    let data3 = fs::read_to_string("src/npcs/npc_convos.json");
    //log::info!("{:?}", &data3);
    let convos: Vec<Convo> = match data3 {
        Ok(content) => serde_json::from_str(&content).unwrap(),
        Err(e) => {
            log::info!("{:?}", e);
            Vec::new()
        }
    };

    let mut ncount = 0;
    let mut icount = 0;
    //let mut cells = vec![vec![Cells::Null; 150]; 50];
    let mut npcs = HashMap::new();
    let mut items = HashMap::new();
    let mut env_inters = HashMap::new();
    for (y, line) in s_map.lines().skip(1).enumerate() {
        for (x, ch) in line.chars().enumerate() {
            let cell = match ch {
                '_' => Cells::Empty,
                '#' => Cells::Transparent,
                ',' => Cells::Grass1,
                '\'' => Cells::Grass2,
                '\"' => Cells::Grass3,
                '·' => Cells::Dirt1,
                '.' => Cells::Dirt2,
                ':' => Cells::Dirt3,
                '*' => Cells::Rock,
                '▒' => Cells::Wall,
                ' ' => Cells::Floor,
                '░' => Cells::Floor2,
                '~' => Cells::Water,
                '═' => Cells::MwH,
                '║' => Cells::MwV,
                '╣' => Cells::MwVL,
                '╠' => Cells::MwVR,
                '╩' => Cells::MwHU,
                '╦' => Cells::MwHD,
                '╝' => Cells::MwUL,
                '╚' => Cells::MwUR,
                '╗' => Cells::MwDL,
                '╔' => Cells::MwDR,
                '╬' => Cells::MwCR,
                '─' => Cells::SwH,
                '│' => Cells::SwV,
                '┤' => Cells::SwVL,
                '├' => Cells::SwVR,
                '┴' => Cells::SwHU,
                '┬' => Cells::SwHD,
                '┘' => Cells::SwUL,
                '└' => Cells::SwUR,
                '┐' => Cells::SwDL,
                '┌' => Cells::SwDR,
                '┼' => Cells::SwCR,
                '≡' => Cells::Cong,
                '°' => Cells::Deg,
                '×' => Cells::Mult,
                '¸' => Cells::Ced,
                '¨' => Cells::Diae,
                '■' => Cells::Blsq,
                '¦' => Cells::VBrk,
                '±' => Cells::PlMin,
                'ø' => Cells::SmZer,
                'Ø' => Cells::BZer,
                '©' => Cells::Cop,
                'Ħ' => Cells::DblBracedGate, //-------
                'ỻ' => Cells::BracedGate,
                'Π' => Cells::Arch,
                'ʭ' => Cells::Bricks,
                'ʬ' => Cells::Crops,
                'ѧ' => Cells::SmallCampfire,
                'Ѧ' => Cells::Campfire,
                'π' => Cells::Table,
                'ж' => Cells::Firewood,
                'Ʌ' => Cells::Tent,
                '&' => Cells::Bush,
                'ᘉ' => Cells::Bramble1,
                'ᘈ' => Cells::Bramble2,
                'ᘍ' => Cells::Bramble3,
                'ᘊ' => Cells::Bramble4,
                // '@' => Cells::NPCM,
                '{' => Cells::LBrce,
                '}' => Cells::RBrce,
                '(' => Cells::LParen,
                ')' => Cells::RParen,
                '¤' => Cells::GenCur,
                'o' => Cells::Item,
                'l' => Cells::Log,
                'c' => Cells::Clinic,
                'p' => Cells::GPost,
                's' => Cells::CPost,
                _ => Cells::Empty,
            };
            cells[y][x] = cell;
            if ch == '@' {
                let def_name = "Kevthony".to_string();
                match npc_types[ncount] {
                    "CommNPC" => {
                        //let com_def = vec!["Welcome to the caves!!".to_string(), "Theres a tonne of folk down here, lerger cities as you go into the cave.".to_string()];
                        let rnd_comms = {
                            let mut tvec = Vec::new();
                            for _ in 0..4 {
                                let tidx = rng.gen_range(0..comms.len());
                                tvec.push(comms[tidx].clone());
                            }
                            tvec
                        };
                        let name = names.choose(&mut rng).unwrap_or(&def_name.clone()).clone();
                        let t_comm = new_comm_npc(name.clone(), x, y, rnd_comms.clone());
                        npcs.insert((x, y), NPCWrap::CommNPC(t_comm.clone()));
                    }
                    "ConvNPC" => {
                        let name = names.choose(&mut rng).unwrap_or(&def_name.clone()).clone();
                        //let comms = vec!["Welcome to the caves!!".to_string(), "Theres a tonne of folk down here, lerger cities as you go into the cave.".to_string()];
                        let conv: Convo = convos
                            .choose(&mut rng)
                            .unwrap_or(&convos[0].clone())
                            .clone();
                        let t_comm = new_conv_npc(name.clone(), x, y, conv.clone());
                        npcs.insert((x, y), NPCWrap::ConvNPC(t_comm.clone()));
                    }
                    _ => todo!(),
                }
                ncount += 1;
            }
            if ch == 'O' {
                match item_types[icount] {
                    "HealthPotion" => {
                        let ti = Item::new_health_potion(x, y);
                        items.insert((x, y), ti.clone());
                    }
                    "Salve" => {
                        let ti = Item::new_salve(x, y);
                        items.insert((x, y), ti.clone());
                    }
                    "Dowel" => {
                        let ti = Item::new_dowel(x, y);
                        items.insert((x, y), ti.clone());
                    }
                    "SmallWoodShield" => {
                        let ti = Item::new_small_wood_shield(x, y);
                        items.insert((x, y), ti.clone());
                    }
                    "Apple" => {
                        let ti = Item::new_apple(x, y);
                        items.insert((x, y), ti.clone());
                    }
                    "BronzeClaymore" => {
                        let ti = Item::new_bronze_claymore(x, y);
                        items.insert((x, y), ti.clone());
                    }
                    "BronzeShortsword" => {
                        let ti = Item::new_bronze_shortsword(x, y);
                        items.insert((x, y), ti.clone());
                    }
                    "BronzeLongsword" => {
                        let ti = Item::new_bronze_longsword(x, y);
                        items.insert((x, y), ti.clone());
                    }
                    "BronzeLightAxe" => {
                        let ti = Item::new_bronze_light_axe(x, y);
                        items.insert((x, y), ti.clone());
                    }
                    "BronzeHeavyAxe" => {
                        let ti = Item::new_bronze_heavy_axe(x, y);
                        items.insert((x, y), ti.clone());
                    }
                    "BronzeWarAxe" => {
                        let ti = Item::new_bronze_war_axe(x, y);
                        items.insert((x, y), ti.clone());
                    }
                    "BronzePickHammer" => {
                        let ti = Item::new_bronze_pick_hammer(x, y);
                        items.insert((x, y), ti.clone());
                    }
                    "WoodStaff" => {
                        let ti = Item::new_wood_staff(x, y);
                        items.insert((x, y), ti.clone());
                    }
                    "LightArmour" => {
                        let ti = Item::new_light_armour(x, y);
                        items.insert((x, y), ti.clone());
                    }
                    _ => {
                        log::info!("itm {:?}", item_types[icount]);
                    }
                }
                icount += 1;
            }
        }
    }
    (cells, npcs, items, env_inters)
}

fn make_small_ruin_feature() -> (
    Vec<Vec<Cells>>,
    HashMap<(usize, usize), NPCWrap>,
    HashMap<(usize, usize), Item>,
    HashMap<(usize, usize), EnvInter>,
) {
    let cells = vec![vec![Cells::Empty; 80]; 20];
    let mut rng = rand::thread_rng();
    parse_map(SMALL_RUINS.choose(&mut rng).unwrap_or(&SMALL_RUIN_1), cells)
}

fn make_abandoned_shack() -> (
    Vec<Vec<Cells>>,
    HashMap<(usize, usize), NPCWrap>,
    HashMap<(usize, usize), Item>,
    HashMap<(usize, usize), EnvInter>,
) {
    let cells = vec![vec![Cells::Empty; 40]; 12];
    let mut rng = rand::thread_rng();
    parse_map(
        ABANDONED_SHACKS
            .choose(&mut rng)
            .unwrap_or(&ABANDONED_SHACK_1),
        cells,
    )
}

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

fn tile_to_chars(tile: &str) -> Vec<Vec<char>> {
    tile.trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn build_field() -> String {
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
                    (up, right) if j == temp.len() - 1 && i == temp[0].len() - 1 => {
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

fn make_field() -> (
    Vec<Vec<Cells>>,
    HashMap<(usize, usize), NPCWrap>,
    HashMap<(usize, usize), Item>,
    HashMap<(usize, usize), EnvInter>,
) {
    let cells = vec![vec![Cells::Empty; 128]; 64];
    parse_map(&build_field(), cells)
}

#[derive(Clone, Debug, PartialEq)]
pub struct Feature {
    pub ftype: FeatureType,
    pub pos: (i16, i16),
    pub map: Vec<Vec<Cells>>,
    pub items: HashMap<(usize, usize), Item>,
    pub npcs: HashMap<(usize, usize), NPCWrap>,
    pub env_inters: HashMap<(usize, usize), EnvInter>,
    pub cont_sent: bool,
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
        let mut rng = rand::thread_rng();
        // let choice = *[
        //     FeatureType::AbandonedShack,
        //     FeatureType::Field,
        //     FeatureType::Ruin,
        // ]
        // .choose(&mut rng)
        // .unwrap_or(&FeatureType::AbandonedShack);
        let choice = FeatureType::Ruin;
        match choice {
            FeatureType::AbandonedShack => self.new_abandoned_shack(pos),
            FeatureType::Field => self.new_field_feature(pos),
            FeatureType::Ruin => self.new_small_ruin_feature(pos),
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
