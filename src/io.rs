use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use crate::error::{ParsingError, ErrorWrapper};

pub fn handle_file(path: &str) -> Result<String, ErrorWrapper> {
    let file = match File::open(path) {
        Ok(file) => file,
        Err(_err) => return Err(ParsingError::new(format!("No such file: {}", path).as_str()).into())
    };
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    Ok(contents)
}
