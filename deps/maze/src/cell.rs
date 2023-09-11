#[derive(Copy, Clone)]
pub enum CellSide {
    Up,
    Down,
    Left,
    Right,
}

#[allow(clippy::struct_excessive_bools)]
#[derive(Copy, Clone, Debug)]
pub struct Cell {
    pub top:     bool,
    pub bottom:  bool,
    pub left:    bool,
    pub right:   bool,
    pub visited: bool,
}

impl Cell {
    pub fn all_sides(&self, mut action: impl FnMut(CellSide)) {
        if self.top {
            action(CellSide::Up)
        }
        if self.bottom {
            action(CellSide::Down)
        }
        if self.left {
            action(CellSide::Left)
        }
        if self.right {
            action(CellSide::Right)
        }
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            top:     true,
            bottom:  false,
            left:    false,
            right:   true,
            visited: false,
        }
    }
}
