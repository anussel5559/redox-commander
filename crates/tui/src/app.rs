use iocraft::prelude::*;

use crate::{
    pages::primary::PrimaryPage,
    shared_components::{box_with_title::BoxWithTitle, time::Clock},
};

mod context;
pub use context::AppContext;

// pub struct ReportedEvent {
//     pub time: SystemTime,
//     pub message: String,
//     pub level: Level,
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

    let mut app_context = hooks.use_state(|| AppContext::default());

    let mut load_config = hooks.use_async_handler(move |_| async move {
        let mut current_context = app_context.read().clone();
        current_context.load_configuration().await;
        app_context.set(current_context);
    });

    if matches!(app_context.read().configuration, None) {
        load_config(());
    }

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

    let mut handle_org_change = hooks.use_async_handler(move |org_id: Option<i32>| async move {
        let mut new_context = app_context.read().clone();
        new_context.current_organization = org_id;
        new_context.load_environments().await;
        app_context.set(new_context);
    });

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
            ContextProvider(value: Context::owned(app_context.read().clone())) {
                Box(
                    width: 100pct,
                    flex_grow: 1.0
                ){
                    #(match cur_page.get() {
                        CurrentPage::Primary => element! { PrimaryPage(change_organization: move |org| handle_org_change(org)) }.into_any(),
                    })
                }
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
