use chrono::*;

pub fn middle_day(year: u32) -> Option<Weekday> {
    if (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 {
        return None;
    } else {
        return Some(NaiveDate::from_yo_opt(year as i32, 183).unwrap().weekday());
    }
}
