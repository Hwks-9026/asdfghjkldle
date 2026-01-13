use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use crate::app::*;
use crate::guess::*;

pub fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Min(5),    // Game Grid
            Constraint::Length(3), // Input/Status
        ])
        .split(f.area());

    // 1. Title
    let title = Paragraph::new("ASDFHJKLDLE")
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));
    f.render_widget(title, chunks[0]);

    // 2. The Grid
    let mut rows = Vec::new();

    // Render past guesses
    for guess in &app.history {
        let spans: Vec<Span> = guess.iter().map(|res| {
            let color = match res.status {
                LetterStatus::Correct => Color::Green,
                LetterStatus::Present => Color::Yellow,
                LetterStatus::Absent => Color::DarkGray,
            };
            Span::styled(
                format!(" {} ", res.letter.to_uppercase()),
                Style::default().bg(color).fg(Color::Black).add_modifier(Modifier::BOLD),
            )
        }).collect();
        rows.push(Line::from(spans));
    }

    // Render current input row (if not game over)
    if !app.game_over && app.history.len() < app.max_guesses {
        let mut current_spans = Vec::new();
        for i in 0..app.word_length {
            let ch = app.current_guess.chars().nth(i).unwrap_or('_');
            current_spans.push(Span::styled(
                format!(" {} ", ch.to_uppercase()),
                Style::default().fg(Color::White).add_modifier(Modifier::DIM),
            ));
        }
        rows.push(Line::from(current_spans));
    }

    let grid = Paragraph::new(rows)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).title(" Guesses "));
    f.render_widget(grid, chunks[1]);

    // 3. Status Message
    let status_text = if app.won {
        format!("VICTORY! The string was {}", app.target)
    } else if app.game_over {
        format!("GAME OVER. The string was {}", app.target)
    } else {
        format!("Type your guess ({}/{})", app.history.len() + 1, app.max_guesses)
    };

    let status = Paragraph::new(status_text).alignment(Alignment::Center);
    f.render_widget(status, chunks[2]);
}
