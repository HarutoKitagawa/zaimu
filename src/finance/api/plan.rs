use chrono::{DateTime, Datelike, Local, NaiveDate, TimeZone};
use dioxus::logger::tracing;
use rust_decimal::Decimal;
use std::str::FromStr;

use super::plan_service;
use super::plan_service::future_inspector;
use super::plan_service::future_inspector::{BalanceStatus, InspectResult};
use super::plan_service::{
    get_part_time_job_repo,
    PartTimeJobRepo,
    get_monthly_outcome_repo,
    get_temporary_outcome_repo,
    income::job::get_or_create_part_time_job_incomes,
    outcome::{
        monthly_outcome::get_or_create_monthly_outcomes,
        temporary_outcome::get_temporary_outcomes,
    },
};
use crate::finance::detail::get_saving_repo;
use crate::finance::plan::outcome::monthly_outcome::MonthlyOutcomeRepo;
use crate::finance::setting::get_opening_and_closing_date;

#[derive(Debug, Clone, PartialEq)]
pub struct IncomeSchema {
    pub name: String,
    pub amount: Decimal,
    pub date: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PartTimeJobIncomeSchema {
    pub id: u64,
    pub name: String,
    pub hourly_wage: Decimal,
    pub hour: Decimal,
    pub payment_date: String,
    pub total: Decimal,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MonthlyOutcomeSchema {
    pub id: u64,
    pub name: String,
    pub amount: Decimal,
    pub payment_date: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FutureInspectResultSchema {
    pub date: String,
    pub amount: Decimal,
    pub incomes: String,
    pub outcomes: String,
}

pub fn get_incomes(year: i32, month: u32) -> Vec<IncomeSchema> {
    let repo = get_part_time_job_repo();
    let (start_date, end_date) = match get_opening_and_closing_date(year, month) {
        Ok((start_date, end_date)) => (start_date, end_date),
        Err(e) => {
            tracing::error!("Failed to get opening and closing date: {}", e);
            return vec![];
        }
    };
    match plan_service::get_incomes(vec![&repo], &start_date, &end_date) {
        Ok(incomes) => incomes
            .into_iter()
            .map(|income| IncomeSchema {
                name: income.name,
                amount: income.amount,
                date: income.date.date_naive().to_string(),
            })
            .collect(),
        Err(e) => {
            tracing::error!("Failed to get incomes: {}", e);
            return vec![];
        }
    }
}

pub fn get_part_time_job_incomes(year: i32, month: u32) -> Vec<PartTimeJobIncomeSchema> {
    let repo = get_part_time_job_repo();
    let (start_date, end_date) = match get_opening_and_closing_date(year, month) {
        Ok((start_date, end_date)) => (start_date, end_date),
        Err(e) => {
            tracing::error!("Failed to get opening and closing date: {}", e);
            return vec![];
        }
    };
    let part_time_jobs = match repo.list_part_time_jobs(&start_date, &end_date) {
        Ok(jobs) => jobs,
        Err(e) => {
            tracing::error!("Failed to get part-time jobs: {}", e);
            return vec![];
        }
    };
    part_time_jobs
        .into_iter()
        .map(|job| {
            let job_payment_date = job.get_payment_date(year, month).unwrap();
            if let Ok(Some(income)) = &repo.get_part_time_job_income_by_part_time_job_id(
                job.id.unwrap(),
                job_payment_date.year(),
                job_payment_date.month(),
            ) {
                return PartTimeJobIncomeSchema {
                    id: income.id.unwrap(),
                    name: income.name.clone(),
                    hourly_wage: income.hourly_wage,
                    hour: income.hour,
                    payment_date: income.payment_date.date_naive().to_string(),
                    total: income.hourly_wage * income.hour,
                };
            } else {
                let income = job
                    .to_part_time_job_income(year, month, Decimal::ZERO, &repo)
                    .unwrap();
                return PartTimeJobIncomeSchema {
                    id: income.id.unwrap(),
                    name: income.name,
                    hourly_wage: income.hourly_wage,
                    hour: income.hour,
                    payment_date: income.payment_date.date_naive().to_string(),
                    total: income.hourly_wage * income.hour,
                };
            }
        })
        .collect()
}

pub fn update_part_time_job_income(
    id: u64,
    name: String,
    hourly_wage: String,
    hour: String,
    payment_date: String,
) {
    let hourly_wage = match Decimal::from_str(&hourly_wage) {
        Ok(hourly_wage) => hourly_wage,
        Err(e) => {
            tracing::error!("Invalid hourly wage '{}': {}", hourly_wage, e);
            return;
        }
    };
    let hour = match Decimal::from_str(&hour) {
        Ok(hour) => hour,
        Err(e) => {
            tracing::error!("Invalid hour '{}': {}", hour, e);
            return;
        }
    };
    let payment_date = match NaiveDate::parse_from_str(&payment_date, "%Y-%m-%d") {
        Ok(payment_date) => match Local
            .with_ymd_and_hms(
                payment_date.year(),
                payment_date.month0() + 1,
                payment_date.day0() + 1,
                0,
                0,
                0,
            )
            .single()
        {
            Some(payment_date) => payment_date,
            None => {
                tracing::error!("Invalid payment date '{}'", payment_date);
                return;
            }
        },
        Err(e) => {
            tracing::error!("Invalid payment date '{}': {}", payment_date, e);
            return;
        }
    };

    let repo = get_part_time_job_repo();
    let income = match repo.get_part_time_job_income_by_id(id) {
        Ok(Some(income)) => income.update(name, hourly_wage, hour, payment_date),
        Ok(None) => {
            tracing::error!("Part-time job income not found: {}", id);
            return;
        }
        Err(e) => {
            tracing::error!("Failed to get part-time job income: {}", e);
            return;
        }
    };
    if let Err(e) = repo.update_part_time_job_income(income) {
        tracing::error!("Failed to update part-time job income: {}", e);
    }
}

pub fn get_monthly_outcomes(year: i32, month: u32) -> Vec<MonthlyOutcomeSchema> {
    let repo = get_monthly_outcome_repo();
    let (start_date, end_date) = match get_opening_and_closing_date(year, month) {
        Ok((start_date, end_date)) => (start_date, end_date),
        Err(e) => {
            tracing::error!("Failed to get opening and closing date: {}", e);
            return vec![];
        }
    };
    let templates = match repo.list_monthly_outcome_template(&start_date, &end_date) {
        Ok(templates) => templates,
        Err(e) => {
            tracing::error!("Failed to get monthly outcome templates: {}", e);
            return vec![];
        }
    };
    templates
        .into_iter()
        .map(|template| {
            let payment_date = template.get_payment_date(year, month).unwrap();
            if let Ok(Some(outcome)) = &repo.get_monthly_outcome_by_template_id(
                template.id.unwrap(),
                payment_date.year(),
                payment_date.month(),
            ) {
                return MonthlyOutcomeSchema {
                    id: outcome.id.unwrap(),
                    name: outcome.name.clone(),
                    amount: outcome.amount,
                    payment_date: outcome.payment_date.date_naive().to_string(),
                };
            } else {
                let outcome = template
                    .to_monthly_outcome(year, month, &repo)
                    .unwrap();
                return MonthlyOutcomeSchema {
                    id: outcome.id.unwrap(),
                    name: outcome.name,
                    amount: outcome.amount,
                    payment_date: outcome.payment_date.date_naive().to_string(),
                };
            }
        })
        .collect()
}

pub fn get_future_inspect(year: i32, month: u32) -> Vec<FutureInspectResultSchema> {
    let part_time_job_repo = get_part_time_job_repo();
    let monthly_outcome_repo = get_monthly_outcome_repo();
    let temporary_outcome_repo = get_temporary_outcome_repo();
    let saving_repo = get_saving_repo();

    match future_inspector::inspect(
        (year, month),
        (year + 2, month),
        &saving_repo,
        vec![Box::new(move |year, month| get_or_create_part_time_job_incomes(year, month, &part_time_job_repo))],
        vec![
            Box::new(move |year, month| get_or_create_monthly_outcomes(year, month, &monthly_outcome_repo)),
            Box::new(move |year, month| get_temporary_outcomes(year, month, &temporary_outcome_repo)),
        ],
    ) {
        Ok(results) => {
            results
                .into_iter()
                .map(|result| {
                    match result {
                        InspectResult {
                            date,
                            balance_status: BalanceStatus::Surplus(amount),
                            incomes,
                            outcomes,
                        } => FutureInspectResultSchema {
                            date: date.date_naive().to_string(),
                            amount,
                            incomes: incomes
                                .iter()
                                .map(|income| income.name.clone() + ": " + &income.amount.to_string())
                                .collect::<Vec<String>>()
                                .join(", "),
                            outcomes: outcomes
                                .iter()
                                .map(|outcome| outcome.name.clone() + ": " + &outcome.amount.to_string())
                                .collect::<Vec<String>>()
                                .join(", "),
                        },
                        InspectResult {
                            date,
                            balance_status: BalanceStatus::Deficit(amount),
                            incomes,
                            outcomes,
                        } => FutureInspectResultSchema {
                            date: date.date_naive().to_string(),
                            amount: -amount,
                            incomes: incomes
                                .iter()
                                .map(|income| income.name.clone() + ": " + &income.amount.to_string())
                                .collect::<Vec<String>>()
                                .join(", "),
                            outcomes: outcomes
                                .iter()
                                .map(|outcome| outcome.name.clone() + ": " + &outcome.amount.to_string())
                                .collect::<Vec<String>>()
                                .join(", "),
                        }
                    }
                })
                .collect()
        },
        Err(e) => {
            tracing::error!("Failed to inspect future: {}", e);
            vec![]
        },
    }
}