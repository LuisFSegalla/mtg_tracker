use crate::game::{Game, Order};

use std::collections::HashMap;
use std::{fmt, println, vec, write};

use log::info;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub decks: Vec<String>,
    pub game_history: Vec<Game>,
    pub win_rate: HashMap<String, f64>,
    win_per_deck: HashMap<String, i32>,
    games_per_deck: HashMap<String, i32>,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Player: {}\n", self.name)?;
        for d in &self.decks {
            write!(f, "Deck: {}\n", d)?;
            write!(f, "Wins: {}\n", self.win_per_deck.get(d).unwrap_or(&0))?;
            write!(
                f,
                "Total games: {}\n",
                self.games_per_deck.get(d).unwrap_or(&0)
            )?;
            write!(
                f,
                "Average win rate: {}\n",
                self.win_rate.get(d).unwrap_or(&0.0)
            )?;
        }
        Ok(())
    }
}

impl Player {
    pub fn new(name: String, decks: Vec<String>) -> Self {
        Player {
            name: name,
            decks: decks,
            game_history: vec![],
            win_rate: HashMap::from([]),
            win_per_deck: HashMap::from([]),
            games_per_deck: HashMap::from([]),
        }
    }

    pub fn add_game(&mut self, game: &Game) {
        if !self.decks.contains(&game.p_deck) {
            info!("Adding {} to the decks list.", game.p_deck);
            self.decks.push(game.p_deck.clone());
            self.games_per_deck.insert(game.p_deck.clone(), 1);
            self.win_per_deck.insert(game.p_deck.clone(), 0);
        } else {
            *self.games_per_deck.entry(game.p_deck.clone()).or_insert(0) += 1;
        }
        if game.win {
            *self.win_per_deck.entry(game.p_deck.clone()).or_insert(0) += 1;
        }
        self.game_history.push(game.clone());
        self.calculate_deck_win_rate(&game.p_deck);
    }

    pub fn calculate_deck_win_rate(&mut self, deck: &String) {
        let games = *self.games_per_deck.get(deck).unwrap() as f64;
        let wins = *self.win_per_deck.get(deck).unwrap() as f64;
        *self.win_rate.entry(deck.clone()).or_insert(0.0) = (wins / games) as f64;
    }

    pub fn get_deck_stats(&mut self, deck: String) -> Result<(), String> {
        if !self.decks.contains(&deck) {
            Err("Deck not registered for this {self.name}".to_string())
        } else {
            self.game_history.sort_by_key(|g| g.p_deck == deck);

            // Sort the match history with a certain deck
            // and split it into a separate array to be worked on;
            let games_with_deck: Vec<Game> = self
                .game_history
                .iter()
                .filter(|val| val.p_deck == deck)
                .cloned()
                .collect();

            let avr_mull = (games_with_deck.iter().fold(0, |acc, x| acc + x.p_mull) as f64)
                / (games_with_deck.len() as f64);

            let mut played_against: HashMap<String, i32> = HashMap::from([]);
            let mut order_vs: HashMap<String, i32> = HashMap::from([]);
            let mut wins: HashMap<String, i32> = HashMap::from([]);
            for g in games_with_deck.iter() {
                // Iterate over the hash map adding a match against each deck
                *played_against.entry(g.opp_deck.clone()).or_insert(0) += 1;

                if g.p_order == Order::Play {
                    *order_vs.entry(g.opp_deck.clone()).or_insert(0) += 1;
                }

                if g.win {
                    *wins.entry(g.opp_deck.clone()).or_insert(0) += 1;
                } else {
                    let _ = *wins.entry(g.opp_deck.clone()).or_insert(0);
                }
            }
            println!(
                "Average mull for {} with {} is {}",
                self.name, deck, avr_mull
            );
            println!(
                "Number of games played with the deck: {}",
                games_with_deck.len()
            );
            println!("Decks Played against: {:?}", played_against);
            println!("Most order: {:?}", order_vs);
            println!("wins: {:?}", wins);

            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_empty_player_creation() {
        let p: Player = Player::new("Player1".to_string(), vec![]);
        assert!(p.decks.len() == 0);
        assert!(p.game_history.len() == 0);
    }

    #[test]
    fn test_filled_player_creation() {
        let game1: Game = Game {
            format: "Pauper".to_string(),
            p_deck: "Cycle Storm".to_string(),
            p_mull: 5,
            p_order: Order::Draw,
            opp_deck: "Mono red madness".to_string(),
            win: false,
        };

        let mut p: Player = Player::new("Player1".to_string(), vec![]);

        p.add_game(&game1);

        assert!(p.decks.len() == 1);
        assert!(p.game_history.len() == 1);
    }

    #[test]
    fn test_add_game() {
        let mut p: Player = Player::new("Player1".to_string(), vec![]);

        let game2: Game = Game {
            format: "Modern".to_string(),
            p_deck: "Izzet Prowess".to_string(),
            p_mull: 5,
            p_order: Order::Draw,
            opp_deck: "UW Belcher".to_string(),
            win: true,
        };

        p.add_game(&game2);

        assert!(p.decks.len() == 1);
        assert!(p.game_history.len() == 1);
        assert!(p.win_per_deck.get(&"Izzet Prowess".to_string()).unwrap() == &1);
        assert!(p.win_rate.get(&"Izzet Prowess".to_string()).unwrap() == &1.0);
    }
}
