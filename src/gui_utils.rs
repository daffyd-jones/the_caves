//gui_utils
use crate::enemy::Enemy;
use crate::enums::{AniType, Cells, Door, Enemies, EnvInter, NPCWrap};
use crate::item::Item;
use crate::map::Map;
use crate::player::Player;
use rand::Rng;
use ratatui::layout::{Constraint, Direction, Layout, Margin};
use ratatui::prelude::Alignment;
use ratatui::prelude::Line;
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::{Span, Text};
use ratatui::widgets::Cell;
use ratatui::widgets::Row;
use ratatui::widgets::Table;
use ratatui::widgets::{Block, Borders, Padding, Paragraph};
use std::collections::HashMap;
use std::time::Duration;

pub enum CustomColors {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    Gray,
    DarkGray,
    LightRed,
    LightGreen,
    LightYellow,
    LightBlue,
    LightMagenta,
    LightCyan,
    White,
}

impl CustomColors {
    pub fn to_custom_color(&self) -> Color {
        match self {
            CustomColors::Black => Color::Rgb(34, 39, 39),
            CustomColors::Red => Color::Rgb(42, 161, 152),
            CustomColors::Green => Color::Rgb(42, 161, 152),
            CustomColors::Yellow => Color::Rgb(42, 161, 152),
            CustomColors::Blue => Color::Rgb(42, 161, 152),
            CustomColors::Magenta => Color::Rgb(42, 161, 152),
            CustomColors::Cyan => Color::Rgb(42, 161, 152),
            CustomColors::Gray => Color::Rgb(42, 161, 152),
            CustomColors::DarkGray => Color::Rgb(42, 161, 152),
            CustomColors::LightRed => Color::Rgb(42, 161, 152),
            CustomColors::LightGreen => Color::Rgb(42, 161, 152),
            CustomColors::LightYellow => Color::Rgb(42, 161, 152),
            CustomColors::LightBlue => Color::Rgb(42, 161, 152),
            CustomColors::LightMagenta => Color::Rgb(42, 161, 152),
            CustomColors::LightCyan => Color::Rgb(42, 161, 152),
            CustomColors::White => Color::Rgb(42, 161, 152),
        }
    }
}

pub fn wrap_text(text: &str, max_width: usize) -> Text {
    let mut lines = Vec::new();
    let mut current_line = String::new();
    for word in text.split_whitespace() {
        if current_line.len() + word.len() + 1 > max_width {
            //lines.push(current_line);
            lines.push(Line::from(current_line.clone()));
            current_line.clear();
        }
        if !current_line.is_empty() {
            current_line.push(' ');
        }
        if word.eq("nl") {
            lines.push(Line::from(current_line.clone()));
            current_line.clear();
        } else {
            current_line.push_str(word);
        }
    }
    if !current_line.is_empty() {
        //lines.push(current_line);
        lines.push(Line::from(current_line));
    }
    //lines
    Text::from(lines)
}

pub struct GuiArgs<'a> {
    pub map: &'a Map,
    pub player: &'a Player,
    pub stats: &'a Vec<u16>,
    pub enemies: &'a HashMap<(usize, usize), Enemy>,
    pub items: &'a HashMap<(usize, usize), Item>,
    pub npcs: &'a HashMap<(usize, usize), NPCWrap>,
    pub env_inter: Option<&'a HashMap<(usize, usize), EnvInter>>,
    pub litems: Option<&'a HashMap<(usize, usize), Item>>,
    pub portals: Option<&'a HashMap<(usize, usize), (usize, usize)>>,
    pub animate: Option<&'a Animation>,
    pub ascii: Option<&'a String>,
}

type Frame = Vec<Vec<(char, Color)>>;
pub struct Animation {
    pub atype: AniType,
    pub pos: (usize, usize),
    pub frame: Option<Frame>,
    pub char: Option<(char, Color)>,
}

// pub fn gui_setup(f: Frame) -> () {
//     let entire_screen_block = Block::default()
//         .style(Style::default().bg(Color::Black))
//         .borders(Borders::NONE);
//     f.render_widget(entire_screen_block, f.area());
//     let chunks = Layout::default()
//         .direction(Direction::Vertical)
//         .margin(1)
//         .constraints(
//             [
//                 Constraint::Percentage(10),
//                 Constraint::Percentage(80),
//                 Constraint::Percentage(10),
//             ]
//             .as_ref(),
//         )
//         .split(f.area());

//     let game_chunks = Layout::default()
//         .direction(Direction::Horizontal)
//         .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
//         .split(chunks[1]);

