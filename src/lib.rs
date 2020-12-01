use reqwest::{Response, Client, ClientBuilder, Request};
use types::{
    auth::{BasicAuth, BearerAuth},
    data::{Body, Json},
    multipart::{Headers, FormData, QueryString},
    proxy::Proxy,
    redirect::RedirectPolicy,
    timeout::Timeout,
    ConfiguresClient,
    ConfiguresBuilder
};
use clap::{ArgMatches};
use error::{ParsingError, ErrorWrapper};
use std::fmt::Display;
use debug::PrettyPrint;

pub mod parser;
pub mod cli;
pub mod types;
pub mod error;
pub mod debug;

#[derive(Debug)]
pub struct RequestParser {
    request: Request,
    client: Client
}

impl RequestParser {
    pub fn new(matches: ArgMatches) -> Result<RequestParser, ErrorWrapper> {
        let client = RequestParser::configure_client(&matches)?;
        let request = RequestParser::configure_request(&client, &matches)?;

        Ok(RequestParser {request, client})
    }

    pub fn configure_client(matches: &ArgMatches) -> Result<Client, ErrorWrapper> {
        let mut client_builder = ClientBuilder::new();
        client_builder = Proxy::build(client_builder, matches)?;
        client_builder = RedirectPolicy::build(client_builder, matches)?;
        Ok(client_builder.build()?)
    }

    pub fn configure_request(client: &Client, matches: &ArgMatches) -> Result<Request, ErrorWrapper> {
        let url = match matches.value_of("url") {
            Some(url) => url,
            None => return Err(ParsingError::new("no url provided").into())
        };
        let mut req_builder = match matches.value_of("method") {
            Some("get") => client.get(url),
            Some("post") => client.post(url),
            Some("put") => client.put(url),
            Some("head") => client.head(url),
            Some("patch") => client.patch(url),
            Some("delete") => client.delete(url),
            Some(other) => return Err(ParsingError::new(format!("invalid method '{}'", other).as_str()).into()),
            None => return Err(ParsingError::new("No method provided").into())
        };
        req_builder = BasicAuth::build(req_builder, matches)?;
        req_builder = BearerAuth::build(req_builder, matches)?;
        req_builder = Body::build(req_builder, matches)?;
        req_builder = Json::build(req_builder, matches)?;
        req_builder = Headers::build(req_builder, matches)?;
        req_builder = FormData::build(req_builder, matches)?;
        req_builder = QueryString::build(req_builder, matches)?;
        req_builder = Timeout::build(req_builder, matches)?;
        Ok(req_builder.build()?)
    }

    pub async fn send(self) -> Result<Response, String> {
        match self.client.execute(self.request).await {
            Ok(response) => Ok(response),
            Err(err) => Err(format!("{}", err))
        }
    }
}

impl Display for RequestParser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", self.prettify().unwrap())
    }
}
