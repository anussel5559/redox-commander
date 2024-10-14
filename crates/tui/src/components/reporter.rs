use tuirealm::command::Cmd;
use tuirealm::props::{Alignment, BorderType, Borders, Color};
use tuirealm::{Component, Event, MockComponent, Sub, SubClause, SubEventClause};

use crate::mock_components::FocusableParagraph;
use crate::{Id, Msg, ReportMessage, UserEvent};

/// Deployment block
#[derive(MockComponent)]
pub struct Reporter {
    component: FocusableParagraph,
}

impl Reporter {
    pub fn new() -> Self {
        Self {
            component: FocusableParagraph::default()
                .borders(
                    Borders::default()
                        .modifiers(BorderType::Rounded)
                        .color(Color::Reset),
                )
                .foreground(Color::Reset)
                .title("Events", Alignment::Left),
        }
    }

    pub fn get_subs() -> Vec<Sub<Id, UserEvent>> {
        vec![Sub::new(
            SubEventClause::User(UserEvent::UpdateReporter(ReportMessage::default())),
            SubClause::Always,
        )]
    }

    pub fn set_value(&mut self, val: Option<ReportMessage>) {
        if let Some(report_message) = val {
            let set_val = format!("[{}] {}", report_message.level, report_message.message);
            self.component.value(Some(set_val));
        }
    }
}

impl Component<Msg, UserEvent> for Reporter {
    fn on(&mut self, ev: Event<UserEvent>) -> Option<Msg> {
        let cmd = match ev {
            Event::User(UserEvent::UpdateReporter(val)) => {
                self.set_value(Some(val));
                Cmd::None
            }
            _ => Cmd::None,
        };
        self.perform(cmd);
        Some(Msg::None)
    }
}
