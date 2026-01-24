use tetris::game::{App, BlockColor, Board, Piece, TetrominoType};

#[test]
fn should_detect_collision_with_left_wall() {
    // Given: App with I-piece
    let mut app = App::new().with_current_piece(Piece::new(TetrominoType::I));

    // When: Move piece far left (x = -1)
    if let Some(piece) = &mut app.current_piece {
        piece.x = -1;
    }

    // Then
    let piece = app.current_piece.as_ref().unwrap();
    assert!(
        App::is_colliding(&app.board, piece),
        "Should detect collision with left wall"
    );
}

#[test]
fn should_detect_collision_with_right_wall() {
    // Given: App with I-piece
    let mut app = App::new().with_current_piece(Piece::new(TetrominoType::I));

    // When: Move piece far right (x = 10, assuming I-piece with x=10 collides instantly)
    // Board width is 10 (0..9). x=10 means the piece origin is at 10.
    // Piece shape checking x+0, etc. will check 10, 11, ... which are >= 10.
    if let Some(piece) = &mut app.current_piece {
        piece.x = 10;
    }

    // Then
    let piece = app.current_piece.as_ref().unwrap();
    assert!(
        App::is_colliding(&app.board, piece),
        "Should detect collision with right wall"
    );
}

#[test]
fn should_detect_collision_with_floor() {
    // Given: App with I-piece
    let mut app = App::new().with_current_piece(Piece::new(TetrominoType::I));

    // When: Move piece to bottom (y = 20)
    if let Some(piece) = &mut app.current_piece {
        piece.y = 20;
    }

    // Then
    let piece = app.current_piece.as_ref().unwrap();
    assert!(
        App::is_colliding(&app.board, piece),
        "Should detect collision with floor"
    );
}

#[test]
fn should_detect_collision_with_existing_blocks() {
    // Given: A board with a block at (3, 1) and an I-piece at (3, 0)
    let mut board = Board::default();
    board.set_cell(3, 1, BlockColor::Red);

    // I-piece shape at (3,0) occupies: (3,1), (4,1), (5,1), (6,1) in relative y=1 row?
    // Wait, let's double check I-piece shape again.
    // I shape:
    // [0,0,0,0]
    // [1,1,1,1] <- y=1
    // ...
    // If piece is at y=0, its blocks are at y=0+1 = 1.
    // So if there is a block at (3, 1), it should collide with the piece's first block.

    let app = App::new()
        .with_board(board)
        .with_current_piece(Piece::new(TetrominoType::I)); // I-piece spawns at y=0

    // When & Then
    let piece = app.current_piece.as_ref().unwrap();
    assert!(
        App::is_colliding(&app.board, piece),
        "Should collide with the block at (3,1) placed by setup"
    );
}

#[test]
fn should_not_detect_collision_in_free_space() {
    // Given
    let app = App::new();

    // When & Then
    let piece = app.current_piece.as_ref().unwrap();
    assert!(
        !App::is_colliding(&app.board, piece),
        "New piece in empty board should not collide"
    );
}
