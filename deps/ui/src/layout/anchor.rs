pub enum Anchor {
    Top,
    Bot,

    Left,
    Right,

    Width,
    Height,

    MaxWidth,
    MaxHeight,

    Size,

    CenterH,
    CenterV,

    Center,

    X,
    Y,

    None,
}

impl Anchor {
    pub(crate) fn has_width(&self) -> bool {
        matches!(self, Self::Width | Self::Size)
    }

    pub(crate) fn has_height(&self) -> bool {
        matches!(self, Self::Height | Self::Size)
    }

    pub(crate) fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }
}
