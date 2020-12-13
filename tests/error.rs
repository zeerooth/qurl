use rstest::*;
use colored:: *;
use qurl::error::{ParsingError, ErrorWrapper};
use std::error::Error;
use std::io::{Error as IOError, ErrorKind};

#[rstest(message,
    case("some error message"),
    case(""),
)]
fn test_parsing_error(message: &str) {
    let parsing_error = ParsingError::new(message);
    assert_eq!(format!("{}", parsing_error), message);
}

#[rstest(error, text,
    case(Box::new(ParsingError::new("test")), "test"),
    case(Box::new(IOError::new(ErrorKind::Other, "Some IO Error happened")), "Some IO Error happened"),
)]
fn test_error_wrapping<E: 'static + Error>(error: E, text: &'static str) {
    let wrapped = ErrorWrapper::from(error);
    assert_eq!(format!("{}", wrapped), format!("{}: {}", "error".bright_red(), text))
}