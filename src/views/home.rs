use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        div {
            id: "home",
            h1 { "Home" }
        }
    }
}
