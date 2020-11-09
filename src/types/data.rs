use reqwest::RequestBuilder;
use super::ConfiguresBuilder;
use crate::error::ErrorWrapper;

pub struct Body;

pub struct Json;

impl ConfiguresBuilder<String> for Body {
    fn modify_builder(request_builder: RequestBuilder, value: String) -> Result<RequestBuilder, ErrorWrapper> {
        Ok(request_builder.body(value.to_string()))
    }
}

impl ConfiguresBuilder<&str> for Json {
    fn modify_builder(request_builder: RequestBuilder, value: &str) -> Result<RequestBuilder, ErrorWrapper> {
        Ok(request_builder.json(&value))
    }
}

