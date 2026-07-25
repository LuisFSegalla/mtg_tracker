use crate::deck::Deck;

# [derive(Debug, Clone, PartialEq, Eq)]
pub enum Order {
    Play,
    Draw
}

# [derive(Debug, Clone)]
pub struct Game {
    pub p_deck: Deck,
    pub p_mull: i32,
    pub p_order: Order,
    pub opp_deck: Deck,
    pub win: bool,
}