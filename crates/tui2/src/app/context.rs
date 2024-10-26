use std::sync::{Arc, Mutex};

use redox_api::{
    models::{
        environment::{Environment, EnvironmentFlag, EnvironmentResource},
        RequestType,
    },
    RedoxRequestClient, Response,
};
use redox_core::{Configuration, ConfigurationFile, Deployment};
use tokio::spawn;

#[derive(Default, Clone)]
pub struct AppContext {
    pub configuration: Option<Configuration>,
    pub current_deployment: Option<Deployment>,
    pub current_organization: Option<i32>,
    pub current_environment: Option<Environment>,
    pub environments: Vec<Environment>,
    pub api_client: Option<Arc<Mutex<RedoxRequestClient>>>,
}

impl AppContext {
    pub async fn load_configuration(&mut self) {
        let configuration_path = ConfigurationFile::try_path(None, None).unwrap();
        let configuration_file = ConfigurationFile::load(configuration_path.clone())
            .await
            .unwrap_or_else(|_| ConfigurationFile::with_path(configuration_path));

        self.configuration = Some(configuration_file.configuration);

        // If the configuration has a deployment with default, set the cur_deployment
        if let Some(deployment) = self
            .configuration
            .clone()
            .map_or(vec![], |c| c.deployments)
            .iter()
            .find(|d| d.default == Some(true))
        {
            self.current_deployment = Some(deployment.clone());
            self.current_organization = deployment.default_org;
            self.load_auth_client();
        }
    }

    pub fn load_auth_client(&mut self) {
        if let Some(deployment) = self.current_deployment.as_ref() {
            let new_auth_client = RedoxRequestClient::new(
                &deployment.api_host,
                &deployment.auth_host,
                &deployment.auth.private_key_file,
                &deployment.auth.kid,
                &deployment.auth.client_id,
            )
            .map_or(None, |f| Some(Arc::new(Mutex::new(f))));

            if let Some(client) = new_auth_client {
                let client_clone = Arc::clone(&client);

                spawn(async move {
                    let mut client = client_clone.lock().unwrap().clone();
                    client.refresh_jwt().await
                });

                self.api_client = Some(client);
            }
        }
    }

    pub async fn load_environments(&mut self) {
        if let (Some(client), Some(org_id)) =
            (self.api_client.as_ref(), self.current_organization.as_ref())
        {
            let mut req_client = client.lock().unwrap().clone();
            let environments = req_client
                .make_request(RequestType::List, EnvironmentResource::new(*org_id))
                .await
                .map(|response| match response {
                    Response::List(payload) => payload.environments,
                    _ => vec![],
                })
                .unwrap_or(vec![]);
            self.environments = environments;

            // if the environment list has a Development flag one, set that to current
            if let Some(env) = self
                .environments
                .iter()
                .find(|e| e.environment_flag == EnvironmentFlag::Development)
            {
                self.current_environment = Some(env.clone());
            }
        }
    }
}
