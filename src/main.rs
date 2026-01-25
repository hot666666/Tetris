use std::io;
use crossterm::event::{self, Event as CrosstermEvent, KeyCode, KeyEventKind};
use ratatui::DefaultTerminal;
use tetris::game::{App, Key, Event};
use tetris::ui;
use tokio::sync::mpsc;
use tokio::time::{self, Duration};
use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = run_app(&mut terminal, App::new()).await;
    ratatui::restore();
    app_result
}

async fn run_app(terminal: &mut DefaultTerminal, mut app: App) -> io::Result<()> {
    let cancel_token = CancellationToken::new();
    // Multi-Producer, Single-Consumer
    let (tx, mut rx) = mpsc::channel(100);

    // Gravity Task (Producer 1)
    let tx_gravity = tx.clone();
    let cancel_gravity = cancel_token.clone();
    tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_millis(500));

        loop {
            tokio::select! {
                _ = cancel_gravity.cancelled() => {
                    break;
                }
                _ = interval.tick() => {
                    if tx_gravity.send(Event::Tick).await.is_err() {
                        break;
                    }
                }
            }
        }
    });

    // Input Task (Producer 2)
    let tx_input = tx.clone();
    let cancel_input = cancel_token.clone();
    tokio::spawn(async move {
        loop {
            let poll_duration = Duration::from_millis(16);
            
            // tokio::select!를 사용하여 입력 대기 중에도 취소 신호를 즉시 처리
            tokio::select! {
                _ = cancel_input.cancelled() => {
                    break;
                }
                has_event = tokio::task::spawn_blocking(move || event::poll(poll_duration)) => {
                    if let Ok(Ok(true)) = has_event {
                        if let Ok(CrosstermEvent::Key(key)) = event::read() {
                            if key.kind == KeyEventKind::Press {
                                let game_key = match key.code {
                                    KeyCode::Char('q') => Some(Key::Quit),
                                    KeyCode::Char('h') => Some(Key::Hold),
                                    KeyCode::Char(' ') => Some(Key::QuickDrop),
                                    KeyCode::Enter => Some(Key::Enter),
                                    KeyCode::Left => Some(Key::Left),
                                    KeyCode::Right => Some(Key::Right),
                                    KeyCode::Down => Some(Key::Down),
                                    KeyCode::Up => Some(Key::Rotate),
                                    _ => None,
                                };

                                if let Some(gk) = game_key {
                                    if tx_input.send(Event::Input(gk)).await.is_err() {
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    });

    // Main Loop (Consumer)
    while !app.exit {
        terminal.draw(|frame| ui::render(&app, frame))?;
        
        if let Some(event) = rx.recv().await {
            match event {
                Event::Input(key) => app.handle_input(key),
                Event::Tick => {
                    if !app.is_game_over {
                        app.handle_input(Key::Down);
                    }
                }
            }
        }
    }

    cancel_token.cancel();
    Ok(())
}
