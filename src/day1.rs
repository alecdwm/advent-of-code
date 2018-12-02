//! After feeling like you've been falling for a few minutes, you look at the device's tiny screen. "Error: Device must be calibrated before first use. Frequency drift detected. Cannot maintain destination lock." Below the message, the device shows a sequence of changes in frequency (your puzzle input). A value like +6 means the current frequency increases by 6; a value like -3 means the current frequency decreases by 3.
//!
//! For example, if the device displays frequency changes of +1, -2, +3, +1, then starting from a frequency of zero, the following changes would occur:
//!
//!     Current frequency  0, change of +1; resulting frequency  1.
//!     Current frequency  1, change of -2; resulting frequency -1.
//!     Current frequency -1, change of +3; resulting frequency  2.
//!     Current frequency  2, change of +1; resulting frequency  3.
//!
//! In this example, the resulting frequency is 3.
//!
//! Here are other example situations:
//!
//!     +1, +1, +1 results in  3
//!     +1, +1, -2 results in  0
//!     -1, -2, -3 results in -6
//!
//! Starting with a frequency of zero, what is the resulting frequency after all of the changes in frequency have been applied?

use std::io;

#[derive(Debug)]
struct FrequencyChange {
    operation: FrequencyOperation,
    magnitude: i64,
}

#[derive(Debug)]
enum FrequencyOperation {
    Add,
    Subtract,
}

pub fn day1() {
    let mut input = String::new();

    {
        let mut line = String::new();

        println!("enter puzzle input followed by an empty line:");
        loop {
            io::stdin().read_line(&mut line).unwrap();
            if line.trim() == "" {
                break;
            }
            input.push_str(&line);
            line.clear();
        }
    }

    let mut frequency: i64 = 0;
    let mut changes = Vec::new();

    for line in input.lines() {
        changes.push(FrequencyChange {
            operation: match line.chars().next() {
                Some('+') => FrequencyOperation::Add,
                Some('-') => FrequencyOperation::Subtract,
                Some(not_found) => panic!("Unhandled operation: '{}'", not_found),
                None => panic!(),
            },
            magnitude: line
                .trim_start_matches(|c| c == '+' || c == '-')
                .parse()
                .expect(&format!("parsing frequency change magnitude '{}'", line)),
        })
    }

    for change in changes.iter() {
        frequency = match change.operation {
            FrequencyOperation::Add => frequency + change.magnitude,
            FrequencyOperation::Subtract => frequency - change.magnitude,
        }
    }

    println!("the resulting frequency: {}", frequency);
}
