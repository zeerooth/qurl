use clap::{App, Arg, crate_version};
use crate::types::{
    auth::{BasicAuth, BearerAuth},
    data::{Body, Json, JsonFile},
    multipart::{Headers, FormData, QueryString},
    proxy::Proxy,
    redirect::RedirectPolicy,
    timeout::Timeout,
};
use crate::types::ProvidesCLIArguments;

pub fn app() -> App<'static> {
    App::new("qURL")
        .about("Quick command-line HTTP request utility written in Rust")
        .setting(clap::AppSettings::AllowMissingPositional)
        .version(crate_version!())
        .arg(
            Arg::new("method")
                .about("HTTP request method")
                .index(1)
                .required(false)
                .default_value("get")
        )
        .arg(
            Arg::new("url")
                .about("Target URL")
                .index(2)
                .required(true)
        )
        .arg(
            Arg::new("verbose")
                .about("Verbose output")
                .short('v')
                .long("verbose")
                .required(false)
        )
        .args(Headers::provide_arguments())
        .args(QueryString::provide_arguments())
        .args(BasicAuth::provide_arguments())
        .args(BearerAuth::provide_arguments())
        .args(Body::provide_arguments())
        .args(Json::provide_arguments())
        .args(JsonFile::provide_arguments())
        .args(FormData::provide_arguments())
        .args(Proxy::provide_arguments())
        .args(RedirectPolicy::provide_arguments())
        .args(Timeout::provide_arguments())
}