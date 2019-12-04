//! --- Day 4: Secure Container ---

use std::ops;

/// You arrive at the Venus fuel depot only to discover it's protected by a password. The Elves had written the password on a sticky note, but someone threw it out.
///
/// However, they do remember a few key facts about the password:
///
///     It is a six-digit number.
///     The value is within the range given in your puzzle input.
///     Two adjacent digits are the same (like 22 in 122345).
///     Going from left to right, the digits never decrease; they only ever increase or stay the same (like 111123 or 135679).
///
/// Other than the range rule, the following are true:
///
///     111111 meets these criteria (double 11, never decreases).
///     223450 does not meet these criteria (decreasing pair of digits 50).
///     123789 does not meet these criteria (no double).
///
/// How many different passwords within the range given in your puzzle input meet these criteria?
pub fn part1() {
    let input = crate::common::read_stdin_to_string();
    let range = ops::RangeInclusive::<i64>::from_str(input.as_str());

    let number_of_passwords = range
        .map(|integer| integer.to_string())
        .filter(|password| test_password_against_facts(password.as_str()))
        .count();

    println!(
        "The number of different passwords within the range given which meet these criteria: {}",
        number_of_passwords
    );
}

fn test_password_against_facts(password: &str) -> bool {
    if password.len() != 6 {
        return false;
    }

    let chars: Vec<char> = password.chars().collect();

    if !chars.windows(2).any(|window| window[0] == window[1]) {
        return false;
    }

    if chars.windows(2).any(|window| window[0] > window[1]) {
        return false;
    }

    true
}

trait FromStr {
    fn from_str(string: &str) -> Self;
}

impl FromStr for ops::RangeInclusive<i64> {
    fn from_str(string: &str) -> Self {
        let mut iter = string
            .trim()
            .split("-")
            .map(|integer| integer.parse())
            .map(|parse_result| parse_result.expect("Failed to parse range integer as i64"));

        let start = iter.next().expect("Failed to parse range start");
        let end = iter.next().expect("Failed to parse range end");

        start..=end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_factchecker() {
        let examples = [("112233", true), ("223450", false), ("123789", false)];

        for example in &examples {
            assert_eq!(test_password_against_facts(example.0), example.1);
        }
    }
}
