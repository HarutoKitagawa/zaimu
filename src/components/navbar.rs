use crate::Route;
use dioxus::prelude::*;

const NAVBAR_CSS: Asset = asset!("/assets/styling/navbar.css");

#[component]
pub fn Navbar() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: NAVBAR_CSS }

        div {
            id: "navbar",
            Link {
                to: Route::Home {},
                "Home"
            }
            Link {
                to: Route::Detail {},
                "明細"
            }
            Link {
                to: Route::Plan {},
                "計画"
            }
        }

        div {
            class: "container",
            Outlet::<Route> {}
        }
    }
}
