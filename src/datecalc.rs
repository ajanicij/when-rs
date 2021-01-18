use crate::date;
use chrono::{Duration, Datelike};
use regex::Regex;

pub enum NumberCheck
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

#[derive(PartialEq, Debug)]
pub enum DateExpression {
    W(u16), // day of the week
    M(u16), // month
    D(u16), // day of the month
    Y(u16), // year
    A(u16), // 1 for the first 7 days of the month, 2 for the next 7, etc.
    Z(u16), // day of the year (1 on New Year Day)
} 

impl DateExpression {
    fn check(&self, date: &date::Date) -> bool {
        match self {
            DateExpression::W(w) => {
                let weekday = date.weekday().number_from_monday();
                return (*w as u32) == weekday;
            },
            DateExpression::M(m) => {
                return (*m as u32) == date.month();
            },
            DateExpression::D(d) => {
                return (*d as u32) == date.day();
            },
            DateExpression::Y(y) => {
                return (*y as i32) == date.year();
            },
            DateExpression::A(a) => {
                return (date.day() / 7) + 1 == *a as u32;
            },
            DateExpression::Z(z) => {
                return date.ordinal() == *z as u32;
            },
        }
    }
}

pub enum DateChecker {
    Spec {
        year: NumberCheck,
        month: NumberCheck,
        day: NumberCheck,
    },
    Expr (
        Vec<DateExpression>,
    )
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

fn parse_month(s: &str) -> Option<u8> {
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
    let matches: Vec<(u8, &str)> = months.iter()
        .filter(|(_, m)| (*m).starts_with(&ls))
        .map(|(ind, m)| (*ind, *m))
        .collect();

    if matches.len() == 0 {
        // No match
        return None;
    }
    if matches.len() > 1 {
        // More than one match; we don't know which month.
        return None;
    }
    Some(matches[0].0)
}

fn parse_month_expression(s: &str) -> Option<NumberCheck> {
    if let Some(n) = parse_month(s) {
        return Some(NumberCheck::Match(n as u32));
    }
    None
}

fn get_date_range(date1: &date::Date, date2: &date::Date) -> Vec<date::Date> {
    let d1 = date1.num_days_from_ce();
    let d2 = date2.num_days_from_ce();
    let diff = d2 - d1;
    let mut v: Vec<date::Date> = Vec::new();
    for d in 0..=diff {
        let date = *date1 + Duration::days(d as i64);
        v.push(date);
    }
    v
}

impl DateChecker {
    pub fn new(expr: &str) -> Result<DateChecker, String> {
        if (expr.find('=') == None) && (expr.find('&') == None) {
            let re = Regex::new(r"\s+").unwrap();
            let split: Vec<&str> = re.split(expr).collect();
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
        // TODO: parse date expression.
        let re = Regex::new(r"\s+").unwrap();
        let split: Vec<&str> = re.split(expr).collect();
        if split.len() % 2 != 1 {
            // Expression must have form e1 & e2 & ... & en
            // Where ei are terms like w=3. So there must be an odd number
            // of words separated by spaces.
            return Err(String::from("Bad date expression"));
        }
        let mut v: Vec<DateExpression> = vec![];
        let mut i = 0;
        loop {
            let re2 = Regex::new("=").unwrap();
            let term = split[i];
            // term must have the form like "w=3".
            let parts: Vec<&str> = re2.split(term).collect();
            if parts.len() != 2 {
                return Err(String::from("Bad date expression"));
            }

            let val = parts[1].parse::<u16>();
            let term;
            match parts[0] {
                "w" => match val {
                    Ok(w) => term = DateExpression::W(w),
                    Err(_) => return Err(String::from("Bad date expression")),
                },
                "m" => match val {
                    // TODO: parse month like "jan", instead of a number.
                    Ok(m) => term = DateExpression::M(m),
                    Err(_) => {
                        let month = parse_month(parts[1]);
                        match month {
                            Some(m) => term = DateExpression::M(m as u16),
                            None => return Err(String::from("Bad date expression")),
                        }
                    }
                },
                "d" => match val {
                    Ok(d) => term = DateExpression::D(d),
                    Err(_) => return Err(String::from("Bad date expression")),
                },
                "y" => match val {
                    Ok(y) => term = DateExpression::Y(y),
                    Err(_) => return Err(String::from("Bad date expression")),
                },
                "a" => match val {
                    Ok(a) => term = DateExpression::A(a),
                    Err(_) => return Err(String::from("Bad date expression")),
                },
                "z" => match val {
                    Ok(z) => term = DateExpression::Z(z),
                    Err(_) => return Err(String::from("Bad date expression")),
                },
                _   => {
                    return Err(String::from("Bad date expression"));
                },
            }
            v.push(term);

            if i + 1 >= split.len() {
                break;
            }
            if split[i+1] != "&" {
                return Err(String::from("Bad date expression"));
            }
            i = i + 2;
        }
        
        // Ok(DateChecker::Expr(vec![DateExpression::W(2)]))
        Ok(DateChecker::Expr(v))
    }

    pub fn check_date_range(&self, first: &date::Date, last: &date::Date) ->
        Vec<date::Date>
    {
        let date_range = get_date_range(first, last);
        let mut v: Vec<date::Date> = vec![];
        for d in date_range {
            if self.check_date(&d) {
                v.push(d);
            }
        }
        v
    }

    pub fn check_date(&self, date: &date::Date) -> bool {
        match self {
            DateChecker::Spec{year, month, day} => {
                if !year.check(date.year() as u32) {
                    return false;
                }

                if !month.check(date.month()) {
                    return false;
                }

                if !day.check(date.day()) {
                    return false;
                }
                return true;
            },
            DateChecker::Expr(v) => {
                for term in v {
                    if !term.check(date) {
                        return false;
                    }
                }
                return true;
            }
        }

        
    }
}

// TODO: parse date expression (2020 Dec 27).
//       There are two types of expressions:
//       1. - Date pattern, such as * Feb 14
//       2. - Conjunction of terms, such as
//            m=jan & w=mon & a=3

pub fn parse_date(s: &str) -> Option<date::Date> {
    let re = Regex::new(r"\s+").unwrap();
    let split: Vec<&str> = re.split(s).collect();
    if split.len() != 3 {
        return None;
    }
    
    let year = split[0].parse::<i32>();
    if year.is_err() {
        return None;
    }
    let year = year.unwrap();

    let month = parse_month(split[1]);
    if month.is_none() {
        return None;
    }
    let month = month.unwrap();

    let day = split[2].parse::<u32>();
    if day.is_err() {
        return None;
    }
    let day = day.unwrap();
    
    Some(date::new_date(year, month as u32, day))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Datelike, NaiveDate};

