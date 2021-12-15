
mod maker;
pub mod cell;

use cell::Cell;
use crate::maker::make;

pub type Grid = Vec<Vec<Cell>>;

#[derive(Debug)]
pub struct Maze {
    pub grid: Grid
}

impl Maze {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            grid: make(width, height)
        }
    }
}

impl Default for Maze {
    fn default() -> Self {
        Self::new(10, 10)
    }
}