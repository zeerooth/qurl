use reqwest::RequestBuilder;
use super::ConfiguresBuilder;
use crate::error::ErrorWrapper;
use crate::parser::delimiter_parser;

pub struct BasicAuth;

impl ConfiguresBuilder<&str> for BasicAuth {
    fn modify_builder(request_builder: RequestBuilder, value: &str) -> Result<RequestBuilder, ErrorWrapper> {
        let auth = delimiter_parser(value, ":")?;
        Ok(request_builder.basic_auth(auth.0, Some(auth.1)))
    }
}
