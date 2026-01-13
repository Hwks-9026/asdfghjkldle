use clap::Parser;
use args::Args;
mod generator;
mod render;
mod guess;
mod args;
mod app;

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;
use std::io;

use crate::app::App;
use crate::render::ui;
use crate::generator::generate_word;

fn main() -> io::Result<()> {
    let args = Args::parse();
    let target = generate_word(&args);
    if args.debug {
        dbg!(&target);
    }
    let mut app = App::new(target, 10, args.length);

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Main Loop
    loop {
        terminal.draw(|f| ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            if app.game_over {
                if key.code == KeyCode::Char('q') || key.code == KeyCode::Esc {
                    break;
                }
            } else {
                match key.code {
                    KeyCode::Char(c) if c.is_ascii_alphabetic() => {
                        let low_c = c.to_ascii_lowercase();
                        app.current_guess[app.cursor_pos] = low_c;
                        if app.cursor_pos < app.word_length - 1 {
                            app.cursor_pos += 1;
                        }
                    }
                    
                    KeyCode::Left => {
                        app.cursor_pos = app.cursor_pos.saturating_sub(1);
                    }
                    KeyCode::Right => {
                        if app.cursor_pos < app.word_length - 1 {
                            app.cursor_pos += 1;
                        }
                    }

                    KeyCode::Backspace => {
                        app.current_guess[app.cursor_pos] = '~';
                        app.cursor_pos = app.cursor_pos.saturating_sub(1);
                    }
                    KeyCode::Enter => {
                        app.submit_guess();
                    }
                    KeyCode::Esc => break,
                    _ => {}
                }
            }
        }
    }

    // Terminal Cleanup
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
