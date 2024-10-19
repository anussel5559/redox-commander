use std::sync::{Arc, Mutex};

use tuirealm::{
    listener::{ListenerResult, Poll},
    Event,
};

use crate::UserEvent;

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
