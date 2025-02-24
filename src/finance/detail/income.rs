use chrono::{DateTime, Local, TimeZone};
use rust_decimal::prelude::*;

#[derive(Debug, Clone)]
pub struct Income {
    pub id: Option<u64>,
    pub name: String,
    pub amount: Decimal,
    pub date: DateTime<Local>,
}

impl Income {
    pub fn new(name: String, amount: Decimal, date: DateTime<Local>) -> Self {
        Self { id: None, name, amount, date }
    }
    
    pub fn try_new(name: String, amount: String, year: i32, month: u32, day: u32) -> Result<Self, anyhow::Error> {
        let amount = Decimal::from_str(&amount)
            .map_err(|e| anyhow::anyhow!("Invalid amount '{}': {}", amount, e))?;

        let date = Local.with_ymd_and_hms(year, month, day, 0, 0, 0)
            .single()
            .ok_or_else(|| anyhow::anyhow!("Invalid date: {}-{:02}-{:02}", year, month, day))?;

        Ok(Self { id: None, name, amount, date })
    }

    pub fn try_update(id: u64, name: String, amount: String, year: i32, month: u32, day: u32) -> Result<Self, anyhow::Error> {
        let amount = Decimal::from_str(&amount)
            .map_err(|e| anyhow::anyhow!("Invalid amount '{}': {}", amount, e))?;

        let date = Local.with_ymd_and_hms(year, month, day, 0, 0, 0)
            .single()
            .ok_or_else(|| anyhow::anyhow!("Invalid date: {}-{:02}-{:02}", year, month, day))?;

        Ok(Self { id: Some(id), name, amount, date })
    }
}

pub trait IncomeRepo {
    fn list(&self, start_date: &DateTime<Local>, end_date: &DateTime<Local>) -> Result<Vec<Income>, anyhow::Error>;
    fn get_by_id(&self, id: u64) -> Result<Option<Income>, anyhow::Error>;
    fn store(&self, income: Income) -> Result<u64, anyhow::Error>;
    fn update(&self, income: Income) -> Result<(), anyhow::Error>;
    fn delete_by_id(&self, id: u64) -> Result<(), anyhow::Error>;
}