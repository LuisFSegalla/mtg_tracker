use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
};

use crate::{
    app::{App, CurrentScreen, CurrentlyEditingGame, CurrentlyEditingPlayer},
    game::{Format, Order},
    player,
};

pub fn ui(frame: &mut Frame, app: &App) {
    //Define my main area to be rendered
    let chunks: std::rc::Rc<[Rect]> = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(frame.area());

    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        "MTG Stats tracker",
        Style::default().fg(Color::Green),
    ))
    .block(title_block);

    frame.render_widget(title, chunks[0]);

    // Layout for showing players and decks
    let inner_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(80),
        ])
        .margin(2)
        .split(chunks[1]);

    render_player_layout(app, frame, &[inner_layout[0]]);
    render_deck_layout(app, frame, &[inner_layout[1]]);

    render_footnotes(app, frame, &chunks);

    // When editing a game will go over every option highlighting it
    if let Some(editing) = &app.current_editing_game {
        let popup_block = Block::default()
            .title("Enter Game information")
            .borders(Borders::NONE)
            .style(Style::default().bg(Color::DarkGray));

        let area = centered_rect(25, 60, frame.area());
        frame.render_widget(popup_block, area);

        let popup_chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Percentage(15),
                Constraint::Percentage(15),
                Constraint::Percentage(15),
                Constraint::Percentage(15),
                Constraint::Percentage(15),
                Constraint::Percentage(15),
            ])
            .split(area);

        let mut format_block = Block::default().title("Format").borders(Borders::ALL);
        let mut p_deck_block = Block::default().title("Player deck").borders(Borders::ALL);
        let mut p_mull_block = Block::default()
            .title("Player mulligan")
            .borders(Borders::ALL);
        let mut p_order_block = Block::default().title("Player order").borders(Borders::ALL);
        let mut opp_deck_block = Block::default()
            .title("Opponent deck")
            .borders(Borders::ALL);
        let mut result_block = Block::default().title("Match Result").borders(Borders::ALL);

        let active_style = Style::default().bg(Color::LightYellow).fg(Color::Black);

        match editing {
            CurrentlyEditingGame::Format => format_block = format_block.style(active_style),
            CurrentlyEditingGame::PlayerDeck => p_deck_block = p_deck_block.style(active_style),
            CurrentlyEditingGame::PlayerMull => p_mull_block = p_mull_block.style(active_style),
            CurrentlyEditingGame::PlayerOrder => p_order_block = p_order_block.style(active_style),
            CurrentlyEditingGame::OpponentDeck => {
                opp_deck_block = opp_deck_block.style(active_style)
            }
            CurrentlyEditingGame::Win => result_block = result_block.style(active_style),
        }

        let format_text = Paragraph::new(match app.format {
            Format::Modern => "Modern".to_string(),
            Format::Pauper => "Pauper".to_string(),
        })
        .block(format_block);
        let p_deck_text = Paragraph::new(app.p_deck.clone()).block(p_deck_block);
        let p_mull_text = Paragraph::new(app.p_mull.clone().to_string()).block(p_mull_block);
        let opp_deck_text = Paragraph::new(app.opp_deck.clone()).block(opp_deck_block);
        let p_order_text = Paragraph::new(match app.p_order {
            Order::Draw => "Draw".to_string(),
            Order::Play => "Play".to_string(),
        })
        .block(p_order_block);
        let result_text = Paragraph::new(match app.win {
            true => "Win",
            false => "Lose",
        })
        .block(result_block);

        frame.render_widget(format_text, popup_chunks[0]);
        frame.render_widget(p_deck_text, popup_chunks[1]);
        frame.render_widget(p_mull_text, popup_chunks[2]);
        frame.render_widget(p_order_text, popup_chunks[3]);
        frame.render_widget(opp_deck_text, popup_chunks[4]);
        frame.render_widget(result_text, popup_chunks[5]);
    }

    // Edit player name in the app
    if let Some(editing) = &app.current_editing_player {
        match editing {
            CurrentlyEditingPlayer::PlayerName => {
                let popup_block = Block::default()
                    .title("Enter Player information")
                    .borders(Borders::NONE)
                    .style(Style::default().bg(Color::DarkGray));

                let area = centered_rect(50, 10, frame.area());
                frame.render_widget(popup_block, area);

                let popup_chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints([Constraint::Percentage(100)])
                    .split(area);

                let name_block = Block::default().title("Player name").borders(Borders::ALL);
                let name_text = Paragraph::new(app.player_name.clone()).block(name_block);
                frame.render_widget(name_text, popup_chunks[0]);
            }
            CurrentlyEditingPlayer::LoadPlayer => {
                let popup_block = Block::default()
                    .title("Player to load from the database")
                    .borders(Borders::NONE)
                    .style(Style::default().bg(Color::LightBlue));

                let area = centered_rect(50, 10, frame.area());
                frame.render_widget(popup_block, area);

                let popup_chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints([Constraint::Percentage(100)])
                    .split(area);

                let name_block = Block::default().title("Player name").borders(Borders::ALL);
                let name_text = Paragraph::new(app.player_name.clone()).block(name_block);
                frame.render_widget(name_text, popup_chunks[0]);
            }
            _ => {}
        }
    }

    if let CurrentScreen::Display = app.current_screen {
        render_display(&app, frame);
    }

    if let CurrentScreen::DeckSelector = app.current_screen {
        render_deck_selector(&app, frame, &inner_layout);
    }

    // Basic exit screen - Copied from the example in ratatui JSON editor
    if let CurrentScreen::Exiting = app.current_screen {
        frame.render_widget(Clear, frame.area()); //this clears the entire screen and anything already drawn
        let popup_block = Block::default()
            .title("Y/N")
            .borders(Borders::NONE)
            .style(Style::default().bg(Color::DarkGray));

        let exit_text = Text::styled("Leaving MTG tracker.", Style::default().fg(Color::Red));
        // the `trim: false` will stop the text from being cut off when over the edge of the block
        let exit_paragraph = Paragraph::new(exit_text)
            .block(popup_block)
            .wrap(Wrap { trim: false });

        let area = centered_rect(60, 25, frame.area());
        frame.render_widget(exit_paragraph, area);
    }

    if let CurrentScreen::ErrorPLayerNotFound = app.current_screen {
        frame.render_widget(Clear, frame.area()); //this clears the entire screen and anything already drawn
        let popup_block = Block::default()
            .title("ERROR")
            .borders(Borders::NONE)
            .style(Style::default().bg(Color::Red));

        let exit_text = Text::styled(
            "Error while trying to add a game to a player not in the database",
            Style::default().fg(Color::Black),
        );
        // the `trim: false` will stop the text from being cut off when over the edge of the block
        let exit_paragraph = Paragraph::new(exit_text)
            .block(popup_block)
            .wrap(Wrap { trim: false });

        let area = centered_rect(60, 25, frame.area());
        frame.render_widget(exit_paragraph, area);
    }
}

