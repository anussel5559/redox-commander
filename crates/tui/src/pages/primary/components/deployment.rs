use redox_core::Deployment as CoreDeployment;
use tuirealm::event::{Key, KeyEvent};
use tuirealm::props::{Alignment, BorderType, Borders, Color, TextModifiers};
use tuirealm::{Component, Event, MockComponent, Sub, SubClause, SubEventClause};

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
                .title("Deployment (d)", Alignment::Left)
                .text_modifiers(TextModifiers::BOLD)
                .text_alignment(Alignment::Center),
        }
    }

    pub fn get_subs() -> Vec<Sub<Id, UserEvent>> {
        vec![Sub::new(
            SubEventClause::User(UserEvent::SetCurrentDeployment(CoreDeployment::default())),
            SubClause::Always,
        )]
    }

    pub fn set_value(&mut self, val: Option<String>) {
        self.component.value(val);
    }
}

impl Component<Msg, UserEvent> for Deployment {
    fn on(&mut self, ev: Event<UserEvent>) -> Option<Msg> {
        match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Enter, ..
            }) => None,
            Event::User(UserEvent::SetCurrentDeployment(dep)) => {
                self.set_value(Some(dep.name.clone()));
                Some(Msg::LoadDeployment(dep))
            }
            _ => None,
        }
    }
}
