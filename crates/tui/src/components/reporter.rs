use std::sync::{Arc, Mutex};
use std::time::UNIX_EPOCH;

use chrono::{DateTime, SecondsFormat};
use tracing::Level;
use tuirealm::command::{Cmd, Direction, Position};
use tuirealm::event::{Key, KeyEvent};
use tuirealm::props::{Alignment, BorderType, Borders, Color, PropPayload, PropValue, TextSpan};
use tuirealm::{
    AttrValue, Attribute, Component, Event, MockComponent, Sub, SubClause, SubEventClause,
};

use crate::mock_components::TextArea;
use crate::{reporting::ReportMessage, Id, Msg, UserEvent};

#[derive(Debug, Clone)]
pub struct FormattedReportMessage {
    report_message: ReportMessage,
    line: TextSpan,
}

#[derive(MockComponent)]
pub struct Reporter {
    lines: Arc<Mutex<Vec<FormattedReportMessage>>>,
    component: TextArea,
}

impl Reporter {
    pub fn new() -> Self {
        let initial_lines = Arc::new(Mutex::new(vec![]));
        Self {
            lines: initial_lines.clone(),
            component: TextArea::default()
                .borders(
                    Borders::default()
                        .modifiers(BorderType::Rounded)
                        .color(Color::Cyan),
                )
                .foreground(Color::Cyan)
                .title("Events (R)", Alignment::Left)
                .text_rows(&vec![]),
        }
    }

    pub fn get_subs() -> Vec<Sub<Id, UserEvent>> {
        vec![Sub::new(
            SubEventClause::User(UserEvent::UpdateReporter(ReportMessage::default())),
            SubClause::Always,
        )]
    }

    fn level_color(level: &Level) -> Color {
        match level {
            &Level::ERROR => Color::Red,
            &Level::WARN => Color::Yellow,
            &Level::INFO => Color::Green,
            &Level::DEBUG => Color::Blue,
            &Level::TRACE => Color::Magenta,
        }
    }

    pub fn set_value(&mut self, val: Option<ReportMessage>) {
        if let Some(report_message) = val {
            let duration_since_epoch = report_message
                .time
                .duration_since(UNIX_EPOCH)
                .expect("time went backwards");
            let datetime = DateTime::from_timestamp(
                duration_since_epoch.as_secs() as i64,
                duration_since_epoch.subsec_nanos(),
            )
            .expect("Somehow the time is invalid.");

            let set_val = format!(
                "{} [{}] {}",
                datetime.to_rfc3339_opts(SecondsFormat::Millis, true),
                report_message.level,
                report_message.message
            );

            let new_message = FormattedReportMessage {
                line: TextSpan::from(set_val).fg(Self::level_color(&report_message.level)),
                report_message,
            };

            // lock the line arc mutex to push the new message on to it.
            let lines = &mut *self.lines.lock().unwrap();

            // Find the correct position to insert the new message
            // since we cannot guarantee the order this method (and it's calling method) are
            // executed in
            let pos = lines
                .binary_search_by(|msg| {
                    new_message
                        .report_message
                        .time
                        .cmp(&msg.report_message.time)
                })
                .unwrap_or_else(|e| e);

            lines.insert(pos, new_message);
            self.component.attr(
                Attribute::Text,
                AttrValue::Payload(PropPayload::Vec(
                    lines
                        .iter()
                        .cloned()
                        .map(|msg| msg.line)
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
