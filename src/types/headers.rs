use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::RequestBuilder;
use clap::{Values, ArgMatches, Arg};
use super::super::parser::{delimiter_parser, cmd_colon_kv_parser};
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
                .validator(cmd_colon_kv_parser)
        ]
    }
}
