mod player;
mod deck;
mod game;

use deck::{Deck, Format};
use player::Player;
use std::{collections::HashMap, println};

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
    println!("{d1}");
    println!("{d2}");
    println!("{d3}");
    println!("{d4}");
 
    let p1: Player = Player {
        name: "Luis".to_string(),
        deck: vec![d1.clone(), d3.clone(), d4.clone()],
        win_rate: HashMap::from([
            (d1.clone(), 0.0),
            (d2.clone(), 0.0),
            (d3.clone(), 0.0),
        ]),
    };
    println!("{p1}")
}
