mod player;
mod deck;
mod game;

use deck::{Deck};
use player::Player;
use std::{println, vec};

use crate::game::{Game, Order, Format};

fn main() {

    let d1: Deck = Deck{
        name: "Cycle Storm".to_string(),
    };

    let d2: Deck = Deck{
        name: "Mono red madness".to_string(),
    };

    let _d3: Deck = Deck{
        name: "Izzet Prowess".to_string(),
    };

    let d4: Deck = Deck{
        name: "Dimir terror".to_string(),
    };

    let mut p1: Player = Player::new(
         "Luis".to_string(), 
         vec![], 
    );

    println!("{p1}");

    p1.decks.sort_by_key(|d| d.name.clone());

    println!("{p1}");

    let game1: Game = Game { 
        format: Format::Pauper,
        p_deck: d1.clone(), 
        p_mull: 5, 
        p_order: Order::Draw,
        opp_deck: d4.clone(), 
        win: false 
    };

    let game2: Game = Game { 
        format: Format::Pauper,
        p_deck: d1.clone(), 
        p_mull: 5, 
        p_order: Order::Draw,
        opp_deck: d4.clone(), 
        win: false 
    };

    let game3: Game = Game { 
        format: Format::Pauper,
        p_deck: d1.clone(), 
        p_mull: 5, 
        p_order: Order::Draw,
        opp_deck: d4.clone(), 
        win: true 
    };

    let game4: Game = Game { 
        format: Format::Pauper,
        p_deck: d1.clone(), 
        p_mull: 5, 
        p_order: Order::Draw,
        opp_deck: d2.clone(), 
        win: false
    };

    let game5: Game = Game { 
        format: Format::Pauper,
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
