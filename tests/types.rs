use rstest::*;
use clap::{ArgMatches};
use qurl::cli::app;
use qurl::types::{
    auth::BasicAuth,
    auth::BearerAuth,
    data::Body,
};
use qurl::types::ConfiguresBuilder;
use qurl::error::ErrorWrapper;
use reqwest::{ClientBuilder, Client, RequestBuilder};

#[fixture]
fn client() -> Client {
    ClientBuilder::new().build().unwrap()
}

#[fixture]
fn request(client: Client) -> RequestBuilder {
    client.get("http://example.com/")
}

#[fixture]
fn example_matches() -> ArgMatches {
    app().get_matches_from(
        vec![
            "qurl",
            "http://example.com/",
            "--basic-auth",
            "username:password",
            "--bearer",
            "SomeToken0987654321",
            "--body",
            "Lorem ipsum dolor sit amet, consectetur adipiscing elit",
        ]
    )
}

#[rstest(value, result,
    case(BasicAuth::get_value(&example_matches()).map(Into::into), Some(String::from("username:password"))),
    case(BearerAuth::get_value(&example_matches()).map(Into::into), Some(String::from("SomeToken0987654321"))),
    case(Body::get_value(&example_matches()).map(Into::into), Some(String::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit"))),
)]
fn test_get_value(value: Option<String>, result: Option<String>) {
    assert_eq!(value, result);
}

#[rstest(value, result,
    case(BasicAuth::process_value("username:password"), Ok(("username", "password"))),
    case(BearerAuth::process_value("SomeToken0987654321"), Ok("SomeToken0987654321")),
    case(Body::process_value("Lorem ipsum dolor sit amet, consectetur adipiscing elit"), Ok("Lorem ipsum dolor sit amet, consectetur adipiscing elit")),
)]
fn test_process_value<T: std::fmt::Debug + std::cmp::PartialEq>(value: Result<T, ErrorWrapper>, result: Result<T, ErrorWrapper>) {
    assert_eq!(value.unwrap(), result.unwrap());
}

