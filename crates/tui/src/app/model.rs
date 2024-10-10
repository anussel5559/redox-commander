use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use tuirealm::{
    listener::{ListenerResult, Poll},
    Application, AttrValue, Attribute, Event, EventListenerCfg, Terminal, Update,
};

use crate::{
    components::GlobalListener,
    pages::{Page, PrimaryPage},
    Id, Msg, UserEvent,
};

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
}

impl Model {
    pub fn new(terminal: Terminal) -> Self {
        let cur_page = Box::new(PrimaryPage);
        let event_queue = InternalEventQueue::new();
        Self {
            app: Self::init_app(Box::new(PrimaryPage), event_queue.clone()),
            quit: false,
            redraw: true,
            terminal,
            cur_page,
            event_queue,
        }
    }

    pub fn view(&mut self) {
        assert!(self
            .terminal
            .draw(|f| {
                self.cur_page.view(&mut self.app, f);
            })
            .is_ok());
    }

    pub fn init_app(
        initial_page: Box<dyn Page>,
        user_queue: InternalEventQueue,
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
        initial_page.mount(&mut app);
        app
    }
}

impl Update<Msg> for Model {
    fn update(&mut self, msg: Option<Msg>) -> Option<Msg> {
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
                Msg::OpenDeploymentModal => None,
                Msg::SetModalStatus(val) => {
                    self.event_queue.push(UserEvent::ModalChanged(val));
                    None
                }
                Msg::SetActive(id) => {
                    let cur_focus = self.app.query(&id, Attribute::Focus).unwrap();
                    if cur_focus == None || cur_focus == Some(AttrValue::Flag(false)) {
                        self.app.active(&id).ok();
                    }
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
