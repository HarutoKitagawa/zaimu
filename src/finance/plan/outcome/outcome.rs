use chrono::{DateTime, Local};
use rust_decimal::Decimal;

#[derive(Debug, Clone)]
pub struct Outcome {
    pub name: String,
    pub date: DateTime<Local>,
    pub amount: Decimal,
}

pub trait ToOutcome {
    fn to_outcome(&self) -> Outcome;
}

pub trait OutcomeRepo {
    fn list_outcomes(
        &self,
        start_date: &DateTime<Local>,
        end_date: &DateTime<Local>,
    ) -> Result<Vec<Outcome>, anyhow::Error>;
}