use crate::game::BOARD_WIDTH;

#[derive(Clone, Copy, PartialEq, Debug)]
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

    pub fn get_default_color(&self) -> super::BlockColor {
        match self {
            TetrominoType::I => super::BlockColor::Cyan,
            TetrominoType::O => super::BlockColor::Yellow,
            TetrominoType::T => super::BlockColor::Purple,
            TetrominoType::S => super::BlockColor::Green,
            TetrominoType::Z => super::BlockColor::Red,
            TetrominoType::J => super::BlockColor::Blue,
            TetrominoType::L => super::BlockColor::Orange,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Piece {
    pub x: i32,
    pub y: i32,
    pub shape: [[u8; 4]; 4],
    pub color: super::BlockColor,
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
            _ => 3,                     // All others are 3x3
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
