use dioxus::prelude::*;
use crate::finance::api::plan::{IncomeSchema, MonthlyOutcomeSchema};

#[component]
pub fn CombinedList(
    year: Signal<i32>,
    month: Signal<u32>,
    incomes: Signal<Vec<IncomeSchema>>,
    outcomes: Signal<Vec<MonthlyOutcomeSchema>>,
) -> Element {
    rsx! {
        div {
            id: "combined-list",
            h2 { "{year()}年{month()}月" }
            h3 { "収入" }
            table {
                thead {
                    tr {
                        th { "名前" }
                        th { "日付" }
                        th { "金額" }
                    }
                }
                tbody {
                    for income in incomes() {
                        tr {
                            td { "{income.name}" }
                            td { "{income.date}" }
                            td {
                                class: "amount",
                                style: "color: lightgreen;",
                                "{income.amount}"
                            }
                        }
                    }
                }
            }
            h3 { "支出" }
            table {
                thead {
                    tr {
                        th { "名前" }
                        th { "日付" }
                        th { "金額" }
                    }
                }
                tbody {
                    for outcome in outcomes() {
                        tr {
                            td { "{outcome.name}" }
                            td { "{outcome.payment_date}" }
                            td {
                                class: "amount",
                                style: "color: red;",
                                "{outcome.amount}"
                            }
                        }
                    }
                }
            }
        }
    }
}