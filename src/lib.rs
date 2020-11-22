use reqwest::Response;
use reqwest::{Client, RequestBuilder, ClientBuilder};
use types::auth::{BasicAuth};
use types::data::{Json, Body};
use types::headers::Headers;
use types::ConfiguresBuilder;
use clap::{ArgMatches};
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
        req_builder = BasicAuth::build(req_builder, matches)?;
        req_builder = Body::build(req_builder, matches)?;
        req_builder = Json::build(req_builder, matches)?;
        req_builder = Headers::build(req_builder, matches)?;
        Ok(req_builder)
    }

    pub async fn send(self) -> Result<Response, String> {
        match self.request_builder.send().await {
            Ok(response) => Ok(response),
            Err(err) => Err(format!("Error sending request: {}", err))
        }
    }
}
