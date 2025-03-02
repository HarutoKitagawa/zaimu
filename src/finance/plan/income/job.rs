use chrono::{DateTime, Local, TimeZone, Datelike};
use rust_decimal::Decimal;
use super::income::{Income, ToIncome, IncomeRepo};
use crate::util::{get_end_of_month, get_next_ym};
use crate::finance::setting::get_opening_and_closing_date;

// 入金日
// 月末, 月中(何日か)
#[derive(Debug, Clone)]
pub enum PaymentTiming {
    End,
    Mid(u32),
    NextMonthEnd,
    NextMonthMid(u32),
}

#[derive(Debug, Clone)]
pub struct PartTimeJob {
    pub id: Option<u64>,
    pub name: String,
    pub payment_timing: PaymentTiming,
    pub start_date: DateTime<Local>,
    pub end_date: Option<DateTime<Local>>,
}

#[derive(Debug, Clone)]
pub struct PartTimeHourlyWage {
    pub part_time_job_id: u64,
    pub hourly_wage: Decimal,
    pub start_year_and_month: (i32, u32),
}

#[derive(Debug, Clone)]
pub struct PartTimeJobIncome {
    pub id: Option<u64>,
    pub part_time_job_id: u64,
    pub name: String,
    pub hourly_wage: Decimal,
    pub hour: Decimal,
    pub payment_date: DateTime<Local>,
}

impl ToIncome for PartTimeJobIncome {
    fn to_income(&self) -> Income {
        Income {
            name: self.name.clone(),
            amount: self.hourly_wage * self.hour,
            date: self.payment_date,
        }
    }
}

impl PartTimeJob {
    pub fn get_hourly_wage(
        &self,
        year: i32,
        month: u32,
        repo: &impl PartTimeJobRepo,
    ) -> Option<PartTimeHourlyWage> {
        repo.get_part_time_job_hourly_wage(self.id.unwrap(), year, month)
            .unwrap_or(None)
    }

    pub fn set_hourly_wage(
        &self,
        hourly_wage: Decimal,
        start_year_and_month: (i32, u32),
        repo: &impl PartTimeJobRepo,
    ) -> Result<(), anyhow::Error> {
        match repo.get_part_time_job_hourly_wage_by_start_year_and_month(
            self.id.unwrap(),
            start_year_and_month,
        ) {
            Ok(Some(_)) => {
                repo.update_part_time_job_hourly_wage(
                    self.id.unwrap(),
                    hourly_wage,
                    start_year_and_month,
                )?;
            }
            _ => {
                repo.store_part_time_job_hourly_wage(
                    self.id.unwrap(),
                    hourly_wage,
                    start_year_and_month,
                )?;
            }
        }
        Ok(())
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
            PaymentTiming::NextMonthEnd => {
                let (year, month) = get_next_ym((year, month));
                return get_end_of_month(year, month)
            },
            PaymentTiming::NextMonthMid(day) => {
                let (year, month) = get_next_ym((year, month));
                Local
                    .with_ymd_and_hms(year, month, day, 0, 0, 0)
                    .single()
                    .ok_or_else(|| anyhow::anyhow!("Invalid date: {}-{:02}-{:02}", year, month, day))
            },
        }
    }

    pub fn to_part_time_job_income(
        &self,
        year: i32,
        month: u32,
        hour: Decimal,
        repo: &impl PartTimeJobRepo,
    ) -> Result<PartTimeJobIncome, anyhow::Error> {
        let hourly_wage = self
            .get_hourly_wage(year, month, repo)
            .map_or(Decimal::ZERO, |wage| wage.hourly_wage);

        let mut income = PartTimeJobIncome {
            id: None,
            part_time_job_id: self.id.unwrap(),
            name: self.name.clone(),
            hourly_wage,
            hour,
            payment_date: self.get_payment_date(year, month)?,
        };

        let id = repo.store_part_time_job_income(income.clone())?;
        income.id = Some(id);
        Ok(income)
    }
}

impl PartTimeJobIncome {
    pub fn update(
        &self,
        name: String,
        hourly_wage: Decimal,
        hour: Decimal,
        payment_date: DateTime<Local>,
    ) -> Self {
        Self {
            id: self.id,
            part_time_job_id: self.part_time_job_id,
            name,
            hourly_wage,
            hour,
            payment_date,
        }
    }
}

pub trait PartTimeJobRepo: IncomeRepo {
    fn list_part_time_jobs(
        &self,
        start_date: &DateTime<Local>,
        end_date: &DateTime<Local>,
    ) -> Result<Vec<PartTimeJob>, anyhow::Error>;
    fn get_part_time_job_by_id(
        &self,
        id: u64,
    ) -> Result<Option<PartTimeJob>, anyhow::Error>;
    fn store_part_time_job(&self, part_time_job: PartTimeJob) -> Result<u64, anyhow::Error>;
    fn update_part_time_job(&self, part_time_job: PartTimeJob) -> Result<(), anyhow::Error>;
    fn get_part_time_job_hourly_wage(
        &self,
        part_time_job_id: u64,
        year: i32,
        month: u32,
    ) -> Result<Option<PartTimeHourlyWage>, anyhow::Error>;
    fn get_part_time_job_hourly_wage_by_start_year_and_month(
        &self,
        part_time_job_id: u64,
        start_year_and_month: (i32, u32),
    ) -> Result<Option<PartTimeHourlyWage>, anyhow::Error>;
    fn store_part_time_job_hourly_wage(
        &self,
        part_time_job_id: u64,
        hourly_wage: Decimal,
        start_year_and_month: (i32, u32),
    ) -> Result<(), anyhow::Error>;
    fn update_part_time_job_hourly_wage(
        &self,
        part_time_job_id: u64,
        hourly_wage: Decimal,
        start_year_and_month: (i32, u32),
    ) -> Result<(), anyhow::Error>;
    fn get_part_time_job_income_by_id(
        &self,
        id: u64,
    ) -> Result<Option<PartTimeJobIncome>, anyhow::Error>;
    fn get_part_time_job_income_by_part_time_job_id(
        &self,
        part_time_job_id: u64,
        year: i32,
        month: u32,
    ) -> Result<Option<PartTimeJobIncome>, anyhow::Error>;
    fn store_part_time_job_income(
        &self,
        part_time_job_income: PartTimeJobIncome,
    ) -> Result<u64, anyhow::Error>;
    fn update_part_time_job_income(
        &self,
        part_time_job_income: PartTimeJobIncome,
    ) -> Result<(), anyhow::Error>;
}

pub fn get_or_create_incomes(
    year: i32,
    month: u32,
    repo: &impl PartTimeJobRepo,
) -> Result<Vec<Income>, anyhow::Error> {
    let (start_date, end_date) = get_opening_and_closing_date(year, month)?;
    let part_time_jobs = repo.list_part_time_jobs(&start_date, &end_date)?;

    let mut incomes = Vec::new();
    for job in part_time_jobs {
        let job_payment_date = job.get_payment_date(year, month)?;
        let income = match repo.get_part_time_job_income_by_part_time_job_id(
            job.id.unwrap(),
            job_payment_date.year(),
            job_payment_date.month(),
        )? {
            Some(income) => income,
            None => job.to_part_time_job_income(year, month, Decimal::ZERO, repo)?,
        };
        incomes.push(income.to_income());
    }
    Ok(incomes)
}