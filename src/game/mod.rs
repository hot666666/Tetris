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
}

// Rust의 기본 derive 기능은 아주 큰 배열(보동 32개 초과)에 대해서는 자동으로 동작하지 않는 경우가 많음
// 따라서 Board 구조체를 위한 Default 트레이트를 수동으로 구현
pub struct Board {
    pub grid: [[Option<BlockColor>; BOARD_WIDTH]; BOARD_HEIGHT],
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

    pub fn exit(&mut self) {
        self.exit = true;
    }

    pub fn move_left(&mut self) {
        if let Some(piece) = &mut self.current_piece {
            piece.x -= 1;
        }
    }

    pub fn move_right(&mut self) {
        if let Some(piece) = &mut self.current_piece {
            piece.x += 1;
        }
    }

    pub fn rotate(&mut self) {
        if let Some(piece) = &mut self.current_piece {}
    }

    pub fn soft_drop(&mut self) {
        if let Some(piece) = &mut self.current_piece {
            piece.y += 1;
        }
    }
}
