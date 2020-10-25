use reqwest::RequestBuilder;
use clap::Values;

pub mod data;
pub mod multivalue;
pub mod auth;

pub trait Configurable {
    fn modify_builder(&self, reqwest_builder: RequestBuilder) -> RequestBuilder;
}

pub trait FromValues: Sized {
    fn from_values(values: Values) -> Result<Self, String>;
}