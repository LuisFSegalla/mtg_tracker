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

    let mut p1: Player = Player::new(
         "Luis".to_string(), 
         vec![], 
         vec![], 
         HashMap::from([])
    );

    println!("{p1}");

    p1.decks.sort_by_key(|d| d.name.clone());

    println!("{p1}");

    let game1: Game = Game { 
        p_deck: d1.clone(), 
        p_mull: 5, 
        p_order: Order::Draw,
        opp_deck: d4.clone(), 
        win: false 
    };

    let game2: Game = Game { 
        p_deck: d1.clone(), 
        p_mull: 5, 
        p_order: Order::Draw,
        opp_deck: d4.clone(), 
        win: false 
    };

    let game3: Game = Game { 
        p_deck: d1.clone(), 
        p_mull: 5, 
        p_order: Order::Draw,
        opp_deck: d4.clone(), 
        win: true 
    };

    let game4: Game = Game { 
        p_deck: d1.clone(), 
        p_mull: 5, 
        p_order: Order::Draw,
        opp_deck: d2.clone(), 
        win: false
    };

    let game5: Game = Game { 
        p_deck: d2.clone(), 
        p_mull: 5, 
        p_order: Order::Draw,
        opp_deck: d4.clone(), 
        win: true 
    };

    p1.add_game(&game1);
    p1.add_game(&game2);
    p1.add_game(&game3);
    p1.add_game(&game4);
    p1.add_game(&game5);
 
    println!("{:?}",p1.win_rate);

    p1.get_deck_stats(d1).unwrap();

}
