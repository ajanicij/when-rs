use crate::date;
use chrono::Datelike;
use regex::Regex;

enum NumberCheck
{
    Any,
    Match(u32)
}

impl NumberCheck {
    fn check(&self, n: u32) -> bool {
        match *self {
            NumberCheck::Match(i) => i == n,
            NumberCheck::Any => true
        }
    }
}

pub enum DateChecker {
    Spec {
        year: NumberCheck,
        month: NumberCheck,
        day: NumberCheck,
    },
    Test {
        // TODO: specified by an expression
        //       e.g: m=jul & c=4
    }
}

fn parse_number_expression(s: &str) -> Option<NumberCheck> {
    match s.parse::<u32>() {
        Ok(v) => Some(NumberCheck::Match(v)),
        Err(_) => {
            if s == "*" {
                Some(NumberCheck::Any)
            } else {
                None
            }
        }
    }
}

fn parse_month_expression(s: &str) -> Option<NumberCheck> {
    let ls = s.to_lowercase();
    let months = vec![
        (1, "january"),
        (2, "february"),
        (3, "march"),
        (4, "april"),
        (5, "may"),
        (6, "june"),
        (7, "july"),
        (8, "august"),
        (9, "september"),
        (10, "october"),
        (11, "november"),
        (12, "december"),
    ];
    let matches: Vec<(u32, &str)> = months.iter()
        .filter(|(ind, m)| (*m).starts_with(&ls))
        .map(|(ind, m)| (*ind, *m))
        .collect();
    if matches.len() == 0 {
        // No match
        return None
    }
    if matches.len() > 1 {
        // More than one match; we don't know which month.
        return None
    }
    Some(NumberCheck::Match(matches[0].0))
}

impl DateChecker {
    pub fn new(expr: &str) -> Result<DateChecker, String> {
        if (expr.find('=') == None) && (expr.find('&') == None) {
            let re = Regex::new(r"\s+");
            if re.is_err() {
                return Err(String::from("Bad date expression"));
            }
            let re = re.unwrap();
            let mut split: Vec<&str> = re.split(expr).collect();
            if split.len() != 3 {
                return Err(String::from("Bad date expression"));
            }
            let year = parse_number_expression(split[0]);
            if year.is_none() {
                return Err(String::from("Bad year"));
            }
            let year = year.unwrap();

            let month = parse_month_expression(split[1]);
            if month.is_none() {
                return Err(String::from("Bad month"));
            }
            let month = month.unwrap();

            let day = parse_number_expression(split[2]);
            if day.is_none() {
                return Err(String::from("Bad day"));
            }
            let day = day.unwrap();

            return Ok(DateChecker::Spec { year, month, day });
        }
        // TODO
        Ok(DateChecker::Test { })
    }

    pub fn CheckDateRange(&self, first: &date::Date, last: &date::Date) -> bool {
        false
    }

    pub fn CheckDate(&self, date: &date::Date) -> bool {
        if let DateChecker::Spec{year, month, day} = self {
            if !year.check(date.year() as u32) {
                return false;
            }

            if !month.check(date.month()) {
                return false;
            }

            if !day.check(date.day()) {
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

        let checker = DateChecker::new("August 1");
        assert!(checker.is_err());

        let checker = DateChecker::new("1999 Jun 17").unwrap();
        assert!(checker.CheckDate(&date));

        // Different ways to specify month
        let checker = DateChecker::new("1999 july 17").unwrap();
        assert!(!checker.CheckDate(&date));

        // Negative test
        let checker = DateChecker::new("2001 Jun 17").unwrap();
        assert!(!checker.CheckDate(&date));

        // TODO
        // Positive test, with *
        let checker = DateChecker::new("* Jun 17").unwrap();
        assert!(checker.CheckDate(&date));

        let date = date::new_date(1969, 5, 14);
        assert!(!checker.CheckDate(&date));
        let checker = DateChecker::new("* May 14").unwrap();
        assert!(checker.CheckDate(&date));
        let checker = DateChecker::new("1969 may *").unwrap();
        assert!(checker.CheckDate(&date));
    }

    #[test]
    fn parsing_date_expression() {
        assert_eq!(1, 1);
        let str = "  one two  three";
        let re = Regex::new(r"\s+").unwrap();
        let mut split: Vec<&str> = re.split(str).collect();
        assert_eq!(split.len(), 4);
        assert_eq!(split[1], "one");
        assert_eq!(split[2], "two");
        assert_eq!(split[3], "three");

        // Parse expressions such as m=may
        let str = "m=oct";
        let re = Regex::new(r"=").unwrap();
        let mut split: Vec<&str> = re.split(str).collect();
        eprintln!("split is {:?}", split);
        assert_eq!(split.len(), 2);
        assert_eq!(split[0], "m");
        assert_eq!(split[1], "oct");
    }
}
