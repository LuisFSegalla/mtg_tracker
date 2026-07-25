mod player;
mod deck;
mod game;

use deck::{Deck, Format};
use player::Player;
use std::{collections::HashMap, println, vec};

use crate::game::{Game, Order};

fn main() {

    let d1: Deck = Deck{
        name: "Cycle Storm".to_string(),
        format: Format::Pauper,
    };

    let d2: Deck = Deck{
        name: "Mono red madness".to_string(),
        format: Format::Pauper,
    };

    let d3: Deck = Deck{
        name: "Izzet Prowess".to_string(),
        format: Format::Modern,
    };

    let d4: Deck = Deck{
        name: "Dimir terror".to_string(),
        format: Format::Pauper,
    };

    let mut p1: Player = Player {
        name: "Luis".to_string(),
        decks: vec![],
        win_per_deck: HashMap::from([]),
        games_per_deck: HashMap::from([]),
        game_history: vec![]
    };

    println!("{p1}");

    p1.decks.sort_by_key(|d| d.name.clone());

    println!("{p1}");

    let game1: Game = Game { 
        p_deck: d1.clone(), 
        p_mull: 5, 
        p_order: Order::Draw,
        opp_deck: d4.clone(), 
        opp_mull: 7, 
        opp_order: Order::Play, 
        win: false 
    };

    let game2: Game = Game { 
        p_deck: d1.clone(), 
        p_mull: 5, 
        p_order: Order::Draw,
        opp_deck: d4.clone(), 
        opp_mull: 7, 
        opp_order: Order::Play, 
        win: false 
    };

    let game3: Game = Game { 
        p_deck: d1.clone(), 
        p_mull: 5, 
        p_order: Order::Draw,
        opp_deck: d4.clone(), 
        opp_mull: 7, 
        opp_order: Order::Play, 
        win: true 
    };

    let game4: Game = Game { 
        p_deck: d2.clone(), 
        p_mull: 5, 
        p_order: Order::Draw,
        opp_deck: d4.clone(), 
        opp_mull: 7, 
        opp_order: Order::Play, 
        win: true 
    };

    let game5: Game = Game { 
        p_deck: d2.clone(), 
        p_mull: 5, 
        p_order: Order::Draw,
        opp_deck: d4.clone(), 
        opp_mull: 7, 
        opp_order: Order::Play, 
        win: true 
    };

    p1.add_game(&game1);
    p1.add_game(&game2);
    p1.add_game(&game3);
    p1.add_game(&game4);
    p1.add_game(&game5);
 
    let w = p1.calculate_win_rate();
    println!("{:?}",w);

}
