use crate::deck::Deck;
use std::{fmt, write};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Player {
    pub name: String,
    pub deck: Vec<Deck>,
    pub win_rate: HashMap<Deck, f64>,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Player: {}\n", self.name)?;
        for d in &self.deck {
            write!(f, "Deck: {}\n", d.name)?;
            write!(f, "winrate: {}\n", self.win_rate.get(d).unwrap_or(&0.0))?;
        }
        Ok(())
    }
}

// impl Player {
//     fn calculate_all_win_rate(&mut self) {
 
//     }

// }