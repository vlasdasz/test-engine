#[derive(Default, Debug)]
pub enum Sign {
    #[default]
    Plus,
    Minus,
}

impl Sign {
    pub fn multiplier(&self) -> f32 {
        match self {
            Self::Plus => 1.0,
            Self::Minus => -1.0,
        }
    }
}

impl ToString for Sign {
    fn to_string(&self) -> String {
        match self {
            Self::Plus => '+',
            Self::Minus => '-',
        }
        .to_string()
    }
}
