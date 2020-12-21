use crate::common::{self, MyResult, ErrWithContext};

#[derive(Debug)]
struct Policy {
    num1: usize,
    num2: usize,
    letter: char,
}

fn parse_input_line(line: &str) -> MyResult<(Policy, &str)> {
    // Line format is: 'num1-num2 letter: password'
    // NOTE: We return error with context if the line has incorrect format.

    fn split_in_2(text: &str, sep: char) -> MyResult<(&str, &str)> {
        let mut parts = text.splitn(2, sep);
        Ok((
            parts.next().context("Missing first part of split")?,
            parts.next().context("Missing second part of split")?,
        ))
    }

    let (policy_part, password_part) = split_in_2(line, ':')
        .context("Failed to split line to policy & password")?;

    // extract useful info from 'policy_part'
    let (numbers, letter_part) = split_in_2(policy_part, ' ')
        .context("Failed to split numbers and letter in policy")?;
    let (num1_str, num2_str) = split_in_2(numbers, '-')
        .context("Failed to split num1 and num2")?;

    Ok((
        Policy {
            num1: num1_str.parse().context("Failed to parse num1 as a number")?, // convert to usize
            num2: num2_str.parse().context("Failed to parse num2 as a number")?, // convert to usize
            letter: letter_part.chars().next().context("letter part is empty")?, // extract the first char
        },
        password_part.trim(), // trim whitespace before/after
    ))
}

fn parse_input(input_path: &str) -> MyResult<Vec<(Policy, String)>> {
    // FIXME: is there a simpler way to write this?

    // This binds the lifetime of the read lines to the function, so they can be referenced
    // until the end of the function.
    let lines = common::read_lines(input_path)?;
    // Parse the lines to structured data types
    let mut parsed_lines = vec![];
    for (index0, line) in lines.iter().enumerate() {
        let (policy, password) = parse_input_line(line)
            .context(format!("Invalid line {}", index0 + 1))?;
        // Ensure we return a new string for the password, to avoid returning a
        // reference to a local variable (the line strings) which doesn't compile.
        parsed_lines.push((policy, password.to_string()));
    }
    Ok(parsed_lines)
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

pub fn solve_part1(input_path: &str) -> MyResult<usize> {
    let inputs = parse_input(input_path).context("Failed to load input")?;
    let number_valid_passwords = inputs
        .iter()
        .filter(|(policy, password)| part1_password_has_letter_count(password, policy))
        .count();
    Ok(number_valid_passwords)
}

pub fn solve_part2(input_path: &str) -> MyResult<usize> {
    let inputs = parse_input(input_path).context("Failed to load input")?;
    let number_valid_passwords = inputs
        .iter()
        .filter(|(policy, password)| part2_password_has_letter_once_at_pos(password, policy))
        .count();
    Ok(number_valid_passwords)
}
