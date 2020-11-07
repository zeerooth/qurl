use std::collections::HashMap;
use reqwest::Response;
use reqwest::{Client, RequestBuilder, ClientBuilder};
use reqwest::header::HeaderMap;
use std::cmp::Ordering;
use types::auth::BasicAuth;
use types::ConfiguresBuilder;
use clap::{Values, ArgMatches};
use std::future::Future;
use error::{ParsingError, ErrorWrapper};

pub mod parser;
pub mod cmd;
pub mod types;
pub mod error;

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct RequestParser {
    request_builder: RequestBuilder
}

impl RequestParser {
    pub fn new(matches: ArgMatches) -> Result<RequestParser, ErrorWrapper> {
        let client = RequestParser::configure_client(&matches)?;
        let request_builder = RequestParser::configure_builder(client, &matches)?;
        Ok(RequestParser {request_builder})
    }

    pub fn configure_client(matches: &ArgMatches) -> Result<Client, ErrorWrapper> {
        let client_builder = ClientBuilder::new();
        let built = client_builder.build()?;
        Ok(built)
    }

    pub fn configure_builder(client: Client, matches: &ArgMatches) -> Result<RequestBuilder, ErrorWrapper> {
        let url = match matches.value_of("url") {
            Some(url) => url,
            None => return Err(ParsingError::new("no url provided").into())
        };
        let mut req_builder = match matches.value_of("method") {
            Some("get") => client.get(url),
            Some(other) => return Err(ParsingError::new(format!("invalid method '{}'", other).as_str()).into()),
            None => return Err(ParsingError::new("No method provided").into())
        };
        if let Some(username) = matches.value_of("username") {
            let password = matches.value_of("password");
            req_builder = BasicAuth::modify_builder(req_builder, (username, password))?;
        }
        Ok(req_builder)
    }

    pub async fn send(self) -> Result<Response, String> {
        match self.request_builder.send().await {
            Ok(response) => Ok(response),
            Err(err) => Err(format!("Error sending request: {}", err))
        }
    }
}
