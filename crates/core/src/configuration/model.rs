use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::{fs::File, path::PathBuf};

use tracing::info;

use crate::util::{parse_yaml, ResultTraced};

#[derive(Debug, Deserialize, Serialize)]
pub struct DeploymentAuth {
    kid: String,
    #[serde(rename = "clientId")]
    client_id: String,
    #[serde(rename = "privateKeyFile")]
    private_key_file: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Deployment {
    pub name: String,
    host: String,
    pub default: Option<bool>,
    auth: DeploymentAuth,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Configuration {
    pub deployments: Vec<Deployment>,
}

impl Configuration {
    /// Load configuration from a file
    pub fn load(path: &PathBuf) -> anyhow::Result<Self> {
        info!(?path, "Loading collection file");

        let load = || {
            let file = File::open(path)?;
            let collection = parse_yaml(&file)?;
            Ok::<_, anyhow::Error>(collection)
        };

        load()
            .context(format!("Error loading data from {path:?}"))
            .traced()
    }
}
