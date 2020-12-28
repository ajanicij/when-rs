use crate::date;
use chrono::Datelike;

pub struct DateChecker {
    year: Option<u32>,
    month: Option<u32>,
    day: Option<u32>
}

impl DateChecker {
    pub fn new(expr: &str) -> DateChecker {
        DateChecker {
            year: Some(1999),
            month: Some(6),
            day: Some(17),
        }
    }

    pub fn CheckDateRange(&self, first: &date::Date, last: &date::Date) -> bool {
        false
    }

    pub fn CheckDate(&self, date: &date::Date) -> bool {
        if let Some(year) = self.year {
            if year as i32 != date.year() {
                return false;
            }
        }

        if let Some(month) = self.month {
            if month != date.month() {
                return false;
            }
        }

        if let Some(day) = self.day {
            if day != date.day() {
                return false;
            }
        }

        true
    }
}

// TODO: parse date expression (2020 Dec 27).
//       There are two types of expressions:
//       1. - Date pattern, such as * Feb 14
//       2. - Conjunction of terms, such as
//            m=jan & w=mon & a=3

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fail() {
        assert_eq!(1, 1);
    }

    #[test]
    fn simple_test_check() {
        let date = date::new_date(1999, 6, 17);
        assert_eq!(date.year(), 1999);
        assert_eq!(date.month(), 6);
        assert_eq!(date.day(), 17);
        let checker = DateChecker::new("1999 Jun 17");
        assert!(checker.CheckDate(&date));
    }
}
