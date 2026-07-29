use rusqlite::{params, Connection, Result};
use std::{error::Error, println};
use crate::player::Player;
use crate::game::Game;


pub fn player_exists(conn: &Connection, key: &String) -> Result<bool>{
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

pub fn create_table() -> Result<Connection, Box<dyn Error>> {
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

pub fn retrieve_player(conn: &Connection, key: &String) -> Result<Player, Box<dyn Error>>{
    let data: Vec<u8> = conn.query_row(
        "SELECT data FROM player WHERE name = ?1", 
        params![key], 
        |r| r.get(0),
    )?;
    let p: Player = serde_json::from_slice(&data)?;
    Ok(p)
}

pub fn update_player(game: &Game, player: &String, conn: &Connection) -> Result<(), Box<dyn Error>>{
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
