use tuirealm::command::{Cmd, CmdResult};
use tuirealm::props::{Alignment, BorderSides, Borders, Color, Style, TextModifiers};
use tuirealm::tui::layout::Rect;
use tuirealm::tui::text::Text;
use tuirealm::tui::widgets::{Block, Paragraph};
use tuirealm::{AttrValue, Attribute, Frame, MockComponent, Props, State, StateValue};

#[derive(Default)]
pub struct FocusableParagraph {
    props: Props,
    pub states: OwnStates,
}

impl FocusableParagraph {
    pub fn borders(mut self, b: Borders) -> Self {
        self.attr(Attribute::Borders, AttrValue::Borders(b));
        self
    }

    pub fn title<S: AsRef<str>>(mut self, t: S, a: Alignment) -> Self {
        self.attr(
            Attribute::Title,
            AttrValue::Title((t.as_ref().to_string(), a)),
        );
        self
    }

    pub fn alignment(mut self, a: Alignment) -> Self {
        self.attr(Attribute::TextAlign, AttrValue::Alignment(a));
        self
    }

    pub fn foreground(mut self, c: Color) -> Self {
        self.attr(Attribute::Foreground, AttrValue::Color(c));
        self
    }

    pub fn background(mut self, c: Color) -> Self {
        self.attr(Attribute::Background, AttrValue::Color(c));
        self
    }

    pub fn inactive(mut self, s: Style) -> Self {
        self.attr(Attribute::FocusStyle, AttrValue::Style(s));
        self
    }

    pub fn text_modifiers(mut self, m: TextModifiers) -> Self {
        self.attr(Attribute::TextProps, AttrValue::TextModifiers(m));
        self
    }

    pub fn text_alignment(mut self, a: Alignment) -> Self {
        self.attr(Attribute::TextAlign, AttrValue::Alignment(a));
        self
    }

    pub fn value(&mut self, val: Option<String>) -> &Self {
        // Set state
        self.states.update_value(val);
        self
    }

    pub fn get_value(&self) -> String {
        match self.states.value {
            Some(ref v) => v.clone(),
            None => "none".into(),
        }
    }
}

impl MockComponent for FocusableParagraph {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        // Render
        let foreground = self
            .props
            .get_or(Attribute::Foreground, AttrValue::Color(Color::Reset))
            .unwrap_color();
        let background = self
            .props
            .get_or(Attribute::Background, AttrValue::Color(Color::Reset))
            .unwrap_color();

        let text_modifiers = self
            .props
            .get_or(
                Attribute::TextProps,
                AttrValue::TextModifiers(TextModifiers::empty()),
            )
            .unwrap_text_modifiers();
        let text_alignment = self
            .props
            .get_or(Attribute::TextAlign, AttrValue::Alignment(Alignment::Left))
            .unwrap_alignment();

        let inactive_style = self
            .props
            .get(Attribute::FocusStyle)
            .map(|x| x.unwrap_style());
        let focus = self
            .props
            .get_or(Attribute::Focus, AttrValue::Flag(false))
            .unwrap_flag();
        let style = match focus {
            true => Style::default().bg(background).fg(foreground),
            false => inactive_style.unwrap_or_default(),
        };
        let borders = self
            .props
            .get_or(Attribute::Borders, AttrValue::Borders(Borders::default()))
            .unwrap_borders();
        let borders_style = match focus {
            true => borders.style(),
            false => inactive_style.unwrap_or_default(),
        };
        let block: Block = Block::default()
            .borders(BorderSides::ALL)
            .border_style(borders_style)
            .border_type(borders.modifiers)
            .style(style);
        let title = self.props.get(Attribute::Title).map(|x| x.unwrap_title());
        let block = match title {
            Some((text, alignment)) => block.title(text).title_alignment(alignment),
            None => block,
        };

        let selected_text: String = self.get_value();
        let mut text_style = style.clone().add_modifier(text_modifiers);
        if self.states.value == None {
            text_style = text_style.add_modifier(TextModifiers::DIM);
        }

        let p: Paragraph = Paragraph::new(Text::styled(selected_text, text_style))
            .block(block)
            .style(style)
            .alignment(text_alignment);
        frame.render_widget(p, area);
    }

    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        self.props.get(attr)
    }

    fn attr(&mut self, attr: Attribute, value: AttrValue) {
        self.props.set(attr, value);
    }

    fn state(&self) -> State {
        State::One(StateValue::String(self.get_value()))
    }

    fn perform(&mut self, _cmd: Cmd) -> CmdResult {
        CmdResult::None
    }
}

#[derive(Debug, Clone, Default)]
pub struct OwnStates {
    value: Option<String>,
}

impl OwnStates {
    pub fn update_value(&mut self, new_val: Option<String>) {
        self.value = new_val;
    }
}
