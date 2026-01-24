use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    symbols::border,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::data::{App, BlockColor, BOARD_HEIGHT, BOARD_WIDTH};

pub fn render(app: &App, frame: &mut ratatui::Frame) {
    let area = frame.area();

    // Game container dimensions
    let board_ui_width = BOARD_WIDTH as u16 * 2 + 2;
    let info_ui_width = 24;
    let gap = 2;
    let total_width = board_ui_width + gap + info_ui_width;
    let total_height = BOARD_HEIGHT as u16 + 2;

    // Check if the terminal is large enough
    if area.width < total_width || area.height < total_height {
        let msg = format!(
            "Window too small! (Required: {}x{}, Current: {}x{})",
            total_width, total_height, area.width, area.height
        );
        Paragraph::new(msg)
            .style(Style::default().fg(Color::Red).bold())
            .alignment(ratatui::layout::Alignment::Center)
            .block(Block::default().borders(Borders::ALL))
            .render(area, frame.buffer_mut());
        return;
    }

    // Outer layout to center vertically
    let v_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(total_height),
            Constraint::Fill(1),
        ])
        .split(area);

    // Inner layout to center horizontally
    let h_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(total_width),
            Constraint::Fill(1),
        ])
        .split(v_chunks[1]);

    let game_area = h_chunks[1];

    // Split game area into Board and Info
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(board_ui_width),
            Constraint::Length(gap),
            Constraint::Length(info_ui_width),
        ])
        .split(game_area);

    render_board(app, chunks[0], frame.buffer_mut());
    render_info_panel(app, chunks[2], frame.buffer_mut());
}

fn to_ratatui_color(color: BlockColor) -> Color {
    match color {
        BlockColor::Cyan => Color::Cyan,
        BlockColor::Blue => Color::Blue,
        BlockColor::Orange => Color::Yellow, // Orange isn't a standard ANSI color
        BlockColor::Yellow => Color::LightYellow,
        BlockColor::Green => Color::Green,
        BlockColor::Purple => Color::Magenta,
        BlockColor::Red => Color::Red,
    }
}

fn render_board(app: &App, area: Rect, buf: &mut Buffer) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_set(border::THICK)
        .border_style(Style::default().fg(Color::Indexed(240))); // Dark grey border

    let inner_area = block.inner(area);
    block.render(area, buf);

    // Render background grid (subtle dots)
    for y in 0..BOARD_HEIGHT {
        for x in 0..BOARD_WIDTH {
            let cx = inner_area.x + (x as u16 * 2);
            let cy = inner_area.y + y as u16;
            if cx < buf.area.width && cy < buf.area.height {
                buf[(cx, cy)].set_symbol("·").set_fg(Color::Indexed(235));
            }
        }
    }

    // Render static blocks on the board
    for y in 0..BOARD_HEIGHT {
        for x in 0..BOARD_WIDTH {
            if let Some(color) = app.board.grid[y][x] {
                render_cell(inner_area, x as i32, y as i32, to_ratatui_color(color), buf);
            }
        }
    }

    // Render current piece
    if let Some(piece) = &app.current_piece {
        for (py, row) in piece.shape.iter().enumerate() {
            for (px, &cell) in row.iter().enumerate() {
                if cell != 0 {
                    render_cell(
                        inner_area,
                        piece.x + px as i32,
                        piece.y + py as i32,
                        to_ratatui_color(piece.color),
                        buf,
                    );
                }
            }
        }
    }
}

fn render_cell(inner_area: Rect, x: i32, y: i32, color: Color, buf: &mut Buffer) {
    if x >= 0 && x < BOARD_WIDTH as i32 && y >= 0 && y < BOARD_HEIGHT as i32 {
        let rx = inner_area.x + (x as u16 * 2);
        let ry = inner_area.y + y as u16;

        // Solid block with color
        buf.set_string(rx, ry, "██", Style::default().fg(color));
    }
}

fn render_info_panel(app: &App, area: Rect, buf: &mut Buffer) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Score
            Constraint::Length(7), // Next
            Constraint::Fill(1),   // Controls
        ])
        .split(area);

    // Score Panel
    Paragraph::new(Line::from(vec![
        Span::raw(" "),
        Span::styled(
            format!("{:06}", app.score),
            Style::default().fg(Color::Yellow).bold(),
        ),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title(" SCORE ")
            .border_style(Color::Indexed(238)),
    )
    .render(chunks[0], buf);

    // Next Piece Panel
    let next_block = Block::default()
        .borders(Borders::ALL)
        .title(" NEXT ")
        .border_style(Color::Indexed(238));
    let next_inner = next_block.inner(chunks[1]);
    next_block.render(chunks[1], buf);
    render_preview_piece(app, next_inner, buf);

    // Controls Panel
    let controls = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled(
                " ← ",
                Style::default()
                    .bg(Color::Indexed(24))
                    .fg(Color::White)
                    .bold(),
            ),
            Span::raw(" Left "),
        ]),
        Line::from(vec![
            Span::styled(
                " → ",
                Style::default()
                    .bg(Color::Indexed(24))
                    .fg(Color::White)
                    .bold(),
            ),
            Span::raw(" Right "),
        ]),
        Line::from(vec![
            Span::styled(
                " ↑ ",
                Style::default()
                    .bg(Color::Indexed(130))
                    .fg(Color::White)
                    .bold(),
            ),
            Span::raw(" Rotate "),
        ]),
        Line::from(vec![
            Span::styled(
                " ↓ ",
                Style::default()
                    .bg(Color::Indexed(28))
                    .fg(Color::White)
                    .bold(),
            ),
            Span::raw(" Drop "),
        ]),
        Line::from(""),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                " Q ",
                Style::default().bg(Color::Red).fg(Color::White).bold(),
            ),
            Span::raw(" Quit "),
        ]),
        Line::from(""),
    ];
    Paragraph::new(controls)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" CONTROLS ")
                .border_style(Color::Indexed(238)),
        )
        .render(chunks[2], buf);
}

fn render_preview_piece(app: &App, area: Rect, buf: &mut Buffer) {
    let piece = &app.next_piece;
    let shape_width = piece.shape[0].len() as u16;
    let shape_height = piece.shape.len() as u16;

    // Center alignment within preview box
    let start_x = area.x + (area.width.saturating_sub(shape_width * 2)) / 2;
    let start_y = area.y + (area.height.saturating_sub(shape_height)) / 2;

    for (py, row) in piece.shape.iter().enumerate() {
        for (px, &cell) in row.iter().enumerate() {
            if cell != 0 {
                let rx = start_x + (px as u16 * 2);
                let ry = start_y + py as u16;
                if rx < area.x + area.width && ry < area.y + area.height {
                    buf.set_string(rx, ry, "██", Style::default().fg(to_ratatui_color(piece.color)));
                }
            }
        }
    }
}
