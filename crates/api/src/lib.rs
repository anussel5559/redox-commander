use anyhow::anyhow;
use chrono::Utc;
use jsonwebtoken::{Algorithm, Header};
use key::Key;
use models::auth::AuthToken;
use reqwest::{Client, StatusCode};
use serde::Serialize;

pub mod key;
pub mod models;

#[derive(Serialize)]
pub struct Claims {
    iss: String,
    sub: String,
    iat: i64,
    exp: i64,
}

#[derive(Clone)]
pub struct Jwt {
    token: String,
    expires_at: i64,
}

#[derive(Clone)]
pub struct Auth {
    pub client_id: String,
    pub kid: String,
    pub jwt: Option<Jwt>,
}

#[derive(Clone)]
pub struct RedoxRequestClient {
    client: Client,
    base_url: String,
    key: Key,
    auth: Auth,
}

impl RedoxRequestClient {
    pub fn new(
        base_url: &String,
        key_file: &String,
        kid: &String,
        client_id: &String,
    ) -> anyhow::Result<Self, anyhow::Error> {
        let key = Key::new(&key_file)?;
        let client = Client::builder().build()?;
        Ok(Self {
            client,
            base_url: base_url.clone(),
            key,
            auth: Auth {
                client_id: client_id.clone(),
                kid: kid.clone(),
                jwt: None,
            },
        })
    }

    fn generate_client_assertion(&self) -> anyhow::Result<String, anyhow::Error> {
        let mut header = Header::new(Algorithm::RS384);
        header.kid = Some(self.auth.kid.clone());

        let now = Utc::now();
        let claims = Claims {
            iss: self.auth.client_id.clone(),
            sub: self.auth.client_id.clone(),
            iat: now.timestamp(),
            exp: now.timestamp() + (60 * 5),
        };
        self.key.generate_signed_jwt(&header, &claims)
    }

    pub async fn retrieve_jwt(&mut self) -> anyhow::Result<(), anyhow::Error> {
        let jwt = self.generate_client_assertion()?;

        let response_jwt = AuthToken::get_auth_token(&self.base_url, &self.client, &jwt)
            .await
            .map_err(|e| {
                anyhow!(format!(
                    "Http failure in retrieving JWT. Status Code: {}.",
                    e.status().unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
                ))
            })?;
        self.auth.jwt = Some(Jwt {
            token: response_jwt.access_token,
            expires_at: Utc::now().timestamp() + response_jwt.expires_in,
        });
        Ok(())
    }
}
