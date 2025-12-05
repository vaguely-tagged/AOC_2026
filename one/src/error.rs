use std::{
    error::Error,
    fmt::{Display, Formatter},
};

#[derive(Debug)]
pub enum DayOneError {
    Utf8Error(std::str::Utf8Error, String),
    ParseIntError(std::num::ParseIntError, String),
    LineNotLongEnough(String),
}

impl Error for DayOneError {}
impl Display for DayOneError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        match self {
            DayOneError::Utf8Error(e, context) => write!(
                f,
                "utf8 error {} failed to parse line {}",
                e.to_string(),
                context
            ),
            DayOneError::ParseIntError(e, context) => {
                write!(f, "int parse error {} one line {}", e.to_string(), context)
            }
            DayOneError::LineNotLongEnough(e) => {
                write!(f, "given line does not have the expected length '{e}'")
            }
        }
    }
}
