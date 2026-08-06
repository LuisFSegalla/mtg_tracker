use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect, Alignment},
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
    render_deck_layout(app, frame, &[inner_layout[1]], false);

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
        }
    }

    if let CurrentScreen::Display = app.current_screen {
        render_deck_layout(app, frame, &[inner_layout[1]], true);
        render_display(&app, frame, &[inner_layout[2]]);
    }

    if let CurrentScreen::DeckSelector = app.current_screen {
        render_deck_layout(app, frame, &[inner_layout[1]], true);
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

    if let CurrentScreen::HelpScreen = app.current_screen {
        let popup_block = Block::default()
            .title(Span::styled("Available commands", Style::default().fg(Color::Black)))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray))
            .style(Style::default()
            .bg(Color::LightYellow)
        );
        let area = centered_rect(40, 25, frame.area());

        let lines = vec![
            "(p) Add a player to the database",
            "(l) Load a player from the database",
            "(g) Add a game for current player",
            "(s) Select a deck",
            "(d) Display selected deck stats"
        ];

        let text = Text::from(
            lines
                .into_iter()
                .map(|l| Line::from(Span::styled(l, Style::default().fg(Color::Red))))
                .collect::<Vec<_>>(),
        );

        // the `trim: false` will stop the text from being cut off when over the edge of the block
        let exit_paragraph = Paragraph::new(text)
            .block(popup_block).alignment(Alignment::Center)
            .wrap(Wrap { trim: false });

            frame.render_widget(exit_paragraph, area);
    }


}

