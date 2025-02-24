use dioxus::prelude::*;
use rust_decimal::Decimal;

#[component]
pub fn Summary(
    total_income: Signal<Decimal>,
    total_outcome: Signal<Decimal>,
    last_month_saving: Signal<Decimal>,
    current_month_saving: Signal<Decimal>,
    handle_create_adjustment: Callback<String>,
) -> Element {
    rsx! {
        div {
            "今月の収支: "
            span {
                style: "color: lightgreen; font-size: 18px;",
                "{total_income()}"
            }
            span { " - " }
            span {
                style: "color: red; font-size: 18px;",
                "{total_outcome()}"
            }
            span { " = " }
            if total_income() - total_outcome() >= Decimal::ZERO {
                span {
                    style: "color: lightgreen; font-size: 22px;",
                    "+{total_income() - total_outcome()}"
                }
            } else {
                span {
                    style: "color: red; font-size: 22px;",
                    "{total_income() - total_outcome()}"
                }
            },
        }

        
        div {
            "先月の貯金: {last_month_saving()}"
        }
        
        div {
            "今月の貯金: {current_month_saving()}"
            input {
                type: "number",
                placeholder: "貯蓄手動入力",
                onchange: move |e| {
                    handle_create_adjustment(e.value());
                },
            }
        }
    }
}