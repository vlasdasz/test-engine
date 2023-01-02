pub enum Method {
    Get,
    Post,
}

impl ToString for Method {
    fn to_string(&self) -> String {
        match self {
            Self::Get => "GET",
            Self::Post => "POST",
        }
        .into()
    }
}
