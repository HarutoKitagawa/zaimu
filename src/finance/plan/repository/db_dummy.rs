use super::income::{Income, IncomeRepo, ToIncome};
use super::job;
use super::job::{PartTimeHourlyWage, PartTimeJob, PartTimeJobIncome, PartTimeJobRepo};
use super::monthly_outcome;
use super::monthly_outcome::{MonthlyOutcome, MonthlyOutcomeRepo, MonthlyOutcomeTemplate};
use super::outcome::{Outcome, OutcomeRepo, ToOutcome};
use chrono::prelude::*;
use rust_decimal_macros::dec;
use std::vec;
use std::{cell::RefCell, collections::HashMap};

thread_local! {
    static PART_TIME_JOB_COLLECTION: RefCell<HashMap<u64, PartTimeJob>> = RefCell::new(HashMap::from_iter(vec![
        (1, PartTimeJob {
            id: Some(1),
            name: "アルバイト1".to_string(),
            payment_timing: job::PaymentTiming::NextMonthMid(21),
            start_date: Local.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).single().unwrap(),
            end_date: Some(Local.with_ymd_and_hms(2025, 12, 31, 0, 0, 0).single().unwrap()),
        }),
        (2, PartTimeJob {
            id: Some(2),
            name: "アルバイト2".to_string(),
            payment_timing: job::PaymentTiming::End,
            start_date: Local.with_ymd_and_hms(2025, 3, 1, 0, 0, 0).single().unwrap(),
            end_date: None,
        })
    ]));
    static PART_TIME_JOB_HOURLY_WAGE_COLLECTION: RefCell<Vec<PartTimeHourlyWage>> = RefCell::new(vec![
        PartTimeHourlyWage {
            part_time_job_id: 1,
            hourly_wage: dec!(1500),
            start_year_and_month: (2025, 1),
        },
        PartTimeHourlyWage {
            part_time_job_id: 1,
            hourly_wage: dec!(1600),
            start_year_and_month: (2025, 5),
        },
        PartTimeHourlyWage {
            part_time_job_id: 2,
            hourly_wage: dec!(1200),
            start_year_and_month: (2025, 3),
        },
    ]);
    static PART_TIME_JOB_INCOME_COLLECTION: RefCell<HashMap<u64, PartTimeJobIncome>> = RefCell::new(HashMap::from_iter(vec![
        (1, PartTimeJobIncome {
            id: Some(1),
            part_time_job_id: 1,
            name: "アルバイト1".to_string(),
            hourly_wage: dec!(1500),
            hour: dec!(8),
            payment_date: Local.with_ymd_and_hms(2025, 4, 21, 0, 0, 0).single().unwrap(),
        })
    ]));
    static MONTHLY_OUTCOME_TEMPLATE_COLLECTION: RefCell<HashMap<u64, MonthlyOutcomeTemplate>> = RefCell::new(HashMap::from_iter(vec![
        (1, MonthlyOutcomeTemplate {
            id: Some(1),
            name: "支出1".to_string(),
            amount: dec!(10000),
            payment_timing: monthly_outcome::PaymentTiming::End,
            start_date: Local.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).single().unwrap(),
            end_date: Some(Local.with_ymd_and_hms(2025, 12, 31, 0, 0, 0).single().unwrap()),
        }),
        (2, MonthlyOutcomeTemplate {
            id: Some(2),
            name: "支出2".to_string(),
            amount: dec!(5000),
            payment_timing: monthly_outcome::PaymentTiming::Mid(15),
            start_date: Local.with_ymd_and_hms(2025, 3, 1, 0, 0, 0).single().unwrap(),
            end_date: None,
        }),
    ]));
    static MONTHLY_OUTCOME_COLLECTION: RefCell<HashMap<u64, MonthlyOutcome>> = RefCell::new(HashMap::from_iter(vec![]));
}

pub struct DummyPartTimeJobRepo;

