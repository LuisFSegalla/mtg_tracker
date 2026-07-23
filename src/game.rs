use crate::deck::Deck;
use crate::player::Player;

# [derive(Debug, Clone)]
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
    pub opp_mull: i32,
    pub opp_order: Order,
    pub win: bool,
}