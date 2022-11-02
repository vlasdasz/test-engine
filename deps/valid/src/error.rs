use std::{error::Error, fmt::Debug};

use strum_macros::Display;

pub type ValidResult<T> = Result<T, ValidError>;

#[derive(Debug, Display, PartialEq)]
pub enum ValidError {
    BadStuff,
}

impl Error for ValidError {}
