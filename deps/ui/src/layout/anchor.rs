use serde::{Deserialize, Serialize};

#[derive(Debug, Default, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum Anchor {
    Top,
    Bot,

    Left,
    Right,

    Width,
    Height,

    MaxWidth,
    MaxHeight,

    MinWidth,
    MinHeight,

    Size,

    CenterX,
    CenterY,

    Center,

    X,
    Y,

    #[default]
    None,
}

impl Anchor {
    pub(crate) fn has_width(self) -> bool {
        matches!(self, Self::Width | Self::Size)
    }

    pub(crate) fn has_height(self) -> bool {
        matches!(self, Self::Height | Self::Size)
    }

    pub(crate) fn is_left(self) -> bool {
        matches!(self, Self::Left)
    }
}
