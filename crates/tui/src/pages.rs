use tuirealm::{tui::layout::Rect, Component, State, Sub};

pub use super::*;

pub struct Render {
    pub id: Id,
    pub area: Rect,
}

pub struct Mount {
    pub id: Id,
    pub component: Box<dyn Component<Msg, UserEvent>>,
    pub subs: Vec<Sub<Id, UserEvent>>,
}

pub trait Page {
    fn mount(&self) -> Vec<Mount>;
    fn required_states(&self) -> Vec<Id>;
    fn view(&self, area: Rect, states: &std::collections::HashMap<Id, State>) -> Vec<Render>;
}

mod primary;
pub use primary::PrimaryPage;
