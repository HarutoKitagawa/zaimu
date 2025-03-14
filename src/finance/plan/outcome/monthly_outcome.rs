use chrono::{DateTime, Local, TimeZone};
use rust_decimal::Decimal;
use crate::util::get_end_of_month;
use crate::finance::setting::get_opening_and_closing_date;
use super::outcome::{Outcome, ToOutcome, OutcomeRepo};

#[derive(Debug, Clone)]
pub enum PaymentTiming {
    End,
    Mid(u32),
}

#[derive(Debug, Clone)]
pub struct MonthlyOutcomeTemplate {
    pub id: Option<u64>,
    pub name: String,
    pub amount: Decimal,
    pub payment_timing: PaymentTiming,
    pub start_date: DateTime<Local>,
    pub end_date: Option<DateTime<Local>>,
}

#[derive(Debug, Clone)]
pub struct MonthlyOutcome {
    pub id: Option<u64>,
    pub monthly_outcome_template_id: u64,
    pub name: String,
    pub amount: Decimal,
    pub payment_date: DateTime<Local>,
}

impl MonthlyOutcomeTemplate {
    pub fn to_monthly_outcome(
        &self,
        year: i32,
        month: u32,
        repo: &impl MonthlyOutcomeRepo,
    ) -> Result<MonthlyOutcome, anyhow::Error> {
        let payment_date = self.get_payment_date(year, month)?;
        let mut monthly_outcome = MonthlyOutcome {
            id: None,
            monthly_outcome_template_id: self.id.unwrap(),
            name: self.name.clone(),
            amount: self.amount.clone(),
            payment_date,
        };
        let id = repo.store_monthly_outcome(monthly_outcome.clone())?;
        monthly_outcome.id = Some(id);
        Ok(monthly_outcome)
    }

    pub fn get_payment_date(
        &self,
        year: i32,
        month: u32,
    ) -> Result<DateTime<Local>, anyhow::Error> {
        match self.payment_timing {
            PaymentTiming::End => get_end_of_month(year, month),
            PaymentTiming::Mid(day) => Local
                .with_ymd_and_hms(year, month, day, 0, 0, 0)
                .single()
                .ok_or_else(|| anyhow::anyhow!("Invalid date: {}-{:02}-{:02}", year, month, day)),
        }
    }
}

impl ToOutcome for MonthlyOutcome {
    fn to_outcome(&self) -> Outcome {
        Outcome {
            name: self.name.clone(),
            amount: self.amount.clone(),
            date: self.payment_date,
        }
    }
}

impl MonthlyOutcome {
    pub fn update(
        &self,
        name: String,
        amount: Decimal,
        payment_date: DateTime<Local>,
    ) -> MonthlyOutcome {
        MonthlyOutcome {
            id: self.id,
            monthly_outcome_template_id: self.monthly_outcome_template_id,
            name,
            amount,
            payment_date,
        }
    }
}

pub trait MonthlyOutcomeRepo: OutcomeRepo {
    fn list_monthly_outcome_template(
        &self,
        start_date: &DateTime<Local>,
        end_date: &DateTime<Local>,
    ) -> Result<Vec<MonthlyOutcomeTemplate>, anyhow::Error>;
    fn store_monthly_outcome(&self, monthly_outcome: MonthlyOutcome) -> Result<u64, anyhow::Error>;
    fn update_monthly_outcome(&self, monthly_outcome: MonthlyOutcome) -> Result<(), anyhow::Error>;
    fn get_monthly_outcome_by_template_id(
        &self,
        monthly_outcome_template_id: u64,
        year: i32,
        month: u32,
    ) -> Result<Option<MonthlyOutcome>, anyhow::Error>;
}

pub fn get_or_create_monthly_outcomes(
    year: i32,
    month: u32,
    repo: &impl MonthlyOutcomeRepo,
) -> Result<Vec<Outcome>, anyhow::Error> {
    let (start_date, end_date) = get_opening_and_closing_date(year, month)?;
    let templates = repo.list_monthly_outcome_template(&start_date, &end_date)?;

    let mut outcomes = Vec::new();

    for template in templates {
        let outcome = match repo.get_monthly_outcome_by_template_id(template.id.unwrap(), year, month)? {
            Some(existing_outcome) => existing_outcome,
            None => template.to_monthly_outcome(year, month, repo)?,
        };
        outcomes.push(outcome.to_outcome());
    }

    Ok(outcomes)
}