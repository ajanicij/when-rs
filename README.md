when-rs is a rewrite of Perl utility [when](http://www.lightandmatter.com/when/when.html).

Please see the when utility's page to get a good idea of its functionality.

when-rs is a work in progress, and at the moment it only implements a subset of when's
functionality. It is also my first Rust project, so there are certainly things that
can be made better/simple/more idiomatic. Any comments, suggestions and PRs are welcome.

## Usage

when-rs --help with display this:

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
