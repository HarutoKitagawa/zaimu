use chrono::{Local, Months, Datelike, TimeZone};

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