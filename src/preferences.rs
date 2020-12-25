use regex::Regex;
use std::collections::HashMap;

pub fn parse_line(line: &str) -> Result<(String, String), String> {
    let re = Regex::new(r"\s*(\S.*)\s*=\s*(\S.*) *$").unwrap();
    let cap = re.captures(line);
    match cap {
        None => return Err("mismatch".to_string()),
        Some(captures) => {
            if captures.len() == 3 {
                let key = String::from(captures[1].trim());
                let value = String::from(captures[2].trim());
                return Ok((key, value));
            } else {
                return Err("mismatch".to_string());
            }
        },
    }
}

pub fn parse_lines<'a, It: Iterator<Item=&'a str>>(lines: It) ->
    HashMap<String, String>
{
    let mut hm = HashMap::new();
    for line in lines {
        // eprintln!("line is {}", line);
        if let Ok((key, value)) = parse_line(line) {
            hm.insert(key, value);
        }
    }
    return hm;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_one_line() {
        let res = parse_line(" abc ghi   =   def ");
        assert!(res.is_ok());
        if let Ok((key, value)) = res {
            assert_eq!(key, "abc ghi");
            assert_eq!(value, "def");
        }
    }

    #[test]
    fn returns_parsing_error() {
        let res = parse_line(" no equal sign");
        assert!(res.is_err());
    }

    #[test]
    fn can_parse_lines() {
        let lines = vec![
            " a = b",
            "c=d",
            "nothing",
            "e  =   f"
        ];

        let hm = parse_lines(lines.iter().map(|s| *s));
        eprintln!("hm is {:?}", hm);
        assert_eq!(hm.len(), 3);
        assert_eq!(hm.get("a"), Some(&String::from("b")));
        assert_eq!(hm.get("c"), Some(&String::from("d")));
        assert_eq!(hm.get("e"), Some(&String::from("f")));
    }
}
