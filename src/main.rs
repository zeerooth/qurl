use std::process;
use qurl::Method;
use qurl::RequestParser;
use qurl::RequestConfig;
use qurl::parser::{delimiter_parser};
use qurl::cmd::app_matches;
use colored::*;
use reqwest::header::{HeaderName, HeaderMap, HeaderValue};

use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create application like normal
    let matches = app_matches();
    let verbose = matches.is_present("verbose");
    let request = parse_arguments(&matches).unwrap_or_else(|err| {
        eprintln!("{}: {} {}", "error".bright_red(), "Problem parsing arguments:", err);
        process::exit(1);
    });
    let built_request = request.build_request();
    if verbose { println!("[DEBUG] Making a request:\n{:#?}", built_request); }
    let response = built_request.send().await?;
    if verbose { println!("[DEBUG] Received response:\n{:#?}", response); }
    println!("{}", response.text().await?);
    if verbose { println!("{}", "[DEBUG] Program finished successfully".green()); }
    Ok(())
}

fn parse_arguments(matches: &clap::ArgMatches) -> Result<RequestParser, String> {
    let method = match matches.value_of("method") {
        Some(arg) => match arg {
            "get" => Method::GET,
            "post" => Method::POST,
            "put" => Method::PUT,
            _ => return Err(String::from("Method is incorrect, must be one of those: get, post, put, patch, head, delete"))
        },
        None => return Err(String::from("No method argument"))
    };
    let url = match matches.value_of("url") {
        Some(arg) => arg,
        None => return Err(String::from("No url provided"))
    };
    let mut config = RequestConfig::new();
    if let Some(headers) = matches.values_of("header") {
        let mut header_map = HeaderMap::new();
        for header in headers {
            match delimiter_parser(header, ":") {
                Ok(parsed) => { 
                    let header_name = match HeaderName::from_bytes(parsed.0.as_bytes()) {
                        Ok(h) => h,
                        Err(_err) => { return Err(format!("Invalid header name: '{}'", parsed.0)) }
                    };
                    header_map.insert(header_name, HeaderValue::from_str(parsed.1).unwrap()); 
                },
                Err(_msg) => {}
            }
        }
        config.headers = Some(header_map);
    }
    if let Some(username) = matches.value_of("username") {
        let password = matches.value_of("password");
        config.auth = Some((username.to_owned(), password.map(str::to_string)));
    }
    if let Some(body) = matches.value_of("body") {
        config.body = Some(body.to_owned());
    }
    if let Some(json) = matches.value_of("json") {
        config.body = Some(json.to_owned());
    }
    Ok(RequestParser::new(method, url.to_owned(), config))
}
