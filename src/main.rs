use clap::{App, Arg};
use std::process;
use std::env;
use std::path;
use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use chrono::{Local, Duration};

mod preferences;
mod date;
mod datecalc;
mod utils;

fn main() {
    let home_dir: String;
    match env::var("HOME") {
        Ok(home) => home_dir = String::from(home),
        Err(_) => {
            eprintln!("HOME directory unknown");
            process::exit(0);
        }
    }

    // println!("HOME: {}", home_dir);
    let preferences_path = path::Path::new(&home_dir).join(".when-rs").join("preferences")
        .to_str()
        .unwrap()
        .to_string();
    println!("preferences: {}", preferences_path);

    let it = std::fs::read_to_string(&preferences_path);
    if let Err(err) = it {
        eprintln!("Couldn't open {}: {}", preferences_path, err);
        process::exit(-1);
    }
    let it = it.unwrap();

    // Read preferences from preferences file.
    let hashmap_preferences = preferences::parse_lines(it.lines());

    // eprintln!("preferences: {:?}", hashmap_preferences);
    let _calendar;
    match hashmap_preferences.get("calendar") {
        Some(p) => _calendar = Path::new(p),
        None => {
            eprintln!("Configuration doesn't have calendar key");
            process::exit(-1);
        }
    }

    // Parse command line arguments.
    let matches = App::new("when-rs")
        .version("0.1")
        .about("Simple personal calendar utility")
        .arg(
            Arg::with_name("future")
                .long("future")
                .takes_value(true)
                .default_value("14")
                .help("How many days into the future the report extends.")
        )
        .arg(
            Arg::with_name("past")
                .long("past")
                .takes_value(true)
                .default_value("-1")
                .help(r#"How many days into the past the report extends.
Like the --future option, --past is interpreted as an offset
relative to the present date, so normally you would want
this to be a negative value. Default: -1"#)
        )
        .arg(
            Arg::with_name("calendar")
                .long("calendar")
                .takes_value(true)
                .help(r#"Your calendar file. The default is to use the
file pointed to by your preferences file, which is
set up the first time you run when-rs."#)
        )
        .get_matches();

    let mut arg_future: i32 = 14;
    let mut arg_past: i32 = -1;

    if let Some(n) = matches.value_of("future") {
        match n.parse::<i32>() {
            Ok(future) => arg_future = future,
            _ => {
                eprintln!("{}", matches.usage());
                process::exit(-1);
            }
        }
        // println!("future is {}", n);
    }

    if let Some(n) = matches.value_of("past") {
        match n.parse::<i32>() {
            Ok(past) => arg_past = past,
            _ => {
                eprintln!("{}", matches.usage());
                process::exit(-1);
            }
        }
    }

    // Get calendar from calendar file, specified in command line or
    // preferences.
    let calendar;
    if let Some(path) = matches.value_of("calendar") {
        calendar = Path::new(path);
    } else if let Some(path) = hashmap_preferences.get("calendar") {
        calendar = Path::new(path);
    } else {
        eprintln!("Configuration doesn't have calendar key");
        process::exit(-1);
    }

    // eprintln!("calendar file is {:?}", calendar);
    let file = File::open(calendar);
    if let Err(err) = file {
        eprintln!("Failure opening {}: {}", calendar.to_str().unwrap(), err);
        process::exit(-1);
    }
    let file = file.unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        if let Ok(line_str) = line {
            // eprintln!("Line: {}", line_str);
            if let Some((expr, descr)) = utils::parse_calendar_line(&line_str) {
                // eprintln!(" -- expression: {}", expr);
                // eprintln!(" -- description: {}", descr);
                if let Ok(checker) = datecalc::DateChecker::new(&expr) {
                    // let date = date::new_date(2021, 1, 2);
                    let today = Local::today().naive_local();
                    let date1 = today + Duration::days(arg_past.into());
                    let date2 = today + Duration::days(arg_future.into());
                    if let Some(date) = checker.check_date_range(&date1, &date2) {
                        if date == today {
                            println!("today {} {}", date.format("%Y %b %e"), descr);
                        } else {
                            println!("      {} {}", date.format("%Y %b %e"), descr);
                        }
                    }
                }
            }
        }
    }
}
