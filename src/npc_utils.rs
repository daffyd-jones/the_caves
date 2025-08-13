//npc utils
use crate::enums::{Cells, NPCWrap, NPCs};
use crate::npc::{BaseNPC, NPC};
use crate::utils::COLLISION_CELLS;
use rand::Rng;

pub fn n_collision(dir: &str, pos: (usize, usize), cells: Vec<Vec<Cells>>) -> bool {
    match dir {
        "UP" => COLLISION_CELLS.contains(&cells[pos.1 - 1][pos.0]),
        "DN" => COLLISION_CELLS.contains(&cells[pos.1 + 1][pos.0]),
        "LF" => COLLISION_CELLS.contains(&cells[pos.1][pos.0 - 1]),
        "RT" => COLLISION_CELLS.contains(&cells[pos.1][pos.0 + 1]),
        _ => false,
    }
}

pub fn npc_move(
    mut npc: Box<dyn NPC>,
    map: Vec<Vec<Cells>>,
    mw: usize,
    mh: usize,
    x: usize,
    y: usize,
) -> ((usize, usize), Box<dyn NPC>) {
    let mut rng = rand::thread_rng();
    let dch = rng.gen_range(0..20);
    if dch % 5 == 0 {
        npc.set_steps(dch);
    }
    let pos = if npc.get_steps() < 5 {
        npc.inc_steps();
        // if y == 0 {(x, y)} else {
        if y <= 10 || n_collision("UP", npc.get_pos(), map.clone()) {
            (x, y)
        } else {
            npc.mmove("UP");
            (x, y - 1)
        }
    } else if npc.get_steps() >= 5 && npc.get_steps() < 10 {
        npc.inc_steps();
        // if x == 0 {(x, y)} else {
        if x <= 10 || n_collision("LF", npc.get_pos(), map.clone()) {
            (x, y)
        } else {
            npc.mmove("LF");
            (x - 1, y)
        }
    } else if npc.get_steps() >= 10 && npc.get_steps() < 15 {
        npc.inc_steps();
        // if y >= mh-5 {(x, y)} else {
        if y >= mh - 10 || n_collision("DN", npc.get_pos(), map.clone()) {
            (x, y)
        } else {
            npc.mmove("DN");
            (x, y + 1)
        }
    } else if npc.get_steps() >= 15 && npc.get_steps() < 20 {
        npc.inc_steps();
        // if x >= mw-5 {(x, y)} else {
        if x >= mw - 10 || n_collision("RT", npc.get_pos(), map.clone()) {
            (x, y)
        } else {
            npc.mmove("RT");
            (x + 1, y)
        }
    } else if npc.get_steps() == 20 {
        npc.set_steps(0);
        (x, y)
    } else {
        (x, y)
    };
    (pos, npc)
    // (pos, Box::new(npc))
}

pub fn box_npc(npc: NPCWrap) -> Box<dyn NPC> {
    match npc {
        NPCWrap::CommNPC(comm_npc) => Box::new(comm_npc),
        NPCWrap::ConvNPC(conv_npc) => Box::new(conv_npc),
        // NPCWrap::ShopNPC(shop_npc) => Box::new(shop_npc),
        NPCWrap::SpawnNPC(spawn_npc) => Box::new(spawn_npc),
        NPCWrap::TradeNPC(trade_npc) => Box::new(trade_npc),
        _ => todo!(),
    }
}

pub fn wrap_nbox(mut nbox: Box<dyn NPC>) -> NPCWrap {
    match nbox.get_ntype() {
        NPCs::CommNPC => {
            if let Some(comm_npc) = nbox.as_comm_npc() {
                NPCWrap::CommNPC(comm_npc.clone())
            } else {
                NPCWrap::BaseNPC(BaseNPC::new())
            }
        }
        NPCs::ConvNPC => {
            if let Some(conv_npc) = nbox.as_conv_npc() {
                NPCWrap::ConvNPC(conv_npc.clone())
            } else {
                NPCWrap::BaseNPC(BaseNPC::new())
            }
        }
        NPCs::ShopNPC => {
            if let Some(shop_npc) = nbox.as_shop_npc() {
                NPCWrap::ShopNPC(shop_npc.clone())
            } else {
                NPCWrap::BaseNPC(BaseNPC::new())
            }
        }
        NPCs::SpawnNPC => {
            if let Some(spawn_npc) = nbox.as_spawn_npc() {
                NPCWrap::SpawnNPC(spawn_npc.clone())
            } else {
                NPCWrap::BaseNPC(BaseNPC::new())
            }
        }
        NPCs::TradeNPC => {
            if let Some(trade_npc) = nbox.as_trade_npc() {
                NPCWrap::TradeNPC(trade_npc.clone())
            } else {
                NPCWrap::BaseNPC(BaseNPC::new())
            }
        }
        _ => todo!(),
    }
}

pub fn shift_npc(npc: NPCWrap, pos: (usize, usize)) -> NPCWrap {
    let mut nbox = box_npc(npc);
    nbox.set_pos(pos);
    wrap_nbox(nbox)
}
