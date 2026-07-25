use crate::deck::Deck;
use crate::game::{self, Game, Order};

use core::panic;
use std::{fmt, println, write};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Player {
    pub name: String,
    pub decks: Vec<Deck>,
    pub game_history: Vec<Game>,
    pub win_rate: HashMap<Deck, f64>,
    win_per_deck: HashMap<Deck, i32>,
    games_per_deck: HashMap<Deck, i32>,

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
    pub fn new(name: String, decks: Vec<Deck>, games: Vec<Game>, win_rate: HashMap<Deck, f64>) -> Self{
        Player {
                name: name,
                decks: decks,
                game_history: games,
                win_rate: win_rate,
                win_per_deck: HashMap::from([]),
                games_per_deck: HashMap::from([]),
            }
    }

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
        self.calculate_deck_win_rate(&game.p_deck);
    }

    pub fn calculate_deck_win_rate(&mut self, deck: &Deck) {
        let games = *self.games_per_deck.get(deck).unwrap() as f64;
        let wins = *self.win_per_deck.get(deck).unwrap() as f64;
        *self.win_rate.entry(deck.clone()).or_insert(0.0) = (wins / games) as f64;
    }


    pub fn calculate_all_win_rate(&mut self)  ->HashMap<Deck,f64> {
        let mut win_rates: HashMap<Deck, f64> = HashMap::from([]);
        for d in self.decks.iter_mut() {
            let games = *self.games_per_deck.get(d).unwrap() as f64;
            let wins = *self.win_per_deck.get(d).unwrap() as f64;
            let win_rate = (wins / games) as f64;
            win_rates.insert(d.clone(), win_rate);
        }
        return win_rates;
    }

    pub fn get_deck_stats(&mut self, deck: Deck) -> Result<(), String>{
        if !self.decks.contains(&deck) {
            Err("Deck not registered for this {self.name}".to_string())
        }
        else {
            self.game_history.sort_by_key(|g| g.p_deck.name == deck.name);
            
            // Sort the match history with a certain deck
            // and split it into a separate array to be worked on;
            let games_with_deck: Vec<Game> = 
            self.game_history.iter()
            .filter(|val| val.p_deck.name == deck.name)
            .cloned()
            .collect();

            let avr_mull = (
                games_with_deck
                .iter()
                .fold(0, |acc, x| acc + x.p_mull)
                as f64
            ) / (games_with_deck.len() as f64);
            println!("Average mull for {} is {}", self.name, avr_mull);

            let mut played_against:HashMap<Deck, i32> =  HashMap::from([]);
            let mut order_vs: HashMap<Deck, i32> = HashMap::from([]);
            let mut wins: HashMap<Deck, i32> = HashMap::from([]); 
            for g in games_with_deck.iter() {
                // Iterate over the hash map adding a match against each deck
                *played_against
                .entry(g.opp_deck.clone())
                .or_insert(0) += 1;

                if g.p_order == Order::Play {
                    *order_vs
                    .entry(g.opp_deck.clone())
                    .or_insert(0) += 1;
                } 
                else {
                    let _ = *order_vs
                    .entry(g.opp_deck.clone())
                    .or_insert(0);
                    
                }

                if g.win {
                    *wins
                    .entry(g.opp_deck.clone())
                    .or_insert(0) += 1;
                } 
                else {
                    let _ = *wins
                    .entry(g.opp_deck.clone())
                    .or_insert(0);
                    
                }

            }
            
            println!("Decks Played against: {:?}",played_against);
            println!("Most order: {:?}",order_vs);
            println!("wins: {:?}",wins);

            Ok(())
        }
    }


}