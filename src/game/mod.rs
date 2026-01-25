pub mod app;
pub mod board;
pub mod piece;
pub mod input;

// 편의성을 위한 재포장 (Re-export)
// 외부(main.rs)에서 쓸 때 경로를 짧게 줄여주는 역할입
// 이게 없으면: use tetris::game::app::App;
// 이게 있으면: use tetris::game::App;
pub use app::{App, GameOverOption};
pub use board::{BlockColor, Board};
pub use input::Key;
pub use piece::{Piece, TetrominoType};

// 전역 상수
pub const BOARD_WIDTH: usize = 10;
pub const BOARD_HEIGHT: usize = 20;

// cfg(test): 'cargo test' 할 때만 이 코드를 컴파일 (배포판엔 포함 X)
#[cfg(test)]
mod tests;
