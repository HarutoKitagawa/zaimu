use chrono::{Local, Datelike};
use dioxus::prelude::*;
use crate::finance::api::plan::get_future_inspect;

const HOME_CSS: Asset = asset!("/assets/styling/home.css");

#[component]
pub fn Home() -> Element {
    let today = Local::now();
    let mut future_inspect_results = use_signal(|| vec![]);

    use_effect(move || {
        future_inspect_results.set(get_future_inspect(today.year(), today.month()));
    });

    rsx! {
        document::Link { rel: "stylesheet", href: HOME_CSS }
        div {
            id: "future-inspect-result-list",
            h1 { "Home" }
            table {
                colgroup {
                    col { style: "width: 15%;" }
                    col { style: "width: 40%;" }
                    col { style: "width: 40%;" }
                    col { style: "width: 15%;" }
                }
                thead {
                    tr {
                        th { "日付" }
                        th { "収入" }
                        th { "支出" }
                        th { "バランス" }
                    }
                }
                tbody {
                    for result in future_inspect_results() {
                        tr {
                            td {
                                style: "text-align: center",
                                "{result.date}"
                            }
                            td {
                                "{result.incomes}"
                            }
                            td {
                                "{result.outcomes}"
                            }
                            td {
                                class: "amount",
                                style: if result.amount.is_sign_positive() {
                                    "background-color: lightgreen"
                                } else {
                                    "background-color: red"
                                },
                                "{result.amount}"
                            }
                        }
                    }
                }
            }
        }
    }
}
