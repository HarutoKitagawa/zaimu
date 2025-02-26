use crate::finance::detail::update_saving;

use super::detail_service;
use super::detail_service::{
    get_adjustment_repo, get_income_repo, get_outcome_repo, get_saving_repo,
    Income, IncomeRepo, Outcome, OutcomeRepo, SavingRepo,
};
use crate::finance::setting::get_opening_and_closing_date;
use chrono::{DateTime, Datelike, Local, NaiveDate};
use dioxus::logger::tracing;
use rust_decimal::prelude::*;

#[derive(Debug, Clone)]
pub struct IncomeSchema {
    pub id: u64,
    pub name: String,
    pub amount: Decimal,
    pub date: String,
}

#[derive(Debug, Clone)]
pub struct OutcomeSchema {
    pub id: u64,
    pub name: String,
    pub amount: Decimal,
    pub date: String,
}

#[derive(Debug, Clone)]
pub struct SavingSchema {
    pub year: i32,
    pub month: u32,
    pub amount: Decimal,
}

pub fn get_incomes(year: i32, month: u32) -> Vec<IncomeSchema> {
    let repo = get_income_repo();
    let (opening_date, closing_date) = match get_opening_and_closing_date(year, month) {
        Ok(x) => x,
        Err(e) => {
            tracing::error!("Failed to get opening and closing date: {}", e);
            return vec![];
        }
    };
    let incomes = match repo.list(&opening_date, &closing_date) {
        Ok(incomes) => incomes,
        Err(e) => {
            tracing::error!("Failed to get incomes: {}", e);
            return vec![];
        }
    };
    incomes
        .into_iter()
        .map(|income| IncomeSchema {
            id: income.id.unwrap(),
            name: income.name,
            amount: income.amount,
            date: format_date(&income.date),
        })
        .collect()
}

pub fn store_income(name: String, amount: String, date: String) {
    let repo = get_income_repo();
    let parsed_date = match NaiveDate::parse_from_str(&date, "%Y-%m-%d") {
        Ok(parsed_date) => parsed_date,
        Err(e) => {
            tracing::error!("Failed to parse date: {}", e);
            return;
        }
    };
    let income = match Income::try_new(
        name,
        amount,
        parsed_date.year(),
        parsed_date.month0() + 1,
        parsed_date.day0() + 1,
    ) {
        Ok(income) => income,
        Err(e) => {
            tracing::error!("Failed to create income: {}", e);
            return;
        }
    };
    match repo.store(income.clone()) {
        Ok(_) => {
            let saving_repo = get_saving_repo();
            match update_saving(
                (income.date.year(), income.date.month()),
                income.amount,
                &saving_repo,
            ) {
                Ok(_) => (),
                Err(e) => {
                    tracing::error!("Failed to update saving: {}", e);
                    return;
                }
            };
        }
        Err(e) => {
            tracing::error!("Failed to store income: {}", e);
            return;
        }
    }
}

pub fn delete_income(id: u64) {
    let repo = get_income_repo();
    match repo.get_by_id(id) {
        Ok(None) => {
            return;
        }
        Ok(Some(prev)) => match repo.delete_by_id(id) {
            Ok(_) => {
                let saving_repo = get_saving_repo();
                match update_saving(
                    (prev.date.year(), prev.date.month()),
                    -prev.amount,
                    &saving_repo,
                ) {
                    Ok(_) => (),
                    Err(e) => {
                        tracing::error!("Failed to update saving: {}", e);
                        return;
                    }
                };
            }
            Err(_) => return,
        },
        Err(_) => return,
    }
}

