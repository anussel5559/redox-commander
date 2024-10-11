use tuirealm::{
    tui::layout::{Constraint, Direction, Layout, Rect},
    State, Sub, SubClause, SubEventClause,
};

use crate::UserEvent;

use super::{Id, Mount, Page, Render};

mod components;
use components::{Deployment, Environment, Organization};

pub struct PrimaryPage;

impl Page for PrimaryPage {
    fn mount(&self) -> Vec<Mount> {
        vec![
            Mount {
                id: Id::Deployment,
                component: Box::new(Deployment::new()),
                subs: vec![Sub::new(
                    SubEventClause::User(UserEvent::SetCurrentDeployment(String::default())),
                    SubClause::Always,
                )],
            },
            Mount {
                id: Id::Organization,
                component: Box::new(Organization::new()),
                subs: vec![],
            },
            Mount {
                id: Id::Environment,
                component: Box::new(Environment::new()),
                subs: vec![],
            },
        ]
    }

    fn required_states(&self) -> Vec<Id> {
        vec![]
    }

    fn view(&self, area: Rect, _states: &std::collections::HashMap<Id, State>) -> Vec<Render> {
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
