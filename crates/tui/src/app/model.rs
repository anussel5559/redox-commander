use std::{
    collections::HashMap,
    io::Stdout,
    path::PathBuf,
    sync::{Arc, Mutex},
    time::{Duration, SystemTime},
};

use redox_api::RedoxRequestClient;
use redox_core::{Configuration, ConfigurationFile, Deployment};
use tracing::Level;
use tuirealm::{
    listener::{ListenerResult, Poll},
    props::{Alignment, Color, TextModifiers},
    ratatui::{
        layout::{Constraint, Direction, Flex, Layout},
        prelude::CrosstermBackend,
        Terminal,
    },
    Application, AttrValue, Attribute, Event, EventListenerCfg,
};

use crate::{
    components::{Clock, GlobalListener, Reporter},
    pages::{Page, PrimaryPage},
    util::ResultReported,
    Id, Msg, ReportMessage, UserEvent,
};

#[derive(Clone)]
pub struct ModelState {
    configuration_path: PathBuf,
    pub configuration: Option<Configuration>,
    pub current_deployment: Option<Deployment>,
    pub api_client: Option<RedoxRequestClient>,
}

impl ModelState {
    fn new(configuration_path: PathBuf) -> Self {
        Self {
            configuration_path,
            configuration: None,
            current_deployment: None,
            api_client: None,
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
    pub terminal: Terminal<CrosstermBackend<Stdout>>,
    // tracks the current page mounted and viewed
    pub cur_page: Box<dyn Page>,
    // internal event queue for user events
    pub event_queue: InternalEventQueue,
    // model state, tracks a few important cross-cutting values
    pub model_state: ModelState,
}

impl Model {
    pub fn new(terminal: Terminal<CrosstermBackend<Stdout>>, configuration_path: PathBuf) -> Self {
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
                // if the reporter is focused, give it more length
                let footer_size = match self.app.query(&Id::Reporter, Attribute::Focus).unwrap() {
                    Some(AttrValue::Flag(true)) => 10,
                    _ => 4,
                };

                let [header, body, footer] = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints([
                        Constraint::Length(1),
                        Constraint::Fill(1),
                        Constraint::Length(footer_size),
                    ])
                    .areas(f.area());

                let [clock_area] = Layout::horizontal([Constraint::Length(8)])
                    .flex(Flex::End)
                    .areas(header);

                self.app.view(&Id::Clock, f, clock_area);
                self.app.view(&Id::Reporter, f, footer);

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
                .crossterm_input_listener(Duration::from_millis(20), 1)
                .add_port(Box::new(user_queue), Duration::from_millis(100), 1)
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
        assert!(app
            .mount(
                Id::Reporter,
                Box::new(Reporter::new()),
                Reporter::get_subs(),
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
                .reported(
                    &self.event_queue,
                    ReportMessage {
                        time: SystemTime::now(),
                        message: format!(
                            "Configuration loaded from {:?}",
                            self.model_state.configuration_path
                        ),
                        level: Level::INFO,
                    },
                )
                .unwrap_or_else(|| {
                    ConfigurationFile::with_path(self.model_state.configuration_path.clone())
                });

        // If the configuration has a deployment with default, trigger that user event
        if let Some(deployment) = configuration_file
            .configuration
            .deployments
            .iter()
            .find(|d| d.default == Some(true))
        {
            self.change_deployment(deployment.clone());
        }

        self.model_state.configuration = Some(configuration_file.configuration);
    }

    fn change_deployment(&mut self, new_deployment: Deployment) {
        let new_auth_client =
            RedoxRequestClient::new(&new_deployment.host, &new_deployment.auth.private_key_file)
                .reported(
                    &self.event_queue,
                    ReportMessage {
                        time: SystemTime::now(),
                        message: format!(
                            "Successfully loaded new request client for {}",
                            new_deployment.name
                        ),
                        level: Level::INFO,
                    },
                )
                .map(|client| Some(client))
                .unwrap_or_else(|| None);

        self.model_state.api_client = new_auth_client;
        self.event_queue
            .push(UserEvent::SetCurrentDeployment(new_deployment.name.clone()));
        self.model_state.current_deployment = Some(new_deployment);
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
