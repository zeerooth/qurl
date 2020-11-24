use clap::{App, Arg, ArgMatches};
use crate::types::{
    auth::BasicAuth,
    data::Body,
    data::Json,
    multipart::Headers,
    multipart::FormData,
    proxy::Proxy,
    redirect::RedirectPolicy
};
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
            Arg::new("verbose")
                .about("verbose output")
                .short('v')
                .long("verbose")
                .required(false)
        )
        .args(Headers::provide_arguments())
        .args(FormData::provide_arguments())
        .args(BasicAuth::provide_arguments())
        .args(Body::provide_arguments())
        .args(Json::provide_arguments())
        .args(Proxy::provide_arguments())
        .args(RedirectPolicy::provide_arguments())
        .arg(
            Arg::new("param")
                .about("querystring parameter")
                .takes_value(true)
                .multiple(true)
                .short('q')
                .long("param")
                .required(false)
        )
        .arg(
            Arg::new("bearer")
                .about("bearer auth token")
                .takes_value(true)
                .short('b')
                .long("bearer")
                .required(false)
        )
        .get_matches()
}