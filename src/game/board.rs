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

#[derive(Clone, Debug, PartialEq)]
pub struct Board {
    pub grid: Vec<Vec<Option<BlockColor>>>,
    pub width: usize,
    pub height: usize,
}

impl Default for Board {
    fn default() -> Self {
        Self::new(BOARD_WIDTH, BOARD_HEIGHT)
    }
}

impl Board {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            grid: vec![vec![None; width]; height],
            width,
            height,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get(&self, x: i32, y: i32) -> Option<BlockColor> {
        if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
            self.grid[y as usize][x as usize]
        } else {
            None
        }
    }

    pub fn set(&mut self, x: usize, y: usize, color: BlockColor) {
        if x < self.width && y < self.height {
            self.grid[y][x] = Some(color);
        }
    }

    pub fn clear_full_lines(&mut self) -> usize {
        let mut lines_cleared = 0;
        let mut write_y = self.height as i32 - 1;

        // 아래부터 위까지 꽉찬 줄 확인하며 지우기
        for read_y in (0..self.height).rev() {
            let is_full = self.grid[read_y].iter().all(|cell| cell.is_some());

            if is_full {
                lines_cleared += 1;
            } else {
                if write_y != read_y as i32 {
                    self.grid[write_y as usize] = self.grid[read_y].clone();
                }
                write_y -= 1;
            }
        }

        // 지워진 줄 만큼 상단에 None으로 초기화
        for y in 0..lines_cleared {
            self.grid[y] = vec![None; self.width];
        }

        lines_cleared
    }
}
