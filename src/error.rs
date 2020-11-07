use std::{error::Error, fmt};
use colored::*;

#[derive(Debug)]
pub struct ErrorWrapper {
    inner: Box<dyn Error>
}

impl fmt::Display for ErrorWrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", "error".bright_red(), self.inner)
    }
}

impl<T: Error + 'static> std::convert::From<T> for ErrorWrapper {
    fn from(error: T) -> Self {
        Self{ inner: Box::new(error) }
    }
}

// impl std::convert::From<reqwest::Error> for ErrorWrapper {
//     fn from(error: reqwest::Error) -> Self {
//         Self{ inner: Box::new(error) }
//     }
// }

// impl std::convert::From<reqwest::header::InvalidHeaderName> for ErrorWrapper {
//     fn from(error: reqwest::header::InvalidHeaderName) -> Self {
//         Self{ inner: Box::new(error) }
//     }
// }

// impl std::convert::From<reqwest::header::InvalidHeaderValue> for ErrorWrapper {
//     fn from(error: reqwest::header::InvalidHeaderValue) -> Self {
//         Self{ inner: Box::new(error) }
//     }
// }

// impl std::convert::From<ParsingError> for ErrorWrapper {
//     fn from(error: ParsingError) -> Self {
//         Self{ inner: Box::new(error) }
//     }
// }

#[derive(Debug)]
pub struct ParsingError {
    pub info: String
}

impl Error for ParsingError {}

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.info)
    }
}

impl ParsingError {
    pub fn new(message: &str) -> Self {
        Self{ info: message.to_string() }
    }
}

impl PartialEq for ParsingError {
    fn eq(&self, other: &Self) -> bool {
        self.info == other.info
    }
}