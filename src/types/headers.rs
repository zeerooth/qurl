use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::RequestBuilder;
use clap::{Values, ArgMatches};
use super::super::parser::delimiter_parser;
use super::ConfiguresBuilder;
use crate::error::{ErrorWrapper, ParsingError};

pub struct Headers;

impl<'a> ConfiguresBuilder<'a, Values<'a>, HeaderMap> for Headers {
    fn modify_builder(request_builder: RequestBuilder, value: HeaderMap) -> Result<RequestBuilder, ErrorWrapper> {
        Ok(request_builder.headers(value))
    }

    fn get_value(matches: &'a ArgMatches) -> Option<Values> {
        matches.values_of("header")
    }

    fn process_value(value: Values) -> Result<HeaderMap, ErrorWrapper> {
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
        Ok(header_map)
    }
}
