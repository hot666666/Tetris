

use crate::game::board::Board;
use crate::game::piece::{Piece, PieceGenerator, RandomPieceGenerator};
use crate::game::{BOARD_HEIGHT, BOARD_WIDTH};

pub struct App {
    pub board: Board,
    pub current_piece: Option<Piece>,
    pub next_pieces: Vec<Piece>,
    pub hold_piece: Option<Piece>,
    pub score: u32,
    pub lines_cleared: u32,
    pub is_game_over: bool,
    pub exit: bool,
    pub generator: Box<dyn PieceGenerator>,
}

// MARK: - 생성 관련

impl App {
    pub fn new() -> Self {
        let mut generator = RandomPieceGenerator::new();
        let current_piece = Some(generator.next_piece());
        let mut next_pieces = Vec::new();
        for _ in 0..3 {
            next_pieces.push(generator.next_piece());
        }

        Self {
            board: Board::default(),
            current_piece,
            next_pieces,
            hold_piece: None,
            score: 0,
            lines_cleared: 0,
            is_game_over: false,
            exit: false,
            generator: Box::new(generator),
        }
    }

    #[cfg(test)]
    pub(super) fn with_board(mut self, board: Board) -> Self {
        self.board = board;
        self
    }

    #[cfg(test)]
    pub(super) fn with_current_piece(mut self, piece: Piece) -> Self {
        self.current_piece = Some(piece);
        self
    }
}

// MARK: - 기능

impl App {
    pub fn exit(&mut self) {
        self.exit = true;
    }

    pub fn move_left(&mut self) {
        if let Some(piece) = &mut self.current_piece {
            piece.x -= 1;
            if Self::is_colliding(&self.board, piece) {
                piece.x += 1;
            }
        }
    }

    pub fn move_right(&mut self) {
        if let Some(piece) = &mut self.current_piece {
            piece.x += 1;
            if Self::is_colliding(&self.board, piece) {
                piece.x -= 1;
            }
        }
    }

    pub fn move_down(&mut self) {
        let mut locked = false;
        if let Some(piece) = &mut self.current_piece {
            piece.y += 1;
            if Self::is_colliding(&self.board, piece) {
                piece.y -= 1;
                locked = true;
            }
        }

        if locked {
            self.commit_piece();
        }
    }

    pub fn rotate(&mut self) {
        if let Some(piece) = &mut self.current_piece {
            let mut rotated_piece = piece.clone();
            rotated_piece.rotate();

            if !Self::is_colliding(&self.board, &rotated_piece) {
                piece.shape = rotated_piece.shape;
            }
        }
    }

    pub fn is_colliding(board: &Board, piece: &Piece) -> bool {
        for y in 0..4 {
            for x in 0..4 {
                if piece.shape[y][x] != 0 {
                    let board_x = piece.x + x as i32;
                    let board_y = piece.y + y as i32;

                    if board_x < 0
                        || board_x >= BOARD_WIDTH as i32
                        || board_y >= BOARD_HEIGHT as i32
                    {
                        return true;
                    }

                    if board_y >= 0 {
                        if board.get(board_x, board_y).is_some() {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }

    fn commit_piece(&mut self) {
        let Some(piece) = self.current_piece.take() else {
            return;
        };

        // 보드에 블록을 추가
        for y in 0..4 {
            for x in 0..4 {
                if piece.shape[y][x] != 0 {
                    let board_x = piece.x + x as i32;
                    let board_y = piece.y + y as i32;

                    self.board
                        .set(board_x as usize, board_y as usize, piece.color);
                }
            }
        }

        // 라인을 체크하고 점수를 추가
        let lines_cleared = self.board.clear_full_lines();
        if lines_cleared > 0 {
            self.score += (lines_cleared * 10) as u32;
            self.lines_cleared += lines_cleared as u32;
        }

        // 다음 블록을 가져오고 새 블록을 큐에 추가
        self.current_piece = Some(self.next_pieces.remove(0));
        self.next_pieces.push(self.generator.next_piece());
    }
}
