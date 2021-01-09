use clap::{App, Arg, SubCommand};
use std::process;
use std::env;
use std::path;
use std::path::Path;
use std::fs::{self, File};
use std::io::{self, BufReader, BufRead, Write};
use chrono::{Local, Duration};
use std::fmt::{Display, Debug};
use atty::Stream;

mod preferences;
mod date;
mod datecalc;
mod utils;

fn get_arg() -> clap::ArgMatches<'static> {
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
        .arg(
            Arg::with_name("header")
                .long("header")
                .takes_value(false)
                .help(r#"Print headers at the top of the output"#)
        )
        .arg(
            Arg::with_name("noheader")
                .long("noheader")
                .takes_value(false)
                .help(r#"Don't print headers at the top of the output"#)
        )
        .subcommand(
            SubCommand::with_name("e")
                .about("runs editor for editing calendar file")
        )
        .subcommand(
            SubCommand::with_name("w")
                .about("print items for the coming week")
        )
        .subcommand(
            SubCommand::with_name("m")
                .about("print items for the comming month")
        )
        .subcommand(
            SubCommand::with_name("y")
                .about("print items for the coming year")
        )
        .get_matches();

    matches
}

fn expect<Data, Error>(r: Result<Data, Error>, message: &str) -> Data
    where Error: Debug + Display
{
    if let Err(err) = r {
        eprintln!("{}: {}", message, err);
        process::exit(-1);
    }
    r.unwrap()
}

fn expect_option<Data>(r: Option<Data>, message: &str) -> Data
{
    if r.is_none() {
        eprintln!("{}", message);
        process::exit(-1);
    }
    r.unwrap()
}

fn prompt(message: &str) -> Option<String> {
    println!("{}", message);
    let mut buffer = String::new();
    let stdin = io::stdin();
    match stdin.read_line(&mut buffer) {
        Ok(_) => Some(buffer.trim_end().to_string()),
        _ => None
    }
}

fn system(v: Vec<&str>) -> bool {
    let s = v.join(" ");
    let v2: Vec<&str> = s.split(" ").collect();
    assert!(v2.len() > 0);
    let cmd = v2[0];
    let args: Vec<&str> = v2.iter().skip(1).map(|s| *s).collect();
    // println!("cmd is {}, args is {:?}", cmd, args);
    let status = process::Command::new(cmd)
        .args(args)
        .status()
        .expect("Failed to execute editor");
    status.success()
}

// home_subdir creates path string of a directory that is under current
// user's home directory, e.g.
// home_subdir(vec![".when".to_string(), "preferences".to_string()]) returns
// string "/home/username/.when/preferences".
fn home_subdir(l: Vec<String>) -> String {
    let home_dir: String;

    home_dir = expect(env::var("HOME"), "HOME unknown");

    let path = path::Path::new(&home_dir);
    let mut path_buf = path.to_path_buf();
    for s in l {
        path_buf = path_buf.join(s);
    }
    let path = path_buf
                   .to_str()
                   .unwrap()
                   .to_string();
    return path;
}

fn initialize(_preferences: &str) {
    if !atty::is(Stream::Stdout) || !atty::is(Stream::Stdin) {
        eprintln!("Not in interactive mode!");
        process::exit(-1);
    }

    // println!("Initializing");
    let a1 = prompt(r#"
You can now set up your calendar. This involves creating a directory ~/.when, and making
a couple of files in it. If you want to do this, type y and hit return."#).unwrap();
    println!("a1 is {}", a1);
    if a1 != "y" {
        process::exit(0);
    }
    // println!("You said yes");
    let mut editor = prompt(r#"
You can edit your calendar file using your favorite editor. Please enter the command you
want to use to run your editor, or hit return to accept this default:
  emacs -nw"#).unwrap();
     if editor == "" {
         editor = "emacs -nw".to_string();
     }

    let when_rs_path = home_subdir(vec![
        ".when-rs".to_string()
    ]);
    let preferences_path = home_subdir(vec![
        ".when-rs".to_string(),
        "preferences".to_string()
    ]);
    let calendar_path = home_subdir(vec![
        ".when-rs".to_string(),
        "calendar".to_string()
    ]);
    println!("preferences_path is {}", preferences_path);
    println!("editor is {}", editor);
    if let Err(error) = fs::create_dir(&when_rs_path) {
        eprintln!("Error creating directory {}: {}", when_rs_path, error);
        process::exit(-1);
    }
    let file = File::create(&preferences_path);
    if let Err(error) = file {
        eprintln!("Error creating file {}: {}", preferences_path, error);
        process::exit(-1);
    }
    let mut file = file.unwrap();
    expect(writeln!(&mut file, "calendar = {}", calendar_path),
        &format!("Writing to file {}", preferences_path));
    expect(writeln!(&mut file, "editor = {}", editor),
        &format!("Writing to file {}", preferences_path));

    let calendar_file = File::create(&calendar_path);
    if let Err(error) = calendar_file {
        eprintln!("@#$% Error creating file {}: {}", calendar_path, error);
        process::exit(-1);
    }

    // process::exit(0);
    println!(r#"
You can now add items to your calendar file. Do ``when-rs --help'' for more information.
    "#);
}

fn main() {
    let preferences_path = home_subdir(vec![
        ".when-rs".to_string(),
        "preferences".to_string()
    ]);
    println!("preferences_path is {}", preferences_path);

    let it = std::fs::read_to_string(&preferences_path);
    let preferences: String;
    if it.is_err() {
        initialize(&preferences_path);
        preferences = std::fs::read_to_string(&preferences_path).unwrap();
        
    } else {
        preferences = it.unwrap();
    }

    // Read preferences from preferences file.
    let hashmap_preferences = preferences::parse_lines(preferences.lines());

//    let _calendar = expect_option(
//        hashmap_preferences.get("calendar"), "Configuration doesn't define calendar");

    // Parse command line arguments.
    let matches = get_arg();

    let mut arg_future: i32 = 14;
    let mut arg_past: i32 = -1;

    if let Some(n) = matches.value_of("future") {
        arg_future = expect(n.parse::<i32>(), matches.usage());
    }

    if let Some(n) = matches.value_of("past") {
        arg_past = expect(n.parse::<i32>(), matches.usage());
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

    if matches.is_present("e") {
        if let Some(editor) = hashmap_preferences.get("editor") {
            // println!("Invoking editor {}", editor);
            let command_arg = format!("{}", calendar.to_str().unwrap());
            let cmd_str = [&editor[..], &command_arg[..]].join(" ");
            let v = cmd_str.split(" ");
            if system(v.collect()) {
                process::exit(0);
            } else {
                eprintln!("Invoking editor failed");
                process::exit(-1);
            }
        }
    }

    let arg_past: i64 = arg_past.into();
    let mut arg_future: i64 = arg_future.into();

    if matches.is_present("y") {
        arg_future = 366;
    } else if matches.is_present("m") {
        arg_future = 31;
    } else if matches.is_present("w") {
        arg_future = 7;
    }

    let mut header: bool = true;

    if matches.is_present("noheader") {
        header = false;
    }

    let today = Local::today().naive_local();
    let yesterday = today.pred();
    let tomorrow = today.succ();
    let date1 = today - Duration::days(arg_past);
    let date2 = today + Duration::days(arg_future);

    // eprintln!("calendar file is {:?}", calendar);
    let file = File::open(calendar);
    if let Err(err) = file {
        eprintln!("Failure opening {}: {}", calendar.to_str().unwrap(), err);
        process::exit(-1);
    }
    let file = file.unwrap();
    let reader = BufReader::new(file);

    if (header) {
        let now = Local::now();
        println!("{} {}\n", today.format("%a %Y %b %e"), now.format("%R"));
    }

    for line in reader.lines() {
        if let Ok(line_str) = line {
            // eprintln!("Line: {}", line_str);
            if let Some((expr, descr)) = utils::parse_calendar_line(&line_str) {
                // eprintln!(" -- expression: {}", expr);
                // eprintln!(" -- description: {}", descr);
                if let Ok(checker) = datecalc::DateChecker::new(&expr) {
                    if let Some(date) = checker.check_date_range(&date1, &date2) {
                        if date == today {
                            println!("today      {} {}", date.format("%Y %b %e"), descr);
                        } else if date == yesterday {
                            println!("yesterday  {} {}", date.format("%Y %b %e"), descr);
                        } else if date == tomorrow {
                            println!("tomorrow   {} {}", date.format("%Y %b %e"), descr);
                        } else {
                            println!("           {} {}", date.format("%Y %b %e"), descr);
                        }
                    }
                }
            }
        }
    }
}
