use std::io;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::DefaultTerminal;
use tetris::game::{App, Key};
use tetris::ui;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = run_app(&mut terminal, App::new());
    ratatui::restore();
    app_result
}

fn run_app(terminal: &mut DefaultTerminal, mut app: App) -> io::Result<()> {
    while !app.exit {
        terminal.draw(|frame| ui::render(&app, frame))?;
        
        if event::poll(std::time::Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    let game_key = match key.code {
                        KeyCode::Char('q') => Some(Key::Quit),
                        KeyCode::Char('h') => Some(Key::Hold),
                        KeyCode::Char(' ') => Some(Key::QuickDrop),
                        KeyCode::Left => Some(Key::Left),
                        KeyCode::Right => Some(Key::Right),
                        KeyCode::Down => Some(Key::Down),
                        KeyCode::Up => Some(Key::Rotate),
                        _ => None,
                    };

                    if let Some(gk) = game_key {
                        app.handle_input(gk);
                    }
                }
            }
        }
    }
    Ok(())
}
