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

#[derive(Clone, Debug)]
pub struct Piece {
    pub x: i32,
    pub y: i32,
    pub shape: Vec<Vec<u8>>,
    pub color: BlockColor,
}

impl Piece {
    pub fn new(shape: Vec<Vec<u8>>, color: BlockColor) -> Self {
        Self {
            x: BOARD_WIDTH as i32 / 2 - 2,
            y: 0,
            shape,
            color,
        }
    }

    // rotation logic can be added here
}

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
        // Simple I-piece for placeholder
        let i_piece = Piece::new(
            vec![
                vec![0, 0, 0, 0],
                vec![1, 1, 1, 1],
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
            ],
            BlockColor::Cyan,
        );

        Self {
            board: Board::default(),
            current_piece: Some(i_piece.clone()),
            next_piece: i_piece,
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

    pub fn soft_drop(&mut self) {
        if let Some(piece) = &mut self.current_piece {
            piece.y += 1;
        }
    }
}
