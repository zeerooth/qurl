use super::error::{ParsingError};

pub fn delimiter_parser<'a>(val: &'a str, delimiter: &str) -> Result<(&'a str, &'a str), ParsingError> {
    if val.contains(delimiter) {
        let splt = val.splitn(2, delimiter).collect::<Vec<&str>>();
        return Ok((splt[0], splt[1]))
    } else {
        Err(ParsingError::new(format!("'{}' must be delimited by '{}'", val, delimiter).as_str()))
    }
}
