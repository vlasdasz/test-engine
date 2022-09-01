pub enum Anchor {
    Top,
    Bot,

    Left,
    Right,

    Width,
    Height,

    CenterH,
    CenterV,

    Center,
}

impl Anchor {
    pub fn is_vertical(&self) -> bool {
        matches!(self, Anchor::Top | Anchor::Bot)
    }
    pub fn is_horizontal(&self) -> bool {
        matches!(self, Anchor::Left | Anchor::Right)
    }
}
