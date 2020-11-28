use reqwest::RequestBuilder;
use super::{ConfiguresBuilder, ProvidesCLIArguments};
use crate::error::ErrorWrapper;
use crate::parser::delimiter_parser;
use clap::{Arg, ArgMatches};

pub struct BasicAuth;
static BASIC_AUTH_ARG: &str = "basic-auth";

impl<'a> ConfiguresBuilder<'a, &'a str, (&'a str, &'a str)> for BasicAuth {
    fn modify_builder(request_builder: RequestBuilder, value: (&'a str, &'a str)) -> Result<RequestBuilder, ErrorWrapper> {
        Ok(request_builder.basic_auth(value.0, Some(value.1)))
    }

    fn get_value(matches: &'a ArgMatches) -> Option<&str> {
        matches.value_of(BASIC_AUTH_ARG)
    }

    fn process_value(value: &'a str) -> Result<(&'a str, &'a str), ErrorWrapper> {
        match delimiter_parser(value, ":") {
            Ok(value) => Ok(value),
            Err(err) => Err(err.into())
        }
    }
}

impl ProvidesCLIArguments for BasicAuth {
    fn provide_arguments() -> Vec<Arg<'static>> {
        vec![
            Arg::new(BASIC_AUTH_ARG)
                .about("Basic Authentication")
                .takes_value(true)
                .short('a')
                .long(BASIC_AUTH_ARG)
                .required(false)
        ]
    }
}
