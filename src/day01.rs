// Here we are in the module 'day01'.
// This line says that we'll use the 'common' module (declared in main.rs)
use crate::common::{self, MyResult, ErrWithContext};

fn find_sum2_to_2020(numbers: &Vec<i32>) -> Option<(i32, i32)> {
    for num1 in numbers.iter() {
        for num2 in numbers.iter() {
            if num1 + num2 == 2020 {
                return Some((*num1, *num2))
            }
        }
    }
    None
}

fn find_sum3_to_2020(numbers: &Vec<i32>) -> Option<(i32, i32, i32)> {
    for num1 in numbers.iter() {
        for num2 in numbers.iter() {
            for num3 in numbers.iter() {
                if num1 + num2 + num3 == 2020 {
                    return Some((*num1, *num2, *num3))
                }
            }
        }
    }
    None
}

fn parse_input(input_path: &str) -> MyResult<Vec<i32>> {
    let mut numbers = vec![];
    for line in common::read_lines(input_path)? {
        numbers.push(
            i32::from_str_radix(line.as_ref(), 10)
                .context(format!("Invalid number '{}'", line))?
        );
    }
    Ok(numbers)

    // common::read_lines(input_path)
    //     .iter()
    //     .map(|line| { i32::from_str_radix(line, 10).expect("Unable to convert line to i32") })
    //     .collect();
}

pub fn solve_part1(input_path: &str) -> MyResult<usize> {
    let input_numbers: Vec<i32> = parse_input(input_path).context("Failed to load input")?;

    let (num1, num2) = find_sum2_to_2020(&input_numbers).expect("part1 failed");
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
