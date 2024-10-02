//settlements
use crate::enums::{Settle};
use crate::settlement::{Settlement};
use std::collections::HashMap;

pub struct Settlements {
    settlements: HashMap<(i128, i128), Settlement>,
}

impl Settlements {
    pub fn new() -> Self {
        let settlements = HashMap::new();
        Self {settlements}
    }

    pub fn demo_self() -> Self {
        let xb = 300 - 75;
        let yb = 200 - 25;
        let mut settlements = HashMap::new();
        let npcs = HashMap::new();
        let demo_settle = Settlement::demo_settle((xb, yb), npcs);
        settlements.insert((xb, yb), demo_settle);
        Self {settlements}
    }

}
