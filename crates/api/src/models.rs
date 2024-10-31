use reqwest::Method;
use serde::{de::DeserializeOwned, Serialize};
use strum::{Display, EnumIter};

pub mod auth;
pub mod environment;

pub struct RequestParts<T>
where
    T: Serialize,
{
    pub path: String,
    pub method: Method,
    pub body: Option<T>,
}

pub trait RedoxApiResource {
    type Item: DeserializeOwned;
    type List: DeserializeOwned;

    fn build_list_request(&self) -> RequestParts<()>;
}

#[derive(Debug, Clone)]
pub enum RequestType {
    List,
}

#[derive(EnumIter, Display)]
pub enum EnvironmentResources {
    Alerts,
    #[strum(to_string = "Auth credential")]
    AuthCredential,
    #[strum(to_string = "Config modifiers")]
    ConfigModifiers,
    Destination,
    Filters,
    Logs,
    Sources,
    #[strum(to_string = "Translation sets")]
    TranslationSets,
}
