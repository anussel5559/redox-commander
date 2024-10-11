use std::io::{self};

use anyhow::Result;

mod app;
mod components;
mod mock_components;
mod pages;

use app::model::Model;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use tuirealm::{
    tui::prelude::CrosstermBackend, AttrValue, Attribute, PollStrategy, Terminal, Update,
};

#[derive(Debug, PartialEq)]
pub enum Msg {
    AppClose,
    Clock,
    SetActive(Id),
    SetModalStatus(bool),
    OpenModal,
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
}

#[derive(Debug, Eq, Clone, PartialOrd, Ord)]
pub enum UserEvent {
    ModalChanged(bool),
    SetCurrentDeployment(String),
    None,
}

impl PartialEq for UserEvent {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (UserEvent::ModalChanged(_), UserEvent::ModalChanged(_)) => true,
            _ => false,
        }
    }
}

pub struct Tui {}

impl Tui {
    pub async fn start() -> Result<()> {
        let term = initialize_terminal()?;
        let mut model = Model::new(term);

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
                Ok(messages) if messages.len() > 0 => {
                    // NOTE: redraw if at least one msg has been processed
                    model.redraw = true;
                    for msg in messages.into_iter() {
                        let mut msg = Some(msg);
                        while msg.is_some() {
                            msg = model.update(msg);
                        }
                    }
                }
                _ => {}
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
fn initialize_terminal() -> anyhow::Result<Terminal> {
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
