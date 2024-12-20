use reqwest::Method;
use serde::Deserialize;

use super::{RedoxApiResource, RequestParts};

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub enum EnvironmentFlag {
    Production,
    Staging,
    Development,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OrgObj {
    pub id: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Environment {
    pub name: String,
    #[serde(rename = "environmentFlag")]
    pub environment_flag: EnvironmentFlag,
    pub id: String,
    pub organization: OrgObj,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnvironmentList {
    pub environments: Vec<Environment>,
}

#[derive(Debug, Clone)]
pub struct EnvironmentResource {
    org_id: i32,
}

impl EnvironmentResource {
    pub fn new(org_id: i32) -> Self {
        Self { org_id }
    }
}

impl RedoxApiResource for EnvironmentResource {
    type Item = Environment;
    type List = EnvironmentList;

    fn build_list_request(&self) -> RequestParts<()> {
        RequestParts {
            path: format!(
                "platform/v1/organizations/{}/environments",
                self.org_id.to_string()
            ),
            method: Method::GET,
            body: None,
        }
    }
}
