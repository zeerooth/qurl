use reqwest::{RequestBuilder, ClientBuilder};
use super::error::ErrorWrapper;
use clap::{Arg, ArgMatches};

pub mod data;
pub mod multipart;
pub mod auth;
pub mod proxy;
pub mod redirect;
pub mod timeout;

pub enum BuilderConfig {
    BaiscAuth(auth::BasicAuth)
}

pub trait ConfiguresBuilder<'a, V, T> {
    fn modify_builder(request_builder: RequestBuilder, value: T) -> Result<RequestBuilder, ErrorWrapper>;
    fn get_value(matches: &'a ArgMatches) -> Option<V>;
    fn process_value(value: V) -> Result<T, ErrorWrapper>;
    fn build(builder: RequestBuilder, matches: &'a ArgMatches) -> Result<RequestBuilder, ErrorWrapper>{
        if let Some(value) = Self::get_value(matches) {
            return Ok(Self::modify_builder(builder, Self::process_value(value)?)?);
        }
        Ok(builder)
    }
}

pub trait ConfiguresClient<'a, V, T> {
    fn modify_client(client_builder: ClientBuilder, value: T) -> Result<ClientBuilder, ErrorWrapper>;
    fn get_value(matches: &'a ArgMatches) -> Option<V>;
    fn process_value(value: V) -> Result<T, ErrorWrapper>;
    fn build(builder: ClientBuilder, matches: &'a ArgMatches) -> Result<ClientBuilder, ErrorWrapper>{
        if let Some(value) = Self::get_value(matches) {
            return Ok(Self::modify_client(builder, Self::process_value(value)?)?);
        }
        Ok(builder)
    }
}

pub trait ProvidesCLIArguments {
    fn provide_arguments() -> Vec<Arg<'static>>;
}
