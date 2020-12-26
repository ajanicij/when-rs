use chrono::{
    Local,
    // DateTime,
    // TimeZone,
    Duration,
};

// pub type Date = chrono::Date<chrono::offset::Local>;

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
}
