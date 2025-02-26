use chrono::{Local, DateTime, Days, Months, Datelike, TimeZone};

pub fn get_next_ym(prev: (i32, u32)) -> (i32, u32) {
    let tmp = Local.with_ymd_and_hms(prev.0, prev.1, 1, 0, 0, 0).unwrap();
    let tmp2 = tmp + Months::new(1);
    (tmp2.year(), tmp2.month())
}

pub fn get_prev_ym(next: (i32, u32)) -> (i32, u32) {
    let tmp = Local.with_ymd_and_hms(next.0, next.1, 1, 0, 0, 0).unwrap();
    let tmp2 = tmp - Months::new(1);
    (tmp2.year(), tmp2.month())
}

pub fn get_end_of_month(year: i32, month: u32) ->  Result<DateTime<Local>, anyhow::Error> {
    let tmp = Local.with_ymd_and_hms(year, month, 1, 0, 0, 0)
        .single()
        .ok_or_else(|| anyhow::anyhow!("Invalid date: {}-{:02}-01", year, month))?;
    Ok(tmp + Months::new(1) - Days::new(1))
}