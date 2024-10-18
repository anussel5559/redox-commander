use tuirealm::{
    ratatui::layout::{Constraint, Direction, Layout, Rect},
    State,
};

use super::{app::model::ModelState, Id, Mount, Page, Render};

mod components;
use components::{Deployment, Environment, Organization};

pub struct PrimaryPage;

impl Page for PrimaryPage {
    fn mount(&self, model_state: &ModelState) -> Vec<Mount> {
        let mut deployment = Deployment::new();

        if let Some(cur_deployment) = &model_state.current_deployment {
            deployment.set_value(Some(cur_deployment.name.clone()));
        }

        vec![
            Mount {
                id: Id::Deployment,
                component: Box::new(deployment),
                subs: Deployment::get_subs(),
            },
            Mount {
                id: Id::Organization,
                component: Box::new(Organization::new()),
                subs: Organization::get_subs(),
            },
            Mount {
                id: Id::Environment,
                component: Box::new(Environment::new()),
                subs: Environment::get_subs(),
            },
        ]
    }

    fn required_states(&self) -> Vec<Id> {
        vec![]
    }

    fn view(
        &self,
        area: Rect,
        _states: &std::collections::HashMap<Id, State>,
        _model_state: &ModelState,
    ) -> Vec<Render> {
        let [header, _body] = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(0)])
            .areas(area);

        let [deployment_area, org_area, env_area] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(33),
                Constraint::Percentage(34),
                Constraint::Percentage(33),
            ])
            .areas(header);

        vec![
            Render {
                id: Id::Deployment,
                area: deployment_area,
            },
            Render {
                id: Id::Organization,
                area: org_area,
            },
            Render {
                id: Id::Environment,
                area: env_area,
            },
        ]
    }
}
