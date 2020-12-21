use std::env;
use std::process::exit;

// These lines DECLARE the modules of my app
// (the actual module impl are either in NAME.rs or NAME/mod.rs)
mod common;
use common::MyResult;
mod day01;
mod day02;
mod day03;

type PartFn = fn(&str) -> MyResult<usize>;

// We define a lifetime in 'Day', to be able to store references in the struct.
struct Day<'a> {
    name: &'a str,
    part1: PartFn,
    part2: PartFn,
    default_input: &'a str,
}

// NOTE: We use a slice of the array to not have to specify its size in the type.
//       See: https://stackoverflow.com/questions/23810032/how-to-specify-const-array-in-global-scope-in-rust
//
// NOTE: I tried to create a `Day::new` function to simplify creation of `Day` but the compiler
//       yelled at me about the fact that only constant things can be called for a static
//       variable, and if I change the constructor to be a const function, it tells me that
//       passing functions (like `day01::solve`) to a const function is unstable and not well
//       supported.. So struct constructor it is! :D
static DAYS: &[Day] = &[
    // using a vec to keep correct order
    Day {
        name: "day01",
        part1: day01::solve_part1,
        part2: day01::solve_part2,
        default_input: "./inputs/day01.txt",
    },
    Day {
        name: "day02",
        part1: day02::solve_part1,
        part2: day02::solve_part2,
        default_input: "./inputs/day02.txt",
    },
    Day {
        name: "day03",
        part1: day03::solve_part1,
        part2: day03::solve_part2,
        default_input: "./inputs/day03.txt",
    },
];

fn print_usage() {
    let prog_name = env::args().next().unwrap_or("prog".to_string());
    let day_names: Vec<_> = DAYS.iter().map(|d| d.name).collect();
    println!("Usage: {} <day> [<custom_input_path>]", prog_name);
    println!(
        "Where <day> is either 'all' or one of: {}",
        day_names.join(", ")
    );
    exit(1);
}

fn run_part(name: &str, part_func: PartFn, input: &str) {
    let part_result = (part_func)(input);
    match part_result {
        Ok(result) => println!("{}: {}", name, result),
        Err(err) => eprintln!("Error: {}", err),
    }
}

fn run_day(day: &Day, input: &str) {
    println!("--- {}", day.name);
    run_part("Part1", day.part1, input);
    run_part("Part2", day.part2, input);
}

fn main() {
    let prog_args: Vec<String> = env::args().collect();
    let first_arg = prog_args.get(1);
    // Converts Option<String> to Option<&str> so I can match on `Some("all")`
    // (necessary because matching on `Some("all".to_string())` does not work
    // and matching on `Some(xyz) if xyz == "all"` is ugly..).
    match first_arg.and_then(|s| Some(s.as_str())) {
        Some("all") => {
            for day in DAYS {
                run_day(day, day.default_input);
            }
        }
        Some(wanted_day) => {
            let matching_day = DAYS.iter().find(|day| day.name == wanted_day);
            match matching_day {
                Some(day) => {
                    let input_path = match prog_args.get(2) {
                        Some(input_path) => input_path,
                        None => day.default_input,
                    };
                    run_day(day, input_path);
                }
                None => {
                    println!("Unknown day '{}'", wanted_day);
                    exit(1);
                }
            };
        }
        None => print_usage(),
    };
}
