//! ## Listener
//!
//! `Listener` is a component which is not rendered. It only purpose is to become a global listener in the application
//! for some kind of events using subscriptions.
//!
//! An example would be a listener for `<q>` key to terminate the application.
//! The Listener allows you not to write a listener for each component for the `q` key, but just to subscribe the listener to it.

use tuirealm::command::{Cmd, CmdResult};
use tuirealm::props::{AttrValue, Attribute, Props};
use tuirealm::ratatui::layout::Rect;
use tuirealm::{Frame, MockComponent, State};

#[derive(Default)]
pub struct Listener {
    props: Props,
}

impl MockComponent for Listener {
    fn view(&mut self, _render: &mut Frame, _area: Rect) {}

    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        self.props.get(attr)
    }

    fn attr(&mut self, attr: Attribute, value: AttrValue) {
        self.props.set(attr, value)
    }

    fn state(&self) -> State {
        State::None
    }

    fn perform(&mut self, _cmd: Cmd) -> CmdResult {
        CmdResult::None
    }
}
