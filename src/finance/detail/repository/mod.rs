mod db_dummy;

use crate::finance::detail::income::IncomeRepo;
use crate::finance::detail::outcome::OutcomeRepo;
use crate::finance::detail::adjustment::AdjustmentRepo;
use crate::finance::detail::saving::SavingRepo;

pub fn get_income_repo() -> impl IncomeRepo {
    db_dummy::DummyIncomeRepo::new()
}

pub fn get_outcome_repo() -> impl OutcomeRepo {
    db_dummy::DummyOutcomeRepo::new()
}

pub fn get_adjustment_repo() -> impl AdjustmentRepo {
    db_dummy::DummyAdjustmentRepo::new()
}

pub fn get_saving_repo() -> impl SavingRepo {
    db_dummy::DummySavingRepo::new()
}