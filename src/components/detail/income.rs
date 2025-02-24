use crate::finance::api::detail::IncomeSchema;
use dioxus::prelude::*;

#[component]
pub fn IncomeList(
    incomes: Signal<Vec<IncomeSchema>>,
    handle_add: Callback<(String, String, String)>,
    handle_edit: Callback<(u64, String, String, String)>,
    handle_delete: Callback<u64>,
) -> Element {
    let mut open_income_add = use_signal(|| false);
    let mut open_income_edit = use_signal(|| false);
    let mut opening_income_id = use_signal(|| 0_u64);

    let handle_add = move |(name, amount, date)| {
        handle_add((name, amount, date));
        open_income_add.set(false);
    };

    let handle_add_cancel = move |_| {
        open_income_add.set(false);
    };

    let handle_edit = move |(id, name, amount, date)| {
        handle_edit((id, name, amount, date));
        open_income_edit.set(false);
    };

    let handle_edit_cancel = move |_| {
        open_income_edit.set(false);
    };

    let mut update_income = move |id: u64| {
        open_income_edit.set(true);
        opening_income_id.set(id);
    };

    rsx! {
        div {
            id: "income-list",
            h2 { "収入" },
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
                        if open_income_edit() && opening_income_id() == income.id {
                            IncomeEdit { id: income.id, name: income.name, amount: income.amount.to_string(), date: income.date, handle_edit, handle_cancel: handle_edit_cancel }
                        } else {
                            tr {
                                td { "{income.name}" }
                                td { "{income.date}" }
                                td {
                                    class: "amount",
                                    "{income.amount}"
                                }
                                td {
                                    button {
                                        onclick: move |_| update_income(income.id),
                                        "編集"
                                    }
                                    button {
                                        onclick: move |_| handle_delete(income.id),
                                        "削除"
                                    }
                                }
                            }
                        }
                    }
                    if open_income_add() {
                        IncomeAdd { handle_add, handle_cancel: handle_add_cancel }
                    }
                }
            }
            if !open_income_add() {
                div {
                    class: "income-add",
                    button {
                        onclick: move |_| open_income_add.set(true),
                        "収入を追加"
                    }
                }
            }
        }
    }
}

#[component]
fn IncomeAdd(handle_add: Callback<(String, String, String)>, handle_cancel: EventHandler<MouseEvent>) -> Element {
    let mut name = use_signal(|| "".to_string());
    let mut amount = use_signal(|| "".to_string());
    let mut date = use_signal(|| "".to_string());

    let handle_add = move |_| {
        handle_add((name(), amount(), date()));
    };

    rsx! {
        tr {
            td {
                input {
                    type: "text",
                    placeholder: "名前",
                    value: "{name}",
                    onchange: move |e| name.set(e.value()),
                }
            }
            td {
                input {
                    type: "date",
                    value: "{date}",
                    onchange: move |e| date.set(e.value()),
                }
            }
            td {
                input {
                    type: "text",
                    placeholder: "金額",
                    value: "{amount}",
                    onchange: move |e| amount.set(e.value()),
                }
            }
            td {
                button {
                    onclick: handle_add,
                    "追加"
                }
                button {
                    onclick: handle_cancel,
                    "キャンセル"
                }
            }
        }
    }
}

#[component]
pub fn IncomeEdit(
    id: u64,
    name: String,
    amount: String,
    date: String,
    handle_edit: Callback<(u64, String, String, String)>,
    handle_cancel: EventHandler<MouseEvent>,
) -> Element {
    let mut name = use_signal(|| name);
    let mut amount = use_signal(|| amount);
    let mut date = use_signal(|| date);

    let handle_edit = move |_| {
        handle_edit((id, name(), amount(), date()));
    };

    rsx! {
        tr {
            td {
                input {
                    type: "text",
                    placeholder: "名前",
                    value: "{name}",
                    onchange: move |e| name.set(e.value()),
                }
            }
            td {
                input {
                    type: "date",
                    value: "{date}",
                    onchange: move |e| date.set(e.value()),
                }
            }
            td {
                input {
                    type: "text",
                    placeholder: "金額",
                    value: "{amount}",
                    onchange: move |e| amount.set(e.value()),
                }
            }
            td {
                button {
                    onclick: handle_edit,
                    "編集"
                }
                button {
                    onclick: handle_cancel,
                    "キャンセル"
                }
            }
        }
    }
}
