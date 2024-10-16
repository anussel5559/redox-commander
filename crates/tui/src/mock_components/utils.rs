use tuirealm::{
    props::{Alignment, Borders, Color, Style, TextModifiers, TextSpan},
    ratatui::{style::Modifier, widgets::Block},
    AttrValue, Attribute, Props,
};

/// ### use_or_default_styles
///
/// Returns the styles to be used; in case in span are default, use props'.
/// The values returned are `(foreground, background, modifiers)`
pub fn use_or_default_styles(props: &Props, span: &TextSpan) -> (Color, Color, Modifier) {
    (
        match span.fg {
            Color::Reset => props
                .get_or(Attribute::Foreground, AttrValue::Color(Color::Reset))
                .unwrap_color(),
            _ => span.fg,
        },
        match span.bg {
            Color::Reset => props
                .get_or(Attribute::Background, AttrValue::Color(Color::Reset))
                .unwrap_color(),
            _ => span.bg,
        },
        match span.modifiers.is_empty() {
            true => props
                .get_or(
                    Attribute::TextProps,
                    AttrValue::TextModifiers(TextModifiers::empty()),
                )
                .unwrap_text_modifiers(),
            false => span.modifiers,
        },
    )
}

/// ### get_block
///
/// Construct a block for widget using block properties.
/// If focus is true the border color is applied, otherwise inactive_style
pub fn get_block<'a>(
    props: Borders,
    title: Option<(String, Alignment)>,
    focus: bool,
    inactive_style: Option<Style>,
) -> Block<'a> {
    let title = title.unwrap_or((String::default(), Alignment::Left));
    Block::default()
        .borders(props.sides)
        .border_style(match focus {
            true => props.style(),
            false => {
                inactive_style.unwrap_or_else(|| Style::default().fg(Color::Reset).bg(Color::Reset))
            }
        })
        .border_type(props.modifiers)
        .title(title.0)
        .title_alignment(title.1)
}
