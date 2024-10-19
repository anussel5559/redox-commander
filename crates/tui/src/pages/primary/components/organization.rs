use tuirealm::event::{Key, KeyEvent};
use tuirealm::props::{Alignment, BorderType, Borders, Color, TextModifiers};
use tuirealm::{Component, Event, MockComponent, Sub, SubClause, SubEventClause};

use crate::mock_components::FocusableParagraph;
use crate::{Id, Msg, UserEvent};

/// Deployment block
#[derive(MockComponent)]
pub struct Organization {
    component: FocusableParagraph,
}

impl Organization {
    pub fn new() -> Self {
        Self {
            component: FocusableParagraph::default()
                .borders(
                    Borders::default()
                        .modifiers(BorderType::Rounded)
                        .color(Color::LightGreen),
                )
                .foreground(Color::LightGreen)
                .title("Organization (o)", Alignment::Left)
                .text_modifiers(TextModifiers::BOLD)
                .text_alignment(Alignment::Center),
        }
    }

    pub fn get_subs() -> Vec<Sub<Id, UserEvent>> {
        vec![Sub::new(
            SubEventClause::User(UserEvent::SetCurrentOrganization(String::default())),
            SubClause::Always,
        )]
    }

    pub fn set_value(&mut self, val: Option<String>) {
        self.component.value(val);
    }
}

impl Component<Msg, UserEvent> for Organization {
    fn on(&mut self, ev: Event<UserEvent>) -> Option<Msg> {
        match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Enter, ..
            }) => None,
            Event::User(UserEvent::SetCurrentOrganization(org)) => {
                self.set_value(Some(org.clone()));
                Some(Msg::LoadEnvironments(org))
            }
            _ => None,
        }
    }
}
