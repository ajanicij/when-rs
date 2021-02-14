when-rs is a rewrite of Perl utility [when](http://www.lightandmatter.com/when/when.html).

Please see the when utility's page to get a good idea of its functionality.

when-rs is a work in progress, and at the moment it only implements a subset of when's
functionality. It is also my first Rust project, so there are certainly things that
can be made better/simple/more idiomatic. Any comments, suggestions and PRs are welcome.

## Usage

when-rs --help will display this:

```
USAGE:
    when-rs [FLAGS] [OPTIONS] [SUBCOMMAND]

FLAGS:
    -h, --help        Prints help information
        --header      Print headers at the top of the output
        --noheader    Don't print headers at the top of the output
    -V, --version     Prints version information

OPTIONS:
        --calendar <calendar>    Your calendar file. The default is to use the
                                 file pointed to by your preferences file, which is
                                 set up the first time you run when-rs.
        --future <future>        How many days into the future the report extends. [default: 14]
        --now <now>              Pretend today is some other date.
                                 For example, --now="2022 Jan 1" pretends that today is 2022 January 1.
        --past <past>            How many days into the past the report extends.
                                 Like the --future option, --past is interpreted as an offset
                                 relative to the present date, so normally you would want
                                 this to be a negative value. Default: -1 [default: -1]

SUBCOMMANDS:
    e       runs editor for editing calendar file
    help    Prints this message or the help of the given subcommand(s)
    m       print items for the comming month
    w       print items for the coming week
    y       print items for the coming year
```

## calendar file

Calendar file consists of lines with the following structure:

    date-pattern, description

For example:

    2021 July 23, Tokyo 2020 opening day

The date pattern has the form

    year month day

Where:

- year is year number (e.g. 2021) or * for "any year".
- month is month name: For January, set this to Ja, Jan, jan, etc. Any prefix
  of a month name is accepted, as long as it is unique: j is not accepted,
  because it can stand for January, June or July; ju could be June or July.
- day is the day of the month or * for "any day".

Any line starting with # is treated as a comment and ignored.

## Initialization

When you run when-rs for the first time, it asks basic questions and creates
directory .when-rs in your home directory and files preferences and calendar
in directory .when-rs.

Then run when-rs to either edit calendar file or process it.

## Simple usage

The simplest usage:

- when-rs
  - Without any parameters, it processes calendar and displays any matches
    for range of days between today and 14 days from today.
- when-rs e
  - Edits calendar

## Date patterns

Date patterns in the calendar have the following rules:

- \<year\> \<month\> \<day\> - self explanatory
  - Example: 2021 Mach 8
  - Month can be the full month, like March, or a prefix, like Mar. Any case is
    accepted, for example, mar, Mar and MAR are all fine. It can't be ambiguous,
    for example, jun is fine, but ju is not because it can mean June or July.
  - Any component can be a '*', which matches any value, so for example
    2021 * 1 is the first day of any month in 2021.
- \<simple-expresion\> & \<simple-expression\> & ... - Logical conjunction of simple
  expressions, where a simple expression can be one of:
  - y=\<year\> - for example, y=2022
  - d=\<day\> - day of the month, for example d=14 is the 14th of the month
  - m=\<month\> - month, for example m=May is May
  - w=\<day\> - day of the week, for example w=1 is Monday
  - a=\<week\> - week of the month, for example a=1 is the first 7 days of the month,
               a=2 is the next 7 days etc.
  - z=\<day\> - day of the year, for example z=1 is January 1

Some examples:

```
* January 1, New Year's Day
2021 July 23, Tokyo Olympic Games 2020 opening day
* Feb 14, Saint Valentine's Day
w=5, TGIF Yay!
```
