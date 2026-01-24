use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    symbols::border,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::game::{App, BOARD_HEIGHT, BOARD_WIDTH, BlockColor};

    pub fn render(app: &App, frame: &mut ratatui::Frame) {
    let area = frame.area();

    // Game container dimensions
    let board_ui_width = BOARD_WIDTH as u16 * 2 + 2; // 22
    // 3:4:3 ratio. Board is 4 parts (22). 1 part = 5.5. 3 parts = 16.5 -> 17.
    let side_panel_width = 17; 
    let gap = 1;
    let total_width = side_panel_width + gap + board_ui_width + gap + side_panel_width; // 58
    let title_height = 8;
    let total_height = BOARD_HEIGHT as u16 + 2 + title_height; // 30

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

    let centered_area = h_chunks[1];

    // Split into Title and Game Content
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(title_height),
            Constraint::Min(0),
        ])
        .split(centered_area);

    render_title(main_chunks[0], frame.buffer_mut());

    let game_area = main_chunks[1];

    // Split game area into Left, Board, Right
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(side_panel_width),
            Constraint::Length(gap),
            Constraint::Length(board_ui_width),
            Constraint::Length(gap),
            Constraint::Length(side_panel_width),
        ])
        .split(game_area);

    render_left_panel(app, chunks[0], frame.buffer_mut());
    render_board(app, chunks[2], frame.buffer_mut());
    render_right_panel(app, chunks[4], frame.buffer_mut());
}

fn render_title(area: Rect, buf: &mut Buffer) {
    let title_text = vec![
        "███████╗███████╗████████╗██████╗ ██╗███████╗",
        "╚══███╔╝██╔════╝╚══██╔══╝██╔══██╗██║██╔════╝",
        "   ██║  █████╗     ██║   ██████╔╝██║███████╗",
        "   ██║  ██╔══╝     ██║   ██╔══██╗██║╚════██║",
        "   ██║  ███████╗   ██║   ██║  ██║██║███████║",
        "   ╚═╝  ╚══════╝   ╚═╝   ╚═╝  ╚═╝╚═╝╚══════╝",
    ];

    let title_height = title_text.len() as u16;
    let title_width = title_text[0].chars().count() as u16;
    
    let start_x = area.x + (area.width.saturating_sub(title_width)) / 2;
    // Add a bit of padding from the top if possible, or just center in the 8-height block
    let start_y = area.y + (area.height.saturating_sub(title_height)) / 2;

    // Ranges for each letter (approximate based on the font)
    // T: 0..8, E: 8..16, T: 16..25, R: 25..33, I: 33..36, S: 36..44
    let color_map = [
        (0, 8, Color::Red),        // T
        (8, 16, Color::Yellow),    // E
        (16, 25, Color::Green),    // T
        (25, 33, Color::Blue),     // R
        (33, 36, Color::Magenta),  // I
        (36, 44, Color::Red),      // S (Recycle Red)
    ];

    for (i, line) in title_text.iter().enumerate() {
        let y = start_y + i as u16;
        if y >= area.y + area.height {
            break;
        }
        
        // Print character by character to apply colors
        // Note: We assume 1 char = 1 width unit for these block characters in this context
        for (j, ch) in line.chars().enumerate() {
            let col_idx = j;
            let mut color = Color::White; // Fallback
            
            // Find which letter range this column belongs to
            for &(start, end, c) in &color_map {
                if col_idx >= start && col_idx < end {
                    color = c;
                    break;
                }
            }

            let x = start_x + j as u16;
            if x < area.x + area.width {
                buf.set_string(x, y, ch.to_string(), Style::default().fg(color).bold());
            }
        }
    }
}

fn to_ratatui_color(color: BlockColor) -> Color {
    match color {
        BlockColor::Cyan => Color::Cyan,
        BlockColor::Blue => Color::Blue,
        BlockColor::Orange => Color::Rgb(255, 165, 0),
        BlockColor::Yellow => Color::Yellow,
        BlockColor::Green => Color::Green,
        BlockColor::Purple => Color::Magenta,
        BlockColor::Red => Color::Red,
    }
}

fn render_board(app: &App, area: Rect, buf: &mut Buffer) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_set(border::Set {
            top_left: border::THICK.top_left,
            top_right: border::THICK.top_right,
            bottom_left: "▀",
            bottom_right: "▀",
            vertical_left: border::THICK.vertical_left,
            vertical_right: border::THICK.vertical_right,
            horizontal_top: border::THICK.horizontal_top,
            horizontal_bottom: "▀",
        })
        .border_style(Style::default().fg(Color::Indexed(240)));

    let inner_area = block.inner(area);
    block.render(area, buf);

    // Render static blocks
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

        // "Embossed" effect using brackets
        buf.set_string(rx, ry, "██", Style::default().fg(color));
    }
}

