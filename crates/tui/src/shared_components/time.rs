use chrono::Local;
use iocraft::prelude::*;
use std::time::Duration;

#[component]
pub fn Clock(mut hooks: Hooks) -> impl Into<AnyElement<'static>> {
    let mut time = hooks.use_state(|| Local::now());

    hooks.use_future(async move {
        loop {
            smol::Timer::after(Duration::from_secs(1)).await;
            time.set(Local::now());
        }
    });

    element! {
        Text(content: format!("{}", time.get().format("%r")))
    }
}
