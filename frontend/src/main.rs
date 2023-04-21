mod about;
mod app;
mod pages;
mod service;

use patternfly_yew::prelude::*;
use yew::prelude::*;

use app::Application;

#[cfg(not(debug_assertions))]
const LOG_LEVEL: log::Level = log::Level::Info;
#[cfg(debug_assertions)]
const LOG_LEVEL: log::Level = log::Level::Trace;

#[function_component(Main)]
pub fn main() -> Html {
    html!(
        <ToastViewer>
            <BackdropViewer>
                <Application />
            </BackdropViewer>
        </ToastViewer>
    )
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(LOG_LEVEL));
    yew::Renderer::<Main>::new().render();
}
