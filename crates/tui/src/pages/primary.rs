use iocraft::prelude::*;

use crate::{app::AppContext, shared_components::box_with_title::BoxWithTitle};

#[derive(Copy, Clone, PartialEq)]
enum Selected {
    None,
    Deployment,
    Organization,
    ResourcesList,
    Environment,
}

#[component]
pub fn PrimaryPage(mut hooks: Hooks) -> impl Into<AnyElement<'static>> {
    let cur_ctx = hooks.use_context::<AppContext>();

    let deployment_name = cur_ctx
        .current_deployment
        .clone()
        .map_or("none".into(), |d| d.name);
    let current_org = cur_ctx
        .current_organization
        .map_or("none".into(), |d| d.to_string());
    let current_env = cur_ctx
        .env_ctx
        .clone()
        .current_environment
        .map_or("none".into(), |d| format!("{} [{}]", d.name, d.id));

    let mut cur_selected = hooks.use_state(|| Selected::None);
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
                    KeyCode::Char('r') => choose_selected(Selected::ResourcesList),
                    KeyCode::Enter => {}
                    _ => cur_selected.set(Selected::None),
                }
            }
            _ => {}
        }
    });

    element! {
        Box(
            width: 100pct,
            flex_direction: FlexDirection::Column
        ){
            // primary resource controls
            Box(
                width: 100pct,
                height: 3,
                flex_direction: FlexDirection::Row,
                margin_top: 0,
            ) {
                Box(min_width: 16, flex_grow: 1.0) {
                    SingleItem(is_selected: cur_selected.get() == Selected::Deployment, title: "Deployment (d)", value: deployment_name)
                }
                Box(min_width: 18, flex_grow: 1.0) {
                    SingleItem(is_selected: cur_selected.get() == Selected::Organization, title: "Organization (o)", value: current_org)
                }
                Box(min_width: 50, flex_grow: 1.0) {
                    SingleItem(is_selected: cur_selected.get() == Selected::Environment, title: "Environment (e)", value: current_env)
                }
            }
            // primary view - resource selection/list
            Box(
                width: 100pct,
                height: 100pct,
                flex_direction: FlexDirection::Row,
                margin_top: 0
            ) {
                Box(max_width: 30) {
                    SingleItem(is_selected: cur_selected.get() == Selected::ResourcesList, title: "Resources (r)", value: "unused")
                }
            }
        }
    }
}

#[derive(Default, Props)]
pub struct PrimaryControlProps<'a> {
    is_selected: bool,
    title: String,
    children: Vec<AnyElement<'a>>,
}

#[component]
pub fn PrimaryControl<'a>(props: &mut PrimaryControlProps<'a>) -> impl Into<AnyElement<'a>> {
    let match_style = match props.is_selected {
        true => (Color::DarkBlue, BorderStyle::Double),
        false => (Color::Reset, BorderStyle::Round),
    };
    element! {
        BoxWithTitle(
            title: &props.title,
            border_style: match_style.1,
            border_color: match_style.0,
        ) {
            #(&mut props.children)
        }
    }
}

#[derive(Default, Props)]
pub struct SingleItemProps {
    is_selected: bool,
    title: String,
    value: String,
}

#[component]
pub fn SingleItem(props: &mut SingleItemProps) -> impl Into<AnyElement<'static>> {
    element! {
        PrimaryControl(
            is_selected: props.is_selected,
            title: &props.title,
        ) {
            Text(content: &props.value, align: TextAlign::Center)
        }
    }
}