fn render_display(app: &App, frame: &mut Frame) {
    let popup_block = Block::default()
        .title("Game info")
        .borders(Borders::NONE)
        .style(Style::default().bg(Color::DarkGray));

    let area = centered_rect(25, 60, frame.area());
    frame.render_widget(popup_block, area);

    let popup_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Percentage(15),
            Constraint::Percentage(15),
            Constraint::Percentage(15),
            Constraint::Percentage(15),
            Constraint::Percentage(15),
            Constraint::Percentage(15),
            Constraint::Percentage(15),
        ])
        .split(area);

    let format_block = Block::default().title("Format").borders(Borders::ALL);
    let p_deck_block = Block::default().title("Player deck").borders(Borders::ALL);
    let p_mull_block = Block::default()
        .title("Player mulligan")
        .borders(Borders::ALL);
    let p_order_block = Block::default().title("Player order").borders(Borders::ALL);
    let opp_deck_block = Block::default()
        .title("Opponent deck")
        .borders(Borders::ALL);
    let result_block = Block::default().title("Match Result").borders(Borders::ALL);

    let format_text = Paragraph::new(match app.format {
        Format::Modern => "Modern".to_string(),
        Format::Pauper => "Pauper".to_string(),
    })
    .block(format_block);
    let p_order_text = Paragraph::new(match app.p_order {
        Order::Draw => "Draw".to_string(),
        Order::Play => "Play".to_string(),
    })
    .block(p_order_block);
    let p_deck_text = Paragraph::new(app.p_deck.clone()).block(p_deck_block);
    let p_mull_text = Paragraph::new(app.p_mull.clone().to_string()).block(p_mull_block);
    let opp_deck_text = Paragraph::new(app.opp_deck.clone()).block(opp_deck_block);
    let result_text = Paragraph::new(match app.win {
        true => "Win",
        false => "Lose",
    })
    .block(result_block);

    frame.render_widget(format_text, popup_chunks[0]);
    frame.render_widget(p_deck_text, popup_chunks[1]);
    frame.render_widget(p_mull_text, popup_chunks[2]);
    frame.render_widget(p_order_text, popup_chunks[3]);
    frame.render_widget(opp_deck_text, popup_chunks[4]);
    frame.render_widget(result_text, popup_chunks[5]);
}

