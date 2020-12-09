mod common;

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

fn main() {
    let input_numbers: Vec<i32> = common::read_lines("./inputs/day01.txt")
        .iter()
        .map(|line| { i32::from_str_radix(line, 10).expect("Unable to convert line to i32") })
        .collect();

    let (num1, num2) = find_sum2_to_2020(&input_numbers).expect("part1 failed");
    println!("Day01 Part1: ({}, {}) - result: {}", num1, num2, num1 * num2);

    let (num1, num2, num3) = find_sum3_to_2020(&input_numbers).expect("part2 failed");
    println!("Day01 Part2: ({}, {}, {}) - result: {}", num1, num2, num3, num1 * num2 * num3);

}
