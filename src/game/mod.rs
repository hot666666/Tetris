pub const BOARD_WIDTH: usize = 10;
pub const BOARD_HEIGHT: usize = 20;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BlockColor {
    Cyan,
    Blue,
    Orange,
    Yellow,
    Green,
    Purple,
    Red,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TetrominoType {
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
}

impl TetrominoType {
    pub fn get_default_shape(&self) -> [[u8; 4]; 4] {
        match self {
            TetrominoType::I => [[0, 0, 0, 0], [1, 1, 1, 1], [0, 0, 0, 0], [0, 0, 0, 0]],
            TetrominoType::O => [[0, 1, 1, 0], [0, 1, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
            TetrominoType::T => [[0, 1, 0, 0], [1, 1, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
            TetrominoType::S => [[0, 1, 1, 0], [1, 1, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
            TetrominoType::Z => [[1, 1, 0, 0], [0, 1, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
            TetrominoType::J => [[1, 0, 0, 0], [1, 1, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
            TetrominoType::L => [[0, 0, 1, 0], [1, 1, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
        }
    }

    pub fn get_default_color(&self) -> BlockColor {
        match self {
            TetrominoType::I => BlockColor::Cyan,
            TetrominoType::O => BlockColor::Yellow,
            TetrominoType::T => BlockColor::Purple,
            TetrominoType::S => BlockColor::Green,
            TetrominoType::Z => BlockColor::Red,
            TetrominoType::J => BlockColor::Blue,
            TetrominoType::L => BlockColor::Orange,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Piece {
    pub x: i32,
    pub y: i32,
    pub shape: [[u8; 4]; 4],
    pub color: BlockColor,
    pub piece_type: TetrominoType,
}

impl Piece {
    pub fn new(piece_type: TetrominoType) -> Self {
        Self {
            x: BOARD_WIDTH as i32 / 2 - 2,
            y: 0,
            shape: piece_type.get_default_shape(),
            color: piece_type.get_default_color(),
            piece_type,
        }
    }

    pub fn rotate(&mut self) {
        let n = match self.piece_type {
            TetrominoType::I => 4,
            TetrominoType::O => return, // O piece conceptually doesn't rotate
            _ => 3, // All others are 3x3
        };

        let mut new_shape = [[0; 4]; 4];
        
        for y in 0..n {
            for x in 0..n {
                new_shape[x][n - 1 - y] = self.shape[y][x];
            }
        }
        self.shape = new_shape;
    }
}

// Rust의 기본 derive 기능은 아주 큰 배열(보동 32개 초과)에 대해서는 자동으로 동작하지 않는 경우가 많음
// 따라서 Board 구조체를 위한 Default 트레이트를 수동으로 구현
pub struct Board {
    pub grid: [[Option<BlockColor>; BOARD_WIDTH]; BOARD_HEIGHT],
}

impl Board {
    pub fn get(&self, x: i32, y: i32) -> Option<BlockColor> {
        if x < 0 || x >= BOARD_WIDTH as i32 || y < 0 || y >= BOARD_HEIGHT as i32 {
            return None;
        }
        self.grid[y as usize][x as usize]
    }

    pub fn set_cell(&mut self, x: usize, y: usize, color: BlockColor) {
        if x < BOARD_WIDTH && y < BOARD_HEIGHT {
            self.grid[y][x] = Some(color);
        }
    }
}

impl Default for Board {
    fn default() -> Self {
        Self {
            grid: [[None; BOARD_WIDTH]; BOARD_HEIGHT],
        }
    }
}

pub struct App {
    pub board: Board,
    pub current_piece: Option<Piece>,
    pub next_piece: Piece,
    pub score: u32,
    pub is_game_over: bool,
    pub exit: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            board: Board::default(),
            current_piece: Some(Piece::new(TetrominoType::I)),
            next_piece: Piece::new(TetrominoType::T),
            score: 0,
            is_game_over: false,
            exit: false,
        }
    }

    pub fn with_board(mut self, board: Board) -> Self {
        self.board = board;
        self
    }

    pub fn with_current_piece(mut self, piece: Piece) -> Self {
        self.current_piece = Some(piece);
        self
    }

    pub fn with_next_piece(mut self, piece: Piece) -> Self {
        self.next_piece = piece;
        self
    }
}

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

    pub fn soft_drop(&mut self) {
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

                    // 1. Check Warning: Out of Bounds (Walls & Floor)
                    if board_x < 0 || board_x >= BOARD_WIDTH as i32 || board_y >= BOARD_HEIGHT as i32 {
                        return true;
                    }

                    // 2. Check Warning: Occupied Cell
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

    pub fn commit_piece(&mut self) {
        let Some(piece) = self.current_piece.take() else {
            return;
        };

        for y in 0..4 {
            for x in 0..4 {
                if piece.shape[y][x] != 0 {
                    let board_x = piece.x + x as i32;
                    let board_y = piece.y + y as i32;

                    self.board.set_cell(board_x as usize, board_y as usize, piece.color);
                }
            }
        }
        // TODO: Clear lines
        // TODO: Spawn next piece
    }
}