// Renders the footnotes on the screen
// Showing which commands are available in each screen
fn render_footnotes(app: &App, frame: &mut Frame, chunks: &[Rect]) {
    let current_navigation_text = vec![
        // The first half of the text
        match app.current_screen {
            CurrentScreen::Main => Span::styled("Normal Mode", Style::default().fg(Color::Green)),
            CurrentScreen::Player => {
                Span::styled("Player select Mode", Style::default().fg(Color::Red))
            }
            CurrentScreen::Stats => {
                Span::styled("Deck stats Mode", Style::default().fg(Color::Blue))
            }
            CurrentScreen::Editing => {
                Span::styled("Editing Game Mode", Style::default().fg(Color::White))
            }
            CurrentScreen::Exiting => Span::styled("Exiting", Style::default().fg(Color::LightRed)),
            CurrentScreen::AddGame => {
                Span::styled("Adding a new game", Style::default().fg(Color::Green))
            }
            CurrentScreen::AddPlayer => {
                Span::styled("Adding a new player", Style::default().fg(Color::Blue))
            }
            CurrentScreen::Display => Span::styled(
                "Dispaly Game/Player information",
                Style::default().fg(Color::Blue),
            ),
            CurrentScreen::ErrorPLayerNotFound => {
                Span::styled("Error screen", Style::default().fg(Color::Blue))
            }
            CurrentScreen::DeckSelector => {
                Span::styled("Deck selector screen", Style::default().fg(Color::Blue))
            }
            CurrentScreen::LoadPlayer => Span::styled(
                "Loading player from database screen",
                Style::default().fg(Color::Blue),
            ),
        }
        .to_owned(),
        // A white divider bar to separate the two sections
        Span::styled(" | ", Style::default().fg(Color::White)),
        // The final section of the text, with hints on what the user is editing
        {
            if let Some(editing) = &app.current_editing_game {
                match editing {
                    CurrentlyEditingGame::Format => Span::styled(
                        "Adding format (Pauper/Mordern)",
                        Style::default().fg(Color::Green),
                    ),
                    CurrentlyEditingGame::PlayerDeck => {
                        Span::styled("Adding your deck name", Style::default().fg(Color::Green))
                    }
                    CurrentlyEditingGame::PlayerMull => Span::styled(
                        "Adding number of cards in your starting hand",
                        Style::default().fg(Color::Green),
                    ),
                    CurrentlyEditingGame::PlayerOrder => Span::styled(
                        "Adding your order (Play/Draw)",
                        Style::default().fg(Color::Green),
                    ),
                    CurrentlyEditingGame::OpponentDeck => Span::styled(
                        "Adding opponent deck name",
                        Style::default().fg(Color::Green),
                    ),
                    CurrentlyEditingGame::Win => {
                        Span::styled("Adding game result", Style::default().fg(Color::Green))
                    }
                }
            } else if app.current_editing_player.is_some() {
                Span::styled(
                    "Adding a player to the database",
                    Style::default().fg(Color::Green),
                )
            } else {
                Span::styled("Not Editing Anything", Style::default().fg(Color::DarkGray))
            }
        },
    ];

    let mode_footer = Paragraph::new(Line::from(current_navigation_text))
        .block(Block::default().borders(Borders::ALL));

    let current_keys_hint = {
        match app.current_screen {
            CurrentScreen::Main => Span::styled(
                "(q) to quit / (e) Enter a player",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::Player => Span::styled(
                "(ESC) to cancel/ (e) to add a game/ (s) to get status",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::Stats => {
                Span::styled("(ESC) to cancel", Style::default().fg(Color::Red))
            }
            CurrentScreen::Editing => Span::styled(
                "(ESC) to cancel/ (e) to add a game/ (s) to get status",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::Exiting => Span::styled(
                "(ESC) to cancel/ (s) to save into database",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::AddGame => Span::styled(
                "(ESC) to cancel/ (Enter) to save into database",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::AddPlayer => Span::styled(
                "(ESC) to cancel/ (Enter) to save into database",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::Display => Span::styled(
                "(ESC) to cancel/ (q) back to main screen",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::ErrorPLayerNotFound => Span::styled(
                "(ESC) / (q) back to main screen",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::DeckSelector => Span::styled(
                "(ESC) / (q) back deck editing screen",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::LoadPlayer => Span::styled(
                "(ESC) back deck editing screen / (ENTER) to load from database",
                Style::default().fg(Color::Red),
            ),
        }
    };

    let key_notes_footer =
        Paragraph::new(Line::from(current_keys_hint)).block(Block::default().borders(Borders::ALL));

    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[2]);

    frame.render_widget(mode_footer, footer_chunks[0]);
    frame.render_widget(key_notes_footer, footer_chunks[1]);
}

// Render the deck selector screen to facilitate choosing among previously selected decks
fn render_deck_selector(app: &App, frame: &mut Frame, area: &[Rect]) {
    let active_style = Style::default().bg(Color::LightYellow).fg(Color::Black);
    let mut decks_list = Vec::<ListItem>::new();

    //Iterate over players list
    for d in app.decks.iter() {
        if d.eq(&app.p_deck) {
            decks_list.push(ListItem::new(Line::from(Span::styled(
                format!("{: <25}", d),
                active_style,
            ))));
        } else {
            decks_list.push(ListItem::new(Line::from(Span::styled(
                format!("{: <25}", d),
                Style::default().fg(Color::Yellow),
            ))));
        }
    }

    let d_list = List::new(decks_list);
    frame.render_widget(d_list, area[1]);
}

/// Copied from the example JSON editor example from ratatui docs
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}

fn render_player_layout(app: &App, frame: &mut Frame, area: &[Rect]) {
    let player_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(10), Constraint::Percentage(90)])
        .margin(2)
        .split(area[0]);

    let player_title_block = Block::bordered().style(Style::default());
    let player_title = Paragraph::new(Text::styled(
        "Player name",
        Style::default().fg(Color::LightBlue),
    ))
    .block(player_title_block);
    frame.render_widget(player_title, player_layout[0]);

    // ToDo: Add list of Decks per player making it possible to select decks with arrow key
    let mut player_list = Vec::<ListItem>::new();

    //Iterate over players list
    for p in app.vec_players.iter() {
        player_list.push(ListItem::new(Line::from(Span::styled(
            format!("{: <25}", p),
            Style::default().fg(Color::Yellow),
        ))));
    }

    let player_list_block = Block::bordered().style(Style::default().fg(Color::DarkGray));

    let p_list = List::new(player_list).block(player_list_block);

    frame.render_widget(p_list, player_layout[1]);
}

fn render_deck_layout(app: &App, frame: &mut Frame, area: &[Rect]) {
    let deck_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(10), Constraint::Percentage(90)])
        .margin(2)
        .split(area[0]);

    let decks_title_block = Block::bordered().style(Style::default());
    let decks_title = Paragraph::new(Text::styled(
        "Player decks",
        Style::default().fg(Color::LightBlue),
    ))
    .block(decks_title_block);
    frame.render_widget(decks_title, deck_layout[0]);

    // ToDo: Add list of Decks per player making it possible to select decks with arrow key
    let mut decks_list = Vec::<ListItem>::new();

    for d in app.decks.iter() {
        decks_list.push(ListItem::new(Line::from(Span::styled(
            format!("{: <25}", d),
            Style::default().fg(Color::Red),
        ))));
    }

    let deck_list_block = Block::bordered().style(Style::default().fg(Color::DarkGray));

    let d_list = List::new(decks_list).block(deck_list_block);

    frame.render_widget(d_list, deck_layout[1]);
}
