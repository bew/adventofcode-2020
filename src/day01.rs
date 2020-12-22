// Here we are in the module 'day01'.
// This line says that we'll use the 'common' module (declared in main.rs)
use crate::common::{self, ErrWithContext, MyResult};

// We accept a ref of slice of i32 instead of a &Vec<i32> for flexibility.
fn find_sum2_to_2020(numbers: &[i32]) -> Option<(i32, i32)> {
    // For loops will call .into_iter() on the the obj passed, so we don't
    // have to write `for x in numbers.into_iter()`.
    // NOTE: .into_iter() consumes the self. But not a pb when self
    //       is a reference!
    //
    // The variable part for the iteration is a rust pattern, here we know
    // that `numbers` is a reference, so the variable would be a reference.
    // Since we want to return it, and the're just numbers anyway, we ask for
    // the value (not the reference) for the current iteration.
    // See: https://users.rust-lang.org/t/reference-syntax-in-for-loop-iterator/597
    for &num1 in numbers {
        for &num2 in numbers {
            if num1 + num2 == 2020 {
                return Some((num1, num2));
            }
        }
    }
    None
}

fn find_sum3_to_2020(numbers: &[i32]) -> Option<(i32, i32, i32)> {
    for &num1 in numbers {
        for &num2 in numbers {
            for &num3 in numbers {
                if num1 + num2 + num3 == 2020 {
                    return Some((num1, num2, num3));
                }
            }
        }
    }
    None
}

fn parse_input(input_path: &str) -> MyResult<Vec<i32>> {
    common::read_lines(input_path)?
        .iter()
        .map(|line| {
            i32::from_str_radix(line, 10).with_context(|| format!("Invalid number '{}'", line))
        })
        .collect()
    // Each entry after that .map call will have type MyResult<i32>, because the
    // from_str_radix call may fail!
    //
    // In Rust world, it is known that a sequence of Result can be collected
    // to a Result with a Vec of the valid results.
    //
    // This is because Result implements FromIterator
    // (see https://doc.rust-lang.org/std/result/enum.Result.html#impl-FromIterator%3CResult%3CA%2C%20E%3E%3E)
    // NOTE: I didn't deduced that, many posts around the internet talk about that!
}

pub fn solve_part1(input_path: &str) -> MyResult<usize> {
    let input_numbers: Vec<i32> = parse_input(input_path).context("Failed to load input")?;

    let (num1, num2) =
        find_sum2_to_2020(&input_numbers).context("cannot find numbers for part1")?;
    // dbg!((num1, num2));
    Ok((num1 * num2) as usize)
}

pub fn solve_part2(input_path: &str) -> MyResult<usize> {
    let input_numbers: Vec<i32> = parse_input(input_path).context("Failed to load input")?;

    let (num1, num2, num3) = find_sum3_to_2020(&input_numbers).expect("part2 failed");
    // dbg!((num1, num2, num3));
    let part2 = num1 * num2 * num3;
    Ok(part2 as usize)
}
