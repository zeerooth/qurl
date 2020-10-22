use std::process;
use qurl::Method;
use qurl::RequestParser;
use qurl::RequestConfig;
use qurl::parser::{cmd_header_parser, delimiter_parser};
use clap::{App, Arg, ArgGroup};
use colored::*;
use reqwest::header::{HeaderName, HeaderMap, HeaderValue};

use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create application like normal
    let matches = App::new("qurl")
        .about("A fast command-line HTTP request utility written in Rust")
        .arg(
            Arg::new("method")
                .about("HTTP request method")
                .index(1)
                .possible_values(&["get", "post", "put"])
                .required(true)
        )
        .arg(
            Arg::new("url")
                .about("target url")
                .index(2)
                .required(true)
        )
        .arg(
            Arg::new("header")
                .about("add a header")
                .takes_value(true)
                .short('H')
                .long("header")
                .required(false)
                .multiple(true)
                .validator(cmd_header_parser)
        )
        .arg(
            Arg::new("verbose")
                .about("verbose output")
                .short('v')
                .long("verbose")
                .required(false)
        )
        .get_matches();
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
    Ok(RequestParser::new(method, url.to_owned(), config))
}
