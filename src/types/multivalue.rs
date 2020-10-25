use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::RequestBuilder;
use super::super::parser::delimiter_parser;
use super::{Configurable, FromValues};
use clap::Values;

pub struct Headers {
    header_map: HeaderMap
}

pub struct FormData {
    data: Vec<(String, String)>
}

impl FromValues for Headers {
    fn from_values(values: Values) -> Result<Self, String> {
        let mut header_map = HeaderMap::new();
        for header in values {
            match delimiter_parser(header, ":") {
                Ok(parsed) => { 
                    let header_name = match HeaderName::from_bytes(parsed.0.as_bytes()) {
                        Ok(h) => h,
                        Err(_err) => { return Err(format!("Invalid header name: '{}'", parsed.0)) }
                    };
                    header_map.insert(header_name, HeaderValue::from_str(parsed.1).unwrap()); 
                },
                Err(msg) => return Err(format!("Couldn't parse header '{}': '{}'", header, msg))
            }
        }
        Ok(Self{ header_map })
    }
}

impl Configurable for Headers {
    fn modify_builder(&self, reqwest_builder: RequestBuilder) -> RequestBuilder {
        reqwest_builder.headers(self.header_map.to_owned())
    }
}