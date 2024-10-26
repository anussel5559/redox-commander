use std::time::Duration;

use iocraft::prelude::*;

use crate::{app::AppContext, shared_components::box_with_title::BoxWithTitle};

#[derive(Copy, Clone, PartialEq)]
enum Selected {
    None,
    Deployment,
    Organization,
    Environment,
}

#[derive(Default, Props)]
pub struct PrimaryPageProps {
    pub change_organization: Handler<'static, Option<i32>>,
}

#[component]
pub fn PrimaryPage(
    mut hooks: Hooks,
    props: &mut PrimaryPageProps,
) -> impl Into<AnyElement<'static>> {
    let cur_ctx = hooks.use_context::<AppContext>().clone();

    let mut change_org_handler = props.change_organization.take();

    let selected_color = Color::Green;
    let deployment_name = cur_ctx.current_deployment.map_or("none".into(), |d| d.name);
    let current_org = cur_ctx
        .current_organization
        .map_or("none".into(), |d| d.to_string());
    let current_env = cur_ctx
        .current_environment
        .map_or("none".into(), |d| format!("{} [{}]", d.name, d.id));

    let mut cur_selected = hooks.use_state(|| Selected::None);
    let match_selected = |selected: Selected| {
        if cur_selected.get() == selected {
            selected_color
        } else {
            Color::Reset
        }
    };
    let mut choose_selected = move |selected: Selected| {
        if cur_selected.get() == selected {
            cur_selected.set(Selected::None)
        } else {
            cur_selected.set(selected)
        }
    };

    hooks.use_terminal_events({
        move |event| match event {
            TerminalEvent::Key(KeyEvent { code, kind, .. }) if kind != KeyEventKind::Release => {
                match code {
                    KeyCode::Char('d') => choose_selected(Selected::Deployment),
                    KeyCode::Char('o') => choose_selected(Selected::Organization),
                    KeyCode::Char('e') => choose_selected(Selected::Environment),
                    KeyCode::Enter => {}
                    _ => cur_selected.set(Selected::None),
                }
            }
            _ => {}
        }
    });

    hooks.use_future(async move {
        smol::Timer::after(Duration::from_millis(5000)).await;
        change_org_handler(Some(122));
    });

    element! {
        Box(
            width: 100pct,
            height: 3,
            flex_direction: FlexDirection::Row,
        ) {
            BoxWithTitle(
                title: "Deployment (d)".to_string(),
                border_style: BorderStyle::Round,
                border_color: match_selected(Selected::Deployment),
            ) {
                Text(content: deployment_name, align: TextAlign::Center)
            }
            BoxWithTitle(
                title: "Organization (o)".to_string(),
                border_style: BorderStyle::Round,
                border_color: match_selected(Selected::Organization),
            ) {
                Text(content: current_org, align: TextAlign::Center)
            }
            Box(min_width: 70) {
                BoxWithTitle(
                    title: "Environment (e)".to_string(),
                    border_style: BorderStyle::Round,
                    border_color: match_selected(Selected::Environment),
                ) {
                    Text(content: current_env, align: TextAlign::Center)
                }
            }
        }
    }
}
