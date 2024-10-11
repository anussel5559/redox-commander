use tuirealm::command::Cmd;
use tuirealm::event::{Key, KeyEvent};
use tuirealm::props::{Alignment, BorderType, Borders, Color, TextModifiers};
use tuirealm::{Component, Event, MockComponent, Sub};

use crate::mock_components::FocusableParagraph;
use crate::{Id, Msg, UserEvent};

/// Deployment block
#[derive(MockComponent)]
pub struct Deployment {
    component: FocusableParagraph,
}

impl Deployment {
    pub fn new() -> Self {
        Self {
            component: FocusableParagraph::default()
                .borders(
                    Borders::default()
                        .modifiers(BorderType::Rounded)
                        .color(Color::LightGreen),
                )
                .foreground(Color::LightGreen)
                .title("Deployment", Alignment::Left)
                .text_modifiers(TextModifiers::BOLD)
                .text_alignment(Alignment::Center),
        }
    }

    pub fn get_subs() -> Vec<Sub<Id, UserEvent>> {
        vec![]
    }

    pub fn set_value(mut self, val: Option<String>) -> Self {
        self.component.value(val);
        self
    }
}

impl Component<Msg, UserEvent> for Deployment {
    fn on(&mut self, ev: Event<UserEvent>) -> Option<Msg> {
        let cmd = match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Enter, ..
            }) => Cmd::None,
            _ => Cmd::None,
        };
        self.perform(cmd);
        Some(Msg::None)
    }
}
