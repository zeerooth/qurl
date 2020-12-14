use reqwest::RequestBuilder;
use super::{ConfiguresBuilder, ProvidesCLIArguments};
use crate::error::ErrorWrapper;
use crate::io::handle_file;
use clap::{ArgMatches, Arg};

pub struct Body;
static BODY_ARG: &str = "body";

impl<'a> ConfiguresBuilder<'a, &'a str, &'a str> for Body {
    fn modify_builder(request_builder: RequestBuilder, value: &'a str) -> Result<RequestBuilder, ErrorWrapper> {
        Ok(request_builder.body(value.to_string()))
    }

    fn get_value(matches: &'a ArgMatches) -> Option<&str> {
        matches.value_of(BODY_ARG)
    }

    fn process_value(value: &'a str) -> Result<&'a str, ErrorWrapper> {
        Ok(value)
    }
}

impl ProvidesCLIArguments for Body {
    fn provide_arguments() -> Vec<Arg<'static>> {
        vec![
            Arg::new(BODY_ARG)
                .about("Request's Body")
                .takes_value(true)
                .short('b')
                .long(BODY_ARG)
                .required(false)
        ]
    }
}

pub struct BodyFile;
static BODY_FILE_ARG: &str = "body-file";

impl<'a> ConfiguresBuilder<'a, &'a str, String> for BodyFile {
    fn modify_builder(request_builder: RequestBuilder, value: String) -> Result<RequestBuilder, ErrorWrapper> {
        Ok(request_builder.body(value))
    }

    fn get_value(matches: &'a ArgMatches) -> Option<&str> {
        matches.value_of(BODY_FILE_ARG)
    }

    fn process_value(value: &'a str) -> Result<String, ErrorWrapper> {
        handle_file(&value)
    }
}

impl ProvidesCLIArguments for BodyFile {
    fn provide_arguments() -> Vec<Arg<'static>> {
        vec![
            Arg::new(BODY_FILE_ARG)
                .about("Request's Body, loaded from file path")
                .conflicts_with(BODY_ARG)
                .takes_value(true)
                .short('F')
                .long(BODY_FILE_ARG)
                .required(false)
        ]
    }
}

pub struct Json;
static JSON_ARG: &str = "json";

impl<'a> ConfiguresBuilder<'a, &'a str, &'a str> for Json {
    fn modify_builder(request_builder: RequestBuilder, value: &'a str) -> Result<RequestBuilder, ErrorWrapper> {
        Ok(request_builder.json(value))
    }

    fn get_value(matches: &'a ArgMatches) -> Option<&str> {
        matches.value_of(JSON_ARG)
    }

    fn process_value(value: &'a str) -> Result<&'a str, ErrorWrapper> {
        Ok(value)
    }
}

impl ProvidesCLIArguments for Json {
    fn provide_arguments() -> Vec<Arg<'static>> {
        vec![
            Arg::new(JSON_ARG)
                .about("Request's data as json")
                .takes_value(true)
                .short('j')
                .long(JSON_ARG)
                .required(false)
        ]
    }
}

pub struct JsonFile;
static JSON_FILE_ARG: &str = "json-file";

impl<'a> ConfiguresBuilder<'a, &'a str, String> for JsonFile {
    fn modify_builder(request_builder: RequestBuilder, value: String) -> Result<RequestBuilder, ErrorWrapper> {
        Ok(request_builder.json(&value))
    }

    fn get_value(matches: &'a ArgMatches) -> Option<&str> {
        matches.value_of(JSON_FILE_ARG)
    }

    fn process_value(value: &'a str) -> Result<String, ErrorWrapper> {
        handle_file(&value)
    }
}

impl ProvidesCLIArguments for JsonFile {
    fn provide_arguments() -> Vec<Arg<'static>> {
        vec![
            Arg::new(JSON_FILE_ARG)
                .about("Request's data as json, loaded from file path")
                .conflicts_with(JSON_ARG)
                .takes_value(true)
                .short('J')
                .long(JSON_FILE_ARG)
                .required(false)
        ]
    }
}
