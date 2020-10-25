use std::collections::HashMap;
use reqwest::Response;
use reqwest::{Client, RequestBuilder};
use reqwest::header::HeaderMap;
use types::Configurable;
use std::cmp::Ordering;

pub mod parser;
pub mod cmd;
pub mod types;

#[cfg(test)]
mod tests;

pub enum Method {
    GET,
    POST,
    PUT
}
pub struct RequestParser {
    config: Vec<Box<dyn Configurable>>,
    reqwest_builder: RequestBuilder
}

impl RequestParser {
    pub fn new(method: Method, url: String, config: Vec<Box<dyn Configurable>>) -> RequestParser {
        let client = Client::new();
        let reqwest_builder = match method {
            Method::GET => client.get(url.as_str()),
            Method::POST => client.post(url.as_str()),
            Method::PUT => client.put(url.as_str())
        };
        RequestParser {config, reqwest_builder}
    }

    pub fn build_request(mut self) -> RequestBuilder{
        for config_element in self.config {
            self.reqwest_builder = config_element.modify_builder(self.reqwest_builder);
        };
        self.reqwest_builder
    }
}