use std::collections::HashMap;
use reqwest::Response;
use reqwest::{Client, RequestBuilder};
use reqwest::header::{HeaderName, HeaderMap, HeaderValue};
use std::cmp::Ordering;

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
    headers: Option<HeaderMap>,
    username: Option<String>,
    password: Option<String>
}

pub struct RequestParser {
    method: Method,
    url: String,
    data: HashMap<String, Vec<String>>,
    reqwest_client: Client,
    reqwest_builder: RequestBuilder
}

impl RequestParser {
    pub fn new(method: Method, url: String, data: HashMap<String, Vec<String>>) -> RequestParser {
        let client = Client::new();
        let req_builder = match method {
            Method::GET => client.get(url.as_str()),
            Method::POST => client.post(url.as_str()),
            Method::PUT => client.put(url.as_str())
        };
        RequestParser {method: method, url: url, data: data, reqwest_client: client, reqwest_builder: req_builder }
    }

    fn basic_auth(mut self, auth: (Option<String>, Option<String>)) -> Self {
        match auth.0 {
            Some(uname) => { 
                self.reqwest_builder = self.reqwest_builder.basic_auth(uname, auth.1);
            },
            None => {}
        };
        return self
    }

    fn headers(mut self, header_map: Option<HeaderMap>) -> Self {
        match header_map {
            Some(h) => {
                self.reqwest_builder = self.reqwest_builder.headers(h);
            }
            None => {}
        }
        return self
    }

    pub fn parse(&self) -> Result<RequestConfig, &'static str> {
        let mut header_map = HeaderMap::new();
        let mut req_config = RequestConfig { headers: None, username: None, password: None };
        for (key, value) in &self.data {
            match key.as_str() {
                "username" => {
                    req_config.username = match handle_unique_argument(&value) {
                        Ok(val) => Some(val.to_owned()),
                        Err(e) => return Err(e)
                    }
                },
                "password" => {
                    req_config.password = match handle_unique_argument(&value) {
                        Ok(val) => Some(val.to_owned()),
                        Err(e) => return Err(e)
                    }
                },
                other => {
                    if other.starts_with("header") {
                        for subval in value {
                            let splt = other.splitn(2, "-").collect::<Vec<&str>>();
                            if splt.len() < 2 {
                                return Err("Invalid header")
                            }
                            header_map.insert(HeaderName::from_lowercase(splt[1].as_bytes()).unwrap(), HeaderValue::from_str(subval.as_str()).unwrap());
                        }
                    }
                }
            }
        }
        req_config.headers = Some(header_map);
        return Ok(req_config)
    }

    pub fn build_request(self, config: RequestConfig) -> RequestBuilder{
        self.basic_auth((config.username, config.password)).headers(config.headers).reqwest_builder
    }
}