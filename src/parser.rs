pub fn delimiter_parser<'a>(val: &'a str, delimiter: &str) -> Result<(&'a str, &'a str), String> {
    if val.contains(delimiter) {
        let splt = val.splitn(2, delimiter).collect::<Vec<&str>>();
        return Ok((splt[0], splt[1]))
    } else {
        Err(format!("Value must be delimited by '{}'", delimiter).to_string())
    }
}

pub fn cmd_header_parser(val: &str) -> Result<(), String> {
    match delimiter_parser(val, ":") {
        Ok(_res) => Ok(()),
        Err(err) => Err(format!("Header: {}", err).to_string()) 
    }
}
