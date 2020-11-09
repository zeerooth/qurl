use super::error::{ParsingError, ErrorWrapper};

pub fn delimiter_parser<'a>(val: &'a str, delimiter: &str) -> Result<(&'a str, &'a str), ParsingError> {
    if val.contains(delimiter) {
        let splt = val.splitn(2, delimiter).collect::<Vec<&str>>();
        return Ok((splt[0], splt[1]))
    } else {
        Err(ParsingError::new(format!("couldn't parse '{}': it must be delimited by '{}'", val, delimiter).as_str()))
    }
}

pub fn cmd_header_parser(val: &str) -> Result<(), ErrorWrapper> {
    match delimiter_parser(val, ":") {
        Ok(_res) => Ok(()),
        Err(err) => Err(ParsingError::new(format!("parsing header failed: {}", err).as_str()).into()) 
    }
}

pub fn cmd_param_parser(val: &str) -> Result<(), ErrorWrapper> {
    match delimiter_parser(val, "=") {
        Ok(_res) => Ok(()),
        Err(err) => Err(ParsingError::new(format!("parsing querystring failed: {}", err).as_str()).into()) 
    }
}
