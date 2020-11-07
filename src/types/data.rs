use reqwest::RequestBuilder;
use super::ConfiguresBuilder;
use crate::error::ErrorWrapper;

pub struct Body;

pub struct Json;

impl ConfiguresBuilder<String> for Body {
    fn modify_builder(reqwest_builder: RequestBuilder, value: String) -> Result<RequestBuilder, ErrorWrapper> {
        Ok(reqwest_builder.body(value.to_string()))
    }
}

impl ConfiguresBuilder<String> for Json {
    fn modify_builder(reqwest_builder: RequestBuilder, value: String) -> Result<RequestBuilder, ErrorWrapper> {
        Ok(reqwest_builder.json(&value))
    }
}

