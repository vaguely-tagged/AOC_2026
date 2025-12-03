use std::{
    error::Error,
    fmt::{Display, Formatter},
};

#[derive(Debug)]
pub enum DayOneError {
    ParseError(std::string::ParseError),
    Utf8Error(std::str::Utf8Error),
    IoError(std::io::Error),
    ParseIntError(std::num::ParseIntError)
}

impl From<std::string::ParseError> for DayOneError {
    fn from(error: std::string::ParseError) -> Self {
        DayOneError::ParseError(error)
    }
}

impl From<std::str::Utf8Error> for DayOneError {
    fn from(error: std::str::Utf8Error) -> Self {
        DayOneError::Utf8Error(error)
    }
}

impl From<std::io::Error> for DayOneError {
    fn from(error: std::io::Error) -> Self {
        DayOneError::IoError(error)
    }
}

impl From<std::num::ParseIntError> for DayOneError {
    fn from(error: std::num::ParseIntError) -> Self {
        DayOneError::ParseIntError(error)
    }
}



impl Error for DayOneError {}
impl Display for DayOneError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Self::ParseError(e) => write!(f, "parse error {:?}", e.to_string()),
            Self::Utf8Error(e) => write!(f, "utf8 error {:?}", e.to_string()),
            Self::IoError(e) => write!(f, "io error {:?}", e.to_string()),
            Self::ParseIntError(e) => write!(f, "parse int error {:?}", e.to_string()),
        }
    }
}
