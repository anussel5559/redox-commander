use tui_realm_stdlib::Phantom;
use tuirealm::{
    command::{Cmd, CmdResult},
    event::{Key, KeyEvent, KeyModifiers},
    tui::layout::Rect,
    AttrValue, Attribute, Component, Event, Frame, MockComponent, State, StateValue, Sub,
    SubClause, SubEventClause,
};

use super::{Id, Msg, UserEvent};

pub struct GlobalListener {
    component: Phantom,
    states: OwnStates,
}

impl GlobalListener {
    pub const KEY_EVENT_QUIT: KeyEvent = KeyEvent {
        code: Key::Char('q'),
        modifiers: KeyModifiers::NONE,
    };
    pub const KEY_EVENT_CTRL_C: KeyEvent = KeyEvent {
        code: Key::Char('c'),
        modifiers: KeyModifiers::CONTROL,
    };

    pub const KEY_EVENT_ORG_FOCUS: KeyEvent = KeyEvent {
        code: Key::Char('o'),
        modifiers: KeyModifiers::NONE,
    };

    pub const KEY_EVENT_ENV_FOCUS: KeyEvent = KeyEvent {
        code: Key::Char('e'),
        modifiers: KeyModifiers::NONE,
    };

    pub const KEY_EVENT_DEP_FOCUS: KeyEvent = KeyEvent {
        code: Key::Char('d'),
        modifiers: KeyModifiers::NONE,
    };

    pub fn new() -> Self {
        Self {
            component: Phantom::default(),
            states: OwnStates::new(),
        }
    }

    pub fn get_subs() -> Vec<Sub<Id, UserEvent>> {
        vec![
            Sub::new(
                SubEventClause::Keyboard(Self::KEY_EVENT_QUIT),
                SubClause::HasState(Id::Listener, State::One(StateValue::Bool(false))),
            ),
            Sub::new(
                SubEventClause::Keyboard(Self::KEY_EVENT_CTRL_C),
                SubClause::Always,
            ),
            Sub::new(
                SubEventClause::Keyboard(Self::KEY_EVENT_DEP_FOCUS),
                SubClause::HasState(Id::Listener, State::One(StateValue::Bool(false))),
            ),
            Sub::new(
                SubEventClause::Keyboard(Self::KEY_EVENT_ORG_FOCUS),
                SubClause::HasState(Id::Listener, State::One(StateValue::Bool(false))),
            ),
            Sub::new(
                SubEventClause::Keyboard(Self::KEY_EVENT_ENV_FOCUS),
                SubClause::HasState(Id::Listener, State::One(StateValue::Bool(false))),
            ),
            Sub::new(
                SubEventClause::User(UserEvent::ModalChanged(true)),
                SubClause::Always,
            ),
        ]
    }

    fn get_modal_state(&self) -> bool {
        self.states.modal_open
    }
}

impl MockComponent for GlobalListener {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        // Render
        self.component.view(frame, area);
    }

    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        self.component.query(attr)
    }

    fn attr(&mut self, attr: Attribute, value: AttrValue) {
        self.component.attr(attr, value);
    }

    fn state(&self) -> State {
        // Return current modal state
        State::One(StateValue::Bool(self.get_modal_state()))
    }

    fn perform(&mut self, cmd: Cmd) -> CmdResult {
        self.component.perform(cmd)
    }
}

impl Component<Msg, UserEvent> for GlobalListener {
    fn on(&mut self, ev: Event<UserEvent>) -> Option<Msg> {
        let _ = match ev {
            Event::Keyboard(Self::KEY_EVENT_QUIT) | Event::Keyboard(Self::KEY_EVENT_CTRL_C) => {
                return Some(Msg::AppClose)
            }
            Event::Keyboard(Self::KEY_EVENT_DEP_FOCUS) => {
                return Some(Msg::SetActive(Id::Deployment))
            }
            Event::Keyboard(Self::KEY_EVENT_ORG_FOCUS) => {
                return Some(Msg::SetActive(Id::SelectAlfa))
            }
            Event::Keyboard(Self::KEY_EVENT_ENV_FOCUS) => {
                return Some(Msg::SetActive(Id::SelectBeta))
            }
            Event::User(UserEvent::ModalChanged(val)) => {
                self.states.set_modal_state(val);
                CmdResult::None
            }
            _ => CmdResult::None,
        };
        Some(Msg::None)
    }
}

struct OwnStates {
    modal_open: bool,
}

impl OwnStates {
    pub fn new() -> Self {
        Self { modal_open: false }
    }

    pub fn set_modal_state(&mut self, state: bool) {
        self.modal_open = state;
    }
}