fn render_left_panel(app: &App, area: Rect, buf: &mut Buffer) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(6), // Hold (Title + 4 height + Border)
            Constraint::Fill(1),   // Spacer
            Constraint::Length(10), // Keys
        ])
        .split(area);

    // Hold Panel
    let hold_block = Block::default()
        .borders(Borders::ALL)
        .title(" HOLD ")
        .border_style(Color::Indexed(238));
    let hold_inner = hold_block.inner(chunks[0]);
    hold_block.render(chunks[0], buf);

    if let Some(piece) = &app.hold_piece {
        render_preview_piece(piece, hold_inner, buf);
    } else {
        // Placeholder for visual feedback as requested
        let placeholder = crate::game::piece::Piece::new(crate::game::piece::TetrominoType::I);
        render_preview_piece(&placeholder, hold_inner, buf);
    }

    // Keys Panel
    let controls = vec![
        Line::from(""),
        Line::from("(←) Left"),
        Line::from("(→) Right"),
        Line::from("(↑) Rotate"),
        Line::from("(↓) Drop"),
        Line::from("(H) Hold"),
        Line::from(""),
        Line::from("(Q) Quit"),
    ];
    
    Paragraph::new(controls)
        .style(Style::default().fg(Color::DarkGray))
        .block(
            Block::default()
                .borders(Borders::ALL) // Changed from TOP to ALL to make it a panel
                .title(" KEY ")
                .border_style(Color::Indexed(238)),
        )
        .render(chunks[2], buf);
}

fn render_right_panel(app: &App, area: Rect, buf: &mut Buffer) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Score
            Constraint::Length(3),  // Lines
            Constraint::Length(1),  // Spacer
            Constraint::Length(15), // Next Pieces (approx 3x Hold height of 4 inner + borders)
        ])
        .split(area);

    // Score
    Paragraph::new(
        Span::styled(
            format!("{:06}", app.score),
            Style::default().fg(Color::Yellow).bold(),
        )
    )
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title(" SCORE ")
            .border_style(Color::Indexed(238)),
    )
    .render(chunks[0], buf);

    // Lines
    Paragraph::new(
        Span::styled(
            format!("{}", app.lines_cleared),
            Style::default().fg(Color::White).bold(),
        )
    )
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title(" LINES ")
            .border_style(Color::Indexed(238)),
    )
    .render(chunks[1], buf);

    // Next Pieces
    let next_block = Block::default()
        .borders(Borders::ALL)
        .title(" NEXT ")
        .border_style(Color::Indexed(238));
    
    let next_inner = next_block.inner(chunks[3]);
    next_block.render(chunks[3], buf);

    // Render next pieces queue
    // We subdivide the next_inner area into 3 equal parts
    let piece_count = 3;
    let piece_height = next_inner.height / piece_count as u16;

    for (i, piece) in app.next_pieces.iter().take(piece_count).enumerate() {
        let piece_area = Rect {
            x: next_inner.x,
            y: next_inner.y + (i as u16 * piece_height),
            width: next_inner.width,
            height: piece_height,
        };
        
        // Draw separator only after the first piece (i.e., at the top of the second piece)
        if i == 1 {
             let separator_y = piece_area.y;
             if separator_y > next_inner.y {
                 for x in piece_area.x .. piece_area.x + piece_area.width {
                     buf.set_string(x, separator_y, "─", Style::default().fg(Color::DarkGray));
                 }
             }
        }

        // Adjust area for the piece to avoid drawing ON the separator + padding
        let draw_area = if i == 1 {
             Rect { y: piece_area.y + 2, height: piece_area.height.saturating_sub(2), ..piece_area }
        } else {
             piece_area
        };

        render_preview_piece(piece, draw_area, buf);
    }
}

fn render_preview_piece(piece: &crate::game::piece::Piece, area: Rect, buf: &mut Buffer) {
    let shape = &piece.shape;
    
    // Calculate bounding box of the non-zero cells
    let mut min_r = shape.len();
    let mut max_r = 0;
    let mut min_c = shape[0].len();
    let mut max_c = 0;
    let mut has_blocks = false;

    for (r, row) in shape.iter().enumerate() {
        for (c, &cell) in row.iter().enumerate() {
            if cell != 0 {
                has_blocks = true;
                if r < min_r { min_r = r; }
                if r > max_r { max_r = r; }
                if c < min_c { min_c = c; }
                if c > max_c { max_c = c; }
            }
        }
    }

    if !has_blocks {
        return;
    }

    let content_height = (max_r - min_r + 1) as u16;
    let content_width = (max_c - min_c + 1) as u16 * 2; // 2 chars wide

    // Center the content in the target area
    let start_x = area.x + (area.width.saturating_sub(content_width)) / 2;
    let start_y = area.y + (area.height.saturating_sub(content_height)) / 2;

    for (r, row) in shape.iter().enumerate() {
        if r < min_r || r > max_r { continue; }
        for (c, &cell) in row.iter().enumerate() {
            if c < min_c || c > max_c { continue; }
            
            if cell != 0 {
                // Determine render position relative to the bounding box start
                let draw_x = start_x + ((c - min_c) as u16 * 2);
                let draw_y = start_y + (r - min_r) as u16;

                if draw_x + 1 < area.x + area.width && draw_y < area.y + area.height {
                     buf.set_string(
                        draw_x,
                        draw_y,
                        "██",
                        Style::default().fg(to_ratatui_color(piece.color)),
                    );
                }
            }
        }
    }
}
