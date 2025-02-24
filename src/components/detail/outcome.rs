use crate::finance::api::detail::OutcomeSchema;
use dioxus::prelude::*;

#[component]
pub fn OutcomeList(
    outcomes: Signal<Vec<OutcomeSchema>>,
    handle_add: Callback<(String, String, String)>,
    handle_edit: Callback<(u64, String, String, String)>,
    handle_delete: Callback<u64>,
) -> Element {
    let mut open_outcome_add = use_signal(|| false);
    let mut open_outcome_edit = use_signal(|| false);
    let mut opening_outcome_id = use_signal(|| 0_u64);

    let handle_add = move |(name, amount, date)| {
        handle_add((name, amount, date));
        open_outcome_add.set(false);
    };

    let handle_edit = move |(id, name, amount, date)| {
        handle_edit((id, name, amount, date));
        open_outcome_edit.set(false);
    };

    let mut update_outcome = move |id: u64| {
        open_outcome_edit.set(true);
        opening_outcome_id.set(id);
    };

    let handle_add_cancel = move |_| {
        open_outcome_add.set(false);
    };

    let handle_edit_cancel = move |_| {
        open_outcome_edit.set(false);
    };

    rsx! {
        div {
            id: "outcome-list",
            h2 { "支出" },
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
                        if open_outcome_edit() && opening_outcome_id() == outcome.id {
                            OutcomeEdit { id: outcome.id, name: outcome.name, amount: outcome.amount.to_string(), date: outcome.date, handle_edit, handle_cancel: handle_edit_cancel }
                        } else {
                            tr {
                                td { "{outcome.name}" }
                                td { "{outcome.date}" }
                                td {
                                    class: "amount",
                                    "{outcome.amount}"
                                }
                                td {
                                    button {
                                        onclick: move |_| update_outcome(outcome.id),
                                        "編集"
                                    }
                                    button {
                                        onclick: move |_| handle_delete(outcome.id),
                                        "削除"
                                    }
                                }
                            }
                        }
                    }
                    if open_outcome_add() {
                        OutcomeAdd { handle_add, handle_cancel: handle_add_cancel }
                    }
                }
            }
            if !open_outcome_add() {
                div {
                    class: "outcome-add",
                    button {
                        onclick: move |_| open_outcome_add.set(true),
                        "支出を追加"
                    }
                }
            }
        }
    }
}

#[component]
fn OutcomeAdd(handle_add: Callback<(String, String, String)>, handle_cancel: EventHandler<MouseEvent>) -> Element {
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
pub fn OutcomeEdit(
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
