use iocraft::prelude::*;
use smol::block_on;

mod app;
mod pages;
mod shared_components;

use app::App;

pub struct Tui;

impl Tui {
    pub fn start() {
        block_on(element!(App).fullscreen()).unwrap()
    }
}
