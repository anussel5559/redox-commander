use tuirealm::{Application, Frame};

pub use super::*;

pub trait Page {
    fn mount(&self, app: &mut Application<Id, Msg, UserEvent>);
    fn view(&self, app: &mut Application<Id, Msg, UserEvent>, frame: &mut Frame<'_>);
}

mod primary;
pub use primary::PrimaryPage;
