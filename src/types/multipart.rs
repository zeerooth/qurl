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
                .about("Add a header")
                .takes_value(true)
                .short('h')
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
                .about("Add a form-data element")
                .takes_value(true)
                .short('f')
                .long("form")
                .required(false)
                .multiple(true)
        ]
    }
}

pub struct QueryString;
static QUERYSTRING_ARG: &str = "query";

impl<'a> ConfiguresBuilder<'a, Values<'a>, Vec<(&'a str, &'a str)>> for QueryString {
    fn modify_builder(request_builder: RequestBuilder, value: Vec<(&str, &str)>) -> Result<RequestBuilder, ErrorWrapper> {
        Ok(request_builder.query(&value))
    }

    fn get_value(matches: &'a ArgMatches) -> Option<Values> {
        matches.values_of(QUERYSTRING_ARG)
    }

    fn process_value(value: Values) -> Result<Vec<(&str, &str)>, ErrorWrapper> {
        let mut query_strings = Vec::new();
        for header in value {
            match delimiter_parser(header, "=") {
                Ok(parsed) => {
                    query_strings.push((parsed.0, parsed.1))
                },
                Err(err) => return Err(ParsingError::new(format!("parsing {} failed: {}", QUERYSTRING_ARG,  err).as_str()).into())
            }
        }
        Ok(query_strings)
    }
}

impl ProvidesCLIArguments for QueryString {
    fn provide_arguments() -> Vec<Arg<'static>> {
        vec![
            Arg::new(QUERYSTRING_ARG)
                .about("Add query parameter to target URL")
                .takes_value(true)
                .multiple(true)
                .short('q')
                .long(QUERYSTRING_ARG)
                .required(false)
        ]
    }
}

