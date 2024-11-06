use iocraft::{
    hooks::{UseContext, UseState, UseTerminalEvents},
    prelude::{component, element, AnyElement, Box as IoBox, Text},
    Color, FlexDirection, Hooks, KeyCode, KeyEvent, KeyEventKind, TerminalEvent,
};
use redox_api::models::EnvironmentResources;
use strum::IntoEnumIterator;

use crate::{
    app::AppContext,
    shared_components::{ListBox, SingleItem},
};

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
                    _ => {}
                }
            }
            _ => {}
        }
    });

    let resource_list_renderer: Box<dyn FnMut(&EnvironmentResources, bool) -> AnyElement<'static>> =
        Box::new(|item, is_selected| {
            let (color, background) = match is_selected {
                true => (Color::Yellow, Color::DarkBlue),
                false => (Color::Reset, Color::Reset),
            };

            element! {
                IoBox(width: 100pct, background_color: Some(background)) {
                    Text(content: item.to_string(), color: Some(color))
                }
            }
            .into_any()
        });

    element! {
        IoBox(
            width: 100pct,
            flex_direction: FlexDirection::Column
        ){
            // primary resource controls
            IoBox(
                width: 100pct,
                height: 3,
                flex_direction: FlexDirection::Row,
                margin_top: 0,
            ) {
                IoBox(min_width: 16, flex_grow: 1.0) {
                    SingleItem(is_selected: cur_selected.get() == Selected::Deployment, title: "Deployment (d)", value: deployment_name)
                }
                IoBox(min_width: 18, flex_grow: 1.0) {
                    SingleItem(is_selected: cur_selected.get() == Selected::Organization, title: "Organization (o)", value: current_org)
                }
                IoBox(min_width: 50, flex_grow: 1.0) {
                    SingleItem(is_selected: cur_selected.get() == Selected::Environment, title: "Environment (e)", value: current_env)
                }
            }
            // primary view - resource selection/list
            IoBox(
                width: 100pct,
                height: 100pct,
                flex_direction: FlexDirection::Row,
                margin_top: 0
            ) {
                IoBox(max_width: 35) {
                    ListBox<EnvironmentResources>(
                        is_selected: cur_selected.get() == Selected::ResourcesList,
                        title: "Resources (r)",
                        items: EnvironmentResources::iter().collect::<Vec<EnvironmentResources>>(),
                        item_renderer: resource_list_renderer,
                    )
                }
            }
        }
    }
}
