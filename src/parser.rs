pub fn delimiter_parser<'a>(val: &'a str, delimiter: &str) -> Result<(&'a str, &'a str), String> {
    if val.contains(delimiter) {
        let splt = val.splitn(2, delimiter).collect::<Vec<&str>>();
        if splt[0].len() == 0 {
            return Err(String::from("Provided empty key"))
        }
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

pub fn cmd_param_parser(val: &str) -> Result<(), String> {
    match delimiter_parser(val, "=") {
        Ok(_res) => Ok(()),
        Err(err) => Err(format!("Querystring parameter: {}", err).to_string()) 
    }
}
