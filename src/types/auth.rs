use reqwest::RequestBuilder;
use std::ops::Index;
use super::ConfiguresBuilder;
use crate::error::ErrorWrapper;

pub struct BasicAuth;

impl ConfiguresBuilder<(&str, Option<&str>)> for BasicAuth {
    fn modify_builder(reqwest_builder: RequestBuilder, value: (&str, Option<&str>)) -> Result<RequestBuilder, ErrorWrapper> {
        Ok(reqwest_builder.basic_auth(value.0, value.1))
    }
}
