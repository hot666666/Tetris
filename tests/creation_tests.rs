use tetris::game::{App, Board, Piece, TetrominoType};

#[test]
fn app_should_initialize_with_default_values() {
    // Given
    // App::new() acts as the setup

    // When
    let app = App::new();

    // Then
    assert_eq!(app.score, 0, "Initial score should be 0");
    assert!(!app.is_game_over, "Game should not be over initially");
    assert!(!app.exit, "Exit flag should be false initially");
    assert!(
        app.current_piece.is_some(),
        "Current piece should be spawned"
    );
}

#[test]
fn board_should_be_empty_on_creation() {
    // Given
    // Board::default() acts as the setup

    // When
    let board = Board::default();

    // Then
    assert_eq!(board.grid.len(), 20, "Board height should be 20");
    assert_eq!(board.grid[0].len(), 10, "Board width should be 10");

    for row in board.grid.iter() {
        for cell in row.iter() {
            assert!(cell.is_none(), "All cells should be None initially");
        }
    }
}

#[test]
fn piece_should_have_correct_shape_and_color_based_on_type() {
    // Given
    let piece_type = TetrominoType::I;

    // When
    let piece = Piece::new(piece_type);

    // Then
    assert_eq!(
        piece.piece_type,
        TetrominoType::I,
        "Piece type should match"
    );
    assert_eq!(piece.x, 3, "Initial X should be centered (10/2 - 2)"); // 10/2 - 2 = 3
    assert_eq!(piece.y, 0, "Initial Y should be 0");

    let expected_shape = TetrominoType::I.get_default_shape();
    assert_eq!(
        piece.shape, expected_shape,
        "Shape should match Tetromino I definition"
    );
}
