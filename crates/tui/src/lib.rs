use iocraft::{element, ElementExt};

mod app;
mod pages;
mod shared_components;

use app::App;

pub struct Tui;

impl Tui {
    pub async fn start() {
        element!(App).fullscreen().await.unwrap()
    }
}
