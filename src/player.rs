use crate::deck::Deck;
use crate::game::Game;

use std::{fmt, println, write};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Player {
    pub name: String,
    pub decks: Vec<Deck>,
    pub win_per_deck: HashMap<Deck, i32>,
    pub games_per_deck: HashMap<Deck, i32>,
    pub game_history: Vec<Game>
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Player: {}\n", self.name)?;
        for d in &self.decks {
            write!(f, "Deck: {}\n", d.name)?;
            write!(f, "Wins: {}\n", self.win_per_deck.get(d).unwrap_or(&0))?;
            write!(f, "Total games: {}\n", self.games_per_deck.get(d).unwrap_or(&0))?;

        }
        Ok(())
    }
}


impl Player {
    pub fn add_game(&mut self, game: &Game) {
        if !self.decks.contains(&game.p_deck) {
            println!("Adding {} to the decks list.", game.p_deck.name);
            self.decks.push(game.p_deck.clone());
            self.games_per_deck.insert(game.p_deck.clone(), 1);
            *self.win_per_deck.entry(game.p_deck.clone()).or_insert(0) = 0;
        }
        else {
            *self.games_per_deck.entry(game.p_deck.clone()).or_insert(0) += 1;
        }
        if game.win {
            *self.win_per_deck.entry(game.p_deck.clone()).or_insert(0) += 1;
        }
        self.game_history.push(game.clone());
    }

    pub fn calculate_win_rate(&mut self)  ->HashMap<Deck,f64> {
        let mut win_rates: HashMap<Deck, f64> = HashMap::from([]);
        for d in self.decks.iter_mut() {
            let games = *self.games_per_deck.get(d).unwrap() as f64;
            let wins = *self.win_per_deck.get(d).unwrap() as f64;
            let win_rate = (wins / games) as f64;
            win_rates.insert(d.clone(), win_rate);
        }
        return win_rates;
    }

}