impl PartTimeJobRepo for DummyPartTimeJobRepo {
    fn list_part_time_jobs(
        &self,
        start_date: &DateTime<Local>,
        end_date: &DateTime<Local>,
    ) -> Result<Vec<PartTimeJob>, anyhow::Error> {
        Ok(PART_TIME_JOB_COLLECTION.with(|collection| {
            collection
                .borrow()
                .clone()
                .into_iter()
                .map(|(_, job)| job)
                .filter(|job| {
                    job.start_date <= *end_date
                        && job
                            .end_date
                            .map_or(true, |end_date| end_date >= *start_date)
                })
                .collect()
        }))
    }
    fn get_part_time_job_by_id(
        &self,
        part_time_job_id: u64,
    ) -> Result<Option<PartTimeJob>, anyhow::Error> {
        Ok(PART_TIME_JOB_COLLECTION
            .with(|collection| collection.borrow().get(&part_time_job_id).cloned()))
    }
    fn store_part_time_job(&self, part_time_job: PartTimeJob) -> Result<u64, anyhow::Error> {
        let id = PART_TIME_JOB_COLLECTION.with(|collection| {
            let id = collection.borrow().len() as u64 + 1;
            collection.borrow_mut().insert(id, part_time_job);
            id
        });
        Ok(id)
    }
    fn update_part_time_job(&self, part_time_job: PartTimeJob) -> Result<(), anyhow::Error> {
        PART_TIME_JOB_COLLECTION.with(|collection| {
            collection
                .borrow_mut()
                .insert(part_time_job.id.unwrap(), part_time_job);
        });
        Ok(())
    }
    fn get_part_time_job_hourly_wage(
        &self,
        part_time_job_id: u64,
        year: i32,
        month: u32,
    ) -> Result<Option<PartTimeHourlyWage>, anyhow::Error> {
        Ok(PART_TIME_JOB_HOURLY_WAGE_COLLECTION.with(|collection| {
            collection
                .borrow()
                .iter()
                .find(|wage| {
                    wage.part_time_job_id == part_time_job_id
                        && wage.start_year_and_month.0 <= year
                        && wage.start_year_and_month.1 <= month
                })
                .cloned()
        }))
    }
    fn get_part_time_job_hourly_wage_by_start_year_and_month(
        &self,
        part_time_job_id: u64,
        start_year_and_month: (i32, u32),
    ) -> Result<Option<PartTimeHourlyWage>, anyhow::Error> {
        Ok(PART_TIME_JOB_HOURLY_WAGE_COLLECTION.with(|collection| {
            collection
                .borrow()
                .iter()
                .find(|wage| {
                    wage.part_time_job_id == part_time_job_id
                        && wage.start_year_and_month == start_year_and_month
                })
                .cloned()
        }))
    }
    fn store_part_time_job_hourly_wage(
        &self,
        part_time_job_id: u64,
        hourly_wage: rust_decimal::Decimal,
        start_year_and_month: (i32, u32),
    ) -> Result<(), anyhow::Error> {
        PART_TIME_JOB_HOURLY_WAGE_COLLECTION.with(|collection| {
            collection.borrow_mut().push(PartTimeHourlyWage {
                part_time_job_id,
                hourly_wage,
                start_year_and_month,
            });
        });
        Ok(())
    }
    fn update_part_time_job_hourly_wage(
        &self,
        part_time_job_id: u64,
        hourly_wage: rust_decimal::Decimal,
        start_year_and_month: (i32, u32),
    ) -> Result<(), anyhow::Error> {
        PART_TIME_JOB_HOURLY_WAGE_COLLECTION.with(|collection| {
            let mut collection = collection.borrow_mut();
            let wage = collection
                .iter_mut()
                .find(|wage| {
                    wage.part_time_job_id == part_time_job_id
                        && wage.start_year_and_month == start_year_and_month
                })
                .unwrap();
            wage.hourly_wage = hourly_wage;
        });
        Ok(())
    }
    fn get_part_time_job_income_by_id(
        &self,
        id: u64,
    ) -> Result<Option<PartTimeJobIncome>, anyhow::Error> {
        Ok(
            PART_TIME_JOB_INCOME_COLLECTION
                .with(|collection| collection.borrow().get(&id).cloned()),
        )
    }
    fn get_part_time_job_income_by_part_time_job_id(
        &self,
        part_time_job_id: u64,
        year: i32,
        month: u32,
    ) -> Result<Option<PartTimeJobIncome>, anyhow::Error> {
        Ok(PART_TIME_JOB_INCOME_COLLECTION.with(|collection| {
            return collection
                .borrow()
                .values()
                .find(|income| {
                    income.part_time_job_id == part_time_job_id
                        && income.payment_date.year() == year
                        && income.payment_date.month() == month
                })
                .cloned();
        }))
    }
    fn store_part_time_job_income(
        &self,
        part_time_job_income: PartTimeJobIncome,
    ) -> Result<u64, anyhow::Error> {
        let id = PART_TIME_JOB_INCOME_COLLECTION.with(|collection| {
            let id = collection.borrow().len() as u64 + 1;
            let part_time_job_income = PartTimeJobIncome {
                id: Some(id),
                ..part_time_job_income
            };
            collection.borrow_mut().insert(id, part_time_job_income);
            id
        });
        Ok(id)
    }
    fn update_part_time_job_income(
        &self,
        part_time_job_income: PartTimeJobIncome,
    ) -> Result<(), anyhow::Error> {
        PART_TIME_JOB_INCOME_COLLECTION.with(|collection| {
            collection
                .borrow_mut()
                .insert(part_time_job_income.id.unwrap(), part_time_job_income);
        });
        Ok(())
    }
}

