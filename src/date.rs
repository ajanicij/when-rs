use chrono::{
    Local,
    Utc,
    Duration,
    NaiveDate,
    Datelike,
    offset,
};

pub type Date = chrono::NaiveDate;

pub fn new_date(year: i32, month: u32, day: u32) -> Date {
    NaiveDate::from_ymd(year, month, day)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn date_calculation() {
        let date = Local::today();
        let date2 = date + Duration::days(3);
        assert_eq!(date2.signed_duration_since(date).num_days(), 3);
        assert_eq!(date2 - date, Duration::days(3));
    }

    #[test]
    fn date_construction() {
        let date = new_date(1999, 6, 17);
        assert_eq!(date.year(), 1999);
        assert_eq!(date.month(), 6);
        assert_eq!(date.day(), 17);
    }
}