    #[test]
    fn fail() {
        assert_eq!(1, 1);
    }

    // Utility function used just for test
    fn new_date(year: i32, month: u32, day: u32) -> date::Date {
        NaiveDate::from_ymd(year, month, day)
    }

    #[test]
    fn simple_check_test() {
        let date = new_date(1999, 6, 17);
        assert_eq!(date.year(), 1999);
        assert_eq!(date.month(), 6);
        assert_eq!(date.day(), 17);

        let checker = DateChecker::new("August 1");
        assert!(checker.is_err());

        let checker = DateChecker::new("1999 Jun 17").unwrap();
        assert!(checker.check_date(&date));

        // Different ways to specify month
        let checker = DateChecker::new("1999 july 17").unwrap();
        assert!(!checker.check_date(&date));

        // Negative test
        let checker = DateChecker::new("2001 Jun 17").unwrap();
        assert!(!checker.check_date(&date));

        // TODO
        // Positive test, with *
        let checker = DateChecker::new("* Jun 17").unwrap();
        assert!(checker.check_date(&date));

        let date = new_date(1969, 5, 14);
        assert!(!checker.check_date(&date));
        let checker = DateChecker::new("* May 14").unwrap();
        assert!(checker.check_date(&date));
        let checker = DateChecker::new("1969 may *").unwrap();
        assert!(checker.check_date(&date));
    }

    #[test]
    fn expression_check_test() {
        let date = new_date(1999, 6, 17);
        let checker = DateChecker::new("m=june & d=17").unwrap();
        assert!(checker.check_date(&date));

        let date = parse_date("2021 Feb 1").unwrap();
        let checker = DateChecker::new("z=32").unwrap();
        assert!(checker.check_date(&date));

        // 2021 September 21 is Tuesday.
        let date = parse_date("2021 Sep 21").unwrap();
        let checker = DateChecker::new("m=9 & w=2").unwrap();
        assert!(checker.check_date(&date));

        // Negative test
        let checker = DateChecker::new("m=july").unwrap();
        assert!(!checker.check_date(&date));
    }

