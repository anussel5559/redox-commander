use chrono::{DateTime, Local, Utc};
use iocraft::prelude::*;
use tracing::{info, Level};

use crate::{pages::primary::PrimaryPage, shared_components::BoxWithTitle};

mod context;
pub use context::AppContext;

#[derive(Clone)]
pub struct ReportedEvent {
    pub time: DateTime<Utc>,
    pub message: String,
    pub level: Level,
}

impl ReportedEvent {
    pub fn new(level: Level, message: String) -> Self {
        ReportedEvent {
            time: Utc::now(),
            message,
            level,
        }
    }
}

#[derive(Copy, Clone)]
pub enum CurrentPage {
    Primary,
}

#[component]
pub fn App(mut hooks: Hooks) -> impl Into<AnyElement<'static>> {
    let (width, height) = hooks.use_terminal_size();
    let mut system = hooks.use_context_mut::<SystemContext>();

    let mut should_exit = hooks.use_state(|| false);
    let mut events = hooks.use_state::<Vec<ReportedEvent>, _>(|| vec![]);
    let mut event_reporter_focus = hooks.use_state(|| false);

    let cur_page = hooks.use_state(|| CurrentPage::Primary);

    let mut app_context = hooks.use_state(|| AppContext::default());

    let mut report_event = move |event: ReportedEvent| {
        info!("Event: {}", event.message);
        let mut updated_events = events.read().clone();
        updated_events.insert(0, event);
        events.set(updated_events);
    };

    let mut load_config = hooks.use_async_handler(move |_| async move {
        let mut current_context = app_context.read().clone();
        current_context.load_configuration().await;
        app_context.set(current_context);
        report_event(ReportedEvent::new(
            Level::INFO,
            "Loaded configuration".into(),
        ));
    });

    let mut update_environments = hooks.use_async_handler(move |_: ()| async move {
        let mut current_context = app_context.read().clone();
        current_context.load_environments().await;
        app_context.set(current_context);
        report_event(ReportedEvent::new(
            Level::INFO,
            format!(
                "Environments for org {} loaded",
                app_context
                    .read()
                    .current_organization
                    .map_or("none".to_string(), |o| o.to_string())
            ),
        ));
    });

    let mut handle_org_change = move |org_id: Option<i32>| {
        let mut new_context = app_context.read().clone();
        new_context.current_organization = org_id;
        app_context.set(new_context);
        report_event(ReportedEvent::new(
            Level::INFO,
            "Organization changed".into(),
        ));
        update_environments(());
    };

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

    if matches!(app_context.read().configuration, None) {
        load_config(());
    }

    {
        let cur_ctx = app_context.read().clone();
        if cur_ctx.current_organization.is_none() {
            // if our context has a deployment with a default_organization, set it
            if let Some(org_id) = cur_ctx
                .current_deployment
                .as_ref()
                .and_then(|d| d.default_org)
            {
                handle_org_change(Some(org_id));
            }
        }
    }

    element! {
        Box(
            // subtract one in case there's a scrollbar
            width: width - 1,
            height,
            padding_top: 1,
            flex_direction: FlexDirection::Column,
        ) {
            ContextProvider(value: Context::owned(app_context.read().clone())) {
                Box(
                    width: 100pct,
                    flex_grow: 1.0
                ){
                    #(match cur_page.get() {
                        CurrentPage::Primary => element! { PrimaryPage }.into_any(),
                    })
                }
            }
            EventReporter(has_focus: event_reporter_focus.get(), events: events.read().clone())
        }
    }
}

#[derive(Default, Props)]
pub struct EventReporterProps {
    events: Vec<ReportedEvent>,
    has_focus: bool,
}

#[component]
pub fn EventReporter(props: &mut EventReporterProps) -> impl Into<AnyElement<'static>> {
    let height: usize = if props.has_focus { 7 } else { 1 };
    let border_color = match props.has_focus {
        true => Color::DarkBlue,
        false => Color::Reset,
    };

    element! {
        BoxWithTitle(
            title: "Events (R)".to_string(),
            border_style: BorderStyle::Round,
            border_color: border_color,
        ) {
            Box(max_height: height as u32, min_height: 1, flex_direction: FlexDirection::Column, width: 100pct) {
                // given our height, we can only show a certain number of events
                #(props.events.iter().take(height).map(|event| {
                    let color = match event.level {
                        Level::ERROR => Color::Red,
                        Level::WARN => Color::Yellow,
                        Level::INFO => Color::Green,
                        Level::DEBUG => Color::Blue,
                        Level::TRACE => Color::Magenta,
                    };
                    element! {
                        Box(width: 100pct, max_height: 1){
                            Text(content: format!("[{}] {} {}", event.level, event.time.with_timezone(&Local).format("%H:%M:%S"), event.message), color: color)
                        }
                    }
                }))
            }
        }
    }
}
