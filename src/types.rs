use reqwest::{RequestBuilder, Client};
use clap::Values;
use super::error::ErrorWrapper;
use std::ops::Index;
use std::fmt::Display;

pub mod data;
pub mod multivalue;
pub mod auth;

pub trait ConfiguresBuilder<T> {
    fn modify_builder(reqwest_builder: RequestBuilder, value: T) -> Result<RequestBuilder, ErrorWrapper>;
}

pub trait ConfiguresClient<T> {
    fn modify_client(reqwest_client: Client, value: T) -> Result<Client, ErrorWrapper>;
} 
