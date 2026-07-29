mod player;
mod game;
mod database;

use player::Player;
use game::{Game, Order};
use database::*;

use serde_json;

use rusqlite::Result;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let mut p1: Player = Player::new(
         "Luis".to_string(), 
         vec![], 
    );

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
    // p1.add_game(&game4);
    // p1.add_game(&game5);

    let conn = create_table()?;


    let json_data: Vec<u8> = serde_json::to_vec(&p1)?;

    if !player_exists(&conn, &p1.name)? {
        println!("Player {} not in the database. Inserting it.", p1.name);
        conn.execute(
            "INSERT INTO player (name, data) VALUES (?1,?2)",
            (&p1.name, &json_data),
        )?;
    }
    update_player(&game4, &p1.name, &conn)?;
    update_player(&game5, &p1.name, &conn)?;

    let mut p: Player = retrieve_player(&conn, &p1.name.clone())?;
    let _ = p.get_deck_stats("Cycle Storm".to_string());
    Ok(())

}
