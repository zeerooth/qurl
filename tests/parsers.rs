use qurl::parser;
use qurl::error;
use rstest::*;

#[rstest(argument, delimiter, expected,
    case("header_name:header_value", ":", Ok(("header_name", "header_value"))),
    case("key-value-more-delimiters", "-", Ok(("key", "value-more-delimiters"))),
    case("key:value", "=", Err(error::ParsingError::new("'key:value' must be delimited by '='")))
)]
fn test_delimiter_parser(argument: &str, delimiter: &str, expected: Result<(&str, &str), error::ParsingError>) {
    assert_eq!(parser::delimiter_parser(argument, delimiter), expected)
}
