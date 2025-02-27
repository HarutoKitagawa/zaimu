use chrono::{DateTime, Local};
use rust_decimal::Decimal;
use std::collections::BTreeMap;

use super::income::Income;
use super::outcome::Outcome;
use crate::finance::detail::saving::SavingRepo;
use crate::util::get_next_ym;

#[derive(Debug)]
pub enum BalanceStatus {
    Deficit(Decimal),
    Surplus(Decimal),
}

#[derive(Debug)]
pub struct InspectResult {
    pub date: DateTime<Local>,
    pub balance_status: BalanceStatus,
    pub incomes: Vec<Income>,
    pub outcomes: Vec<Outcome>,
}

pub fn inspect<I, O>(
    start_ym: (i32, u32),
    end_ym: (i32, u32),
    saving_repo: &impl SavingRepo,
    income_factories: Vec<I>,
    outcome_factories: Vec<O>,
) -> Result<Vec<InspectResult>, anyhow::Error>
where
    I: Fn(i32, u32) -> Result<Vec<Income>, anyhow::Error>,
    O: Fn(i32, u32) -> Result<Vec<Outcome>, anyhow::Error>,
{
    let mut incomes = Vec::new();
    let mut outcomes = Vec::new();

    let mut current_ym = start_ym;
    while current_ym.0 < end_ym.0 || (current_ym.0 == end_ym.0 && current_ym.1 <= end_ym.1) {
        for factory in &income_factories {
            let factory_incomes = factory(current_ym.0, current_ym.1)?;
            incomes.extend(factory_incomes);
        }

        for factory in &outcome_factories {
            let factory_outcomes = factory(current_ym.0, current_ym.1)?;
            outcomes.extend(factory_outcomes);
        }

        current_ym = get_next_ym(current_ym);
    }

    let mut daily_balance: BTreeMap<DateTime<Local>, (Decimal, Decimal, Vec<Income>, Vec<Outcome>)> = BTreeMap::new();

    for income in incomes {
        let entry = daily_balance
            .entry(income.date)
            .or_insert((Decimal::ZERO, Decimal::ZERO, Vec::new(), Vec::new()));
        entry.0 += income.amount;
        entry.2.push(income);
    }

    for outcome in outcomes {
        let entry = daily_balance
            .entry(outcome.date)
            .or_insert((Decimal::ZERO, Decimal::ZERO, Vec::new(), Vec::new()));
        entry.1 += outcome.amount;
        entry.3.push(outcome);
    }

    let latest_saving = match saving_repo.get(&(start_ym.0, start_ym.1))? {
        Some(saving) => saving.amount,
        None => Decimal::ZERO,
    };
    let mut balance = latest_saving;
    let mut results = Vec::new();
    for (date, (total_income, total_outcome, incomes, outcomes)) in daily_balance {
        balance += total_income - total_outcome;
        let balance_status = if balance >= Decimal::ZERO {
            BalanceStatus::Surplus(balance)
        } else {
            BalanceStatus::Deficit(-balance)
        };

        results.push(InspectResult {
            date,
            balance_status,
            incomes,
            outcomes,
        });
    }

    Ok(results)
}
