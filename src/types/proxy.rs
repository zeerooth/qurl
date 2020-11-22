use reqwest::{ClientBuilder, Proxy as ReqwestProxy};
use clap::{ArgMatches, Arg};
use super::{ConfiguresClient, ProvidesCLIArguments};
use crate::error::ErrorWrapper;
use crate::parser::delimiter_parser;

pub struct ProxyData<'a> {
    pub url: &'a str,
    pub auth: Option<&'a str>
}

pub struct Proxy;

impl<'a> ConfiguresClient<'a, ProxyData<'a>, ReqwestProxy> for Proxy {
    fn modify_client(client_builder: ClientBuilder, value: ReqwestProxy) -> Result<ClientBuilder, ErrorWrapper> {
        Ok(client_builder.proxy(value))
    }

    fn get_value(matches: &'a ArgMatches) -> Option<ProxyData> {
        if let Some(url) = matches.value_of("proxy") {
            let auth = matches.value_of("auth");
            return Some(ProxyData { url, auth });
        };
        None
    }

    fn process_value(value: ProxyData) -> Result<ReqwestProxy, ErrorWrapper> {
        let mut proxy = ReqwestProxy::all(value.url)?;
        if let Some(auth) = value.auth {
            let auth = delimiter_parser(auth, ":")?;
            proxy = proxy.basic_auth(auth.0, auth.1);
        };
        Ok(proxy)
    }
}

impl ProvidesCLIArguments for Proxy {
    fn provide_arguments() -> Vec<Arg<'static>> {
        vec![
            Arg::new("proxy")
                .about("url for the proxy")
                .takes_value(true)
                .short('P')
                .long("proxy")
                .required(false),
            Arg::new("proxy-auth")
                .about("proxy basic auth")
                .takes_value(true)
                .short('A')
                .long("proxy-auth")
                .required(false)
        ]
    }
}
