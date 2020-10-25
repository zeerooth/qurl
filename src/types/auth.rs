use reqwest::RequestBuilder;
use super::Configurable;

pub struct BasicAuth {
    pub username: String,
    pub password: Option<String>
}

impl From<(&str, Option<&str>)> for BasicAuth {
    fn from(auth: (&str, Option<&str>)) -> Self {
        BasicAuth{ username: auth.0.to_owned(), password: auth.1.map(str::to_string) }
    }
}

impl Configurable for BasicAuth {
    fn modify_builder(&self, reqwest_builder: RequestBuilder) -> RequestBuilder {
        reqwest_builder.basic_auth(&self.username, self.password.as_ref())
    }
}