//     let block = Block::default().title("Game").borders(Borders::ALL);
//     f.render_widget(block.clone(), game_chunks[0]);
//     let block_area = game_chunks[0];
//     f.render_widget(block.clone(), block_area);
//     let inner_area = block_area.inner(Margin::default());
//     let in_h = inner_area.height as usize;
//     let in_w = inner_area.width as usize;
//     if in_h != self.viewport_dim.1 && in_w != self.viewport_dim.0 {
//         // map.set_viewport(in_h, in_w);
//         self.viewport_dim = (in_w, in_h);
//     }
//     (inner_area, game_chunks)
// }

pub fn draw_map<'a>(gui_args: &GuiArgs, ani_cnt: u8) -> Paragraph<'a> {
    let map = gui_args.map.clone();
    let start_row = map.viewport_y;
    let end_row = (map.viewport_y + map.viewport_height).min(map.cells.len());
    let start_col = map.viewport_x;
    let end_col = (map.viewport_x + map.viewport_width).min(map.cells[0].len());
    let mut text = Vec::new();
    // log::info!("\nEnvinters: {:?}", env_inters);
    let ani_parts = {
        if let Some(animate) = gui_args.animate {
            match animate.atype {
                AniType::Player => (
                    AniType::Player,
                    (0, 0),
                    vec![vec![(' ', Color::Black)]],
                    animate.char.unwrap(),
                ),
                AniType::Char => (
                    AniType::Char,
                    animate.pos,
                    vec![vec![(' ', Color::Black)]],
                    animate.char.unwrap(),
                ),
                AniType::Area => (
                    AniType::Area,
                    animate.pos,
                    animate.frame.clone().unwrap(),
                    (' ', Color::Black),
                ),
                _ => (
                    AniType::Null,
                    (0, 0),
                    vec![vec![(' ', Color::Black)]],
                    (' ', Color::Black),
                ),
            }
        } else {
            (
                AniType::Null,
                (0, 0),
                vec![vec![(' ', Color::Black)]],
                (' ', Color::Black),
            )
        }
    };
    for (j, row) in map.cells[start_row..end_row].iter().enumerate() {
        let mut line = Vec::new();
        for (i, &cell) in row[start_col..end_col].iter().enumerate() {
            let (symbol, color) = {
                let ix = i + start_col;
                let jy = j + start_row;
                if (ix, jy) == (gui_args.player.x, gui_args.player.y)
                    && ani_parts.0 == AniType::Player
                {
                    ani_parts.3
                } else if (ix, jy) == (gui_args.player.x, gui_args.player.y) {
                    ('¡', Color::LightYellow)
                } else if ani_parts.0 == AniType::Char && ani_parts.1 == (ix, jy) {
                    ani_parts.3
                } else if ani_parts.0 == AniType::Area
                    && (ani_parts.1 .0..ani_parts.2[0].len()).contains(&ix)
                    && (ani_parts.1 .1..ani_parts.2.len()).contains(&jy)
                {
                    ani_parts.2[jy - ani_parts.1 .1][ix - ani_parts.1 .0]
                } else if let Some(enemy) = gui_args.enemies.get(&(ix, jy)) {
                    match enemy.etype {
                        Enemies::Bug => ('Ѫ', Color::Red),
                        Enemies::Spider => ('ẅ', Color::Red),
                        Enemies::Snake => ('ʑ', Color::Red),
                        Enemies::Slime => ('ǚ', Color::Red),
                        Enemies::Bandit => ('ḯ', Color::Red),
                        Enemies::CrazedExplorer => ('ĩ', Color::Red),
                        Enemies::Goblin => ('ớ', Color::Red),
                        Enemies::Ghoul => ('ή', Color::Red),
                        Enemies::Golem => ('Ṏ', Color::Red),
                        _ => todo!(),
                    }
                } else if gui_args.portals.unwrap().contains_key(&(ix, jy)) {
                    ('@', Color::Blue)
                } else if let Some(npcw) = gui_args.npcs.get(&(ix, jy)) {
                    // ï î ì í  Í Î Ï Ì
                    match npcw {
                        NPCWrap::CommNPC(_) => ('í', Color::Blue),
                        NPCWrap::ConvNPC(_) => ('ì', Color::LightBlue),
                        NPCWrap::ShopNPC(_) => ('ì', Color::Yellow),
                        NPCWrap::SpawnNPC(_) => ('î', Color::Cyan),
                        NPCWrap::TradeNPC(_) => ('ï', Color::LightGreen),
                        _ => todo!(),
                    }
                } else if let Some(item) = gui_args.items.get(&(ix, jy)) {
                    item.icon
                } else if let Some(item) = gui_args.litems.unwrap().get(&(ix, jy)) {
                    item.icon
                } else if let Some(env) = gui_args.env_inter.unwrap().get(&(ix, jy)) {
                    let env_col = {
                        if ani_cnt % 3 == 0 {
                            Color::Green
                        } else {
                            Color::DarkGray
                        }
                    };
                    match env {
                        EnvInter::Records => ('│', Color::Green),
                        EnvInter::Clinic => ('─', Color::Green),
                        EnvInter::GuildPost => ('─', Color::Green),
                        EnvInter::ChurchPost => ('─', Color::Green),
                        EnvInter::Construction => ('ì', Color::Blue),
                        EnvInter::Cauldron => ('℧', Color::Green),
                        EnvInter::Herbalist => ('ì', Color::Yellow),
                        EnvInter::Door(Door::VLocked(_)) => ('╎', Color::White),
                        EnvInter::Door(Door::VOpen) => ('🮀', Color::White),
                        EnvInter::Door(Door::HLocked(_)) => ('╌', Color::White),
                        EnvInter::Door(Door::HOpen) => (' ', Color::White),
                        _ => todo!(),
                    }
                } else {
                    match cell {
                        Cells::Empty => (' ', Color::White),
                        Cells::Dirt1 => ('·', Color::DarkGray),
                        Cells::Dirt2 => ('.', Color::DarkGray),
                        Cells::Dirt3 => (':', Color::DarkGray),
                        Cells::Grass1 => (',', Color::LightGreen),
                        Cells::Grass2 => ('\'', Color::LightMagenta),
                        Cells::Grass3 => ('\"', Color::Green),
                        Cells::Bramble1 => ('ᘉ', Color::Green),
                        Cells::Bramble2 => ('ᘈ', Color::Green),
                        Cells::Bramble3 => ('ᘍ', Color::Green),
                        Cells::Bramble4 => ('ᘊ', Color::Green),
                        Cells::Bush => ('&', Color::Green),
                        Cells::Rock => ('*', Color::DarkGray),
                        Cells::Wall => ('▒', Color::DarkGray),
                        Cells::Wall2 => ('▓', Color::DarkGray),
                        Cells::Wall3 => ('█', Color::DarkGray),
                        Cells::Wall4 => ('░', Color::Red),
                        Cells::ULCorner1 => ('🬵', Color::DarkGray),
                        Cells::ULCorner2 => ('🬞', Color::DarkGray),
                        Cells::ULCorner3 => ('🬶', Color::DarkGray),
                        Cells::ULCorner4 => ('🭄', Color::DarkGray),
                        Cells::ULCorner5 => ('🭊', Color::DarkGray),
                        Cells::URCorner1 => ('🬱', Color::DarkGray),
                        Cells::URCorner2 => ('🬏', Color::DarkGray),
                        Cells::URCorner3 => ('🬳', Color::DarkGray),
                        Cells::URCorner4 => ('🭏', Color::DarkGray),
                        Cells::URCorner5 => ('🬿', Color::DarkGray),
                        Cells::DLCorner1 => ('🬊', Color::DarkGray),
                        Cells::DLCorner2 => ('🬁', Color::DarkGray),
                        Cells::DLCorner3 => ('🬙', Color::DarkGray),
                        Cells::DLCorner4 => ('🭕', Color::DarkGray),
                        Cells::DLCorner5 => ('🭥', Color::DarkGray),
                        Cells::DRCorner1 => ('🬆', Color::DarkGray),
                        Cells::DRCorner2 => ('🬀', Color::DarkGray),
                        Cells::DRCorner3 => ('🬥', Color::DarkGray),
                        Cells::DRCorner4 => ('🭠', Color::DarkGray),
                        Cells::DRCorner5 => ('🭚', Color::DarkGray),
                        Cells::Broken1 => ('🬤', Color::DarkGray),
                        Cells::Broken2 => ('🬗', Color::DarkGray),
                        Cells::Broken3 => ('🬐', Color::DarkGray),
                        Cells::Broken4 => ('🬑', Color::DarkGray),
                        Cells::Broken5 => ('🬮', Color::DarkGray),
                        Cells::Broken6 => ('🬡', Color::DarkGray),
                        Cells::Roots => ('ඉ', Color::Yellow),
                        Cells::NPCM => (' ', Color::White),
                        Cells::Floor => ('░', Color::Black),
                        Cells::Floor2 => ('░', Color::Gray),
                        Cells::MwH => ('═', Color::Gray),
                        Cells::MwV => ('║', Color::Gray),
                        Cells::MwVL => ('╣', Color::Gray),
                        Cells::MwVR => ('╠', Color::Gray),
                        Cells::MwHU => ('╩', Color::Gray),
                        Cells::MwHD => ('╦', Color::Gray),
                        Cells::MwUL => ('╝', Color::Gray),
                        Cells::MwUR => ('╚', Color::Gray),
                        Cells::MwDL => ('╗', Color::Gray),
                        Cells::MwDR => ('╔', Color::Gray),
                        Cells::MwCR => ('╬', Color::Gray),
                        Cells::SwH => ('─', Color::Gray),
                        Cells::SwV => ('│', Color::Gray),
                        Cells::SwVL => ('┤', Color::Gray),
                        Cells::SwVR => ('├', Color::Gray),
                        Cells::SwHU => ('┴', Color::Gray),
                        Cells::SwHD => ('┬', Color::Gray),
                        Cells::SwUL => ('┘', Color::Gray),
                        Cells::SwUR => ('└', Color::Gray),
                        Cells::SwDL => ('┐', Color::Gray),
                        Cells::SwDR => ('┌', Color::Gray),
                        Cells::SwCR => ('┼', Color::Gray),
                        Cells::Cong => ('≡', Color::Magenta),
                        Cells::Deg => ('°', Color::Cyan),
                        Cells::Mult => ('×', Color::Magenta),
                        Cells::Ced => ('¸', Color::LightBlue),
                        Cells::Diae => ('¨', Color::LightBlue),
                        Cells::Inter => ('·', Color::LightBlue),
                        Cells::Blsq => ('■', Color::LightBlue),
                        Cells::VBrk => ('¦', Color::LightBlue),
                        Cells::PlMin => ('±', Color::LightBlue),
                        Cells::SmZer => ('ø', Color::LightBlue),
                        Cells::BZer => ('Ø', Color::LightBlue),
                        Cells::Cop => ('©', Color::LightRed),
                        Cells::DblBracedGate => ('Ħ', Color::DarkGray),
                        Cells::BracedGate => ('ỻ', Color::DarkGray),
                        Cells::Arch => ('Π', Color::DarkGray),
                        Cells::Bricks => ('ʭ', Color::DarkGray),
                        Cells::Crops => ('ʬ', Color::Yellow),
                        Cells::SmallCampfire => ('ѧ', Color::LightRed),
                        Cells::Campfire => ('Ѧ', Color::LightRed),
                        Cells::Table => ('π', Color::DarkGray),
                        Cells::Jar => ('ṑ', Color::DarkGray),
                        Cells::Chair => ('⑁', Color::DarkGray),
                        Cells::Firewood => ('ж', Color::Red),
                        Cells::Tent => ('Ʌ', Color::Gray),
                        Cells::LBrce => {
                            if ani_cnt % 2 == 0 {
                                ('{', Color::LightBlue)
                            } else {
                                ('{', Color::Magenta)
                            }
                        }
                        Cells::RBrce => {
                            if ani_cnt % 2 == 0 {
                                ('}', Color::LightBlue)
                            } else {
                                ('}', Color::Magenta)
                            }
                        }
                        Cells::LParen => {
                            if ani_cnt % 2 == 0 {
                                ('(', Color::Magenta)
                            } else {
                                ('(', Color::Red)
                            }
                        }
                        Cells::RParen => {
                            if ani_cnt % 2 == 0 {
                                (')', Color::Magenta)
                            } else {
                                (')', Color::Red)
                            }
                        }
                        Cells::GenCur => {
                            if ani_cnt % 2 == 0 {
                                ('¤', Color::Red)
                            } else {
                                ('¤', Color::Yellow)
                            }
                        }
                        Cells::Water => {
                            let aa = (ani_cnt as usize + jy) + (ix + ani_cnt as usize / 2);
                            // log::info!("aaaaa {}", aa);
                            if aa % 6 == 0 {
                                ('~', Color::White)
                            } else {
                                ('~', Color::LightBlue)
                            }
                        }
                        // Cells::Null => ('#', Color::Red),
                        _ => (' ', Color::Red),
                    }
                }
            };
            let span = Span::styled(symbol.to_string(), Style::new().fg(color));
            line.push(span);
        }
        let line: Line = Line::from(line);
        text.push(line);
    }
    let texts: Text<'a> = text.into_iter().collect();
    Paragraph::new(texts).block(
        Block::default()
            .borders(Borders::NONE)
            .padding(Padding {
                left: 1,
                right: 1,
                top: 1,
                bottom: 1,
            })
            .style(Style::default().bg(Color::Black)),
    )
}
