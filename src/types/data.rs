use std::str::FromStr;
use reqwest::RequestBuilder;
use super::Configurable;

pub struct Body {
    pub body: String
}

pub struct Json {
    pub json: String
}

impl FromStr for Body {
    type Err = String;
    fn from_str(val: &str) -> Result<Self, Self::Err> {
        Ok(Self { body: val.to_owned() })
    }
}

impl FromStr for Json {
    type Err = String;
    fn from_str(val: &str) -> Result<Self, Self::Err> {
        Ok(Self { json: val.to_owned() })
    }
}

impl Configurable for Body {
    fn modify_builder(&self, reqwest_builder: RequestBuilder) -> RequestBuilder {
        reqwest_builder.body(self.body.to_string())
    }
}

impl Configurable for Json {
    fn modify_builder(&self, reqwest_builder: RequestBuilder) -> RequestBuilder {
        reqwest_builder.json(&self.json)
    }
}

