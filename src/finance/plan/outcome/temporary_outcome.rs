use chrono::{DateTime, Local};
use rust_decimal::Decimal;
use super::{Outcome, ToOutcome, OutcomeRepo};
use crate::finance::setting::get_opening_and_closing_date;

#[derive(Debug, Clone)]
pub struct TemporaryOutcome {
    pub id: Option<u64>,
    pub name: String,
    pub amount: Decimal,
    pub date: DateTime<Local>,
}

impl ToOutcome for TemporaryOutcome {
    fn to_outcome(&self) -> Outcome {
        Outcome {
            name: self.name.clone(),
            date: self.date.clone(),
            amount: self.amount.clone(),
        }
    }
}

pub trait TemporaryOutcomeRepo: OutcomeRepo {
    fn list_temporary_outcomes(
        &self,
        start_date: &DateTime<Local>,
        end_date: &DateTime<Local>,
    ) -> Result<Vec<TemporaryOutcome>, anyhow::Error>;
    fn store_temporary_outcome(&self, temporary_outcome: TemporaryOutcome) -> Result<u64, anyhow::Error>;
    fn update_temporary_outcome(&self, temporary_outcome: TemporaryOutcome) -> Result<(), anyhow::Error>;
    fn get_temporary_outcome_by_id(&self, id: u64) -> Result<Option<TemporaryOutcome>, anyhow::Error>;
}

pub fn get_temporary_outcomes(
    year: i32,
    month: u32,
    repo: &impl TemporaryOutcomeRepo,
) -> Result<Vec<Outcome>, anyhow::Error> {
    let (start_date, end_date) = get_opening_and_closing_date(year, month)?;
    repo.list_temporary_outcomes(&start_date, &end_date)?
        .iter()
        .map(|outcome| Ok(outcome.to_outcome()))
        .collect()
}