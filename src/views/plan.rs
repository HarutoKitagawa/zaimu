use chrono::{Datelike, Local};
use dioxus::prelude::*;

use crate::util::get_next_ym;

use crate::components::plan::{CombinedList, PartTimeJobIncomes};
use crate::finance::api::plan::{
    get_incomes, get_part_time_job_incomes, update_part_time_job_income,
};

const PLAN_CSS: Asset = asset!("/assets/styling/plan.css");

#[component]
pub fn Plan() -> Element {
    let today = Local::now();
    let (next_year, next_month) = get_next_ym((today.year(), today.month()));
    let (next_next_year, next_next_month) = get_next_ym((next_year, next_month));
    let mut year = use_signal(|| next_year);
    let mut month = use_signal(|| next_month);
    let mut next_year = use_signal(|| next_next_year);
    let mut next_month = use_signal(|| next_next_month);
    let mut part_time_job_incomes = use_signal(|| vec![]);
    let mut incomes = use_signal(|| vec![]);
    let mut next_month_incomes = use_signal(|| vec![]);

    use_effect(move || {
        part_time_job_incomes.set(get_part_time_job_incomes(year(), month()));
        incomes.set(get_incomes(year(), month()));
        next_month_incomes.set(get_incomes(next_year(), next_month()));
    });

    let mut handle_change_year_month = move |y: i32, m: u32| {
        year.set(y);
        month.set(m);
        next_year.set(get_next_ym((y, m)).0);
        next_month.set(get_next_ym((y, m)).1);
    };

    let handle_edit_part_time_job = move |(id, name, hourly_wage, hour, payment_date)| {
        update_part_time_job_income(id, name, hourly_wage, hour, payment_date);
        part_time_job_incomes.set(get_part_time_job_incomes(year(), month()));
        incomes.set(get_incomes(year(), month()));
        next_month_incomes.set(get_incomes(next_year(), next_month()));
    };

    rsx! {
        document::Link { rel: "stylesheet", href: PLAN_CSS }
        div {
            id: "plan",
            h2 { "計画" }
        }

        label {
            for: "year",
            "対象年"
        }
        input {
            id: "year",
            type: "number",
            min: "{today.year()}",
            value: year(),
            onchange: move |e| handle_change_year_month(e.value().parse().unwrap_or(Local::now().year()), month())
        }
        label {
            for: "month",
            "対象月"
        }
        input {
            id: "month",
            type: "number",
            min: if year() > today.year() { "1" } else { today.month().to_string() },
            max: "12",
            value: month(),
            onchange: move |e| handle_change_year_month(year(), e.value().parse().unwrap_or(1))
        }
        PartTimeJobIncomes { year, month, part_time_job_incomes, handle_edit_part_time_job }
        CombinedList { year, month, incomes }
        CombinedList { year: next_year, month: next_month, incomes: next_month_incomes }
    }
}
