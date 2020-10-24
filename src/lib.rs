use std::collections::HashMap;
use reqwest::Response;
use reqwest::{Client, RequestBuilder};
use reqwest::header::HeaderMap;
use std::cmp::Ordering;

pub mod parser;
pub mod cmd;

#[cfg(test)]
mod tests;

pub enum Method {
    GET,
    POST,
    PUT
}

pub struct RequestConfig {
    pub headers: Option<HeaderMap>,
    pub params: Option<Vec<(String, String)>>,
    pub auth: Option<(String, Option<String>)>,
    pub bearer: Option<String>,
    pub json: Option<String>,
    pub body: Option<String>
}

impl RequestConfig {
    pub fn new() -> RequestConfig {
        RequestConfig {
            headers: None,
            auth: None,
            bearer: None,
            json: None,
            body: None,
            params: None
        }
    }
}

pub struct RequestParser {
    config: RequestConfig,
    reqwest_builder: RequestBuilder
}

impl RequestParser {
    pub fn new(method: Method, url: String, config: RequestConfig) -> RequestParser {
        let client = Client::new();
        let req_builder = match method {
            Method::GET => client.get(url.as_str()),
            Method::POST => client.post(url.as_str()),
            Method::PUT => client.put(url.as_str())
        };
        RequestParser {config: config, reqwest_builder: req_builder}
    }

    pub fn build_request(mut self) -> RequestBuilder{
        if let Some(headers) = self.config.headers {
            self.reqwest_builder = self.reqwest_builder.headers(headers);
        };
        if let Some(auth) = self.config.auth {
            self.reqwest_builder = self.reqwest_builder.basic_auth(auth.0, auth.1);
        };
        if let Some(bearer) = self.config.bearer {
            self.reqwest_builder = self.reqwest_builder.bearer_auth(bearer);
        };
        if let Some(body) = self.config.body {
            self.reqwest_builder = self.reqwest_builder.body(body);
        };
        if let Some(json) = self.config.json {
            self.reqwest_builder = self.reqwest_builder.json(json.as_str());
        };
        self.reqwest_builder
    }
}