fn render_display(app: &App, frame: &mut Frame, area: &[Rect]) {
    let stats_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(10), Constraint::Percentage(90)])
        .margin(2)
        .split(area[0]);

    let width = stats_layout[0].width.saturating_sub(2) as usize;
    let fmt = format!(
        "{:^width$}",
        format!("Deck: {} stats", app.p_deck),
        width = width.clone()
    );
    let decks_title_block = Block::bordered().style(Style::default());
    let decks_title = Paragraph::new(Text::styled(fmt, Style::default().fg(Color::LightBlue)))
        .block(decks_title_block);
    frame.render_widget(decks_title, stats_layout[0]);

    // Split the larger part of the block into smaller columns for each stat.
    let stats_table = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(14), // decks
            Constraint::Percentage(14), // Num Games
            Constraint::Percentage(18), // P/D
            Constraint::Percentage(18), // Wins
            Constraint::Percentage(18), // Win rate
            Constraint::Percentage(18), // Avr mulligan
        ])
        .split(stats_layout[1]);

    // For each column add a title and the stats
    let deck_column = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(10), Constraint::Percentage(90)])
        .split(stats_table[0]);

    let num_games_column = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(10), Constraint::Percentage(90)])
        .split(stats_table[1]);

    let play_draw_column = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(10), Constraint::Percentage(90)])
        .split(stats_table[2]);

    let wins_column = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(10), Constraint::Percentage(90)])
        .split(stats_table[3]);

    let win_rate_column = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(10), Constraint::Percentage(90)])
        .split(stats_table[4]);

    let mulls_column = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(10), Constraint::Percentage(90)])
        .split(stats_table[5]);

    // if Player has a deck selected we'll show the stats
    if !app.p_deck.is_empty() && !app.player.is_empty() {
        let p = app.player.get(0).unwrap();
        match p.get_deck_stats(app.p_deck.clone()) {
            Ok(stats) => {
                let mut deck_names = Vec::<ListItem>::new();
                let mut deck_num_games = Vec::<ListItem>::new();
                let mut play_draw = Vec::<ListItem>::new();
                let mut wins = Vec::<ListItem>::new();
                let mut mulls = Vec::<ListItem>::new();
                let mut wr = Vec::<ListItem>::new();

                for (d, n) in stats.deck_played_against.iter() {
                    // Adding deck names list
                    let mut text_width = deck_column[1].width.saturating_sub(2) as usize;
                    let mut format =
                        format!("{:^width$}", format!("{}", d), width = text_width.clone());
                    deck_names.push(ListItem::new(Line::from(Span::styled(
                        format.clone(),
                        Style::default().fg(Color::Red),
                    ))));

                    // Adding number of games played against decks
                    text_width = num_games_column[1].width.saturating_sub(2) as usize;
                    format = format!("{:^width$}", format!("{}", n), width = text_width.clone());
                    deck_num_games.push(ListItem::new(Line::from(Span::styled(
                        format!("{: <25}", format.clone()),
                        Style::default().fg(Color::Red),
                    ))));

                    // Adding Play/Draw order
                    let _p = stats.play_draw_order.get(d).unwrap();
                    let _d = (_p - n).abs();
                    text_width = play_draw_column[1].width.saturating_sub(2) as usize;
                    format = format!(
                        "{:^width$}",
                        format!("{}/{}", _p, _d),
                        width = text_width.clone()
                    );
                    play_draw.push(ListItem::new(Line::from(Span::styled(
                        format!("{}", format.clone()),
                        Style::default().fg(Color::Red),
                    ))));

                    // Adding number of wins against the deck
                    let _w = stats.wins_against.get(d).unwrap();
                    text_width = wins_column[1].width.saturating_sub(2) as usize;
                    format = format!("{:^width$}", format!("{}", _w), width = text_width.clone());
                    wins.push(ListItem::new(Line::from(Span::styled(
                        format!("{}", format.clone()),
                        Style::default().fg(Color::Red),
                    ))));

                    // Adding number of wins against the deck
                    let _mull = stats.avr_mull.get(d).unwrap();
                    text_width = mulls_column[1].width.saturating_sub(2) as usize;
                    format = format!(
                        "{:^width$}",
                        format!("{}", _mull),
                        width = text_width.clone()
                    );
                    mulls.push(ListItem::new(Line::from(Span::styled(
                        format!("{}", format.clone()),
                        Style::default().fg(Color::Red),
                    ))));

                    // Adding win rate of deck
                    let _wr = stats.win_rate.get(d).unwrap();
                    text_width = win_rate_column[1].width.saturating_sub(2) as usize;
                    format = format!("{:^width$}", format!("{}", _wr), width = text_width.clone());
                    wr.push(ListItem::new(Line::from(Span::styled(
                        format!("{}", format.clone()),
                        Style::default().fg(Color::Red),
                    ))));
                }

                let deck_names_block = Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::DarkGray));
                let deck_num_games_block = Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::DarkGray));
                let deck_play_draw_block = Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::DarkGray));
                let deck_wins_block = Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::DarkGray));
                let deck_avr_mull_block = Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::DarkGray));
                let deck_win_rate_block = Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::DarkGray));

                let deck_names_list = List::new(deck_names).block(deck_names_block);
                let deck_num_games_list = List::new(deck_num_games).block(deck_num_games_block);
                let deck_play_draw_list = List::new(play_draw).block(deck_play_draw_block);
                let deck_wins_list = List::new(wins).block(deck_wins_block);
                let deck_avr_mull_list = List::new(mulls).block(deck_avr_mull_block);
                let deck_win_rate_list = List::new(wr).block(deck_win_rate_block);

                // Render the title for the deck column
                let deck_column_title_width = deck_column[0].width.saturating_sub(2) as usize;
                let text = format!(
                    "{:^width$}",
                    format!("Deck"),
                    width = deck_column_title_width.clone()
                );
                let deck_column_name_block = Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::DarkGray));
                let paragraph = Paragraph::new(Span::styled(
                    format!("{}", text.clone()),
                    Style::default().fg(Color::Red),
                ))
                .block(deck_column_name_block);
                frame.render_widget(paragraph, deck_column[0]);

                // Render the title for the number of games column
                let num_games_column_title_width =
                    num_games_column[0].width.saturating_sub(2) as usize;
                let text = format!(
                    "{:^width$}",
                    format!("Number of games"),
                    width = num_games_column_title_width.clone()
                );
                let num_games_column_name_block = Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::DarkGray));
                let paragraph = Paragraph::new(Span::styled(
                    format!("{}", text.clone()),
                    Style::default().fg(Color::Red),
                ))
                .block(num_games_column_name_block);
                frame.render_widget(paragraph, num_games_column[0]);

                // Render the title for the number of games column
                let play_draw_column_title_width =
                    play_draw_column[0].width.saturating_sub(2) as usize;
                let text = format!(
                    "{:^width$}",
                    format!("Play / Draw"),
                    width = play_draw_column_title_width.clone()
                );
                let play_draw_column_name_block = Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::DarkGray));
                let paragraph = Paragraph::new(Span::styled(
                    format!("{}", text.clone()),
                    Style::default().fg(Color::Red),
                ))
                .block(play_draw_column_name_block);
                frame.render_widget(paragraph, play_draw_column[0]);

                // Render the number of wins against the deck
                let wins_column_title_width = wins_column[0].width.saturating_sub(2) as usize;
                let text = format!(
                    "{:^width$}",
                    format!("Wins"),
                    width = wins_column_title_width.clone()
                );
                let wins_column_name_block = Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::DarkGray));
                let paragraph = Paragraph::new(Span::styled(
                    format!("{}", text.clone()),
                    Style::default().fg(Color::Red),
                ))
                .block(wins_column_name_block);
                frame.render_widget(paragraph, wins_column[0]);

                // Render the average mulligan against the deck
                let mulls_column_title_width = mulls_column[0].width.saturating_sub(2) as usize;
                let text = format!(
                    "{:^width$}",
                    format!("Average mulligan"),
                    width = mulls_column_title_width.clone()
                );
                let mulls_column_name_block = Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::DarkGray));
                let paragraph = Paragraph::new(Span::styled(
                    format!("{}", text.clone()),
                    Style::default().fg(Color::Red),
                ))
                .block(mulls_column_name_block);
                frame.render_widget(paragraph, mulls_column[0]);

                // Render the average win rate against the deck
                let wr_column_title_width = win_rate_column[0].width.saturating_sub(2) as usize;
                let text = format!(
                    "{:^width$}",
                    format!("Average win rate"),
                    width = wr_column_title_width.clone()
                );
                let wr_column_name_block = Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::DarkGray));
                let paragraph = Paragraph::new(Span::styled(
                    format!("{}", text.clone()),
                    Style::default().fg(Color::Red),
                ))
                .block(wr_column_name_block);
                frame.render_widget(paragraph, win_rate_column[0]);

                // Renders the stats for each deck
                frame.render_widget(deck_names_list, deck_column[1]);
                frame.render_widget(deck_num_games_list, num_games_column[1]);
                frame.render_widget(deck_play_draw_list, play_draw_column[1]);
                frame.render_widget(deck_wins_list, wins_column[1]);
                frame.render_widget(deck_avr_mull_list, mulls_column[1]);
                frame.render_widget(deck_win_rate_list, win_rate_column[1]);
            }
            Err(err) => {
                frame.render_widget(Clear, frame.area()); //this clears the entire screen and anything already drawn
                let popup_block = Block::default()
                    .title("ERROR")
                    .borders(Borders::NONE)
                    .style(Style::default().bg(Color::Red));

                let exit_text = Text::styled(err, Style::default().fg(Color::Black));
                // the `trim: false` will stop the text from being cut off when over the edge of the block
                let exit_paragraph = Paragraph::new(exit_text)
                    .block(popup_block)
                    .wrap(Wrap { trim: false });

                let area = centered_rect(60, 25, frame.area());
                frame.render_widget(exit_paragraph, area);
            }
        }
    }
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
            CurrentScreen::HelpScreen => Span::styled(
                "Available commands from the main screen",
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
                "(q) to quit / (h) open available commands menu",
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
                "(ESC) / (q) back to main screen",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::LoadPlayer => Span::styled(
                "(ESC) back to deck editing screen / (ENTER) to load from database",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::HelpScreen => Span::styled(
                "(ESC) back to main screen",
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

fn render_deck_layout(app: &App, frame: &mut Frame, area: &[Rect], deck_selector: bool) {
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
    if deck_selector {
        let active_style = Style::default().bg(Color::LightYellow).fg(Color::Black);
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
    } else {
        for d in app.decks.iter() {
            decks_list.push(ListItem::new(Line::from(Span::styled(
                format!("{: <25}", d),
                Style::default().fg(Color::Red),
            ))));
        }
    }

    let deck_list_block = Block::bordered().style(Style::default().fg(Color::DarkGray));

    let d_list = List::new(decks_list).block(deck_list_block);

    frame.render_widget(d_list, deck_layout[1]);
}
