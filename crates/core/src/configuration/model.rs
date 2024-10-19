use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::{fs::File, path::PathBuf};

use tracing::info;

use crate::util::{parse_yaml, ResultTraced};

#[derive(Debug, Default, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DeploymentAuth {
    pub kid: String,
    #[serde(rename = "clientId")]
    pub client_id: String,
    #[serde(rename = "privateKeyFile")]
    pub private_key_file: String,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Deployment {
    pub name: String,
    #[serde(rename = "authHost")]
    pub auth_host: Option<String>,
    #[serde(rename = "apiHost")]
    pub api_host: String,
    pub default: Option<bool>,
    #[serde(rename = "defaultOrg")]
    pub default_org: Option<i32>,
    pub auth: DeploymentAuth,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
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
