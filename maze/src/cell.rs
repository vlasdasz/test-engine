#[derive(Copy, Clone, Debug)]
pub struct Cell {
    pub up:      bool,
    pub down:    bool,
    pub left:    bool,
    pub rigth:   bool,
    pub visited: bool,
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
