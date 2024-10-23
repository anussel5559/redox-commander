use iocraft::prelude::*;

use crate::shared_components::box_with_title::BoxWithTitle;

#[derive(Copy, Clone, PartialEq)]
enum Selected {
    None,
    Deployment,
    Organization,
    Environment,
}

#[component]
pub fn PrimaryPage(mut hooks: Hooks) -> impl Into<AnyElement<'static>> {
    let selected_color = Color::Green;

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
                    KeyCode::Enter => {},
                    _ => cur_selected.set(Selected::None),
                }
            }
            _ => {}
        }
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
                Text(content: "none".to_string(), align: TextAlign::Center)
            }
            BoxWithTitle(
                title: "Organization (o)".to_string(),
                border_style: BorderStyle::Round,
                border_color: match_selected(Selected::Organization),
            ) {
                Text(content: "none".to_string(), align: TextAlign::Center)
            }
            BoxWithTitle(
                title: "Environment (e)".to_string(),
                border_style: BorderStyle::Round,
                border_color: match_selected(Selected::Environment),
            ) {
                Text(content: "none".to_string(), align: TextAlign::Center)
            }
        }
    }
}
