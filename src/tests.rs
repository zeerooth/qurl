use super::*;
use rstest::rstest;

#[rstest(argument, delimiter, expected,
    case("header_name:header_value", ":", Ok(("header_name", "header_value"))),
)]
fn test_delimiter_parser(argument: &str, delimiter: &str, expected: Result<(&str, &str), String>) {
    assert_eq!(parser::delimiter_parser(argument, delimiter), expected)
}
