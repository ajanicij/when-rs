use regex::Regex;

pub fn parse_calendar_line(line: &str) -> Option<(String, String)> {
    let re = Regex::new(r"([^,]+),(.*)").unwrap();
    match re.captures(line) {
        None => None,
        Some(captures) => {
            Some( (
                       captures.get(1).unwrap().as_str().to_string(),
                       captures.get(2).unwrap().as_str().to_string(),
                   )
                )
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn positive_test() {
        let r = parse_calendar_line("abc,def");
        assert!(r.is_some());
        let p = r.unwrap();
        assert_eq!(p.0, "abc");
        assert_eq!(p.1, "def");

        let r = parse_calendar_line("expr,This, my friend, is the description");
        assert!(r.is_some());
        let r = r.unwrap();
        assert_eq!(r.0, "expr");
        assert_eq!(r.1, "This, my friend, is the description");
    }

    #[test]
    fn negative_test() {
        let r = parse_calendar_line("abc def");
        assert!(r.is_none());
    }
}
