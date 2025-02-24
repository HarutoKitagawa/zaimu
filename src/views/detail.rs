use crate::components::detail::{IncomeList, OutcomeList, Summary};
use crate::finance::api::detail::*;
use chrono::{Local, Datelike};
use rust_decimal::Decimal;
use dioxus::prelude::*;
use crate::util::get_prev_ym;

const DETAIL_CSS: Asset = asset!("/assets/styling/detail.css");

#[component]
pub fn Detail() -> Element {
    let today = Local::now();
    let mut year = use_signal(|| today.year());
    let mut month = use_signal(|| today.month());
    let mut incomes = use_signal(|| get_incomes(2025, 2));
    let mut outcomes = use_signal(|| get_outcomes(2025, 2));
    let mut total_income = use_signal(|| Decimal::ZERO);
    let mut total_outcome = use_signal(|| Decimal::ZERO);
    let mut last_month_saving = use_signal(|| {
        let (last_year, last_month) = get_prev_ym((year(), month()));
        get_saving(last_year, last_month).amount
    });
    let mut current_month_saving = use_signal(|| get_saving(year(), month()).amount);

    use_effect(move || {
        total_income.set(incomes().iter().fold(Decimal::ZERO, |acc, x| acc + x.amount));
        total_outcome.set(outcomes().iter().fold(Decimal::ZERO, |acc, x| acc + x.amount));
        let (last_year, last_month) = get_prev_ym((year(), month()));
        last_month_saving.set(get_saving(last_year, last_month).amount);
        current_month_saving.set(get_saving(year(), month()).amount);
    });

    use_effect(move || {
        incomes.set(get_incomes(year(), month()));
        outcomes.set(get_outcomes(year(), month()));
        let (last_year, last_month) = get_prev_ym((year(), month()));
        last_month_saving.set(get_saving(last_year, last_month).amount);
        current_month_saving.set(get_saving(year(), month()).amount);
    });

    let handle_add_income = move |(name, amount, date)| {
        store_income(name, amount, date);
        incomes.set(get_incomes(year(), month()));
    };

    let handle_edit_income = move |(id, name, amount, date)| {
        update_income(id, name, amount, date);
        incomes.set(get_incomes(year(), month()));
    };

    let handle_delete_income = move |id| {
        delete_income(id);
        incomes.set(get_incomes(year(), month()));
    };

    let handle_add_outcome = move |(name, amount, date)| {
        store_outcome(name, amount, date);
        outcomes.set(get_outcomes(year(), month()));
    };

    let handle_edit_outcome = move |(id, name, amount, date)| {
        update_outcome(id, name, amount, date);
        outcomes.set(get_outcomes(year(), month()));
    };

    let handle_delete_outcome = move |id| {
        delete_outcome(id);
        outcomes.set(get_outcomes(year(), month()));
    };

    let handle_create_adjustment = move |saving_input| {
        create_adjustment(saving_input, year(), month());
        incomes.set(get_incomes(year(), month()));
        outcomes.set(get_outcomes(year(), month()));
    };

    rsx! {
        document::Link { rel: "stylesheet", href: DETAIL_CSS }
        label {
            for: "year",
            "対象年"
        }
        input { 
            id: "year",
            type: "number",
            max: "{today.year()}",
            value: year(),
            onchange: move |e| year.set(e.value().parse().unwrap_or(Local::now().year()))
        }
        label {
            for: "month",
            "対象月"
        }
        input {
            id: "month",
            type: "number",
            min: "1",
            max: if year() < today.year() { "12" } else { today.month().to_string() },
            value: month(),
            onchange: move |e| month.set(e.value().parse().unwrap_or(Local::now().month()))
        }
        IncomeList { incomes, handle_add: handle_add_income, handle_edit: handle_edit_income, handle_delete: handle_delete_income }
        OutcomeList { outcomes, handle_add: handle_add_outcome, handle_edit: handle_edit_outcome, handle_delete: handle_delete_outcome}
        Summary { total_income, total_outcome, last_month_saving, current_month_saving, handle_create_adjustment }
    }
}
