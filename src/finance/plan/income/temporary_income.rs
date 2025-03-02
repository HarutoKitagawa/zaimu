use chrono::{DateTime, Local};
use rust_decimal::Decimal;
use super::income::{Income, ToIncome};

pub struct TemporaryIncome {
    id: Option<u64>,
    pub name: String,
    pub amount: Decimal,
    pub date: DateTime<Local>,
}

pub trait TemporaryIncomeRepo {
    fn list_temporary_incomes(
        &self,
        start_date: &DateTime<Local>,
        end_date: &DateTime<Local>,
    ) -> Result<Vec<TemporaryIncome>, anyhow::Error>;
}

impl ToIncome for TemporaryIncome {
    fn to_income(&self) -> Income {
        Income {
            name: self.name.clone(),
            amount: self.amount.clone(),
            date: self.date.clone(),
        }
    }
}