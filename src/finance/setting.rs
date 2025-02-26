use chrono::{DateTime, Datelike, Local, TimeZone};
use crate::util::get_end_of_month;

pub fn get_opening_and_closing_date(year: i32, month: u32) -> Result<(DateTime<Local>, DateTime<Local>), anyhow::Error> {
    let start_date = Local.with_ymd_and_hms(year, month, 1, 0, 0, 0)
        .single()
        .ok_or_else(|| anyhow::anyhow!("Invalid date: {}-{:02}-01", year, month))?;
    let day = get_end_of_month(year, month)?.day();
    let end_date = Local.with_ymd_and_hms(year, month, day, 23, 59, 59)
        .single()
        .ok_or_else(|| anyhow::anyhow!("Invalid date: {}-{:02}-{}", year, month, day))?;
    Ok((start_date, end_date))
}