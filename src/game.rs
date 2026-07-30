use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Order {
    Play,
    Draw,
}

pub enum Format {
    Pauper,
    Modern
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    pub format: String,
    pub p_deck: String,
    pub p_mull: u8,
    pub p_order: Order,
    pub opp_deck: String,
    pub win: bool,
}
