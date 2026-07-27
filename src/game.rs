# [derive(Debug, Clone, PartialEq, Eq)]
pub enum Order {
    Play,
    Draw
}

# [derive(Debug, Clone)]
pub struct Game {
    pub format: String,
    pub p_deck: String,
    pub p_mull: i32,
    pub p_order: Order,
    pub opp_deck: String,
    pub win: bool,
}