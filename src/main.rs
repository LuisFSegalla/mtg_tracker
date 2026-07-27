mod player;
mod game;

use player::Player;
use std::{println, vec};

use crate::game::{Game, Order};


fn main() -> Result<(), Box<dyn std::error::Error>>{

    let mut p1: Player = Player::new(
         "Luis".to_string(), 
         vec![], 
    );

    println!("{p1}");

    let game1: Game = Game { 
        format: "Pauper".to_string(),
        p_deck: "Cycle Storm".to_string(), 
        p_mull: 5, 
        p_order: Order::Draw,
        opp_deck: "Dimir terror".to_string(), 
        win: false 
    };

    let game2: Game = Game { 
        format: "Pauper".to_string(),
        p_deck: "Cycle Storm".to_string(), 
        p_mull: 5, 
        p_order: Order::Draw,
        opp_deck: "Dimir terror".to_string(), 
        win: false 
    };

    let game3: Game = Game { 
        format: "Pauper".to_string(),
        p_deck: "Cycle Storm".to_string(), 
        p_mull: 5, 
        p_order: Order::Draw,
        opp_deck: "Dimir terror".to_string(),
        win: true 
    };

    let game4: Game = Game { 
        format: "Pauper".to_string(),
        p_deck: "Cycle Storm".to_string(), 
        p_mull: 5, 
        p_order: Order::Draw,
        opp_deck: "Mono red madness".to_string(), 
        win: false
    };

    let game5: Game = Game { 
        format: "Pauper".to_string(),
        p_deck: "Mono red madness".to_string(), 
        p_mull: 5, 
        p_order: Order::Draw,
        opp_deck: "Dimir terror".to_string(), 
        win: true 
    };

    p1.add_game(&game1);
    p1.add_game(&game2);
    p1.add_game(&game3);
    p1.add_game(&game4);
    p1.add_game(&game5);
 
    println!("{:?}",p1.win_rate);

    p1.get_deck_stats("Cycle Storm".to_string()).unwrap();
    Ok(())
}
