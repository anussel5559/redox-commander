use std::collections::HashMap;

use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct AuthTokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub scope: String,
}

pub struct AuthToken;

impl AuthToken {
    pub async fn get_auth_token(
        base_url: &str,
        client: &Client,
        client_assertion: &str,
    ) -> Result<AuthTokenResponse, reqwest::Error> {
        let url = format!("{}/v2/auth/token", base_url);

        let mut form_body = HashMap::new();
        form_body.insert("grant_type".to_string(), "client_credentials".to_string());
        form_body.insert(
            "client_assertion_type".to_string(),
            "urn:ietf:params:oauth:client-assertion-type:jwt-bearer".to_string(),
        );
        form_body.insert("client_assertion".to_string(), client_assertion.to_string());

        let response = client
            .post(&url)
            .form(&form_body)
            .send()
            .await?
            .error_for_status()?
            .json::<AuthTokenResponse>()
            .await?;
        Ok(response)
    }
}
