use iocraft::prelude::*;

#[derive(Props)]
pub struct BoxWithTitleProps<'a> {
    pub title: String,
    pub border_style: BorderStyle,
    pub border_color: Color,
    pub align_content: AlignContent,
    pub children: Vec<AnyElement<'a>>,
}

impl Default for BoxWithTitleProps<'_> {
    fn default() -> Self {
        Self {
            title: String::new(),
            border_style: BorderStyle::Single,
            border_color: Color::Reset,
            align_content: AlignContent::Center,
            children: Vec::new(),
        }
    }
}

#[component]
pub fn BoxWithTitle<'a>(props: &mut BoxWithTitleProps<'a>) -> impl Into<AnyElement<'a>> {
    element! {
        Box(
            width: 100pct,
            border_style: props.border_style,
            border_color: props.border_color,
            flex_direction: FlexDirection::Column,
        ) {
            Box(margin_top: -1, margin_left: 1) {
                Text(content: format!("{}", props.title))
            }
            Box(
                justify_content: props.align_content,
                width: 100pct,
            ) {
                #(&mut props.children)
            }
        }
    }
}
