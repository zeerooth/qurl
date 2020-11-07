use super::*;
use rstest::*;

#[fixture]
pub fn command() -> u32 { 42 }

#[rstest(argument, delimiter, expected,
    case("header_name:header_value", ":", Ok(("header_name", "header_value"))),
    case("key:value", "=", Err(error::ParsingError::new("value must be delimited by ':'")))
)]
fn test_delimiter_parser(argument: &str, delimiter: &str, expected: Result<(&str, &str), error::ParsingError>) {
    assert_eq!(parser::delimiter_parser(argument, delimiter), expected)
}
