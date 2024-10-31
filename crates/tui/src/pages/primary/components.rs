use iocraft::{
    prelude::{component, element, AnyElement, Box as IoBox, Props, Text},
    FlexDirection,
};
use redox_api::models::EnvironmentResources;

use crate::shared_components::PrimaryControl;

#[derive(Props)]
pub struct ListResourcesProps {
    pub is_selected: bool,
    pub title: String,
    pub items: Vec<EnvironmentResources>,
    pub item_renderer: Box<dyn FnMut(&EnvironmentResources) -> AnyElement<'static>>,
}

impl Default for ListResourcesProps {
    fn default() -> Self {
        Self {
            is_selected: false,
            title: String::new(),
            items: Vec::new(),
            item_renderer: Box::new(|item| element! { Text(content: item.to_string()) }.into_any()),
        }
    }
}

#[component]
pub fn ListResources(props: &mut ListResourcesProps) -> impl Into<AnyElement<'static>> {
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
