use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{Arc, Mutex},
    time::{Duration, SystemTime},
};

use redox_core::{Configuration, ConfigurationFile, Deployment};
use tracing::info;
use tuirealm::{
    listener::{ListenerResult, Poll},
    props::{Alignment, Color, TextModifiers},
    tui::layout::{Constraint, Direction, Flex, Layout},
    Application, AttrValue, Attribute, Event, EventListenerCfg, Terminal,
};

use crate::{
    components::{Clock, GlobalListener},
    pages::{Page, PrimaryPage},
    Id, Msg, UserEvent,
};

#[derive(Debug)]
pub struct ModelState {
    configuration_path: PathBuf,
    pub configuration: Option<Configuration>,
    pub current_deployment: Option<Deployment>,
}

impl ModelState {
    fn new(configuration_path: PathBuf) -> Self {
        Self {
            configuration_path,
            configuration: None,
            current_deployment: None,
        }
    }
}

pub struct Model {
    /// Application
    pub app: Application<Id, Msg, UserEvent>,
    /// Indicates that the application must quit
    pub quit: bool,
    /// Tells whether to redraw interface
    pub redraw: bool,
    /// Used to draw to terminal
    pub terminal: Terminal,
    // tracks the current page mounted and viewed
    pub cur_page: Box<dyn Page>,
    // internal event queue for user events
    pub event_queue: InternalEventQueue,
    // model state, tracks a few important cross-cutting values
    pub model_state: ModelState,
}

impl Model {
    pub fn new(terminal: Terminal, configuration_path: PathBuf) -> Self {
        let cur_page = Box::new(PrimaryPage);
        let event_queue = InternalEventQueue::new();
        let init_model_state = ModelState::new(configuration_path);
        Self {
            app: Self::init_app(
                Box::new(PrimaryPage),
                event_queue.clone(),
                &init_model_state,
            ),
            quit: false,
            redraw: true,
            terminal,
            cur_page,
            event_queue,
            model_state: init_model_state,
        }
    }

    pub fn view(&mut self) {
        let required_states = self.cur_page.required_states();
        let mut states = HashMap::new();

        for id in required_states {
            if let Ok(state) = self.app.state(&id) {
                states.insert(id, state);
            }
        }
        assert!(self
            .terminal
            .draw(|f| {
                let [header, body] = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints([Constraint::Length(1), Constraint::Min(0)])
                    .areas(f.size());

                let [clock_area] = Layout::horizontal([Constraint::Length(8)])
                    .flex(Flex::End)
                    .areas(header);

                self.app.view(&Id::Clock, f, clock_area);
                self.cur_page
                    .view(body, &states, &self.model_state)
                    .into_iter()
                    .for_each(|render| {
                        self.app.view(&render.id, f, render.area);
                    });
            })
            .is_ok());
    }

    pub fn init_app(
        initial_page: Box<dyn Page>,
        user_queue: InternalEventQueue,
        init_model_state: &ModelState,
    ) -> Application<Id, Msg, UserEvent> {
        let mut app: Application<Id, Msg, UserEvent> = Application::init(
            EventListenerCfg::default()
                .default_input_listener(Duration::from_millis(20))
                .port(Box::new(user_queue), Duration::from_millis(100))
                .poll_timeout(Duration::from_millis(10))
                .tick_interval(Duration::from_secs(1)),
        );
        assert!(app
            .mount(
                Id::Listener,
                Box::new(GlobalListener::new()),
                GlobalListener::get_subs()
            )
            .is_ok());
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
        initial_page
            .mount(init_model_state)
            .into_iter()
            .for_each(|mount| {
                assert!(app.mount(mount.id, mount.component, mount.subs).is_ok());
            });
        app
    }

    async fn load_configuration(&mut self) {
        let configuration_file =
            ConfigurationFile::load(self.model_state.configuration_path.clone())
                .await
                .unwrap_or_else(|_| {
                    ConfigurationFile::with_path(self.model_state.configuration_path.clone())
                });
        info!("Configuration file loaded");
        // If the configuration has a deployment with default, trigger that user event
        if let Some(deployment) = configuration_file
            .configuration
            .deployments
            .iter()
            .find(|d| d.default == Some(true))
        {
            self.event_queue
                .push(UserEvent::SetCurrentDeployment(deployment.name.clone()));
        }
        self.model_state.configuration = Some(configuration_file.configuration);
    }

    pub async fn update(&mut self, msg: Option<Msg>) -> Option<Msg> {
        if let Some(msg) = msg {
            // Set redraw
            self.redraw = true;
            match msg {
                Msg::AppClose => {
                    self.quit = true; // Terminate
                    None
                }
                Msg::Clock => None,
                Msg::None => None,
                Msg::OpenModal => {
                    self.event_queue.push(UserEvent::ModalChanged(true));
                    None
                }
                Msg::SetActive(id) => {
                    let cur_focus = self.app.query(&id, Attribute::Focus).unwrap();
                    if cur_focus == None || cur_focus == Some(AttrValue::Flag(false)) {
                        self.app.active(&id).ok();
                    }
                    None
                }
                Msg::LoadConfiguration => {
                    self.load_configuration().await;
                    None
                }
            }
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub struct InternalEventQueue {
    events: Arc<Mutex<Vec<UserEvent>>>,
}

impl InternalEventQueue {
    pub fn new() -> Self {
        Self {
            events: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn push(&self, event: UserEvent) {
        let mut events = self.events.lock().unwrap();
        events.push(event);
    }
}

impl Poll<UserEvent> for InternalEventQueue {
    fn poll(&mut self) -> ListenerResult<Option<Event<UserEvent>>> {
        let mut events = self.events.lock().unwrap();
        if events.is_empty() {
            Ok(None)
        } else {
            Ok(Some(Event::User(events.pop().unwrap())))
        }
    }
}
