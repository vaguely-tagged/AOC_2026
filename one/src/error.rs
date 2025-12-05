use std::{
    backtrace::{self, Backtrace},
    error::Error,
    fmt::{Display, Formatter},
};

#[derive(Debug)]
pub struct DayOneError {
    kind: DayOneErrorKind,
    backtrace: Backtrace,
}

#[derive(Debug)]
enum DayOneErrorKind {
    ParseError(std::string::ParseError),
    Utf8Error(std::str::Utf8Error),
    IoError(std::io::Error),
    ParseIntError(std::num::ParseIntError),
}

impl From<std::string::ParseError> for DayOneError {
    fn from(error: std::string::ParseError) -> Self {
        DayOneError {
            kind: DayOneErrorKind::ParseError(error),
            backtrace: backtrace::Backtrace::capture(),
        }
    }
}

impl From<std::str::Utf8Error> for DayOneError {
    fn from(error: std::str::Utf8Error) -> Self {
        DayOneError {
            kind: DayOneErrorKind::Utf8Error(error),
            backtrace: backtrace::Backtrace::capture(),
        }
    }
}

impl From<std::io::Error> for DayOneError {
    fn from(error: std::io::Error) -> Self {
        DayOneError {
            kind: DayOneErrorKind::IoError(error),
            backtrace: backtrace::Backtrace::capture(),
        }
    }
}

impl From<std::num::ParseIntError> for DayOneError {
    fn from(error: std::num::ParseIntError) -> Self {
        DayOneError {
            kind: DayOneErrorKind::ParseIntError(error),
            backtrace: backtrace::Backtrace::capture(),
        }
    }
}

impl Error for DayOneError {}
impl Display for DayOneError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        match &self.kind {
            DayOneErrorKind::ParseError(e) => write!(
                f,
                "parse error {} bt {}",
                e.to_string(),
                self.backtrace.to_string()
            ),
            DayOneErrorKind::Utf8Error(e) => write!(
                f,
                "utf8 error {} bt {}",
                e.to_string(),
                self.backtrace.to_string()
            ),
            DayOneErrorKind::IoError(e) => write!(
                f,
                "io error {} bt {}",
                e.to_string(),
                self.backtrace.to_string()
            ),
            DayOneErrorKind::ParseIntError(e) => write!(
                f,
                "int parse error {} bt {}",
                e.to_string(),
                self.backtrace.to_string()
            ),
        }
    }
}

// Self::ParseError(e) => write!(f, "parse error {:?} bt: ", e.kind),
// Self::Utf8Error(e) => write!(f, "utf8 error {:?}", e.to_string()),
// Self::IoError(e) => write!(f, "io error {:?}", e.to_string()),
// Self::ParseIntError(e) => write!(f, "parse int error {:?}", e.to_string()),
