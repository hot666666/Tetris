mod data;
mod ui;

use std::io;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::DefaultTerminal;
use data::App;

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
                    match key.code {
                        KeyCode::Char('q') => app.exit(),
                        KeyCode::Left => app.move_left(),
                        KeyCode::Right => app.move_right(),
                        KeyCode::Down => app.soft_drop(),
                        // Add more key handlers here
                        _ => {}
                    }
                }
            }
        }
    }
    Ok(())
}
