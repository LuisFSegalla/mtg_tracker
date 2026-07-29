mod player;
mod game;

use player::Player;
use std::{error::Error, println, vec};

use crate::game::{Game, Order};
use serde_json;

use rusqlite::{params, Connection, Result};

fn player_exists(conn: &Connection, key: &String) -> Result<bool>{
    println!("Checking if {} is in the Database.",key);
    conn.query_row(
        "SELECT EXISTS(SELECT name FROM player WHERE name = ?1)",
        params![key],
        |row|{
            let exists: i32 =  row.get(0)?;
            Ok(exists != 0)
        }
    )

}

fn create_table() -> Result<Connection, Box<dyn Error>> {
    let conn = Connection::open("/workspaces/mtg_tracker/my_db.db3")?;
    println!("{}", conn.is_autocommit());
    conn.execute(
        "CREATE TABLE IF NOT EXISTS player (
            id   INTEGER PRIMARY KEY,
            name TEXT NON NULL UNIQUE,
            data BLOB
        )",
        (), // empty list of parameters.
    )?;
    return Ok(conn);

}

fn retrieve_player(conn: &Connection, key: &String) -> Result<Player, Box<dyn Error>>{
    let data: Vec<u8> = conn.query_row(
        "SELECT data FROM player WHERE name = ?1", 
        params![key], 
        |r| r.get(0),
    )?;
    let p: Player = serde_json::from_slice(&data)?;
    Ok(p)
}

fn update_player(game: &Game, player: &String, conn: &Connection) -> Result<(), Box<dyn Error>>{
    let mut p: Player = retrieve_player(conn, player)?;
    p.add_game(game);
    let json_data: Vec<u8> = serde_json::to_vec(&p)?;
    conn.execute(
        "INSERT INTO player (name, data) VALUES (?1,?2)
            ON CONFLICT(name) DO UPDATE SET data = excluded.data",
        (&player, &json_data),
    )?;    
    Ok(())
}

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
