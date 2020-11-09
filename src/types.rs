use reqwest::{RequestBuilder, ClientBuilder};
use super::error::ErrorWrapper;

pub mod data;
pub mod multivalue;
pub mod auth;
pub mod proxy;

pub trait ConfiguresBuilder<T> {
    fn modify_builder(request_builder: RequestBuilder, value: T) -> Result<RequestBuilder, ErrorWrapper>;
}

pub trait ConfiguresClient<T> {
    fn modify_client(client_builder: ClientBuilder, value: T) -> Result<ClientBuilder, ErrorWrapper>;
} 
