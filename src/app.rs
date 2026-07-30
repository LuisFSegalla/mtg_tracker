use crate::game::{Game, Order, Format};
use std::{io, string};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
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
    ErrorPLayerNotFound,
    DeckSelector,
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
    pub deck_index: usize,
    pub current_screen: CurrentScreen,
    pub current_editing_game: Option<CurrentlyEditingGame>,
    pub current_editing_player: Option<CurrentlyEditingPlayer>,
    pub format: Format,
    pub p_deck: String,
    pub p_mull: u8,
    pub p_order: Order,
    pub opp_deck: String,
    pub win: bool,
}

impl App {
    pub fn new() -> App {
        App {
            player_name: "".to_string(),
            vec_players: vec![],
            decks: vec![],
            deck_index: 0,
            current_screen: CurrentScreen::Main,
            current_editing_game: None,
            current_editing_player: None,
            format: Format::Pauper,
            p_deck: "".to_string(),
            p_mull: 0,
            p_order: Order::Draw,
            opp_deck: "".to_string(),
            win: true,
        }
    }
    pub fn add_player(&mut self, players: &Vec<String>) {
        self.vec_players.clear();
        for p in players.iter() {
            self.vec_players.push(p.clone());
        }
    }
}
