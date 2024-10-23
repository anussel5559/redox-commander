use iocraft::prelude::*;

use crate::{
    pages::primary::PrimaryPage,
    shared_components::{box_with_title::BoxWithTitle, time::Clock},
};

// pub struct ReportedEvent {
//     pub time: SystemTime,
//     pub message: String,
//     pub level: Level,
// }

// pub enum AppContexts {
//     Events(Vec<ReportedEvent>),
// }

#[derive(Copy, Clone)]
pub enum CurrentPage {
    Primary,
}

#[component]
pub fn App(mut hooks: Hooks) -> impl Into<AnyElement<'static>> {
    let (width, height) = hooks.use_terminal_size();
    let mut system = hooks.use_context_mut::<SystemContext>();
    let mut should_exit = hooks.use_state(|| false);

    let mut event_reporter_focus = hooks.use_state(|| false);
    let cur_page = hooks.use_state(|| CurrentPage::Primary);

    hooks.use_terminal_events({
        move |event| match event {
            TerminalEvent::Key(KeyEvent {
                code,
                kind,
                modifiers,
                ..
            }) if kind != KeyEventKind::Release => match (code, modifiers) {
                (KeyCode::Char('q'), _) => should_exit.set(true),
                (KeyCode::Char('R'), KeyModifiers::SHIFT) => {
                    event_reporter_focus.set(!event_reporter_focus.get())
                }
                (_, _) => {}
            },
            _ => {}
        }
    });

    if should_exit.get() {
        system.exit();
    }

    element! {
        Box(
            // subtract one in case there's a scrollbar
            width: width - 1,
            height,
            padding_top: 1,
            flex_direction: FlexDirection::Column,
        ) {
            Box(
                width: 100pct,
                margin_right: 1,
                justify_content: JustifyContent::End,
            ) {
                Clock()
            }
            Box(
                width: 100pct,
                flex_grow: 1.0
            ){
                #(match cur_page.get() {
                    CurrentPage::Primary => element! { PrimaryPage() }.into_any(),
                })
            }
            EventReporter(has_focus: event_reporter_focus.get())
        }
    }
}

#[derive(Default, Props)]
pub struct EventReporterProps {
    has_focus: bool,
}

#[component]
pub fn EventReporter(props: &mut EventReporterProps) -> impl Into<AnyElement<'static>> {
    let height = if props.has_focus { 5 } else { 1 };
    let border_color = if props.has_focus {
        Color::Red
    } else {
        Color::Reset
    };

    element! {
        BoxWithTitle(
            title: "Events (R)".to_string(),
            border_style: BorderStyle::Round,
            border_color: border_color,
        ) {
            Box(height: height) {
                Text(content: "No events to show".to_string())
            }
        }
    }
}
