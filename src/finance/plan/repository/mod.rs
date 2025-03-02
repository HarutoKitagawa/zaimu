use super::income;
use super::outcome;
mod db_dummy;
use db_dummy::{DummyPartTimeJobRepo, DummyMonthlyOutcomeRepo};

pub fn get_part_time_job_repo() -> impl income::job::PartTimeJobRepo {
    DummyPartTimeJobRepo::new()
}

pub fn get_monthly_outcome_repo() -> impl outcome::monthly_outcome::MonthlyOutcomeRepo {
    DummyMonthlyOutcomeRepo::new()
}