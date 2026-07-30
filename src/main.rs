mod player;
mod game;
mod database;
mod app;
mod ui;

// use player::Player;
use game::{Game, Order};
// use database::*;
use app::{App};
use ui::ui;

use serde_json;

// Possible exit fro the main function
use rusqlite::Result;

// TUI rendering library
use ratatui::{
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    Terminal,
};

use std::{error::Error, io};

use crate::app::{CurrentScreen, CurrentlyEditingGame, CurrentlyEditingPlayer};


fn main() -> Result<(), Box<dyn std::error::Error>>{

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;


    Ok(())


}


fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> 
where
    io::Error: From<B::Error>,

{
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                // Skip events that are not KeyEventKind::Press
                continue;
            }
            match app.current_screen {
                CurrentScreen::Main =>
                    match key.code {
                    KeyCode::Char('p') => {
                        app.current_screen = CurrentScreen::Player;
                    }
                    KeyCode::Char('e') => {
                        app.current_screen = CurrentScreen::Editing;
                    }
                    KeyCode::Char('d') => {
                        app.current_screen = CurrentScreen::Display;
                    }
                    KeyCode::Char('q') => {
                        app.current_screen = CurrentScreen::Exiting;
                    }
                    _ => {}
                }
                CurrentScreen::Editing =>
                    match key.code {
                    KeyCode::Char('p') => {
                        app.current_screen = CurrentScreen::AddPlayer;
                        app.current_editing_player = Some(CurrentlyEditingPlayer::PlayerName);
                    }
                    KeyCode::Char('q') => {
                        app.current_screen = CurrentScreen::Exiting;
                    }
                    _ => {}
                }

                CurrentScreen::Player => 
                    match key.code {
                    KeyCode::Char('e') => {
                        app.current_screen = CurrentScreen::Player;
                    }
                    KeyCode::Char('q') => {
                        app.current_screen = CurrentScreen::Exiting;
                    }
                        
                        _ => {}
                    }

                CurrentScreen::AddGame if key.kind == KeyEventKind::Press =>
                    match key.code {
                        KeyCode::Enter => {
                            app.current_screen = CurrentScreen::Main;
                            app.current_editing_player = None;
                            app.current_editing_game = None;

                        }
                        KeyCode::Esc => {
                            app.current_screen = CurrentScreen::Main;
                            app.current_editing_player = None;
                            app.current_editing_game = None;

                        }
                        KeyCode::Char('1') => {
                            app.current_editing_game = Some(CurrentlyEditingGame::Format);

                        }

                        _ => {}
                        
                    }

                CurrentScreen::AddPlayer if key.kind == KeyEventKind::Press =>
                // Adding a player requires the user to input the player name only;
                // Todo:
                // * Add logic to add player to the database if not present and to warn if player already present
                    match key.code {
                        KeyCode::Enter => {
                            app.current_screen = CurrentScreen::Main;
                            app.current_editing_player = None;
                            app.current_editing_game = None;
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
                CurrentScreen::Stats =>
                    match key.code {
                    KeyCode::Char('e') => {
                        app.current_screen = CurrentScreen::Exiting;
                    }
                    KeyCode::Char('q') => {
                        app.current_screen = CurrentScreen::Exiting;
                    }
                    _ => {}
                }
                CurrentScreen::Exiting =>
                    match key.code {
                    KeyCode::Char('y') => {
                        return Ok(true);
                    }
                    KeyCode::Char('n') | KeyCode::Char('q') => {
                        return Ok(false);
                    }
                    _ => {}
                }                
                CurrentScreen::Display =>
                    match key.code {
                        KeyCode::Esc => {
                            app.current_screen = CurrentScreen::Main;
                            app.current_editing_player = None;
                            app.current_editing_game = None;
                            app.player_name = "".to_string();
                        }
                        KeyCode::Char('n') | KeyCode::Char('q') => {
                            return Ok(false);
                        }
                    _ => {}
                }
                _ => {}

        }

    }
    }
}