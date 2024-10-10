use tui_realm_stdlib::Select;
use tuirealm::{
    command::{Cmd, Direction},
    event::{Key, KeyEvent},
    props::{Alignment, BorderType, Borders, Color},
    Component, Event, MockComponent,
};

use crate::{Msg, UserEvent};

#[derive(MockComponent)]
pub struct SelectAlfa {
    component: Select,
}

impl Default for SelectAlfa {
    fn default() -> Self {
        Self {
            component: Select::default()
                .borders(
                    Borders::default()
                        .modifiers(BorderType::Rounded)
                        .color(Color::LightGreen),
                )
                .foreground(Color::LightGreen)
                .title("Select your ice cream flavour 🍦", Alignment::Center)
                .rewind(true)
                .highlighted_color(Color::LightGreen)
                .highlighted_str(">> ")
                .choices(&[
                    "vanilla",
                    "chocolate",
                    "coconut",
                    "mint",
                    "strawberry",
                    "lemon",
                    "hazelnut",
                    "coffee",
                ]),
        }
    }
}

impl Component<Msg, UserEvent> for SelectAlfa {
    fn on(&mut self, ev: Event<UserEvent>) -> Option<Msg> {
        let cmd = match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Down, ..
            }) => Cmd::Move(Direction::Down),
            Event::Keyboard(KeyEvent { code: Key::Up, .. }) => Cmd::Move(Direction::Up),
            Event::Keyboard(KeyEvent {
                code: Key::Enter, ..
            }) => {
                self.perform(Cmd::Submit);
                return Some(Msg::SetModalStatus(self.component.states.tab_open));
            }
            Event::Keyboard(KeyEvent {
                code: Key::Delete | Key::Backspace | Key::Esc,
                ..
            }) => {
                self.perform(Cmd::Cancel);
                return Some(Msg::SetModalStatus(self.component.states.tab_open));
            }
            _ => Cmd::None,
        };
        self.perform(cmd);
        Some(Msg::None)
    }
}

#[derive(MockComponent)]
pub struct SelectBeta {
    component: Select,
}

impl Default for SelectBeta {
    fn default() -> Self {
        Self {
            component: Select::default()
                .borders(
                    Borders::default()
                        .modifiers(BorderType::Rounded)
                        .color(Color::LightYellow),
                )
                .foreground(Color::LightYellow)
                .title("Select your topping 🧁", Alignment::Center)
                .rewind(false)
                .highlighted_color(Color::LightYellow)
                .highlighted_str(">> ")
                .choices(&[
                    "hazelnuts",
                    "chocolate",
                    "maple cyrup",
                    "smarties",
                    "raspberries",
                ]),
        }
    }
}

impl Component<Msg, UserEvent> for SelectBeta {
    fn on(&mut self, ev: Event<UserEvent>) -> Option<Msg> {
        let cmd = match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Down, ..
            }) => Cmd::Move(Direction::Down),
            Event::Keyboard(KeyEvent { code: Key::Up, .. }) => Cmd::Move(Direction::Up),
            Event::Keyboard(KeyEvent {
                code: Key::Enter, ..
            }) => {
                self.perform(Cmd::Submit);
                return Some(Msg::SetModalStatus(self.component.states.tab_open));
            }
            Event::Keyboard(KeyEvent {
                code: Key::Delete | Key::Backspace | Key::Esc,
                ..
            }) => {
                self.perform(Cmd::Cancel);
                return Some(Msg::SetModalStatus(self.component.states.tab_open));
            }
            _ => Cmd::None,
        };
        self.perform(cmd);
        Some(Msg::None)
    }
}
