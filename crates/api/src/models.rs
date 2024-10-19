use reqwest::Method;
use serde::{de::DeserializeOwned, Serialize};

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

pub enum RequestType {
    List,
}
