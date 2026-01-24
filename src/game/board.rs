use crate::game::{BOARD_HEIGHT, BOARD_WIDTH};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum BlockColor {
    Cyan,
    Yellow,
    Purple,
    Green,
    Red,
    Blue,
    Orange,
}

#[derive(Clone, Debug)]
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

impl Board {
    pub fn get(&self, x: i32, y: i32) -> Option<BlockColor> {
        if x >= 0 && x < BOARD_WIDTH as i32 && y >= 0 && y < BOARD_HEIGHT as i32 {
            self.grid[y as usize][x as usize]
        } else {
            None
        }
    }

    pub fn set(&mut self, x: usize, y: usize, color: BlockColor) {
        if x < BOARD_WIDTH && y < BOARD_HEIGHT {
            self.grid[y][x] = Some(color);
        }
    }

    pub fn clear_full_lines(&mut self) -> usize {
        let mut lines_cleared = 0;
        let mut write_y = BOARD_HEIGHT - 1;

        // 아래부터 위까지 꽉찬 줄 확인하며 지우기 -> 보드 새로 업데이트
        for read_y in (0..BOARD_HEIGHT).rev() {
            let is_full = self.grid[read_y].iter().all(|cell| cell.is_some());

            if is_full {
                lines_cleared += 1;
            } else {
                if write_y != read_y {
                    self.grid[write_y] = self.grid[read_y];
                }

                // Prevent underflow for usize
                if write_y > 0 {
                    write_y -= 1;
                }
            }
        }

        // 지워진 줄 만큼 상단에 None으로 초기화
        for y in 0..lines_cleared {
            self.grid[y] = [None; BOARD_WIDTH];
        }

        lines_cleared
    }
}
