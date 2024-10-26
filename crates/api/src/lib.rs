use core::fmt;
use std::{
    fmt::{Debug, Formatter},
    sync::Arc,
};

use anyhow::anyhow;
use chrono::Utc;
use jsonwebtoken::{Algorithm, Header};
use key::Key;
use models::{auth::AuthToken, RedoxApiResource, RequestType};
use reqwest::{Client, Method, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::from_value;
use tokio::sync::Mutex;

pub mod key;
pub mod models;

#[derive(Serialize)]
pub struct Claims {
    iss: String,
    sub: String,
    iat: i64,
    exp: i64,
}

#[derive(Default, Debug, Clone)]
pub struct Jwt {
    token: String,
    expires_at: i64,
}

#[derive(Default, Debug, Clone)]
pub struct Auth {
    client_id: String,
    kid: String,
    jwt: Arc<Mutex<Option<Jwt>>>,
}

#[derive(Default, Clone)]
pub struct RedoxRequestClient {
    client: Client,
    base_url: String,
    auth_url: Option<String>,
    key: Key,
    auth: Auth,
}

// Cheap trait implementations to get this working with UserEvents in the TUI
// since unfortunately I cannot derive these traits by default for one reason or another
impl Debug for RedoxRequestClient {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("RedoxRequestClient")
            .field("base_url", &self.base_url)
            .field("auth", &self.auth)
            .finish()
    }
}

impl PartialEq for RedoxRequestClient {
    fn eq(&self, other: &Self) -> bool {
        self.base_url == other.base_url
    }
}

impl Eq for RedoxRequestClient {}

impl PartialOrd for RedoxRequestClient {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for RedoxRequestClient {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.base_url.cmp(&other.base_url)
    }
}

impl RedoxRequestClient {
    pub fn new(
        base_url: &String,
        auth_url: &Option<String>,
        key_file: &String,
        kid: &String,
        client_id: &String,
    ) -> anyhow::Result<Self, anyhow::Error> {
        let key = Key::new(&key_file)?;
        let client = Client::builder().build()?;
        Ok(Self {
            client,
            base_url: base_url.clone(),
            auth_url: auth_url.clone(),
            key,
            auth: Auth {
                client_id: client_id.clone(),
                kid: kid.clone(),
                jwt: Arc::new(Mutex::new(None)),
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

    async fn get_new_jwt(&self) -> anyhow::Result<Jwt, anyhow::Error> {
        let jwt = self.generate_client_assertion()?;

        let url = match &self.auth_url {
            Some(url) => url,
            None => &self.base_url,
        };

        let response_jwt = AuthToken::get_auth_token(url, &self.client, &jwt)
            .await
            .map_err(|e| {
                anyhow!(format!(
                    "Http failure in retrieving JWT. Status Code: {}.",
                    e.status().unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
                ))
            })?;

        Ok(Jwt {
            token: response_jwt.access_token,
            expires_at: Utc::now().timestamp() + response_jwt.expires_in - 10,
        })
    }

    pub async fn refresh_jwt(&mut self) -> anyhow::Result<(), anyhow::Error> {
        let mut current_jwt = self.auth.jwt.lock().await;

        if current_jwt.is_none()
            || current_jwt.as_ref().unwrap().expires_at < Utc::now().timestamp()
        {
            let new_jwt = self.get_new_jwt().await?;
            *current_jwt = Some(new_jwt);
        }

        Ok(())
    }

    pub async fn make_request<R>(
        &mut self,
        request_type: RequestType,
        resource: R,
    ) -> anyhow::Result<Response<R>, anyhow::Error>
    where
        R: RedoxApiResource,
    {
        self.refresh_jwt().await?;

        if let Some(jwt) = self.auth.jwt.lock().await.clone() {
            let request_config = match request_type {
                RequestType::List => resource.build_list_request(),
            };

            let request = match request_config.method {
                Method::GET => self
                    .client
                    .get(&format!("{}/{}", self.base_url, request_config.path)),
                _ => unimplemented!(),
            };

            let response = request
                .header("Authorization", format!("Bearer {}", jwt.token))
                .send()
                .await?;

            let response_body = response.json::<GeneralApiResponse>().await?;

            match request_type {
                RequestType::List => {
                    let list = from_value(response_body.payload)?;
                    Ok(Response::List(list))
                } // _ => {
                  //     let item = from_value(response_body.payload)?;
                  //     Ok(Response::Single(item))
                  // }
            }
        } else {
            Err(anyhow!("No JWT available."))
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct Meta {
    version: String,
}

#[derive(Deserialize, Serialize)]
pub struct GeneralApiResponse {
    pub meta: Meta,
    pub payload: serde_json::Value,
}

pub enum Response<R: RedoxApiResource> {
    Single(R::Item),
    List(R::List),
}
