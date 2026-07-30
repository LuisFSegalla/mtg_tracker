mod app;
mod database;
mod game;
mod player;
mod ui;

use player::Player;
use game::{Game, Order, Format};
use database::*;
use app::App;
use ui::ui;

use serde_json;

// Possible exit fro the main function
use rusqlite::{Connection, Result};

// TUI rendering library
use ratatui::{
    Terminal,
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
        execute,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
    },
};

use std::{error::Error, io::{self, Stdout}};

use crate::app::{CurrentScreen, CurrentlyEditingGame, CurrentlyEditingPlayer};

fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Creating database
    let conn: Connection = create_table()?;


    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app, &conn);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    match res {
        Err(err) => panic!("Program exited with err: {:?}",err),
        _ => println!("Program finished nicely")
    }   

    Ok(())
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<Stdout>>, app: &mut App, db: &Connection) -> Result<(), Box<dyn Error>>
{
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                // Skip events that are not KeyEventKind::Press
                continue;
            }
            match app.current_screen {
                // Main screen
                // Display players in the database and moves to addPlayer/addGame/Display
                CurrentScreen::Main => match key.code {
                    KeyCode::Char('p') => {
                        app.current_screen = CurrentScreen::AddPlayer;
                        app.current_editing_player = Some(CurrentlyEditingPlayer::PlayerName);
                    }
                    KeyCode::Char('e') => {
                        app.current_screen = CurrentScreen::AddGame;
                        app.current_editing_game = Some(CurrentlyEditingGame::Format);
                    }
                    KeyCode::Char('d') => {
                        app.current_screen = CurrentScreen::Display;
                    }
                    KeyCode::Char('q') => {
                        app.current_screen = CurrentScreen::Exiting;
                    }
                    _ => {}
                },
                // Add game info that will be added to a player database
                CurrentScreen::AddGame if key.kind == KeyEventKind::Press => match key.code {
                    // Check every character and places it in the current selected space
                    // ToDo:
                    // * Add binary choices for win
                    KeyCode::Char(value) => match app.current_editing_game {
                        Some(CurrentlyEditingGame::PlayerDeck) => {
                            app.p_deck.push(value);
                        }
                        Some(CurrentlyEditingGame::PlayerMull) => {
                            let mull = value.to_digit(10).unwrap() as u8;
                            if mull > 7 {
                                app.p_mull = 7;
                            } else {
                                app.p_mull = mull;
                            }
                        }
                        Some(CurrentlyEditingGame::OpponentDeck) => {
                            app.opp_deck.push(value);
                        }
                        _ => {}
                    }
                    KeyCode::Backspace => match app.current_editing_game {
                        Some(CurrentlyEditingGame::PlayerDeck) => {
                            app.p_deck.pop();
                        }
                        Some(CurrentlyEditingGame::PlayerMull) => {
                            app.p_mull = 0;
                        }
                        Some(CurrentlyEditingGame::OpponentDeck) => {
                            app.opp_deck.pop();
                        }
                        Some(CurrentlyEditingGame::Win) => {
                            app.win = false;
                        }
                        _ => {}
                    }
                    KeyCode::Enter => {
                        app.current_screen = CurrentScreen::Main;
                        app.current_editing_player = None;
                        app.current_editing_game = None;
                        let g: Game = Game {
                            format: match app.format {
                                Format::Modern => "Modern".to_string(),
                                Format::Pauper => "Pauper".to_string(),
                            },
                            p_deck: app.p_deck.clone(),
                            p_mull: app.p_mull.clone(), 
                            p_order: app.p_order.clone(), 
                            opp_deck: app.p_deck.clone(), 
                            win: app.win.clone()
                        };
                        // Player exists in our app and database
                        if player_exists(db, &app.player_name.to_lowercase())?  {
                            update_player(&g, &app.player_name.to_lowercase(), db)?;
                        }
                        else {
                            app.current_screen = CurrentScreen::ErrorPLayerNotFound;
                        }
                    }
                    KeyCode::Esc => {
                        app.current_screen = CurrentScreen::Main;
                        app.current_editing_player = None;
                        app.current_editing_game = None;
                    }
                    KeyCode::Tab => match app.current_editing_game {
                        Some(CurrentlyEditingGame::Format) => {
                            app.current_editing_game = Some(CurrentlyEditingGame::PlayerDeck)
                        }
                        Some(CurrentlyEditingGame::PlayerDeck) => {
                            app.current_editing_game = Some(CurrentlyEditingGame::PlayerMull)
                        }
                        Some(CurrentlyEditingGame::PlayerMull) => {
                            app.current_editing_game = Some(CurrentlyEditingGame::PlayerOrder)
                        }
                        Some(CurrentlyEditingGame::PlayerOrder) => {
                            app.current_editing_game = Some(CurrentlyEditingGame::OpponentDeck)
                        }
                        Some(CurrentlyEditingGame::OpponentDeck) => {
                            app.current_editing_game = Some(CurrentlyEditingGame::Win)
                        }
                        Some(CurrentlyEditingGame::Win) => {
                            app.current_editing_game = Some(CurrentlyEditingGame::Format)
                        }
                        _ => {}
                    }
                    KeyCode::Up => match app.current_editing_game {
                        Some(CurrentlyEditingGame::Format) => match app.format {
                            Format::Modern => app.format = Format::Pauper,
                            Format::Pauper => app.format = Format::Modern,
                        }
                        Some(CurrentlyEditingGame::PlayerOrder) => match app.p_order {
                            Order::Draw => app.p_order = Order::Play,
                            Order::Play => app.p_order = Order::Draw,
                        }
                        Some(CurrentlyEditingGame::Win) => match app.win {
                            true => app.win = false,
                            false => app.win = true,
                        }
                        _ => {}
                    }
                    KeyCode::Down => match app.current_editing_game {
                        Some(CurrentlyEditingGame::Format) => match app.format {
                            Format::Modern => app.format = Format::Pauper,
                            Format::Pauper => app.format = Format::Modern,
                        }
                        Some(CurrentlyEditingGame::PlayerOrder) => match app.p_order {
                            Order::Draw => app.p_order = Order::Play,
                            Order::Play => app.p_order = Order::Draw,
                        }
                        Some(CurrentlyEditingGame::Win) => match app.win {
                            true => app.win = false,
                            false => app.win = true,
                        }
                        _ => {}
                    }
                    _ => {}
                },
                // Add a player to the database;
                // If player not already in the database create a new entry for it
                // Adding a player requires the user to input the player name only;
                // Todo:
                // * Add logic to add player to the database if not present and to warn if player already present
                CurrentScreen::AddPlayer if key.kind == KeyEventKind::Press =>
                {
                    match key.code {
                        KeyCode::Enter => {
                            app.current_screen = CurrentScreen::Main;
                            app.current_editing_player = None;
                            app.current_editing_game = None;
                            // Adding the player to the database
                            // but keeping track of all the players that were added
                            // in the app so we can have a list
                            let p: Player = Player::new(
                                 app.player_name.to_lowercase().clone(), 
                                 vec![], 
                            );
                            let _ = add_player(db, &p);
                            let p = get_player_names(db)?;
                            app.add_player(&p);
                        }
                        KeyCode::Backspace => {
                            app.player_name.pop();
                        }
                        KeyCode::Esc => {
                            app.current_screen = CurrentScreen::Main;
                            app.current_editing_player = None;
                            app.current_editing_game = None;
                            app.player_name = "".to_string();
                        }
                        KeyCode::Char(value) => {
                            app.player_name.push(value);
                        }
                        _ => {}
                    }
                }
                CurrentScreen::Exiting => match key.code {
                    KeyCode::Char('y') => {
                        return Ok(());
                    }
                    KeyCode::Char('n') | KeyCode::Char('q') => {
                        return Ok(());
                    }
                    _ => {}
                },
                CurrentScreen::Display => match key.code {
                    KeyCode::Esc => {
                        app.current_screen = CurrentScreen::Main;
                        app.current_editing_player = None;
                        app.current_editing_game = None;
                        app.player_name = "".to_string();
                    }
                    KeyCode::Char('n') | KeyCode::Char('q') => {
                        return Ok(())
                    }
                    _ => {}
                },
                CurrentScreen::ErrorPLayerNotFound => match key.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        app.current_screen = CurrentScreen::Main;
                        app.current_editing_player = None;
                        app.current_editing_game = None;
                    }
                    _ => {}
                }

                _ => {}
            }
        }
    }
}
