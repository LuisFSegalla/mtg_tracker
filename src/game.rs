use crate::deck::Deck;
use std::{fmt, write};


# [derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Format {
    Pauper,
    Modern,
} 

impl fmt::Display for Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Format::Modern => write!(f, "Modern"),
            Format::Pauper => write!(f, "Pauper"),
        }
    }
}


# [derive(Debug, Clone, PartialEq, Eq)]
pub enum Order {
    Play,
    Draw
}

# [derive(Debug, Clone)]
pub struct Game {
    pub format: Format,
    pub p_deck: Deck,
    pub p_mull: i32,
    pub p_order: Order,
    pub opp_deck: Deck,
    pub win: bool,
}