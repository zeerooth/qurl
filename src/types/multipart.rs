use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::RequestBuilder;
use clap::{Values, ArgMatches, Arg};
use super::super::parser::{delimiter_parser};
use super::{ConfiguresBuilder, ProvidesCLIArguments};
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

impl ProvidesCLIArguments for Headers {
    fn provide_arguments() -> Vec<Arg<'static>> {
        vec![
            Arg::new("header")
                .about("add a header")
                .takes_value(true)
                .short('H')
                .long("header")
                .required(false)
                .multiple(true)
        ]
    }
}

pub struct FormData;

impl<'a> ConfiguresBuilder<'a, Values<'a>, Vec<(&'a str, &'a str)>> for FormData {
    fn modify_builder(request_builder: RequestBuilder, value: Vec<(&str, &str)>) -> Result<RequestBuilder, ErrorWrapper> {
        Ok(request_builder.form(&value))
    }

    fn get_value(matches: &'a ArgMatches) -> Option<Values> {
        matches.values_of("form")
    }

    fn process_value(value: Values) -> Result<Vec<(&str, &str)>, ErrorWrapper> {
        let mut form_data = Vec::new();
        for header in value {
            match delimiter_parser(header, "=") {
                Ok(parsed) => { 
                    form_data.push((parsed.0, parsed.1))
                },
                Err(err) => return Err(ParsingError::new(format!("parsing form-data failed: {}", err).as_str()).into())
            }
        }
        Ok(form_data)
    }
}

impl ProvidesCLIArguments for FormData {
    fn provide_arguments() -> Vec<Arg<'static>> {
        vec![
            Arg::new("form")
                .about("add a form-data element")
                .takes_value(true)
                .short('F')
                .long("form")
                .required(false)
                .multiple(true)
        ]
    }
}
