use tracing::Level;
use tui_realm_stdlib::Textarea;
use tuirealm::command::{Cmd, Direction, Position};
use tuirealm::event::{Key, KeyEvent};
use tuirealm::props::{Alignment, BorderType, Borders, Color, PropPayload, PropValue, TextSpan};
use tuirealm::{
    AttrValue, Attribute, Component, Event, MockComponent, Sub, SubClause, SubEventClause,
};

use crate::{Id, Msg, ReportMessage, UserEvent};

#[derive(MockComponent)]
pub struct Reporter {
    lines: Vec<TextSpan>,
    component: Textarea,
}

impl Reporter {
    pub fn new() -> Self {
        let initial_lines = vec![];
        Self {
            lines: initial_lines.clone(),
            component: Textarea::default()
                .borders(
                    Borders::default()
                        .modifiers(BorderType::Rounded)
                        .color(Color::Cyan),
                )
                .foreground(Color::Cyan)
                .title("Events (R)", Alignment::Left)
                .text_rows(&initial_lines),
        }
    }

    pub fn get_subs() -> Vec<Sub<Id, UserEvent>> {
        vec![Sub::new(
            SubEventClause::User(UserEvent::UpdateReporter(ReportMessage::default())),
            SubClause::Always,
        )]
    }

    fn level_color(level: Level) -> Color {
        match level {
            Level::ERROR => Color::Red,
            Level::WARN => Color::Yellow,
            Level::INFO => Color::Green,
            Level::DEBUG => Color::Blue,
            Level::TRACE => Color::Magenta,
        }
    }

    pub fn set_value(&mut self, val: Option<ReportMessage>) {
        if let Some(report_message) = val {
            let set_val = format!("[{}] {}", report_message.level, report_message.message);
            self.lines.insert(
                0,
                TextSpan::from(set_val).fg(Self::level_color(report_message.level)),
            );
            self.component.attr(
                Attribute::Text,
                AttrValue::Payload(PropPayload::Vec(
                    self.lines
                        .iter()
                        .cloned()
                        .map(PropValue::TextSpan)
                        .collect(),
                )),
            );
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
            Event::Keyboard(KeyEvent {
                code: Key::Down, ..
            }) => Cmd::Move(Direction::Down),
            Event::Keyboard(KeyEvent { code: Key::Up, .. }) => Cmd::Move(Direction::Up),
            Event::Keyboard(KeyEvent {
                code: Key::PageDown,
                ..
            }) => Cmd::Scroll(Direction::Down),
            Event::Keyboard(KeyEvent {
                code: Key::PageUp, ..
            }) => Cmd::Scroll(Direction::Up),
            Event::Keyboard(KeyEvent {
                code: Key::Home, ..
            }) => Cmd::GoTo(Position::Begin),
            Event::Keyboard(KeyEvent { code: Key::End, .. }) => Cmd::GoTo(Position::End),
            _ => Cmd::None,
        };
        self.perform(cmd);
        Some(Msg::None)
    }
}
