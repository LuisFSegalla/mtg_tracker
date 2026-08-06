use crate::game::Game;
use crate::player::Player;
use log::{info};
use rusqlite::{Connection, Result, params};
use std::{error::Error};

pub fn player_exists(conn: &Connection, key: &String) -> Result<bool> {
    info!("Checking if {} is in the Database.", key);
    conn.query_row(
        "SELECT EXISTS(SELECT name FROM player WHERE name = ?1)",
        params![key],
        |row| {
            let exists: i32 = row.get(0)?;
            Ok(exists != 0)
        },
    )
}

pub fn create_table() -> Result<Connection, Box<dyn Error>> {
    let conn = Connection::open("/tmp/my_db.db3")?;
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

pub fn load_player(conn: &Connection, key: &String) -> Result<Player, Box<dyn Error>> {
    let data: Vec<u8> = conn.query_row(
        "SELECT data FROM player WHERE name = ?1",
        params![key],
        |r| r.get(0),
    )?;
    let p: Player = serde_json::from_slice(&data)?;
    Ok(p)
}

pub fn add_player(conn: &Connection, p: &Player) -> Result<(), Box<dyn Error>> {
    let json_data: Vec<u8> = serde_json::to_vec(&p)?;

    if !player_exists(&conn, &p.name)? {
        info!("Adding new player {}", p.name);
        conn.execute(
            "INSERT INTO player (name, data) VALUES (?1,?2)",
            (&p.name, &json_data),
        )?;
    }
    Ok(())
}

pub fn update_player(
    game: &Game,
    player: &String,
    conn: &Connection,
) -> Result<(), Box<dyn Error>> {
    let mut p: Player = load_player(conn, player)?;
    p.add_game(game);
    let json_data: Vec<u8> = serde_json::to_vec(&p)?;
    conn.execute(
        "INSERT INTO player (name, data) VALUES (?1,?2)
            ON CONFLICT(name) DO UPDATE SET data = excluded.data",
        (&player, &json_data),
    )?;
    Ok(())
}

pub fn get_player_names(conn: &Connection) -> Result<Vec<String>, Box<dyn Error>> {
    info!("Returning all players in the database.");
    let mut stmt = conn.prepare("SELECT name FROM player")?;
    let keys: Vec<String> = stmt
        .query_map([], |row| row.get::<_, String>(0))?
        .collect::<Result<Vec<_>, _>>()?;
    return Ok(keys);
}
