use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::RequestBuilder;
use clap::Values;
use super::super::parser::delimiter_parser;
use super::ConfiguresBuilder;
use crate::error::{ErrorWrapper, ParsingError};

pub struct Headers;

pub struct FormData;

impl<'a> ConfiguresBuilder<Values<'a>> for Headers {
    fn modify_builder(request_builder: RequestBuilder, value: Values<'a>) -> Result<RequestBuilder, ErrorWrapper> {
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
        Ok(request_builder.headers(header_map))
    }
}