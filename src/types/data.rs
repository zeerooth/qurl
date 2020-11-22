use reqwest::RequestBuilder;
use super::{ConfiguresBuilder, ProvidesCLIArguments};
use crate::error::ErrorWrapper;
use clap::{ArgMatches, Arg};

pub struct Body;

impl<'a> ConfiguresBuilder<'a, &'a str, &'a str> for Body {
    fn modify_builder(request_builder: RequestBuilder, value: &'a str) -> Result<RequestBuilder, ErrorWrapper> {
        Ok(request_builder.body(value.to_string()))
    }

    fn get_value(matches: &'a ArgMatches) -> Option<&str> {
        matches.value_of("body")
    }

    fn process_value(value: &'a str) -> Result<&'a str, ErrorWrapper> {
        Ok(value)
    }
}

impl ProvidesCLIArguments for Body {
    fn provide_arguments() -> Vec<Arg<'static>> {
        vec![
            Arg::new("body")
                .about("request body")
                .takes_value(true)
                .short('B')
                .long("body")
                .required(false)
        ]
    }
}

pub struct Json;

impl<'a> ConfiguresBuilder<'a, &'a str, &'a str> for Json {
    fn modify_builder(request_builder: RequestBuilder, value: &'a str) -> Result<RequestBuilder, ErrorWrapper> {
        Ok(request_builder.json(value))
    }

    fn get_value(matches: &'a ArgMatches) -> Option<&str> {
        matches.value_of("json")
    }

    fn process_value(value: &'a str) -> Result<&'a str, ErrorWrapper> {
        Ok(value)
    }
}

impl ProvidesCLIArguments for Json {
    fn provide_arguments() -> Vec<Arg<'static>> {
        vec![
            Arg::new("json")
                .about("json data")
                .takes_value(true)
                .short('J')
                .long("json")
                .required(false)
        ]
    }
}

pub struct FormData;
