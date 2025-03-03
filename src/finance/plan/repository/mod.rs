use super::income;
use super::outcome;
mod db_dummy;
use db_dummy::{DummyMonthlyOutcomeRepo, DummyPartTimeJobRepo, DummyTemporaryOutcomeRepo};

pub fn get_part_time_job_repo() -> impl income::job::PartTimeJobRepo {
    DummyPartTimeJobRepo::new()
}

pub fn get_monthly_outcome_repo() -> impl outcome::monthly_outcome::MonthlyOutcomeRepo {
    DummyMonthlyOutcomeRepo::new()
}

pub fn get_temporary_outcome_repo() -> impl outcome::temporary_outcome::TemporaryOutcomeRepo {
    DummyTemporaryOutcomeRepo::new()
}