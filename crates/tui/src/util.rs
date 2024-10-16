use std::ops::Deref;
use tracing::{debug, error, info, trace, warn, Level};

use crate::{app::model::InternalEventQueue, ReportMessage, UserEvent};

/// Extension trait for [Result]
pub trait ResultReported<T, E>: Sized {
    /// If this result is an error, send it over the message channel to be
    /// shown the user, and return `None`. If it's `Ok`, return `Some`.
    fn reported(self, event_queue: &InternalEventQueue, report_message: ReportMessage)
        -> Option<T>;
}

impl<T, E> ResultReported<T, E> for Result<T, E>
where
    E: Into<anyhow::Error>,
{
    fn reported(
        self,
        event_queue: &InternalEventQueue,
        report_message: ReportMessage,
    ) -> Option<T> {
        match self {
            Ok(value) => {
                let log_message = |level: &Level, message: &str| match level {
                    &Level::ERROR => error!(message = message),
                    &Level::WARN => warn!(message = message),
                    &Level::INFO => info!(message = message),
                    &Level::DEBUG => debug!(message = message),
                    &Level::TRACE => trace!(message = message),
                };

                log_message(&report_message.level, report_message.message.as_str());
                event_queue.push(UserEvent::UpdateReporter(report_message));
                Some(value)
            }
            Err(err) => {
                // Trace this too, because anything that should be shown to the
                // user should also be logged
                let err = err.into();
                error!(error = err.deref());
                event_queue.push(UserEvent::UpdateReporter(ReportMessage {
                    message: format!("{}", err),
                    level: Level::ERROR,
                    ..report_message
                }));
                None
            }
        }
    }
}
