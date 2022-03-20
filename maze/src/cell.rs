pub enum CellSide {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Copy, Clone, Debug)]
pub struct Cell {
    pub up:      bool,
    pub down:    bool,
    pub left:    bool,
    pub rigth:   bool,
    pub visited: bool,
}

impl Cell {
    pub fn all_sides(&self, mut action: impl FnMut(CellSide)) {
        if self.up {
            action(CellSide::Up)
        }
        if self.down {
            action(CellSide::Down)
        }
        if self.left {
            action(CellSide::Left)
        }
        if self.rigth {
            action(CellSide::Right)
        }
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            up:      true,
            down:    true,
            left:    true,
            rigth:   true,
            visited: false,
        }
    }
}
