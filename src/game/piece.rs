use crate::game::BOARD_WIDTH;
use rand::{Rng, SeedableRng};

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

    pub fn from_index(index: usize) -> Self {
        match index % 7 {
            0 => TetrominoType::I,
            1 => TetrominoType::O,
            2 => TetrominoType::T,
            3 => TetrominoType::S,
            4 => TetrominoType::Z,
            5 => TetrominoType::J,
            _ => TetrominoType::L,
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

// MARK: - Trait PieceGenerator

pub trait PieceGenerator {
    // 다음 조각을 하나 반환합니다.
    fn next_piece(&mut self) -> Piece;
}

pub struct RandomPieceGenerator {
    generator: rand::rngs::StdRng,
}

// MARK: - Implementation

impl RandomPieceGenerator {
    pub fn new() -> Self {
        Self {
            generator: rand::rngs::StdRng::from_os_rng(),
        }
    }
}

impl PieceGenerator for RandomPieceGenerator {
    fn next_piece(&mut self) -> Piece {
        let piece_type = self.generator.random_range(0..7);
        Piece::new(TetrominoType::from_index(piece_type))
    }
}

#[cfg(test)]
struct ConstantPieceGenerator {
    piece_type_index: usize,
}

#[cfg(test)]
impl ConstantPieceGenerator {
    fn new(piece_type_index: usize) -> Self {
        Self { piece_type_index }
    }
}

#[cfg(test)]
impl PieceGenerator for ConstantPieceGenerator {
    fn next_piece(&mut self) -> Piece {
        let piece_type = TetrominoType::from_index(self.piece_type_index);
        self.piece_type_index = (self.piece_type_index + 1) % 7;
        Piece::new(piece_type)
    }
}