    #[test]
    fn parsing_date_expression() {
        assert_eq!(1, 1);
        let str = "  one two  three";
        let re = Regex::new(r"\s+").unwrap();
        let split: Vec<&str> = re.split(str).collect();
        assert_eq!(split.len(), 4);
        assert_eq!(split[1], "one");
        assert_eq!(split[2], "two");
        assert_eq!(split[3], "three");

        // Parse expressions such as m=may
        let str = "m=oct";
        let re = Regex::new(r"=").unwrap();
        let split: Vec<&str> = re.split(str).collect();
        eprintln!("split is {:?}", split);
        assert_eq!(split.len(), 2);
        assert_eq!(split[0], "m");
        assert_eq!(split[1], "oct");
    }

    #[test]
    fn parsing_test_variables() {
        let checker = DateChecker::new("w=2");
        assert!(!checker.is_err());
        let checker = checker.unwrap();
        match checker {
            DateChecker::Expr(v) => {
                assert_eq!(v.len(), 1);
                // TODO: check that v[0] is DateExpression::W(2).
                assert_eq!(v[0], DateExpression::W(2));
            },
            _ => assert!(false),
        }

        let checker = DateChecker::new("m=feb");
        assert!(!checker.is_err());
    }

    #[test]
    fn parsing_test_variables_negative() {
        let checker = DateChecker::new("w=2 &");
        assert!(checker.is_err());

        let checker = DateChecker::new("abc &");
        assert!(checker.is_err());
    }

    #[test]
    fn creating_date_range() {
        let date1 = new_date(2020, 12, 28);
        let date2 = new_date(2021, 1, 3);
        let r = get_date_range(&date1, &date2);
        eprintln!("r is {:?}", r);
        assert_eq!(r.len(), 7);
    }

    #[test]
    fn check_date_range() {
        let date1 = new_date(2020, 12, 28);
        let date2 = new_date(2021, 1, 3);
        let checker = DateChecker::new("* Jan 2").unwrap();
        assert_eq!(checker.check_date_range(&date1, &date2).len(), 1);

        let checker = DateChecker::new("* Jan 4").unwrap();
        assert_eq!(checker.check_date_range(&date1, &date2).len(), 0);

        let checker = DateChecker::new("2020 decem 27").unwrap();
        assert_eq!(checker.check_date_range(&date1, &date2).len(), 0);

        let checker = DateChecker::new("2020 decem 28").unwrap();
        assert_eq!(checker.check_date_range(&date1, &date2).len(), 1);
    }

    #[test]
    fn parse_month_test() {
        let month_str = "jan";
        let month = parse_month(month_str);
        assert!(month.is_some());
        assert_eq!(month.unwrap(), 1);

        // Now unsuccessful parse.
        let month_str = "ju";
        let month = parse_month(month_str);
        assert!(month.is_none());
    }

    #[test]
    fn parse_date_test() {
        let date = parse_date("2021 Jan 9");
        assert!(date.is_some());
        // TODO:
        let date = date.unwrap();
        assert_eq!(date.year(), 2021);
        assert_eq!(date.month(), 1);
        assert_eq!(date.day(), 9);
    }

    #[test]
    fn check_date_term_test() {
        // Test w
        let term = DateExpression::W(3); // Wednesday
        let date = parse_date("2038 Jan 20").unwrap();
        assert!(term.check(&date));

        // Negative test
        let date = parse_date("2020 Jan 3").unwrap();
        assert!(!term.check(&date));

        // Test m
        let term = DateExpression::M(7); // July
        let date = parse_date("2021 July 11").unwrap();
        assert!(term.check(&date));

        // Negative test
        let date = parse_date("2020 Jan 3").unwrap();
        assert!(!term.check(&date));

        // Test d
        let term = DateExpression::D(17);
        let date = parse_date("2021 Feb 17").unwrap();
        assert!(term.check(&date));

        // Negative test
        let date = parse_date("1969 may 14").unwrap();
        assert!(!term.check(&date));

        // Test y
        let term = DateExpression::Y(2001);
        let date = parse_date("2001 january 1").unwrap();
        assert!(term.check(&date));

        // Negative test
        let date = parse_date("2020 Dec 31").unwrap();
        assert!(!term.check(&date));

        // Test a
        let term = DateExpression::A(2);
        let date = parse_date("2021 Feb 8").unwrap();
        assert!(term.check(&date));

        // Negative test
        let date = parse_date("2001 january 1").unwrap();
        assert!(!term.check(&date));

        // Test z
        let term = DateExpression::Z(32);
        let date = parse_date("2021 Feb 1").unwrap();
        assert!(term.check(&date));

        // Negative test
        let date = parse_date("2001 Mar 1").unwrap();
        assert!(!term.check(&date));
    }
}
