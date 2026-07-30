use crate::game::{Game, Order};
use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};

pub enum CurrentScreen {
    Main,
    Player,
    Editing,
    Stats,
    Exiting,
    AddPlayer,
    AddGame,
    Display,
}

pub enum CurrentlyEditingGame {
    Format,
    PlayerDeck,
    PlayerMull,
    PlayerOrder,
    OpponentDeck,
    Win,
}

pub enum CurrentlyEditingPlayer {
    PlayerName,
}

pub struct App {
    pub player_name: String,
    pub vec_players: Vec<String>,
    pub decks: Vec<String>,
    pub current_screen: CurrentScreen,
    pub current_editing_game: Option<CurrentlyEditingGame>,
    pub current_editing_player: Option<CurrentlyEditingPlayer>,
    pub format: String,
    pub p_deck: String,
    pub p_mull: u8,
    pub p_order: String,
    pub opp_deck: String,
    pub win: bool,
}

impl App {
    pub fn new() -> App {
        App {
            player_name: "".to_string(),
            vec_players: vec![],
            decks: vec![],
            current_screen: CurrentScreen::Main,
            current_editing_game: None,
            current_editing_player: None,
            format: "".to_string(),
            p_deck: "".to_string(),
            p_mull: 0, 
            p_order: "".to_string(), 
            opp_deck: "".to_string(),
            win: true
        }
    }
    pub fn add_player(&mut self) {
        self.vec_players.push(self.player_name.clone());
        self.player_name.clear();
    }

}