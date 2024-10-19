use std::{
    io::{self, Stdout},
    path::PathBuf,
    time::SystemTime,
};

use anyhow::Result;

mod app;
mod components;
mod mock_components;
mod pages;
mod util;

use app::model::Model;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use redox_api::RedoxRequestClient;
use redox_core::{ConfigurationFile, Deployment};
use tracing::Level;
use tuirealm::{
    ratatui::prelude::CrosstermBackend, ratatui::Terminal, AttrValue, Attribute, PollStrategy,
};

#[derive(Debug, PartialEq)]
pub enum Msg {
    AppClose,
    Clock,
    SetActive(Id),
    LoadConfiguration,
    OpenModal,
    LoadDeployment(Deployment),
    FinalizeDeployment(Deployment, Option<RedoxRequestClient>),
    None,
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Id {
    Clock,
    Label,
    Listener,
    Deployment,
    Organization,
    Environment,
    Reporter,
}

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub struct ReportMessage {
    pub time: SystemTime,
    pub message: String,
    pub level: Level,
}

impl Default for ReportMessage {
    fn default() -> Self {
        Self {
            time: SystemTime::now(),
            message: String::new(),
            level: Level::INFO,
        }
    }
}

#[derive(Debug, Eq, Clone, PartialOrd, Ord)]
pub enum UserEvent {
    ModalChanged(bool),
    SetCurrentDeployment(Deployment),
    DeploymentLoadFinished(Deployment, Option<RedoxRequestClient>),
    SetCurrentOrganization(String),
    SetCurrentEnvironment(String),
    UpdateReporter(ReportMessage),
    None,
}

impl PartialEq for UserEvent {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            _ => true,
        }
    }
}

pub struct Tui {}

impl Tui {
    pub async fn start(collection_path: Option<PathBuf>) -> Result<()> {
        let configuration_path = ConfigurationFile::try_path(None, collection_path)?;
        let term = initialize_terminal()?;

        let mut model = Model::new(term, configuration_path);

        while !model.quit {
            // Tick
            match model.app.tick(PollStrategy::Once) {
                Err(err) => {
                    assert!(model
                        .app
                        .attr(
                            &Id::Label,
                            Attribute::Text,
                            AttrValue::String(format!("Application error: {}", err)),
                        )
                        .is_ok());
                }
                Ok(mut messages) => {
                    if model.model_state.configuration.is_none() {
                        messages.push(Msg::LoadConfiguration);
                    }
                    if messages.len() > 0 {
                        // NOTE: redraw if at least one msg has been processed
                        model.redraw = true;
                        for msg in messages.into_iter() {
                            let mut msg = Some(msg);
                            while msg.is_some() {
                                msg = model.update(msg).await;
                            }
                        }
                    }
                }
            }
            // Redraw
            if model.redraw {
                model.view();
                model.redraw = false;
            }
        }
        // Terminate terminal
        restore_terminal()?;
        Ok(())
    }
}

/// Set up terminal for TUI
fn initialize_terminal() -> anyhow::Result<Terminal<CrosstermBackend<Stdout>>> {
    crossterm::terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    crossterm::execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    Ok(Terminal::new(backend)?)
}

/// Return terminal to initial state
fn restore_terminal() -> anyhow::Result<()> {
    crossterm::terminal::disable_raw_mode()?;
    crossterm::execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture)?;
    Ok(())
}
