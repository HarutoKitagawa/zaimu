use dioxus::prelude::*;
use dioxus::logger::tracing::Level;

use components::Navbar;
use views::{Home, Detail, Plan};

mod components;
mod views;
mod finance;
mod util;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/detail")]
    Detail {},
    #[route("/plan")]
    Plan {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::logger::init(Level::DEBUG).expect("failed to initialize logger");
    dioxus::launch(App);
}

#[component]
fn App() -> Element {

    rsx! {
        // Global app resources
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        Router::<Route> {}
    }
}