impl IncomeRepo for DummyPartTimeJobRepo {
    fn list_incomes(
        &self,
        start_date: &DateTime<Local>,
        end_date: &DateTime<Local>,
    ) -> Result<Vec<Income>, anyhow::Error> {
        Ok(PART_TIME_JOB_INCOME_COLLECTION.with(|collection| {
            collection
                .borrow()
                .clone()
                .into_iter()
                .filter(|(_, income)| {
                    income.payment_date >= *start_date && income.payment_date <= *end_date
                })
                .map(|(_, income)| income.to_income())
                .collect()
        }))
    }
}

impl DummyPartTimeJobRepo {
    pub fn new() -> Self {
        Self
    }
}

pub struct DummyMonthlyOutcomeRepo;

impl MonthlyOutcomeRepo for DummyMonthlyOutcomeRepo {
    fn list_monthly_outcome_template(
        &self,
        start_date: &DateTime<Local>,
        end_date: &DateTime<Local>,
    ) -> Result<Vec<MonthlyOutcomeTemplate>, anyhow::Error> {
        Ok(MONTHLY_OUTCOME_TEMPLATE_COLLECTION.with(|collection| {
            collection
                .borrow()
                .clone()
                .into_iter()
                .filter(|(_, template)| {
                    template.start_date <= *end_date
                        && template
                            .end_date
                            .map_or(true, |end_date| end_date >= *start_date)
                })
                .map(|(_, template)| template)
                .collect()
        }))
    }
    fn store_monthly_outcome(&self, monthly_outcome: MonthlyOutcome) -> Result<u64, anyhow::Error> {
        let id = MONTHLY_OUTCOME_COLLECTION.with(|collection| {
            let id = collection.borrow().len() as u64 + 1;
            let monthly_outcome = MonthlyOutcome {
                id: Some(id),
                ..monthly_outcome
            };
            collection.borrow_mut().insert(id, monthly_outcome);
            id
        });
        Ok(id)
    }
    fn update_monthly_outcome(&self, monthly_outcome: MonthlyOutcome) -> Result<(), anyhow::Error> {
        MONTHLY_OUTCOME_COLLECTION.with(|collection| {
            collection
                .borrow_mut()
                .insert(monthly_outcome.id.unwrap(), monthly_outcome);
        });
        Ok(())
    }
    fn get_monthly_outcome_by_template_id(
        &self,
        monthly_outcome_template_id: u64,
        year: i32,
        month: u32,
    ) -> Result<Option<MonthlyOutcome>, anyhow::Error> {
        Ok(MONTHLY_OUTCOME_COLLECTION.with(|collection| {
            collection
                .borrow()
                .values()
                .find(|outcome| {
                    outcome.monthly_outcome_template_id == monthly_outcome_template_id
                        && outcome.payment_date.year() == year
                        && outcome.payment_date.month() == month
                })
                .cloned()
        }))
    }
}

impl OutcomeRepo for DummyMonthlyOutcomeRepo {
    fn list_outcomes(
        &self,
        start_date: &DateTime<Local>,
        end_date: &DateTime<Local>,
    ) -> Result<Vec<Outcome>, anyhow::Error> {
        Ok(MONTHLY_OUTCOME_COLLECTION.with(|collection| {
            collection
                .borrow()
                .clone()
                .into_iter()
                .filter(|(_, outcome)| {
                    outcome.payment_date >= *start_date && outcome.payment_date <= *end_date
                })
                .map(|(_, outcome)| outcome.to_outcome())
                .collect()
        }))
    }
}

impl DummyMonthlyOutcomeRepo {
    pub fn new() -> Self {
        Self
    }
}
