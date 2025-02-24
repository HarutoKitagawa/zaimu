use dioxus::prelude::*;

#[component]
pub fn Plan() -> Element {
    rsx! {
        div {
            id: "plan",
            h1 { "Plan" }
        }
    }
}