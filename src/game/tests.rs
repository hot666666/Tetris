// 부모의 비공개 항목을 가져올 때 crate:: 보단 super::를 사용
use super::*;

// MARK: - Helpers

fn place_piece(board: &mut Board, piece: &Piece) {
    for y in 0..4 {
        for x in 0..4 {
            if piece.shape[y][x] != 0 {
                let bx = piece.x + x as i32;
                let by = piece.y + y as i32;
                if bx >= 0 && bx < board.width as i32 && by >= 0 && by < board.height as i32 {
                    board.set(bx as usize, by as usize, piece.color);
                }
            }
        }
    }
}

// MARK: - Line Clearing Tests

#[test]
fn should_clear_full_lines() {
    let mut board = Board::default();
    
    // Fill the bottom line
    for x in 0..BOARD_WIDTH {
        board.set(x, BOARD_HEIGHT - 1, BlockColor::Cyan);
    }
    
    // Fill the line above it
    for x in 0..BOARD_WIDTH {
        board.set(x, BOARD_HEIGHT - 2, BlockColor::Red);
    }

    // Call clear_full_lines
    let cleared = board.clear_full_lines();
    
    // Expect 2 lines cleared
    assert_eq!(cleared, 2);
    
    // Review: Bottom lines should now be empty (default is None)
    // But wait, if we cleared lines, the lines above (which were empty) should have fallen down.
    // So the bottom line should be empty.
    for x in 0..BOARD_WIDTH {
        assert!(board.get(x as i32, (BOARD_HEIGHT - 1) as i32).is_none());
        assert!(board.get(x as i32, (BOARD_HEIGHT - 2) as i32).is_none());
    }
}

#[test]
fn should_drop_blocks_above_cleared_lines() {
    let mut board = Board::default();
    
    // Setup:
    // Line 19 (Bottom): Full
    // Line 18: Full
    // Line 17: Partial (Some blocks)
    
    for x in 0..BOARD_WIDTH {
        board.set(x, BOARD_HEIGHT - 1, BlockColor::Cyan);
        board.set(x, BOARD_HEIGHT - 2, BlockColor::Cyan);
    }
    
    // Add a block at (0, 17)
    board.set(0, BOARD_HEIGHT - 3, BlockColor::Red);
    
    let cleared = board.clear_full_lines();
    assert_eq!(cleared, 2);
    
    // Expect: 
    // The block at (0, 17) should drop 2 lines to (0, 19).
    assert_eq!(board.get(0, (BOARD_HEIGHT - 1) as i32), Some(BlockColor::Red));
    
    // Original position should be empty
    assert!(board.get(0, (BOARD_HEIGHT - 3) as i32).is_none());
}

#[test]
fn should_detect_collision_with_left_wall() {
    let mut app = App::new().with_current_piece(Piece::new(TetrominoType::I));
    if let Some(piece) = &mut app.current_piece {
        piece.x = -1;
    }
    let piece = app.current_piece.as_ref().unwrap();
    assert!(App::is_colliding(&app.board, piece));
}

#[test]
fn should_detect_collision_with_right_wall() {
    let mut app = App::new().with_current_piece(Piece::new(TetrominoType::I));
    if let Some(piece) = &mut app.current_piece {
        piece.x = 10;
    }
    let piece = app.current_piece.as_ref().unwrap();
    assert!(App::is_colliding(&app.board, piece));
}

#[test]
fn should_detect_collision_with_floor() {
    let mut app = App::new().with_current_piece(Piece::new(TetrominoType::I));
    if let Some(piece) = &mut app.current_piece {
        piece.y = 20;
    }
    let piece = app.current_piece.as_ref().unwrap();
    assert!(App::is_colliding(&app.board, piece));
}

#[test]
fn should_detect_collision_with_existing_blocks() {
    let mut board = Board::default();
    board.set(3, 1, BlockColor::Red);
    let app = App::new()
        .with_board(board)
        .with_current_piece(Piece::new(TetrominoType::I));
    let piece = app.current_piece.as_ref().unwrap();
    assert!(App::is_colliding(&app.board, piece));
}

// MARK: - Rotation Tests

#[test]
fn should_not_rotate_if_collision_occurs() {
    let mut board = Board::default();
    board.set(4, 2, BlockColor::Red);
    let mut app = App::new()
        .with_board(board)
        .with_current_piece(Piece::new(TetrominoType::T));
    let original_shape = app.current_piece.as_ref().unwrap().shape;
    app.rotate();
    assert_eq!(app.current_piece.as_ref().unwrap().shape, original_shape);
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

// MARK: - Hold & QuickDrop Tests

#[test]
fn should_hold_piece_and_swap_with_next() {
    // Given
    let mut app = App::new();
    let initial_current = app.current_piece.as_ref().unwrap().piece_type;
    let initial_next = app.next_pieces[0].piece_type;

    // When
    app.hold();

    // Then
    assert_eq!(app.hold_piece.as_ref().unwrap().piece_type, initial_current);
    assert_eq!(app.current_piece.as_ref().unwrap().piece_type, initial_next);
    assert!(app.has_held_this_turn);
}

#[test]
fn should_swap_current_with_held_piece() {
    // Given
    let piece_i = Piece::new(TetrominoType::I);
    let piece_o = Piece::new(TetrominoType::O);
    let mut app = App::new()
        .with_current_piece(piece_i)
        .with_hold_piece(piece_o);

    // When
    app.hold();

    // Then
    assert_eq!(app.hold_piece.as_ref().unwrap().piece_type, TetrominoType::I);
    assert_eq!(app.current_piece.as_ref().unwrap().piece_type, TetrominoType::O);
}

#[test]
fn should_only_hold_once_per_turn() {
    // Given
    let mut app = App::new();
    app.hold();
    let held_after_first = app.hold_piece.as_ref().unwrap().piece_type;

    // When (Try holding again immediately)
    app.hold();
    
    // Then (Should still be the same, didn't swap back)
    assert_eq!(app.hold_piece.as_ref().unwrap().piece_type, held_after_first);
}

#[test]
fn should_reset_hold_limit_after_commit() {
    // Given
    let mut app = App::new();
    app.hold();
    assert!(app.has_held_this_turn);

    // O piece near the bottom to force lock
    let mut piece = Piece::new(TetrominoType::O);
    piece.y = 18; 
    app = app.with_current_piece(piece);
    
    // When (Piece locked and committed)
    app.move_down(); 
    
    // Then
    assert!(!app.has_held_this_turn);
}

#[test]
fn should_hard_drop_piece_to_bottom() {
    // Given
    let mut piece = Piece::new(TetrominoType::O);
    piece.x = 1;
    piece.y = 0;

    let board = Board::new(5, 10);
    let mut app = App::new()
        .with_board(board.clone())
        .with_current_piece(piece.clone());

    // When
    app.quick_drop();
    
    // Then
    // 5x10 보드에서 O-piece(2x2)가 바닥에 닿으면 piece.y는 8이 됨
    let mut expected_board = board;
    let mut expected_piece = piece;
    expected_piece.y = 8;
    place_piece(&mut expected_board, &expected_piece);
    
    assert_eq!(app.board, expected_board, "Hard drop failed to land piece at expected coordinates correctly");
}
