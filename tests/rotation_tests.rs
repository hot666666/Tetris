use tetris::game::{App, BlockColor, Board, Piece, TetrominoType};

#[test]
fn should_not_rotate_if_collision_occurs() {
    // Given: T-piece near a wall or block
    let mut board = Board::default();
    board.set_cell(4, 2, BlockColor::Red);

    // T at (3,0)
    let mut app = App::new()
        .with_board(board)
        .with_current_piece(Piece::new(TetrominoType::T));

    let original_shape = app.current_piece.as_ref().unwrap().shape;

    // When: Attempt to rotate
    app.rotate();

    // Then: Shape should remain unchanged
    let current_shape = app.current_piece.as_ref().unwrap().shape;
    assert_eq!(
        current_shape, original_shape,
        "Piece should NOT rotate if target cells are blocked"
    );
}

#[test]
fn should_rotate_i_piece_full_cycle() {
    // Given: I piece
    // [[0, 0, 0, 0], [1, 1, 1, 1], [0, 0, 0, 0], [0, 0, 0, 0]]
    let mut app = App::new().with_current_piece(Piece::new(TetrominoType::I));

    // 0 -> 90: Vertical (Col 2)
    app.rotate();
    let expected_90 = [[0, 0, 1, 0], [0, 0, 1, 0], [0, 0, 1, 0], [0, 0, 1, 0]];
    assert_eq!(
        app.current_piece.as_ref().unwrap().shape,
        expected_90,
        "I-Piece: 0 -> 90"
    );

    // 90 -> 180: Horizontal (Row 2)
    app.rotate();
    let expected_180 = [[0, 0, 0, 0], [0, 0, 0, 0], [1, 1, 1, 1], [0, 0, 0, 0]];
    assert_eq!(
        app.current_piece.as_ref().unwrap().shape,
        expected_180,
        "I-Piece: 90 -> 180"
    );

    // 180 -> 270: Vertical (Col 1)
    app.rotate();
    let expected_270 = [[0, 1, 0, 0], [0, 1, 0, 0], [0, 1, 0, 0], [0, 1, 0, 0]];
    assert_eq!(
        app.current_piece.as_ref().unwrap().shape,
        expected_270,
        "I-Piece: 180 -> 270"
    );

    // 270 -> 0: Horizontal (Row 1) (Back to start)
    app.rotate();
    let expected_0 = [[0, 0, 0, 0], [1, 1, 1, 1], [0, 0, 0, 0], [0, 0, 0, 0]];
    assert_eq!(
        app.current_piece.as_ref().unwrap().shape,
        expected_0,
        "I-Piece: 270 -> 0"
    );
}

#[test]
fn should_rotate_o_piece_full_cycle() {
    // Given: O piece
    // [[0, 1, 1, 0], [0, 1, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0]]
    let mut app = App::new().with_current_piece(Piece::new(TetrominoType::O));

    let expected = [[0, 1, 1, 0], [0, 1, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0]];

    // Check 4 rotations (should all be identical)
    for i in 1..=4 {
        app.rotate();
        assert_eq!(
            app.current_piece.as_ref().unwrap().shape,
            expected,
            "O-Piece: Rotation {}",
            i
        );
    }
}

#[test]
fn should_rotate_t_piece_full_cycle() {
    // Given: T piece
    // [[0, 1, 0, 0], [1, 1, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0]]
    let mut app = App::new().with_current_piece(Piece::new(TetrominoType::T));

    // 0 -> 90
    app.rotate();
    let expected_90 = [[0, 1, 0, 0], [0, 1, 1, 0], [0, 1, 0, 0], [0, 0, 0, 0]];
    assert_eq!(
        app.current_piece.as_ref().unwrap().shape,
        expected_90,
        "T-Piece: 0 -> 90"
    );

    // 90 -> 180
    app.rotate();
    let expected_180 = [[0, 0, 0, 0], [1, 1, 1, 0], [0, 1, 0, 0], [0, 0, 0, 0]];
    assert_eq!(
        app.current_piece.as_ref().unwrap().shape,
        expected_180,
        "T-Piece: 90 -> 180"
    );

    // 180 -> 270
    app.rotate();
    let expected_270 = [[0, 1, 0, 0], [1, 1, 0, 0], [0, 1, 0, 0], [0, 0, 0, 0]];
    assert_eq!(
        app.current_piece.as_ref().unwrap().shape,
        expected_270,
        "T-Piece: 180 -> 270"
    );

    // 270 -> 0 (Back to start)
    app.rotate();
    let expected_0 = [[0, 1, 0, 0], [1, 1, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0]];
    assert_eq!(
        app.current_piece.as_ref().unwrap().shape,
        expected_0,
        "T-Piece: 270 -> 0"
    );
}