pub fn update_income(id: u64, name: String, amount: String, date: String) {
    let repo = get_income_repo();
    match repo.get_by_id(id) {
        Ok(None) => return,
        Ok(Some(prev)) => {
            let parsed_date = match NaiveDate::parse_from_str(&date, "%Y-%m-%d") {
                Ok(parsed_date) => parsed_date,
                Err(e) => {
                    tracing::error!("Failed to parse date: {}", e);
                    return;
                }
            };
            let new_income = match Income::try_update(
                id,
                name,
                amount,
                parsed_date.year(),
                parsed_date.month0() + 1,
                parsed_date.day0() + 1,
            ) {
                Ok(income) => income,
                Err(e) => {
                    tracing::error!("Failed to update income: {}", e);
                    return;
                }
            };
            match repo.update(new_income.clone()) {
                Ok(_) => {
                    let saving_repo = get_saving_repo();
                    if prev.date.year() == new_income.date.year()
                    && prev.date.month() == new_income.date.month()
                    {
                        let amount_diff = new_income.amount - prev.amount;
                        match update_saving(
                            (new_income.date.year(), new_income.date.month()),
                            amount_diff,
                            &saving_repo,
                        ) {
                            Ok(_) => (),
                            Err(e) => {
                                tracing::error!("Failed to update saving: {}", e);
                                return;
                            }
                        };
                        return;
                    } else {
                        match update_saving(
                            (prev.date.year(), prev.date.month()),
                            -prev.amount,
                            &saving_repo,
                        ) {
                            Ok(_) => (),
                            Err(e) => {
                                tracing::error!("Failed to update saving: {}", e);
                                return;
                            }
                        };
                        match update_saving(
                            (new_income.date.year(), new_income.date.month()),
                            new_income.amount,
                            &saving_repo,
                        ) {
                            Ok(_) => (),
                            Err(e) => {
                                tracing::error!("Failed to update saving: {}", e);
                                return;
                            }
                        };
                    }
                }
                Err(e) => {
                    tracing::error!("Failed to update income: {}", e);
                    return;
                }
            }
        }
        Err(e) => {
            tracing::error!("Failed to get income: {}", e);
            return;
        }
    }
}

pub fn get_outcomes(year: i32, month: u32) -> Vec<OutcomeSchema> {
    let repo = get_outcome_repo();
    let (opening_date, closing_date) = match get_opening_and_closing_date(year, month) {
        Ok(x) => x,
        Err(e) => {
            tracing::error!("Failed to get opening and closing date: {}", e);
            return vec![];
        }
    };
    let outcomes = match repo.list(&opening_date, &closing_date) {
        Ok(outcomes) => outcomes,
        Err(e) => {
            tracing::error!("Failed to get outcomes: {}", e);
            vec![]
        }
    };
    outcomes
        .into_iter()
        .map(|outcome| OutcomeSchema {
            id: outcome.id.unwrap(),
            name: outcome.name,
            amount: outcome.amount,
            date: format_date(&outcome.date),
        })
        .collect()
}

pub fn store_outcome(name: String, amount: String, date: String) {
    let repo = get_outcome_repo();
    let parsed_date = match NaiveDate::parse_from_str(&date, "%Y-%m-%d") {
        Ok(parsed_date) => parsed_date,
        Err(e) => {
            tracing::error!("Failed to parse date: {}", e);
            return;
        }
    };
    let outcome = match Outcome::try_new(
        name,
        amount,
        parsed_date.year(),
        parsed_date.month0() + 1,
        parsed_date.day0() + 1,
    ) {
        Ok(outcome) => outcome,
        Err(e) => {
            tracing::error!("Failed to create outcome: {}", e);
            return;
        }
    };
    match repo.store(outcome.clone()) {
        Ok(_) => {
            let saving_repo = get_saving_repo();
            match update_saving(
                (outcome.date.year(), outcome.date.month()),
                -outcome.amount,
                &saving_repo,
            ) {
                Ok(_) => (),
                Err(e) => {
                    tracing::error!("Failed to update saving: {}", e);
                    return;
                }
            };
        }
        Err(e) => {
            tracing::error!("Failed to store outcome: {}", e);
            return;
        }
    }
}

pub fn delete_outcome(id: u64) {
    let repo = get_outcome_repo();
    match repo.get_by_id(id) {
        Ok(None) => {
            return;
        }
        Ok(Some(prev)) => match repo.delete_by_id(id) {
            Ok(_) => {
                let saving_repo = get_saving_repo();
                match update_saving(
                    (prev.date.year(), prev.date.month()),
                    prev.amount,
                    &saving_repo,
                ) {
                    Ok(_) => (),
                    Err(e) => {
                        tracing::error!("Failed to update saving: {}", e);
                        return;
                    }
                };
            }
            Err(_) => return,
        },
        Err(_) => return,
    }
}

