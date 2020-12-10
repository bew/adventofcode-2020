use std::env;
use std::process::exit;

// These lines DECLARE the modules of my app
// (the actual module impl are either in NAME.rs or NAME/mod.rs)
mod common;
mod day01;

fn main() {
    let mut day_funcs = vec![]; // vec instead of map to keep order
    day_funcs.push(("day01", day01::solve));

    let mut prog_args = env::args();
    prog_args.next(); // skip program' path
    match prog_args.next() {
        Some(is_all) if is_all == "all" => {
            for day_entry in day_funcs {
                println!("--- {}", day_entry.0);
                day_entry.1();
            }
        },
        Some(wanted_day) => {
            let matching_day = day_funcs.iter()
                .find(|day_entry| { day_entry.0 == wanted_day });
            match matching_day {
                Some(day_entry) => day_entry.1(),
                None => {
                    println!("Unknown day '{}'", wanted_day);
                    exit(1);
                }
            };
        },
        None => {
            println!("Usage: prog <day>");
            println!(
                "Where <day> is either 'all' or one of: {}",
                day_funcs.iter()
                    .map(|e| { e.0 }).collect::<Vec<&str>>()
                    .join(", ")
            );
            exit(1);
        }
    };
}
