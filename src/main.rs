mod player;
mod game;

use player::Player;
use std::{println, vec};

use crate::game::{Game, Order};
use serde_json;

use rusqlite::{params, Connection, Result};


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
    p1.add_game(&game4);
    p1.add_game(&game5);
 
    // println!("{}",p1);

    // p1.get_deck_stats("Cycle Storm".to_string()).unwrap();

    let conn = Connection::open("/workspaces/mtg_tracker/my_db.db3")?;
    println!("{}", conn.is_autocommit());
    conn.execute(
        "CREATE TABLE IF NOT EXISTS player (
            id   INTEGER PRIMARY KEY,
            data BLOB
        )",
        (), // empty list of parameters.
    )?;


    let json_data = serde_json::to_vec(&p1)?;

    conn.execute(
        "INSERT INTO player (data) VALUES (?1)",
        params![json_data],
    )?;

     
    let mut stmt = conn.prepare("SELECT id, data FROM player")?;
    let player_iter = stmt.query_map([], |row| {
        Ok((
            row.get::<_, i32>(0)?,    // id (column 0)
            row.get::<_, Vec<u8>>(1)?,   // data (column 1)
        ))
    })?;

    for player in player_iter {
        let (id,data) = player?;
        let deserialized_data: Player = serde_json::from_slice(&data)?;
        println!("Read from id {} and got: {}",id, deserialized_data);
    }

    Ok(())
}
