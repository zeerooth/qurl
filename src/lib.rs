use std::collections::HashMap;
use reqwest::Response;
use reqwest::{Client, RequestBuilder};
use reqwest::header::HeaderMap;
use std::cmp::Ordering;
pub mod parser;

fn split_key_value<'a>(string: &'a str) -> Option<(&'a str, &'a str)> {
    let splt = string.splitn(2, "=").collect::<Vec<&str>>();
    if splt.len() < 2 {
        return None
    }
    return Some((splt[0], splt[1]))
}

fn handle_unique_argument<'a>(values: &'a Vec<String>) -> Result<&'a str, &'static str> {
    match 1.cmp(&values.len()) {
        Ordering::Less => Err("No value provided"),
        Ordering::Equal => Ok(values[0].as_str()),
        Ordering::Greater => Err("More than one value provided")
    }
}

pub enum Method {
    GET,
    POST,
    PUT
}

pub struct RequestConfig {
    pub headers: Option<HeaderMap>,
    pub username: Option<String>,
    pub password: Option<String>
}

impl RequestConfig {
    pub fn new() -> RequestConfig {
        RequestConfig {headers: None, username: None, password: None}
    }
}

pub struct RequestParser {
    method: Method,
    url: String,
    config: RequestConfig,
    reqwest_client: Client,
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
        RequestParser {method: method, url: url, config: config, reqwest_client: client, reqwest_builder: req_builder }
    }

    pub fn build_request(mut self) -> RequestBuilder{
        match self.config.username {
            Some(uname) => { 
                self.reqwest_builder = self.reqwest_builder.basic_auth(uname, self.config.password);
            },
            None => {}
        };
        match self.config.headers {
            Some(h) => {
                self.reqwest_builder = self.reqwest_builder.headers(h);
            }
            None => {}
        }
        self.reqwest_builder
    }
}