use reqwest::RequestBuilder;
use super::ConfiguresBuilder;
use crate::error::ErrorWrapper;
use clap::ArgMatches;

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

pub struct FormData;
