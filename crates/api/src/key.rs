use anyhow::anyhow;
use std::{fs::File, io::Read};

use jsonwebtoken::EncodingKey;

#[derive(Clone)]
pub struct Key {
    file: String,
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
            file: file_path.clone(),
            signing_key: Some(encoding_key),
        })
    }
}
