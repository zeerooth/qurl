use reqwest::{ClientBuilder, Proxy as ReqwestProxy};
use super::ConfiguresClient;
use crate::error::ErrorWrapper;
use crate::parser::delimiter_parser;

pub struct ProxyData<'a> {
    pub url: &'a str,
    pub auth: Option<&'a str>
}

pub struct Proxy;

impl<'a> ConfiguresClient<ProxyData<'a>> for Proxy {
    fn modify_client(client_builder: ClientBuilder, value: ProxyData) -> Result<ClientBuilder, ErrorWrapper> {
        let mut proxy = ReqwestProxy::all(value.url)?;
        if let Some(auth) = value.auth {
            let auth = delimiter_parser(auth, ":")?;
            proxy = proxy.basic_auth(auth.0, auth.1);
        }
        Ok(client_builder.proxy(proxy))
    }
}
