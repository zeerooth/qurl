use reqwest::ClientBuilder;
use reqwest::redirect::Policy;
use super::{ConfiguresClient, ProvidesCLIArguments};
use crate::error::{ErrorWrapper, ParsingError};
use clap::{Arg, ArgMatches};

pub struct RedirectPolicy;
static REDIRECT_ARG: &str = "max-redirects";

impl<'a> ConfiguresClient<'a, &'a str, Policy> for RedirectPolicy {
    fn modify_client(client_builder: ClientBuilder, value: Policy) -> Result<ClientBuilder, ErrorWrapper> {
        Ok(client_builder.redirect(value))
    }

    fn get_value(matches: &'a ArgMatches) -> Option<&str> {
        matches.value_of(REDIRECT_ARG)
    }

    fn process_value(value: &str) -> Result<Policy, ErrorWrapper> {
        let count = match value.parse::<usize>() {
            Ok(count) => count,
            Err(err) => return Err(ParsingError::new(format!("parsing max-redirects failed: {}", err).as_str()).into())
        };
        Ok(Policy::limited(count))
    }
}

impl ProvidesCLIArguments for RedirectPolicy {
    fn provide_arguments() -> Vec<Arg<'static>> {
        vec![
            Arg::new(REDIRECT_ARG)
                .about("Set the maximum number of redirects the program will follow [default: 10]")
                .takes_value(true)
                .short('r')
                .long(REDIRECT_ARG)
                .required(false)
        ]
    }
}
