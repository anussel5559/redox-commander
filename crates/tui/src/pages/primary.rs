use std::time::SystemTime;

use tuirealm::{
    props::{Alignment, Color, TextModifiers},
    tui::layout::{Constraint, Direction, Layout},
    Application, Frame, State,
};

use super::{Id, Msg, Page, UserEvent};

mod components;
use components::{Clock, Deployment, SelectAlfa, SelectBeta};

pub struct PrimaryPage;

impl Page for PrimaryPage {
    fn mount(&self, app: &mut Application<Id, Msg, UserEvent>) {
        assert!(app
            .mount(
                Id::Clock,
                Box::new(
                    Clock::new(SystemTime::now())
                        .alignment(Alignment::Center)
                        .background(Color::Reset)
                        .foreground(Color::Cyan)
                        .modifiers(TextModifiers::DIM)
                ),
                Clock::get_subs()
            )
            .is_ok());
        assert!(app
            .mount(
                Id::Deployment,
                Box::new(Deployment::new().set_value(Some("Candi".into()))),
                vec![]
            )
            .is_ok());
        assert!(app
            .mount(
                Id::Organization,
                Box::new(Deployment::new().set_value(Some("6942".into()))),
                vec![]
            )
            .is_ok());
        assert!(app
            .mount(
                Id::Environment,
                Box::new(Deployment::new().set_value(Some("Staging".into()))),
                vec![]
            )
            .is_ok());
        assert!(app
            .mount(Id::SelectAlfa, Box::new(SelectAlfa::default()), vec![])
            .is_ok());
        assert!(app
            .mount(Id::SelectBeta, Box::new(SelectBeta::default()), vec![])
            .is_ok());

        assert!(app.active(&Id::SelectAlfa).is_ok());
    }

    fn view(&self, app: &mut Application<Id, Msg, UserEvent>, frame: &mut Frame<'_>) {
        let [clock_area, header, body] = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Length(1), Constraint::Length(3), Constraint::Min(0)])
            .areas(frame.size());

        let clock_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Fill(1), Constraint::Length(8)])
            .split(clock_area);

        let [deployment_area, org_area, env_area] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(33),
                Constraint::Percentage(34),
                Constraint::Percentage(33),
            ])
            .areas(header);

        let select_alfa_len = match app.state(&Id::SelectAlfa) {
            Ok(State::One(_)) => 3,
            _ => 8,
        };
        let select_beta_len = match app.state(&Id::SelectBeta) {
            Ok(State::One(_)) => 3,
            _ => 8,
        };

        let [select_alfa_area, select_beta_area] = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(select_alfa_len),
                Constraint::Length(select_beta_len),
            ])
            .areas(body);
        app.view(&Id::Deployment, frame, deployment_area);
        app.view(&Id::Organization, frame, org_area);
        app.view(&Id::Environment, frame, env_area);
        app.view(&Id::Clock, frame, clock_chunks[1]);
        app.view(&Id::SelectAlfa, frame, select_alfa_area);
        app.view(&Id::SelectBeta, frame, select_beta_area);
    }
}
