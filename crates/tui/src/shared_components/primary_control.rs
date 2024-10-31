use iocraft::{
    prelude::{
        component, element, AnyElement, BorderStyle, Box as IoBox, Color, Props, Text, TextAlign,
    },
    FlexDirection,
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
    pub item_renderer: Box<dyn FnMut(&T) -> AnyElement<'static>>,
}

impl<T> Default for ListBoxProps<T> {
    fn default() -> Self {
        Self {
            is_selected: false,
            title: String::new(),
            items: Vec::new(),
            item_renderer: Box::new(|_| element! { Text(content: "not implemented") }.into_any()),
        }
    }
}

#[component]
pub fn ListBox<T: 'static>(props: &mut ListBoxProps<T>) -> impl Into<AnyElement<'static>> {
    let item_renderer = props.item_renderer.as_mut();
    element! {
        PrimaryControl(
            is_selected: props.is_selected,
            title: &props.title,
        ) {
            IoBox(flex_direction: FlexDirection::Column, margin_right: 1, margin_left: 1) {
                #(props.items.iter().map(|item| {
                    item_renderer(item)
                }))
            }
        }
    }
}
