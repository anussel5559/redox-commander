use iocraft::{
    hooks::{UseState, UseTerminalEvents},
    prelude::{
        component, element, AnyElement, BorderStyle, Box as IoBox, Color, Props, Text, TextAlign,
    },
    FlexDirection, Hooks, KeyCode, KeyEvent, KeyEventKind, TerminalEvent,
};

use crate::shared_components::BoxWithTitle;

#[derive(Default, Props)]
pub struct PrimaryControlProps<'a> {
    pub is_selected: bool,
    pub title: String,
    pub children: Vec<AnyElement<'a>>,
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
    pub is_selected: bool,
    pub title: String,
    pub value: String,
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

#[derive(Props)]
pub struct ListBoxProps<T> {
    pub is_selected: bool,
    pub title: String,
    pub items: Vec<T>,
    pub item_renderer: Box<dyn FnMut(&T, bool) -> AnyElement<'static>>,
    pub selected_index: usize,
}

impl<T> Default for ListBoxProps<T> {
    fn default() -> Self {
        Self {
            is_selected: false,
            title: String::new(),
            items: Vec::new(),
            item_renderer: Box::new(|_, _| {
                element! { Text(content: "not implemented") }.into_any()
            }),
            selected_index: 0,
        }
    }
}

#[component]
pub fn ListBox<T: 'static>(
    mut hooks: Hooks,
    props: &mut ListBoxProps<T>,
) -> impl Into<AnyElement<'static>> {
    let item_renderer = props.item_renderer.as_mut();

    let items_length = props.items.len();
    let mut cur_selection = hooks.use_state(|| props.selected_index);
    let mut is_selected = hooks.use_state(|| props.is_selected);

    // a little wonky, but necessary to avoid checking props.is_selected
    // in the use_terminal_events closure since the `item_renderer` box
    // cannot be sent between threads safely
    match (props.is_selected, is_selected.get()) {
        (true, false) => is_selected.set(true),
        (false, true) => is_selected.set(false),
        _ => {}
    }

    let mut selection_up = move || {
        if cur_selection.get() > 0 && is_selected.get() {
            cur_selection.set(cur_selection.get() - 1)
        }
    };

    let mut selection_down = move || {
        if cur_selection.get() < items_length - 1 && is_selected.get() {
            cur_selection.set(cur_selection.get() + 1)
        }
    };

    hooks.use_terminal_events({
        move |event| match event {
            TerminalEvent::Key(KeyEvent { code, kind, .. }) if kind != KeyEventKind::Release => {
                match code {
                    KeyCode::Up => selection_up(),
                    KeyCode::Down => selection_down(),
                    _ => {}
                }
            }
            _ => {}
        }
    });

    element! {
        PrimaryControl(
            is_selected: props.is_selected,
            title: &props.title,
        ) {
            IoBox(flex_direction: FlexDirection::Column, margin_right: 1, margin_left: 1) {
                #(props.items.iter().enumerate().map(|(index, item)| {
                    item_renderer(item, cur_selection.get() == index)
                }))
            }
        }
    }
}
