use chrono::{DateTime, Local};
use rust_decimal::Decimal;

pub struct Income {
    pub name: String,
    pub amount: Decimal,
    pub date: DateTime<Local>,
}

pub trait ToIncome {
    fn to_income(&self) -> Income;
}

pub trait IncomeRepo {
    fn list_incomes(
        &self,
        start_date: &DateTime<Local>,
        end_date: &DateTime<Local>,
    ) -> Result<Vec<Income>, anyhow::Error>;
}

pub fn get_incomes<T: IncomeRepo>(
    repos: Vec<&T>,
    start_date: &DateTime<Local>,
    end_date: &DateTime<Local>,
) -> Result<Vec<Income>, anyhow::Error> {
    let mut incomes = Vec::new();
    for repo in repos {
        incomes.append(&mut repo.list_incomes(start_date, end_date)?);
    }
    Ok(incomes)
}
