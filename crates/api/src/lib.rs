use reqwest::Client;

pub mod key;

use key::Key;

#[derive(Clone)]
pub struct Jwt {
    token: String,
    iat: i64,
    expires_at: i64,
}

#[derive(Clone)]
pub struct RedoxRequestClient {
    client: Client,
    base_url: String,
    key: Key,
    jwt: Option<Jwt>,
}

impl RedoxRequestClient {
    pub fn new(base_url: &String, key_file: &String) -> anyhow::Result<Self, anyhow::Error> {
        let key = Key::new(&key_file)?;
        let client = Client::builder().build()?;
        Ok(Self {
            client,
            base_url: base_url.clone(),
            key,
            jwt: None,
        })
    }
}
