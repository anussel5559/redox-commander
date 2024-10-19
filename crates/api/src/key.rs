use anyhow::anyhow;
use serde::Serialize;
use std::{fs::File, io::Read};

use jsonwebtoken::{encode, EncodingKey, Header};

#[derive(Clone, Default)]
pub struct Key {
    signing_key: Option<EncodingKey>,
}

impl Key {
    pub fn new(file_path: &String) -> anyhow::Result<Self, anyhow::Error> {
        let mut file = File::open(file_path).map_err(|e| {
            anyhow!(format!(
                "Failed to open file at path {}. Error: {:?}",
                file_path, e
            ))
        })?;
        let mut data = String::new();
        file.read_to_string(&mut data)?;

        let encoding_key = EncodingKey::from_rsa_pem(data.as_bytes())
            .map_err(|e| anyhow!(format!("Failed to load in pem file. Error: {:?}", e)))?;

        anyhow::Ok(Self {
            signing_key: Some(encoding_key),
        })
    }

    pub fn generate_signed_jwt<T>(
        &self,
        header: &Header,
        claims: &T,
    ) -> anyhow::Result<String, anyhow::Error>
    where
        T: Serialize,
    {
        let key = self
            .signing_key
            .as_ref()
            .ok_or_else(|| anyhow!("No signing key found"))?;

        encode(header, claims, key)
            .map_err(|e| anyhow!(format!("Failed to encode jwt. Error: {:?}", e)))
    }
}
