use reqwest::RequestBuilder;
use std::time::Duration;
use super::{ConfiguresBuilder, ProvidesCLIArguments};
use crate::error::{ErrorWrapper, ParsingError};
use clap::{Arg, ArgMatches};

pub struct Timeout;
static TIMEOUT_ARG: &str = "timeout";

impl<'a> ConfiguresBuilder<'a, &'a str, u64> for Timeout {
    fn modify_builder(client_builder: RequestBuilder, value: u64) -> Result<RequestBuilder, ErrorWrapper> {
        Ok(client_builder.timeout(Duration::from_millis(value)))
    }

    fn get_value(matches: &'a ArgMatches) -> Option<&str> {
        matches.value_of(TIMEOUT_ARG)
    }

    fn process_value(value: &str) -> Result<u64, ErrorWrapper> {
        let time = match value.parse::<u64>() {
            Ok(time) => time,
            Err(err) => return Err(ParsingError::new(format!("parsing {} failed: {}", TIMEOUT_ARG, err).as_str()).into())
        };
        Ok(time)
    }
}

impl ProvidesCLIArguments for Timeout {
    fn provide_arguments() -> Vec<Arg<'static>> {
        vec![
            Arg::new(TIMEOUT_ARG)
                .about("Set the timeout (in ms) for connect, read and write operations (defaults to no timeout, although it may be different depending on your system configuration)")
                .takes_value(true)
                .short('t')
                .long(TIMEOUT_ARG)
                .required(false)
        ]
    }
}
