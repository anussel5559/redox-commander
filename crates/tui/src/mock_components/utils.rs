use tuirealm::{
    props::{Alignment, Borders, Color, Style, TextModifiers, TextSpan},
    ratatui::{
        style::Modifier,
        text::{Line as Spans, Span},
        widgets::Block,
    },
    AttrValue, Attribute, Props,
};
use unicode_width::UnicodeWidthStr;

/// ### wrap_spans
///
/// Given a vector of `TextSpans`, it creates a list of `Spans` which mustn't exceed the provided width parameter.
/// Each `Spans` in the returned `Vec` is a line in the text.
pub fn wrap_spans<'a>(spans: &[TextSpan], width: usize, props: &Props) -> Vec<Spans<'a>> {
    // Prepare result (capacity will be at least spans.len)
    let mut res: Vec<Spans> = Vec::with_capacity(spans.len());
    // Prepare environment
    let mut line_width: usize = 0; // Incremental line width; mustn't exceed `width`.
    let mut line_spans: Vec<Span> = Vec::new(); // Current line; when done, push to res and re-initialize
    for span in spans.iter() {
        // Get styles
        let (fg, bg, tmod) = use_or_default_styles(props, span);
        // Check if width would exceed...
        if line_width + span.content.width() > width {
            // Check if entire line is wider than the area
            if span.content.width() > width {
                // Wrap
                let span_lines = textwrap::wrap(span.content.as_str(), width);
                // iter lines
                for span_line in span_lines.iter() {
                    // Check if width would exceed...
                    if line_width + span_line.width() > width {
                        // New line
                        res.push(Spans::from(line_spans));
                        line_width = 0;
                        line_spans = Vec::new();
                    }
                    // Increment line width
                    line_width += span_line.width();
                    // Push to line
                    line_spans.push(Span::styled(
                        span_line.to_string(),
                        Style::default().fg(fg).bg(bg).add_modifier(tmod),
                    ));
                }
                // Go to next iteration
                continue;
            } else {
                // Just initialize a new line
                res.push(Spans::from(line_spans));
                line_width = 0;
                line_spans = Vec::new();
            }
        }
        // Push span to line
        line_width += span.content.width();
        line_spans.push(Span::styled(
            span.content.to_string(),
            Style::default().fg(fg).bg(bg).add_modifier(tmod),
        ));
    }
    // if there are still elements in spans, push to result
    if !line_spans.is_empty() {
        res.push(Spans::from(line_spans));
    }
    // return res
    res
}

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