pub fn update_outcome(id: u64, name: String, amount: String, date: String) {
    let repo = get_outcome_repo();
    match repo.get_by_id(id) {
        Ok(None) => return,
        Ok(Some(prev)) => {
            let parsed_date = match NaiveDate::parse_from_str(&date, "%Y-%m-%d") {
                Ok(parsed_date) => parsed_date,
                Err(e) => {
                    tracing::error!("Failed to parse date: {}", e);
                    return;
                }
            };
            let new_outcome = match Outcome::try_update(
                id,
                name,
                amount,
                parsed_date.year(),
                parsed_date.month0() + 1,
                parsed_date.day0() + 1,
            ) {
                Ok(outcome) => outcome,
                Err(e) => {
                    tracing::error!("Failed to update outcome: {}", e);
                    return;
                }
            };
            match repo.update(new_outcome.clone()) {
                Ok(_) => {
                    let saving_repo = get_saving_repo();
                    if prev.date.year() == new_outcome.date.year()
                        && prev.date.month() == new_outcome.date.month()
                    {
                        let amount_diff = new_outcome.amount - prev.amount;
                        match update_saving(
                            (new_outcome.date.year(), new_outcome.date.month()),
                            -amount_diff,
                            &saving_repo,
                        ) {
                            Ok(_) => (),
                            Err(e) => {
                                tracing::error!("Failed to update saving: {}", e);
                                return;
                            }
                        };
                        return;
                    } else {
                        match update_saving(
                            (prev.date.year(), prev.date.month()),
                            prev.amount,
                            &saving_repo,
                        ) {
                            Ok(_) => (),
                            Err(e) => {
                                tracing::error!("Failed to update saving: {}", e);
                                return;
                            }
                        };
                        match update_saving(
                            (new_outcome.date.year(), new_outcome.date.month()),
                            -new_outcome.amount,
                            &saving_repo,
                        ) {
                            Ok(_) => (),
                            Err(e) => {
                                tracing::error!("Failed to update saving: {}", e);
                                return;
                            }
                        };
                    }
                }
                Err(e) => {
                    tracing::error!("Failed to update outcome: {}", e);
                    return;
                }
            }
        }
        Err(e) => {
            tracing::error!("Failed to get outcome: {}", e);
            return;
        }
    }
}

pub fn create_adjustment(saving_input: String, year: i32, month: u32) {
    let i_repo = get_income_repo();
    let o_repo = get_outcome_repo();
    let s_repo = get_saving_repo();
    let a_repo = get_adjustment_repo();

    let saving_input = match Decimal::from_str(&saving_input) {
        Ok(balance_input) => balance_input,
        Err(e) => {
            tracing::error!("Failed to parse balance input: {}", e);
            return;
        }
    };
    match detail_service::create_adjustment(
        saving_input,
        year,
        month,
        &i_repo,
        &o_repo,
        &s_repo,
        &a_repo,
    ) {
        Ok(_) => (),
        Err(e) => {
            tracing::error!("Failed to create adjustment: {}", e);
            return;
        }
    }
}

pub fn get_saving(year: i32, month: u32) -> SavingSchema {
    let s_repo = get_saving_repo();
    match s_repo.get(&(year, month)) {
        Ok(Some(saving)) => SavingSchema {
            year: saving.key.0,
            month: saving.key.1,
            amount: saving.amount,
        },
        Ok(None) => SavingSchema {
            year,
            month,
            amount: Decimal::ZERO,
        },
        Err(e) => {
            tracing::error!("Failed to get saving: {}", e);
            SavingSchema {
                year,
                month,
                amount: Decimal::ZERO,
            }
        }
    }
}

fn format_date(date: &DateTime<Local>) -> String {
    let date_string = date.to_string();
    date_string
        .split(" ")
        .next()
        .map(|s| s.to_string())
        .unwrap_or(date_string)
}
