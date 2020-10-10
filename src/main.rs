use std::env;
use std::process;
use qurl::Method;
use qurl::RequestParser;
use clap::{App, Arg, ArgGroup};
use colored::*;

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
        // Add the version arguments
        .arg("--username      'set version manually'")
        .arg("--password      'auto inc major'")
        .arg("--bearer        'auto inc minor'")
        // Create a group, make it required, and add the above arguments
        .group(
            ArgGroup::new("authentication")
                .args(&["username", "password", "bearer"]),
        )
        // Arguments can also be added to a group individually, these two arguments
        // are part of the "input" group which is not required
        .arg(Arg::from("[INPUT_FILE] 'some regular input'").group("input"))
        .arg(Arg::from("--spec-in [SPEC_IN] 'some special input argument'").group("input"))
        // Now let's assume we have a -c [config] argument which requires one of
        // (but **not** both) the "input" arguments
        .arg(
            Arg::new("header")
                .about("add a header") // Displayed when showing help info
                .takes_value(true) // MUST be set to true in order to be an "option" argument
                .short('H') // This argument is triggered with "-i"
                .long("header") // This argument is triggered with "--input"
                .multiple(true) // Set to true if you wish to allow multiple occurrences
                .required(false) // By default this argument MUST be present
                .min_values(2)
                .max_values(2)
        )
        .get_matches();

    let request = parse_arguments(env::args()).unwrap_or_else(|err| {
        eprintln!("{} {}", "Problem parsing arguments:".red(), err.red());
        process::exit(1);
    });
    let req_config = match request.parse() {
        Ok(config) => config,
        Err(err) => {
            eprintln!("{} {}", "Problem parsing arguments:".red(), err.red());
            process::exit(1);
        }
    };
    let built_request = request.build_request(req_config);
    println!("[DEBUG] Making a request:\n{:#?}", built_request);
    let response = built_request.send().await?;
    println!("[DEBUG] Received response:\n{:#?}", response);
    println!("{}", response.text().await?);
    println!("{:#?}", "[DEBUG] Program finished successfully".green());
    Ok(())
}

fn parse_arguments(mut args: env::Args) -> Result<RequestParser, &'static str> {
    args.next();
    let method = match args.next() {
        Some(arg) => match arg.as_str() {
            "get" => Method::GET,
            "post" => Method::POST,
            "put" => Method::PUT,
            _ => return Err("Method is incorrect, must be one of those: get, post, put, patch, head, delete")
        },
        None => return Err("No method argument")
    };
    let url = match args.next() {
        Some(arg) => arg,
        None => return Err("No url provided")
    };
    let mut data = HashMap::<String, Vec<String>>::new();
    let mut current_key = String::new();
    loop {
        match args.next() {
            Some(arg) => {
                let mut arg_name = arg;
                if arg_name.starts_with("--") {
                    arg_name = arg_name[2..].to_string();
                }
                else if arg_name.starts_with("-") {
                    arg_name = arg_name[1..].to_string();
                    arg_name = match arg_name.as_str() {
                        "H" => String::from("header"),
                        _ => return Err("Short argument doesn't exist")
                    };
                }
                else if !current_key.is_empty() {
                    data.entry(current_key.clone()).or_insert(Vec::new()).push(arg_name);
                    continue;
                }
                else {
                    return Err("Bad argument");
                }
                current_key = arg_name.clone();
                data.entry(arg_name).or_insert(Vec::new());
            },
            None => break
        };
    }
    return Ok(RequestParser::new(method, url, data))
}