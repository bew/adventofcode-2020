use crate::common;

#[derive(Debug)]
struct Policy {
    num1: usize,
    num2: usize,
    letter: char,
}

fn parse_input_line(line: &str) -> (Policy, &str) {
    // Line format is: 'num1-num2 letter: password'
    // NOTE: We assume the format is correct, we panic otherwise.

    fn split_in_2(text: &str, sep: char) -> (&str, &str) {
        let mut parts = text.splitn(2, sep);
        (parts.next().unwrap(), parts.next().unwrap())
    }

    let (policy_part, password_part) = split_in_2(line, ':');

    // extract useful info from 'policy_part'
    let (numbers, letter_part) = split_in_2(policy_part, ' ');
    let (min_str, max_str) = split_in_2(numbers, '-');

    (
        Policy {
            num1: min_str.parse().unwrap(), // convert to usize
            num2: max_str.parse().unwrap(), // convert to usize
            letter: letter_part.chars().next().unwrap(), // extract the first char
        },
        password_part.trim(), // trim whitespace before/after
    )
}

fn parse_input(input_path: &str) -> Vec<(Policy, String)> {
    // FIXME: is there a simpler way to write this?

    // This binds the lifetime of the read lines to the function, so they can be referenced
    // until the end of the function.
    let lines = common::read_lines(input_path);
    // Parse the lines to structured data types
    let parsed_lines: Vec<(Policy, &str)> = lines
        .iter()
        .map(|line| parse_input_line(line))
        .collect();
    // Ensure we return a new string for the password, to avoid returning a
    // reference to a local variable (the line strings) which doesn't compile.
    //
    // We use 'into_iter' to iterate over T instead of over &T.
    // Meaning that 'parsed_lines' is not usable after this loop,
    // it is 'consumed' by the iteration.
    parsed_lines
        .into_iter()
        .map(|(policy, password)| (policy, password.to_string()))
        .collect()
}

fn part1_password_has_letter_count(password: &str, policy: &Policy) -> bool {
    // For part1, num1 & num2 are the minimum & maximum number of the given letter
    let (min_count, max_count) = (policy.num1, policy.num2);
    let mandatory_letter_count = password.matches(policy.letter).count();
    (min_count..=max_count).contains(&mandatory_letter_count)
}

fn part2_password_has_letter_once_at_pos(password: &str, policy: &Policy) -> bool {
    // For part2, num1 & num2 are letter position (1-indexed)
    let (pos1, pos2) = (policy.num1, policy.num2);
    let passwd_chars: Vec<_> = password.chars().collect();
    (passwd_chars[pos1 - 1] == policy.letter) ^ (passwd_chars[pos2 - 1] == policy.letter)
}

pub fn solve() {
    let inputs = parse_input("./inputs/day02.txt");

    // part1
    let number_valid_passwords = inputs
        .iter()
        .filter(|(policy, password)| part1_password_has_letter_count(password, policy))
        .count();
    println!("Day02 Part1: {}", number_valid_passwords);

    // part2
    let number_valid_passwords = inputs
        .iter()
        .filter(|(policy, password)| part2_password_has_letter_once_at_pos(password, policy))
        .count();
    println!("Day02 Part2: {}", number_valid_passwords);
}
