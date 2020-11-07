use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::RequestBuilder;
use super::super::parser::delimiter_parser;
use super::ConfiguresBuilder;
use crate::error::{ErrorWrapper, ParsingError};

pub struct Headers {
    header_map: HeaderMap
}

pub struct FormData {
    data: Vec<(String, String)>
}

impl ConfiguresBuilder<Vec<&str>> for Headers {
    fn modify_builder(reqwest_builder: RequestBuilder, value: Vec<&str>) -> Result<RequestBuilder, ErrorWrapper> {
        let mut header_map = HeaderMap::new();
        for header in value {
            match delimiter_parser(header, ":") {
                Ok(parsed) => { 
                    let header_name = HeaderName::from_bytes(parsed.0.as_bytes())?;
                    header_map.insert(header_name, HeaderValue::from_str(parsed.1)?); 
                },
                Err(err) => return Err(ParsingError::new(format!("parsing headers failed: {}", err).as_str()).into())
            }
        }
        Ok(reqwest_builder.headers(header_map))
    }
}