#[test]
fn should_rotate_s_piece_full_cycle() {
    // Given: S piece
    // [[0, 1, 1, 0], [1, 1, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]]
    let mut app = App::new().with_current_piece(Piece::new(TetrominoType::S));

    // 0 -> 90
    app.rotate();
    let expected_90 = [[0, 1, 0, 0], [0, 1, 1, 0], [0, 0, 1, 0], [0, 0, 0, 0]];
    assert_eq!(
        app.current_piece.as_ref().unwrap().shape,
        expected_90,
        "S-Piece: 0 -> 90"
    );

    // 90 -> 180
    app.rotate();
    let expected_180 = [[0, 0, 0, 0], [0, 1, 1, 0], [1, 1, 0, 0], [0, 0, 0, 0]];
    assert_eq!(
        app.current_piece.as_ref().unwrap().shape,
        expected_180,
        "S-Piece: 90 -> 180"
    );

    // 180 -> 270
    app.rotate();
    let expected_270 = [[1, 0, 0, 0], [1, 1, 0, 0], [0, 1, 0, 0], [0, 0, 0, 0]];
    assert_eq!(
        app.current_piece.as_ref().unwrap().shape,
        expected_270,
        "S-Piece: 180 -> 270"
    );

    // 270 -> 0
    app.rotate();
    let expected_0 = [[0, 1, 1, 0], [1, 1, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]];
    assert_eq!(
        app.current_piece.as_ref().unwrap().shape,
        expected_0,
        "S-Piece: 270 -> 0"
    );
}

#[test]
fn should_rotate_z_piece_full_cycle() {
    // Given: Z piece
    // [[1, 1, 0, 0], [0, 1, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0]]
    let mut app = App::new().with_current_piece(Piece::new(TetrominoType::Z));

    // 0 -> 90
    app.rotate();
    let expected_90 = [[0, 0, 1, 0], [0, 1, 1, 0], [0, 1, 0, 0], [0, 0, 0, 0]];
    assert_eq!(
        app.current_piece.as_ref().unwrap().shape,
        expected_90,
        "Z-Piece: 0 -> 90"
    );

    // 90 -> 180
    app.rotate();
    let expected_180 = [[0, 0, 0, 0], [1, 1, 0, 0], [0, 1, 1, 0], [0, 0, 0, 0]];
    assert_eq!(
        app.current_piece.as_ref().unwrap().shape,
        expected_180,
        "Z-Piece: 90 -> 180"
    );

    // 180 -> 270
    app.rotate();
    let expected_270 = [[0, 1, 0, 0], [1, 1, 0, 0], [1, 0, 0, 0], [0, 0, 0, 0]];
    assert_eq!(
        app.current_piece.as_ref().unwrap().shape,
        expected_270,
        "Z-Piece: 180 -> 270"
    );

    // 270 -> 0
    app.rotate();
    let expected_0 = [[1, 1, 0, 0], [0, 1, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0]];
    assert_eq!(
        app.current_piece.as_ref().unwrap().shape,
        expected_0,
        "Z-Piece: 270 -> 0"
    );
}

#[test]
fn should_rotate_j_piece_full_cycle() {
    // Given: J piece
    // [[0, 1, 0, 0], [0, 1, 0, 0], [1, 1, 0, 0], [0, 0, 0, 0]]
    let mut app = App::new().with_current_piece(Piece::new(TetrominoType::J));

    // 0 -> 90
    app.rotate();
    let expected_90 = [[0, 1, 1, 0], [0, 1, 0, 0], [0, 1, 0, 0], [0, 0, 0, 0]];
    assert_eq!(
        app.current_piece.as_ref().unwrap().shape,
        expected_90,
        "J-Piece: 0 -> 90"
    );

    // 90 -> 180
    app.rotate();
    let expected_180 = [[0, 0, 0, 0], [1, 1, 1, 0], [0, 0, 1, 0], [0, 0, 0, 0]];
    assert_eq!(
        app.current_piece.as_ref().unwrap().shape,
        expected_180,
        "J-Piece: 90 -> 180"
    );

    // 180 -> 270
    app.rotate();
    let expected_270 = [[0, 1, 0, 0], [0, 1, 0, 0], [1, 1, 0, 0], [0, 0, 0, 0]];
    assert_eq!(
        app.current_piece.as_ref().unwrap().shape,
        expected_270,
        "J-Piece: 180 -> 270"
    );

    // 270 -> 0
    app.rotate();
    let expected_0 = [[1, 0, 0, 0], [1, 1, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0]];
    assert_eq!(
        app.current_piece.as_ref().unwrap().shape,
        expected_0,
        "J-Piece: 270 -> 0"
    );
}

#[test]
fn should_rotate_l_piece_full_cycle() {
    // Given: L piece
    // [[0, 1, 0, 0], [0, 1, 0, 0], [0, 1, 1, 0], [0, 0, 0, 0]]
    let mut app = App::new().with_current_piece(Piece::new(TetrominoType::L));

    // 0 -> 90
    app.rotate();
    let expected_90 = [[0, 1, 0, 0], [0, 1, 0, 0], [0, 1, 1, 0], [0, 0, 0, 0]];
    assert_eq!(
        app.current_piece.as_ref().unwrap().shape,
        expected_90,
        "L-Piece: 0 -> 90"
    );

    // 90 -> 180
    app.rotate();
    let expected_180 = [[0, 0, 0, 0], [1, 1, 1, 0], [1, 0, 0, 0], [0, 0, 0, 0]];
    assert_eq!(
        app.current_piece.as_ref().unwrap().shape,
        expected_180,
        "L-Piece: 90 -> 180"
    );

    // 180 -> 270
    app.rotate();
    let expected_270 = [[1, 1, 0, 0], [0, 1, 0, 0], [0, 1, 0, 0], [0, 0, 0, 0]];
    assert_eq!(
        app.current_piece.as_ref().unwrap().shape,
        expected_270,
        "L-Piece: 180 -> 270"
    );

    // 270 -> 0
    app.rotate();
    let expected_0 = [[0, 0, 1, 0], [1, 1, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0]];
    assert_eq!(
        app.current_piece.as_ref().unwrap().shape,
        expected_0,
        "L-Piece: 270 -> 0"
    );
}
