use std::env;
use std::process::exit;

// These lines DECLARE the modules of my app
// (the actual module impl are either in NAME.rs or NAME/mod.rs)
mod common;
mod day01;
mod day02;

type DayFn = fn() -> ();

// We use a slice of the array to not have to specify its size in the type.
// See: https://stackoverflow.com/questions/23810032/how-to-specify-const-array-in-global-scope-in-rust
static DAYS: &[(&str, DayFn)] = &[
    // using a vec to keep correct order
    ("day01", day01::solve),
    ("day02", day02::solve),
];

fn print_usage() {
    let prog_name = env::args().next().unwrap_or("prog".to_string());
    let day_names: Vec<_> = DAYS.iter().map(|e| e.0).collect();
    println!("Usage: {} <day>", prog_name);
    println!(
        "Where <day> is either 'all' or one of: {}",
        day_names.join(", ")
    );
    exit(1);
}

fn main() {
    let prog_args: Vec<String> = env::args().collect();
    let first_arg = prog_args.get(1);
    // Converts Option<String> to Option<&str> so I can match on Some("all")
    // (necessary because matching on Some("all".to_string()) does not work).
    match first_arg.and_then(|s| Some(s.as_str())) {
        Some("all") => {
            for day in DAYS {
                println!("--- {}", day.0);
                day.1();
            }
        }
        Some(wanted_day) => {
            let matching_day = DAYS.iter().find(|day| day.0 == wanted_day);
            match matching_day {
                Some(day) => day.1(),
                None => {
                    println!("Unknown day '{}'", wanted_day);
                    exit(1);
                }
            };
        }
        None => print_usage(),
    };
}
