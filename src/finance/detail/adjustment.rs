use chrono::{DateTime, Local};
use rust_decimal::Decimal;
use super::{
    income::{Income, IncomeRepo}, outcome::{Outcome, OutcomeRepo}, setting::get_opening_and_closing_date, saving::{Saving, SavingRepo, update_saving},
};

#[derive(Debug, Clone)]
pub enum AdjustmentKind {
    Income(u64),
    Outcome(u64),
}

pub type AdjustmentKey = (i32, u32);

#[derive(Debug, Clone)]
pub struct Adjustment {
    pub kind: AdjustmentKind,
    pub amount: Decimal,
    pub date: DateTime<Local>,
}

impl Adjustment {
    pub fn new(kind: AdjustmentKind, amount: Decimal, date: DateTime<Local>) -> Self {
        Self { kind, amount, date }
    }
}

pub fn create_adjustment<I, O, S, A>(
    saving_input: Decimal,
    year: i32,
    month: u32,
    income_repo: &I,
    outcome_repo: &O,
    saving_repo: &S,
    adjustment_repo: &A,
)
    -> Result<(), anyhow::Error>
where I: IncomeRepo, O: OutcomeRepo, S: SavingRepo, A: AdjustmentRepo,
{
    let key = &(year, month);
    if let Some(adjustment) = adjustment_repo.get(key)? {
        adjustment_repo.delete(key)?;
        match adjustment.kind {
            AdjustmentKind::Income(id) => income_repo.delete_by_id(id)?,
            AdjustmentKind::Outcome(id) => outcome_repo.delete_by_id(id)?,
        }
    }
    let (_, closing_date) = get_opening_and_closing_date(year, month)?;
    let saving = match saving_repo.get(key)? {
        Some(saving) => saving,
        None => Saving::new(*key, Decimal::ZERO),
    };
    let adjustment_amount = saving_input - saving.amount;
    update_saving((year, month), adjustment_amount, saving_repo)?;
    let adjustment = if adjustment_amount > Decimal::ZERO {
        let income = Income::new("調整金".to_string(), adjustment_amount, closing_date);
        let id = income_repo.store(income)?;
        Adjustment::new(AdjustmentKind::Income(id), adjustment_amount, closing_date)
    } else if adjustment_amount < Decimal::ZERO {
        let outcome = Outcome::new("調整金".to_string(), -adjustment_amount, closing_date);
        let id = outcome_repo.store(outcome)?;
        Adjustment::new(AdjustmentKind::Outcome(id), -adjustment_amount, closing_date)
    } else {
        return Ok(());
    };
    adjustment_repo.store(key, adjustment)
}

pub trait AdjustmentRepo {
    fn get(&self, key: &AdjustmentKey) -> Result<Option<Adjustment>, anyhow::Error>;
    fn store(&self, key: &AdjustmentKey, adjustment: Adjustment) -> Result<(), anyhow::Error>;
    fn delete(&self, key: &AdjustmentKey) -> Result<(), anyhow::Error>;
}