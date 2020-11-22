use clap::{App, Arg, ArgMatches};
use super::parser::{cmd_colon_kv_parser, cmd_param_parser};
use crate::types::auth::BasicAuth;
use crate::types::ProvidesCLIArguments;

pub fn app_matches() -> ArgMatches {
    App::new("qurl")
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
                .validator(cmd_colon_kv_parser)
        )
        .args(
            BasicAuth::provide_arguments()
        )
        .arg(
            Arg::new("bearer")
                .about("bearer auth token")
                .takes_value(true)
                .short('b')
                .long("bearer")
                .required(false)
        )
        .arg(
            Arg::new("body")
                .about("request body")
                .takes_value(true)
                .short('B')
                .long("body")
                .required(false)
        )
        .arg(
            Arg::new("json")
                .about("json data")
                .takes_value(true)
                .short('J')
                .long("json")
                .required(false)
        )
        .arg(
            Arg::new("param")
                .about("querystring parameter")
                .takes_value(true)
                .multiple(true)
                .short('q')
                .long("param")
                .required(false)
                .validator(cmd_param_parser)
        )
        .arg(
            Arg::new("verbose")
                .about("verbose output")
                .short('v')
                .long("verbose")
                .required(false)
        )
        .get_matches()
}