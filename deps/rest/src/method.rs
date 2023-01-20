use std::fmt::{Display, Formatter};

pub enum Method {
    Get,
    Post,
}

impl Display for Method {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let st = match self {
            Self::Get => "GET",
            Self::Post => "POST",
        };

        write!(f, "{st}")
    }
}
