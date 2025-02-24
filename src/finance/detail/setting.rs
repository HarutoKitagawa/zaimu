use chrono::{DateTime, Local, NaiveDate, TimeZone};

pub fn get_opening_and_closing_date(year: i32, month: u32) -> Result<(DateTime<Local>, DateTime<Local>), anyhow::Error> {
    let start_date = Local.with_ymd_and_hms(year, month, 1, 0, 0, 0)
        .single()
        .ok_or_else(|| anyhow::anyhow!("Invalid date: {}-{:02}-01", year, month))?;
    let day = days_in_month(year, month)?;
    let end_date = Local.with_ymd_and_hms(year, month, day, 23, 59, 59)
        .single()
        .ok_or_else(|| anyhow::anyhow!("Invalid date: {}-{:02}-{}", year, month, day))?;
    Ok((start_date, end_date))
}

fn days_in_month(year: i32, month: u32) -> Result<u32, anyhow::Error> {
    let start_of_month = match NaiveDate::from_ymd_opt(year, month, 1) {
        Some(start_of_month) => start_of_month,
        None => return Err(anyhow::anyhow!("Invalid date: {}-{:02}-01", year, month)),
    };
    let start_of_next_month = if month == 12 {
        match NaiveDate::from_ymd_opt(year + 1, 1, 1) {
            Some(start_of_next_month) => start_of_next_month,
            None => return Err(anyhow::anyhow!("Invalid date: {}-01-01", year + 1)),
        }
    } else {
        match NaiveDate::from_ymd_opt(year, month + 1, 1) {
            Some(start_of_next_month) => start_of_next_month,
            None => return Err(anyhow::anyhow!("Invalid date: {}-{:02}-01", year, month + 1)),
        }
    };
    let duration = start_of_next_month.signed_duration_since(start_of_month);
    Ok(duration.num_days() as u32)
}